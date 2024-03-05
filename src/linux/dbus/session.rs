use std::{
  sync::mpsc,
  thread::{self, JoinHandle},
  time::Duration,
};

use dbus::{
  blocking::{
    stdintf::org_freedesktop_dbus::{ReleaseNameReply, RequestNameReply},
    Connection,
  },
  channel::Sender,
  Message,
};
use dbus_crossroads::Crossroads;

pub struct DBusSession {
  dbus_connection_handle: JoinHandle<()>,
  register_name: mpsc::Sender<(String, Crossroads, oneshot::Sender<bool>)>,
  unregister_name: mpsc::Sender<(String, oneshot::Sender<bool>)>,
  emit_message: mpsc::Sender<Message>,
}

impl DBusSession {
  pub fn new() -> Self {
    let (register_name_sender, register_name_receiver) =
      mpsc::channel::<(String, Crossroads, oneshot::Sender<bool>)>();
    let (unregister_name_sender, unregister_name_receiver) =
      mpsc::channel::<(String, oneshot::Sender<bool>)>();
    let (emit_message_sender, emit_message_receiver) = mpsc::channel::<Message>();

    let dbus_connection_handle = thread::spawn(move || {
      let mut media_player: Option<Crossroads> = None;
      loop {
        let connection_result = Connection::new_session();
        if let Ok(connection) = connection_result {
          loop {
            match register_name_receiver.try_recv() {
              Ok((name, crossroads, response)) => {
                if let Ok(request_name_reply) = connection.request_name(&name, false, true, true) {
                  if request_name_reply == RequestNameReply::PrimaryOwner {
                    media_player = Some(crossroads);
                    let _ = response.send(true);
                  } else {
                    let _ = response.send(false);
                  }
                } else {
                  let _ = response.send(false);
                }
                // }
              }
              _ => {}
            }
            match unregister_name_receiver.try_recv() {
              Ok((name, response)) => {
                if media_player.is_some() {
                  media_player = None;
                  if let Ok(release_name_reply) = connection.release_name(&name) {
                    if release_name_reply == ReleaseNameReply::Released {
                      let _ = response.send(true);
                    } else {
                      let _ = response.send(false);
                    }
                  } else {
                    let _ = response.send(false);
                  }
                }
              }
              _ => {}
            }
            match emit_message_receiver.try_recv() {
              Ok(message) => {
                let _ = connection.send(message);
              }
              _ => {}
            }
            let _ = connection
              .channel()
              .read_write(Some(Duration::from_secs(0)));
            loop {
              if let Some(message) = connection.channel().pop_message() {
                if let Some(crossroads) = media_player.as_mut() {
                  let _ = crossroads.handle_message(message, &connection);
                }
              } else {
                break;
              }
            }
          }
        }
      }
    });

    Self {
      dbus_connection_handle,
      register_name: register_name_sender,
      unregister_name: unregister_name_sender,
      emit_message: emit_message_sender,
    }
  }

  pub fn register(&self, name: &String, crossroads: Crossroads) -> bool {
    let name = format!("org.mpris.MediaPlayer2.{}", &name);
    let (response_sender, response_receiver) = oneshot::channel();
    let _ = self
      .register_name
      .send((name.to_owned(), crossroads, response_sender));
    match response_receiver.recv_timeout(Duration::from_secs(1)) {
      Ok(result) => result,
      _ => false,
    }
  }

  pub fn unregister(&self, name: &String) -> bool {
    let name = format!("org.mpris.MediaPlayer2.{}", &name);
    let (response_sender, response_receiver) = oneshot::channel();
    let _ = self
      .unregister_name
      .send((name.to_owned(), response_sender));
    match response_receiver.recv_timeout(Duration::from_secs(1)) {
      Ok(result) => result,
      _ => false,
    }
  }

  pub fn emit_message(&self, message: Message) {
    let _ = self.emit_message.send(message);
  }
}

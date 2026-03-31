use std::{
  time::Duration,
};
use dbus::channel::{MatchingReceiver, Sender, Token};
use dbus::Message;
use dbus::message::MatchRule;
use dbus::nonblock::stdintf::org_freedesktop_dbus::{ReleaseNameReply, RequestNameReply};
use dbus_crossroads::Crossroads;
use dbus_tokio::connection;
use tokio::select;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;
use crate::linux::TOKIO_HANDLE;

pub struct DBusSession {
  dbus_connection_handle: JoinHandle<()>,
  register_name: mpsc::UnboundedSender<(String, Crossroads, oneshot::Sender<bool>)>,
  unregister_name: mpsc::UnboundedSender<(String, oneshot::Sender<bool>)>,
  emit_message: mpsc::UnboundedSender<Message>,
}

impl DBusSession {
  pub fn new() -> napi::Result<Self> {
    let (register_name_sender, mut register_name_receiver) =
      mpsc::unbounded_channel::<(String, Crossroads, oneshot::Sender<bool>)>();
    let (unregister_name_sender, mut unregister_name_receiver) =
      mpsc::unbounded_channel::<(String, oneshot::Sender<bool>)>();
    let (emit_message_sender, mut emit_message_receiver) = mpsc::unbounded_channel::<Message>();
    let (resource, conn) = connection::new_session_sync().map_err(|err| {
      napi::Error::from_reason(err.message().unwrap_or("Failed to create DBus session"))
    })?;

    let handle = TOKIO_HANDLE.get().expect("Tokio runtime not initialized");
    let _guard = handle.enter();

    let dbus_connection_handle = tokio::spawn(async move {
      let mut receive_token: Option<Token> = None;

      loop {
        select! {
          val = register_name_receiver.recv() => {
            if let Some((name, mut crossroads, response)) = val {
              if let Ok(request_name_reply) = conn.request_name(&name, false, true, true).await {
                if request_name_reply == RequestNameReply::PrimaryOwner {
                  receive_token = Some(conn.start_receive(MatchRule::new_method_call(), Box::new(move |msg, conn| {
                    crossroads.handle_message(msg, conn).unwrap();
                    true
                  })));
                  let _ = response.send(true);
                } else {
                  let _ = response.send(false);
                }
              } else {
                let _ = response.send(false);
              }
            }
          }
          val = unregister_name_receiver.recv() => {
            if let Some((name, response)) = val {
              if let Ok(release_name_reply) = conn.release_name(&name).await {
                if release_name_reply == ReleaseNameReply::Released {
                  if let Some(token) = receive_token {
                    conn.stop_receive(token);
                  }
                  let _ = response.send(true);
                } else {
                  let _ = response.send(false);
                }
              } else {
                let _ = response.send(false);
              }
            }
          }
          val = emit_message_receiver.recv() => {
            if let Some(message) = val {
              let _ = conn.send(message);
            }
          }
        }
      }
    });

    let dbus_connection_handle = tokio::spawn(async {
      let err = resource.await;
      // Lost connection to dbus
    });

    Ok(Self {
      dbus_connection_handle,
      register_name: register_name_sender,
      unregister_name: unregister_name_sender,
      emit_message: emit_message_sender,
    })
  }

  pub fn register(&self, name: &String, crossroads: Crossroads) -> bool {
    let name = format!("org.mpris.MediaPlayer2.{}", &name);
    let (response_sender, response_receiver) = oneshot::channel();
    let _ = self
      .register_name
      .send((name.to_owned(), crossroads, response_sender));
    match response_receiver.recv() {
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
    match response_receiver.recv() {
      Ok(result) => result,
      _ => false,
    }
  }

  pub fn emit_message(&self, message: Message) {
    let _ = self.emit_message.send(message);
  }
}

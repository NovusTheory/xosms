mod media_player;

use core::panic;
use dbus::arg::{PropMap, RefArg};
use dbus::blocking::stdintf::org_freedesktop_dbus::{
    EmitsChangedSignal, PropertiesPropertiesChanged,
};
use dbus::blocking::Connection;
use dbus::channel::Sender;
use dbus::message::SignalArgs;
use dbus::{arg, Message, Path};
use dbus_crossroads::Crossroads;
use neon::event::Channel;
use neon::prelude::*;
use std::sync::{mpsc, Arc, RwLock};

use crate::media_service::service_trait::MediaServiceTrait;
use media_player::{
    register_org_mpris_media_player2, register_org_mpris_media_player2_player,
    OrgMprisMediaPlayer2, OrgMprisMediaPlayer2Player,
};

pub struct MediaService {
    state: Arc<RwLock<MprisPlayerState>>,
    crossroads_thread_handle: std::thread::JoinHandle<()>,
    crossroads_thread_term: mpsc::Sender<()>,
    dbus_send: mpsc::Sender<Message>,
}

impl Finalize for MediaService {
    fn finalize<'a, C: Context<'a>>(self, cx: &mut C) {
        self.crossroads_thread_term.send(());
        self.crossroads_thread_handle.join();
    }
}

impl MediaService {
    pub fn new(service_name: String, identity: String) -> Self {
        let connection = Connection::new_session().expect("Failed to create DBus session");
        let requested = match connection.request_name(
            format!("org.mpris.MediaPlayer2.{}", service_name),
            false,
            false,
            true,
        ) {
            Ok(dbus::blocking::stdintf::org_freedesktop_dbus::RequestNameReply::PrimaryOwner) => {
                true
            }
            Ok(_) => false,
            Err(e) => false,
        };
        if !requested {
            panic!("DBus name already taken");
        }

        let mut crossroads = Crossroads::new();

        let mpris_iface_token = register_org_mpris_media_player2::<MprisPlayer>(&mut crossroads);
        let mpris_player_iface_token =
            register_org_mpris_media_player2_player::<MprisPlayer>(&mut crossroads);

        let state = Arc::new(RwLock::new(MprisPlayerState {
            identity,
            can_go_next: false,
            can_go_previous: false,
            can_play: false,
            can_pause: false,
            can_seek: false,
            can_control: false,
            media_type: 0,
            playback_status: 2,
            artist: "".to_string(),
            album_artist: "".to_string(),
            album_title: "".to_string(),
            title: "".to_string(),
            track_id: "".to_string(),
            button_callback: None,
            metadata: PropMap::new(),
        }));

        crossroads.insert(
            "/org/mpris/MediaPlayer2",
            &[mpris_iface_token, mpris_player_iface_token],
            MprisPlayer {
                state: state.clone(),
            },
        );

        let (thread_term_tx, thread_term_rx) = mpsc::channel::<()>();
        let (dbus_tx, dbus_rx) = mpsc::channel::<Message>();
        let crossroads_thread_handle = std::thread::spawn(move || {
            // We add the Crossroads instance to the connection so that incoming method calls will be handled.
            use dbus::channel::MatchingReceiver;
            connection.start_receive(
                dbus::message::MatchRule::new_method_call(),
                Box::new(move |msg, conn| {
                    crossroads.handle_message(msg, conn).unwrap();
                    true
                }),
            );

            // Serve clients forever.
            loop {
                match thread_term_rx.try_recv() {
                    Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                        connection.release_name(connection.unique_name());
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                }
                match dbus_rx.try_recv() {
                    Ok(message) => {
                        connection.send(message);
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {}
                    Err(mpsc::TryRecvError::Empty) => {}
                }
                connection
                    .process(std::time::Duration::from_millis(1000))
                    .expect("Failed to process DBus connection");
            }
        });

        Self {
            state,
            crossroads_thread_handle,
            crossroads_thread_term: thread_term_tx,
            dbus_send: dbus_tx,
        }
    }
}

impl MediaServiceTrait for MediaService {
    // region Control
    fn is_enabled(&self) -> Result<bool, String> {
        Ok(self.state.read().unwrap().can_control)
    }

    fn set_is_enabled(&mut self, enabled: bool) -> Result<(), String> {
        {
            self.state.write().unwrap().can_control = enabled;
        }
        let mut ppc = PropertiesPropertiesChanged {
            interface_name: "org.mpris.MediaPlayer2.Player".to_string(),
            changed_properties: Default::default(),
            invalidated_properties: vec![],
        };
        ppc.add_prop("CanControl", EmitsChangedSignal::True, || Box::new(enabled));
        self.dbus_send
            .send(ppc.to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap()));

        Ok(())
    }
    // endregion Control

    // region Buttons
    fn is_play_enabled(&self) -> Result<bool, String> {
        Ok(self.state.read().unwrap().can_play)
    }

    fn set_is_play_enabled(&mut self, enabled: bool) -> Result<(), String> {
        {
            self.state.write().unwrap().can_play = enabled;
        }
        let mut ppc = PropertiesPropertiesChanged {
            interface_name: "org.mpris.MediaPlayer2.Player".to_string(),
            changed_properties: Default::default(),
            invalidated_properties: vec![],
        };
        ppc.add_prop("CanPlay", EmitsChangedSignal::True, || Box::new(enabled));
        self.dbus_send
            .send(ppc.to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap()));

        Ok(())
    }

    fn is_pause_enabled(&self) -> Result<bool, String> {
        Ok(self.state.read().unwrap().can_pause)
    }

    fn set_is_pause_enabled(&mut self, enabled: bool) -> Result<(), String> {
        {
            self.state.write().unwrap().can_pause = enabled;
        }
        let mut ppc = PropertiesPropertiesChanged {
            interface_name: "org.mpris.MediaPlayer2.Player".to_string(),
            changed_properties: Default::default(),
            invalidated_properties: vec![],
        };
        ppc.add_prop("CanPause", EmitsChangedSignal::True, || Box::new(enabled));
        self.dbus_send
            .send(ppc.to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap()));

        Ok(())
    }

    fn is_previous_enabled(&self) -> Result<bool, String> {
        Ok(self.state.read().unwrap().can_go_previous)
    }

    fn set_is_previous_enabled(&mut self, enabled: bool) -> Result<(), String> {
        {
            self.state.write().unwrap().can_go_previous = enabled;
        }
        let mut ppc = PropertiesPropertiesChanged {
            interface_name: "org.mpris.MediaPlayer2.Player".to_string(),
            changed_properties: Default::default(),
            invalidated_properties: vec![],
        };
        ppc.add_prop("CanGoPrevious", EmitsChangedSignal::True, || {
            Box::new(enabled)
        });
        self.dbus_send
            .send(ppc.to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap()));

        Ok(())
    }

    fn is_next_enabled(&self) -> Result<bool, String> {
        Ok(self.state.read().unwrap().can_go_next)
    }

    fn set_is_next_enabled(&mut self, enabled: bool) -> Result<(), String> {
        {
            self.state.write().unwrap().can_go_next = enabled;
        }
        let mut ppc = PropertiesPropertiesChanged {
            interface_name: "org.mpris.MediaPlayer2.Player".to_string(),
            changed_properties: Default::default(),
            invalidated_properties: vec![],
        };
        ppc.add_prop("CanGoNext", EmitsChangedSignal::True, || Box::new(enabled));
        self.dbus_send
            .send(ppc.to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap()));

        Ok(())
    }
    // endregion Buttons

    // region Media Information
    fn get_media_type(&self) -> Result<i32, String> {
        Ok(self.state.read().unwrap().media_type)
    }

    fn set_media_type(&mut self, media_type: i32) -> Result<(), String> {
        self.state.write().unwrap().media_type = media_type;

        Ok(())
    }

    fn get_playback_status(&self) -> Result<i32, String> {
        Ok(self.state.read().unwrap().playback_status)
    }

    fn set_playback_status(&mut self, status: i32) -> Result<(), String> {
        {
            self.state.write().unwrap().playback_status = status;
        }
        let mut ppc = PropertiesPropertiesChanged {
            interface_name: "org.mpris.MediaPlayer2.Player".to_string(),
            changed_properties: Default::default(),
            invalidated_properties: vec![],
        };
        ppc.add_prop("PlaybackStatus", EmitsChangedSignal::True, || {
            let status = match self.state.read().unwrap().playback_status {
                0 => "Stopped",
                1 => "Stopped",
                2 => "Stopped",
                3 => "Playing",
                4 => "Paused",
                _ => panic!("Invalid playback status provided"),
            };
            Box::new(status.to_string())
        });
        self.dbus_send
            .send(ppc.to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap()));

        Ok(())
    }

    fn get_artist(&self) -> Result<String, String> {
        Ok(self.state.read().unwrap().artist.clone())
    }

    fn set_artist(&mut self, artist: String) -> Result<(), String> {
        {
            self.state.write().unwrap().metadata.insert(
                "xesam:artist".to_string(),
                dbus::arg::Variant(Box::new(vec![artist.clone()])),
            );
        }
        {
            self.state.write().unwrap().artist = artist;
        }
        let mut ppc = PropertiesPropertiesChanged {
            interface_name: "org.mpris.MediaPlayer2.Player".to_string(),
            changed_properties: Default::default(),
            invalidated_properties: vec![],
        };
        ppc.add_prop("Metadata", EmitsChangedSignal::True, || {
            self.state.read().unwrap().metadata.box_clone()
        });
        self.dbus_send
            .send(ppc.to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap()));

        Ok(())
    }

    fn get_album_artist(&self) -> Result<String, String> {
        Ok(self.state.read().unwrap().album_artist.clone())
    }

    fn set_album_artist(&mut self, album_artist: String) -> Result<(), String> {
        {
            self.state.write().unwrap().metadata.insert(
                "xesam:albumArtist".to_string(),
                dbus::arg::Variant(Box::new(vec![album_artist.clone()])),
            );
        }
        {
            self.state.write().unwrap().album_artist = album_artist;
        }
        let mut ppc = PropertiesPropertiesChanged {
            interface_name: "org.mpris.MediaPlayer2.Player".to_string(),
            changed_properties: Default::default(),
            invalidated_properties: vec![],
        };
        ppc.add_prop("Metadata", EmitsChangedSignal::True, || {
            self.state.read().unwrap().metadata.box_clone()
        });
        self.dbus_send
            .send(ppc.to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap()));

        Ok(())
    }

    fn get_album_title(&self) -> Result<String, String> {
        Ok(self.state.read().unwrap().album_title.clone())
    }

    fn set_album_title(&mut self, album_title: String) -> Result<(), String> {
        {
            self.state.write().unwrap().metadata.insert(
                "xesam:album".to_string(),
                dbus::arg::Variant(Box::new(album_title.clone())),
            );
        }
        {
            self.state.write().unwrap().album_title = album_title;
        }
        let mut ppc = PropertiesPropertiesChanged {
            interface_name: "org.mpris.MediaPlayer2.Player".to_string(),
            changed_properties: Default::default(),
            invalidated_properties: vec![],
        };
        ppc.add_prop("Metadata", EmitsChangedSignal::True, || {
            self.state.read().unwrap().metadata.box_clone()
        });
        self.dbus_send
            .send(ppc.to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap()));

        Ok(())
    }

    fn get_title(&self) -> Result<String, String> {
        Ok(self.state.read().unwrap().title.clone())
    }

    fn set_title(&mut self, title: String) -> Result<(), String> {
        {
            self.state.write().unwrap().metadata.insert(
                "xesam:title".to_string(),
                dbus::arg::Variant(Box::new(title.clone())),
            );
        }
        {
            self.state.write().unwrap().title = title;
        }
        let mut ppc = PropertiesPropertiesChanged {
            interface_name: "org.mpris.MediaPlayer2.Player".to_string(),
            changed_properties: Default::default(),
            invalidated_properties: vec![],
        };
        ppc.add_prop("Metadata", EmitsChangedSignal::True, || {
            self.state.read().unwrap().metadata.box_clone()
        });
        self.dbus_send
            .send(ppc.to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap()));

        Ok(())
    }

    fn get_track_id(&self) -> Result<String, String> {
        Ok(self.state.read().unwrap().track_id.clone())
    }

    fn set_track_id(&mut self, track_id: String) -> Result<(), String> {
        {
            self.state.write().unwrap().metadata.insert(
                "mpris:trackId".to_string(),
                dbus::arg::Variant(Box::new(track_id.clone())),
            );
        }
        {
            self.state.write().unwrap().track_id = track_id;
        }
        let mut ppc = PropertiesPropertiesChanged {
            interface_name: "org.mpris.MediaPlayer2.Player".to_string(),
            changed_properties: Default::default(),
            invalidated_properties: vec![],
        };
        ppc.add_prop("Metadata", EmitsChangedSignal::True, || {
            self.state.read().unwrap().metadata.box_clone()
        });
        self.dbus_send
            .send(ppc.to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap()));

        Ok(())
    }

    fn set_thumbnail(&mut self, thumbnail_type: i32, thumbnail: String) -> Result<(), String> {
        let art_url = match thumbnail_type {
            1 => thumbnail,
            2 => thumbnail,
            _ => {
                return Err(format!(
                    "Thumbnail type is not supported on this operating system: {}",
                    thumbnail_type
                ))
            }
        };
        {
            self.state.write().unwrap().metadata.insert(
                "mpris:artUrl".to_string(),
                dbus::arg::Variant(Box::new(art_url)),
            );
        }
        let mut ppc = PropertiesPropertiesChanged {
            interface_name: "org.mpris.MediaPlayer2.Player".to_string(),
            changed_properties: Default::default(),
            invalidated_properties: vec![],
        };
        ppc.add_prop("Metadata", EmitsChangedSignal::True, || {
            self.state.read().unwrap().metadata.box_clone()
        });
        self.dbus_send
            .send(ppc.to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap()));

        Ok(())
    }
    // endregion Media Information

    // region Media Timeline
    fn set_timeline(&mut self, start_time: u64, end_time: u64, position: u64, min_seek_time: u64, max_seek_time: u64) -> Result<(), String> {
        Ok(())
    }
    // endregion Media Timeline

    // region Events
    fn set_button_pressed_callback(
        &mut self,
        callback: Root<JsFunction>,
        channel: Channel,
    ) -> Result<i64, String> {
        self.state.write().unwrap().button_callback =
            Some(MprisPlayerButtonCallback { callback, channel });

        Ok(-1)
    }

    fn remove_button_pressed_callback(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn set_playback_position_change_callback(
        &mut self,
        callback: Root<JsFunction>,
        channel: Channel,
    ) -> Result<i64, String> {
        Ok(-1)
    }

    fn remove_playback_position_change_callback(&mut self) -> Result<(), String> {
        Ok(())
    }
    // endregion Events
}

pub struct MprisPlayerState {
    identity: String,
    can_go_next: bool,
    can_go_previous: bool,
    can_play: bool,
    can_pause: bool,
    can_seek: bool,
    can_control: bool,
    media_type: i32,
    playback_status: i32,
    artist: String,
    album_artist: String,
    album_title: String,
    title: String,
    track_id: String,
    button_callback: Option<MprisPlayerButtonCallback>,
    metadata: dbus::arg::PropMap,
}

pub struct MprisPlayerButtonCallback {
    callback: Root<JsFunction>,
    channel: Channel,
}

pub struct MprisPlayer {
    state: Arc<RwLock<MprisPlayerState>>,
}

impl MprisPlayer {
    fn call_js_button_callback(&self, button: String) {
        if self.state.read().unwrap().button_callback.is_some() {
            let state = self.state.read().unwrap();
            let button_callback = state.button_callback.as_ref().unwrap();
            let state = self.state.clone();
            button_callback.channel.send(move |mut cx| {
                let state = state.read().unwrap();
                if state.button_callback.is_some() {
                    let button_callback = state.button_callback.as_ref().unwrap();
                    let callback = button_callback.callback.to_inner(&mut cx);
                    let this = cx.undefined();
                    let args = [cx.string(button).upcast()];
                    callback.call(&mut cx, this, args);
                }

                Ok(())
            });
        }
    }
}

impl OrgMprisMediaPlayer2 for MprisPlayer {
    fn raise(&mut self) -> Result<(), dbus::MethodErr> {
        Ok(())
    }
    fn quit(&mut self) -> Result<(), dbus::MethodErr> {
        Ok(())
    }
    fn can_quit(&self) -> Result<bool, dbus::MethodErr> {
        Ok(false)
    }
    fn can_raise(&self) -> Result<bool, dbus::MethodErr> {
        Ok(false)
    }
    fn has_track_list(&self) -> Result<bool, dbus::MethodErr> {
        Ok(false)
    }
    fn identity(&self) -> Result<String, dbus::MethodErr> {
        Ok(self.state.read().unwrap().identity.clone())
    }
    fn desktop_entry(&self) -> Result<String, dbus::MethodErr> {
        Ok("".to_string())
    }
    fn supported_uri_schemes(&self) -> Result<Vec<String>, dbus::MethodErr> {
        Ok(vec!["".to_string()])
    }
    fn supported_mime_types(&self) -> Result<Vec<String>, dbus::MethodErr> {
        Ok(vec!["".to_string()])
    }
}
impl OrgMprisMediaPlayer2Player for MprisPlayer {
    fn next(&mut self) -> Result<(), dbus::MethodErr> {
        self.call_js_button_callback("next".to_string());

        Ok(())
    }
    fn previous(&mut self) -> Result<(), dbus::MethodErr> {
        self.call_js_button_callback("previous".to_string());

        Ok(())
    }
    fn pause(&mut self) -> Result<(), dbus::MethodErr> {
        self.call_js_button_callback("pause".to_string());

        Ok(())
    }
    fn play_pause(&mut self) -> Result<(), dbus::MethodErr> {
        self.call_js_button_callback("playpause".to_string());

        Ok(())
    }
    fn stop(&mut self) -> Result<(), dbus::MethodErr> {
        self.call_js_button_callback("stop".to_string());

        Ok(())
    }
    fn play(&mut self) -> Result<(), dbus::MethodErr> {
        self.call_js_button_callback("play".to_string());

        Ok(())
    }
    fn seek(&mut self, offset: i64) -> Result<(), dbus::MethodErr> {
        Ok(())
    }
    fn set_position(
        &mut self,
        track_id: dbus::Path<'static>,
        position: i64,
    ) -> Result<(), dbus::MethodErr> {
        Ok(())
    }
    fn open_uri(&mut self, uri: String) -> Result<(), dbus::MethodErr> {
        Ok(())
    }
    fn set_volume_(&mut self, volume: f64) -> Result<(), dbus::MethodErr> {
        Ok(())
    }
    fn playback_status(&self) -> Result<String, dbus::MethodErr> {
        let playback_status = match self.state.read().unwrap().playback_status {
            0 => "Stopped",
            1 => "Stopped",
            2 => "Stopped",
            3 => "Playing",
            4 => "Paused",
            _ => panic!("Invalid playback status provided"),
        };
        Ok(playback_status.to_string())
    }
    fn loop_status(&self) -> Result<String, dbus::MethodErr> {
        Ok("None".to_string())
    }
    fn set_loop_status(&self, value: String) -> Result<(), dbus::MethodErr> {
        Ok(())
    }
    fn rate(&self) -> Result<f64, dbus::MethodErr> {
        Ok(1.0)
    }
    fn set_rate(&self, value: f64) -> Result<(), dbus::MethodErr> {
        Ok(())
    }
    fn shuffle(&self) -> Result<bool, dbus::MethodErr> {
        Ok(false)
    }
    fn set_shuffle(&self, value: bool) -> Result<(), dbus::MethodErr> {
        Ok(())
    }
    fn metadata(&self) -> Result<arg::PropMap, dbus::MethodErr> {
        // Since this method (because of crossroads & file generation) it requires an arg::PropMap
        // which makes this require a really ugly loop of the PropMap stored in the state and cloning everything to then ship off to DBus
        let origin_metadata = &self.state.read().unwrap().metadata;
        let mut metadata = PropMap::new();

        let mut origin_iter = origin_metadata.iter();
        while let Some(value) = origin_iter.next() {
            metadata.insert(value.0.clone(), arg::Variant((value.1).0.box_clone()));
        }
        Ok(metadata)
    }
    fn volume(&self) -> Result<f64, dbus::MethodErr> {
        Ok(1.0)
    }
    fn position(&self) -> Result<i64, dbus::MethodErr> {
        Ok(0)
    }
    fn minimum_rate(&self) -> Result<f64, dbus::MethodErr> {
        Ok(1.0)
    }
    fn maximum_rate(&self) -> Result<f64, dbus::MethodErr> {
        Ok(1.0)
    }
    fn can_go_next(&self) -> Result<bool, dbus::MethodErr> {
        let can_go_next = self.state.read().unwrap().can_go_next;
        Ok(can_go_next)
    }
    fn can_go_previous(&self) -> Result<bool, dbus::MethodErr> {
        let can_go_previous = self.state.read().unwrap().can_go_previous;
        Ok(can_go_previous)
    }
    fn can_play(&self) -> Result<bool, dbus::MethodErr> {
        let can_play = self.state.read().unwrap().can_play;
        Ok(can_play)
    }
    fn can_pause(&self) -> Result<bool, dbus::MethodErr> {
        let can_pause = self.state.read().unwrap().can_pause;
        Ok(can_pause)
    }
    fn can_seek(&self) -> Result<bool, dbus::MethodErr> {
        let can_seek = self.state.read().unwrap().can_seek;
        Ok(can_seek)
    }
    fn can_control(&self) -> Result<bool, dbus::MethodErr> {
        let can_control = self.state.read().unwrap().can_control;
        Ok(can_control)
    }
}

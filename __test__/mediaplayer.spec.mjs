import test from "ava";

import { fileURLToPath } from "url";
import {
  MediaPlayer,
  MediaPlayerMediaType,
  MediaPlayerPlaybackStatus,
  MediaPlayerThumbnail,
  MediaPlayerThumbnailType,
} from "../index.js";

test.serial("can create mediaplayer", (t) => {
  t.notThrows(() => new MediaPlayer("xosms-test", "Xosms Test"));
});

//
// ACTIVATION TESTS
//
test.serial("can activate", (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => player.activate());
});

test.serial("can activate while already active", (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  player.activate();
  t.notThrows(() => player.activate());
});

test.serial("can deactivate after activate", (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  player.activate();
  t.notThrows(() => player.deactivate());
});

test.serial("can deactivate before activate", (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => player.deactivate());
});

//
// BUTTONPRESSED EVENT TESTS
//
test.serial("can add buttonpressed event", (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => player.on("buttonpressed", () => {}));
});

test.serial("can remmove buttonpressed event", (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => {
    let listener = () => {};
    player.on("buttonpressed", listener);
    player.off("buttonpressed", listener);
  });
});

test.serial("can remmove buttonpressed event before adding", (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => {
    let listener = () => {};
    player.off("buttonpressed", listener);
  });
});

//
// POSITIONCHANGED EVENT TESTS
//
test.serial("can add positionchanged event", (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => player.on("positionchanged", () => {}));
});

test.serial("can remmove positionchanged event", (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => {
    let listener = () => {};
    player.on("positionchanged", listener);
    player.off("positionchanged", listener);
  });
});

test.serial("can remmove positionchanged event before adding", (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => {
    let listener = () => {};
    player.off("positionchanged", listener);
  });
});

//
// UPDATE TESTS
//
test.serial("can run update", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => player.update());
});

//
// THUMBNAIL TESTS
//
test.serial("can set file thumbnail", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  let thumbnail = await MediaPlayerThumbnail.create(
    MediaPlayerThumbnailType.File,
    fileURLToPath(import.meta.url)
  );
  t.notThrows(() => {
    player.setThumbnail(thumbnail);
  });
});

test.serial("can set uri thumbnail", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  let thumbnail = await MediaPlayerThumbnail.create(
    MediaPlayerThumbnailType.Uri,
    "https://via.placeholder.com/1.png"
  );
  t.notThrows(() => {
    player.setThumbnail(thumbnail);
  });
});

//
// TIMELINE TESTS
//
test.serial("can set timeline", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => {
    player.setTimeline(60, 0);
  });
});

test.serial("cannot set timeline duration below 0", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.throws(() => {
    player.setTimeline(-1, 0);
  });
});

test.serial("cannot set timeline position below 0", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.throws(() => {
    player.setTimeline(60, -1);
  });
});

test.serial("cannot set timeline position above duration", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.throws(() => {
    player.setTimeline(60, 61);
  });
});

//
// MEDIATYPE PROPERTY TESTS
//
test.serial("cannot set media type to unknown", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.throws(() => (player.mediaType = MediaPlayerMediaType.Unknown));
});

test.serial("can set media type to music", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => (player.mediaType = MediaPlayerMediaType.Music));
});

test.serial("can get media type before set", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => player.mediaType);
});

test.serial("media type before set is unknown", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.assert(() => player.mediaType == MediaPlayerMediaType.Unknown);
});

//
// PLAYBACK RATE PROPERTY TESTS
//
test.serial("cannot set  to unknown", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.throws(() => (player.playbackStatus = MediaPlayerPlaybackStatus.Unknown));
});

test.serial("can set playback rate", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(
    () => (player.playbackRate = 1)
  );
});

test.serial("can set playback rate to float", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => player.playbackRate = 1.5);
});

//
// PLAYBACKSTATUS PROPERTY TESTS
//
test.serial("cannot set playback status to unknown", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.throws(() => (player.playbackStatus = MediaPlayerPlaybackStatus.Unknown));
});

test.serial("can set playback status to music", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(
    () => (player.playbackStatus = MediaPlayerPlaybackStatus.Playing)
  );
});

test.serial("can get playback status before set", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => player.playbackStatus);
});

test.serial("playback status before set is unknown", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.assert(() => player.playbackStatus == MediaPlayerPlaybackStatus.Unknown);
});

//
// BUTTON ENABLEMENT PROPERTY TESTS
//
test.serial("can get play button enabled before set", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => player.playButtonEnabled);
});

test.serial("can set play button enabled", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => (player.playButtonEnabled = true));
});


test.serial("can get pause button enabled before set", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => player.pauseButtonEnabled);
});

test.serial("can set pause button enabled", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => (player.pauseButtonEnabled = true));
});


test.serial("can get stop button enabled before set", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => player.stopButtonEnabled);
});

test.serial("can set stop button enabled", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => (player.stopButtonEnabled = true));
});


test.serial("can get previous button enabled before set", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => player.previousButtonEnabled);
});

test.serial("can set previous button enabled", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => (player.previousButtonEnabled = true));
});


test.serial("can get next button enabled before set", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => player.nextButtonEnabled);
});

test.serial("can set next button enabled", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  t.notThrows(() => (player.nextButtonEnabled = true));
});

//
// TITLE PROPERTY TESTS
//
test.serial("can get title before set", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  player.mediaType = MediaPlayerMediaType.Music;
  t.notThrows(() => player.title);
});

test.serial("title before set is empty string", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  player.mediaType = MediaPlayerMediaType.Music;
  t.assert(player.title === "");
});

test.serial("can set title", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  player.mediaType = MediaPlayerMediaType.Music;
  t.notThrows(() => (player.title = "Test Title"));
});

//
// ARTIST PROPERTY TESTS
//
test.serial("can get artist before set", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  player.mediaType = MediaPlayerMediaType.Music;
  t.notThrows(() => player.artist);
});

test.serial("artist before set is empty string", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  player.mediaType = MediaPlayerMediaType.Music;
  t.assert(player.artist === "");
});

test.serial("can set artist", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  player.mediaType = MediaPlayerMediaType.Music;
  t.notThrows(() => (player.artist = "Test Artist"));
});

//
// ALBUM TITLE PROPERTY TESTS
//
test.serial("can get album title before set", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  player.mediaType = MediaPlayerMediaType.Music;
  t.notThrows(() => player.albumTitle);
});

test.serial("album title before set is empty string", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  player.mediaType = MediaPlayerMediaType.Music;
  t.assert(player.albumTitle === "");
});

test.serial("can set album title", async (t) => {
  let player = new MediaPlayer("xosms-test", "Xosms Test");
  player.mediaType = MediaPlayerMediaType.Music;
  t.notThrows(() => (player.albumTitle = "Test Artist"));
});

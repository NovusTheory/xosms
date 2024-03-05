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
  t.notThrows(() => new MediaPlayer("xosms", "Xosms Test"));
});

test.before(t => {
  t.context.player = new MediaPlayer("xosms", "Xosms Test");
})


//
// ACTIVATION TESTS
//
test.serial("can activate", (t) => {
  t.notThrows(() => t.context.player.activate());
});

test.serial("can activate while already active", (t) => {
  t.notThrows(() => t.context.player.activate());
});

test.serial("can deactivate after activate", (t) => {
  t.notThrows(() => t.context.player.deactivate());
});

test.serial("can deactivate while already deactive", (t) => {
  t.notThrows(() => t.context.player.deactivate());
});

test.serial("activate before rest of tests", (t) => {
  t.context.player.activate();
  t.pass();
});

//
// BUTTONPRESSED EVENT TESTS
//
test.serial("can add buttonpressed event", (t) => {
  t.notThrows(() => t.context.player.on("buttonpressed", () => {}));
});

test.serial("can remmove buttonpressed event", (t) => {
  t.notThrows(() => {
    let listener = () => {};
    t.context.player.on("buttonpressed", listener);
    t.context.player.off("buttonpressed", listener);
  });
});

test.serial("can remmove buttonpressed event before adding", (t) => {
  t.notThrows(() => {
    let listener = () => {};
    t.context.player.off("buttonpressed", listener);
  });
});

//
// POSITIONCHANGED EVENT TESTS
//
test.serial("can add positionchanged event", (t) => {
  t.notThrows(() => t.context.player.on("positionchanged", () => {}));
});

test.serial("can remmove positionchanged event", (t) => {
  t.notThrows(() => {
    let listener = () => {};
    t.context.player.on("positionchanged", listener);
    t.context.player.off("positionchanged", listener);
  });
});

test.serial("can remmove positionchanged event before adding", (t) => {
  t.notThrows(() => {
    let listener = () => {};
    t.context.player.off("positionchanged", listener);
  });
});

//
// THUMBNAIL TESTS
//
test.serial("can set file thumbnail", async (t) => {
  let thumbnail = await MediaPlayerThumbnail.create(
    MediaPlayerThumbnailType.File,
    fileURLToPath(import.meta.url)
  );
  t.notThrows(() => {
    t.context.player.setThumbnail(thumbnail);
  });
});

test.serial("can set uri thumbnail", async (t) => {
  let thumbnail = await MediaPlayerThumbnail.create(
    MediaPlayerThumbnailType.Uri,
    "https://via.placeholder.com/1.png"
  );
  t.notThrows(() => {
    t.context.player.setThumbnail(thumbnail);
  });
});

//
// TIMELINE TESTS
//
test.serial("can set timeline", async (t) => {
  t.notThrows(() => {
    t.context.player.setTimeline(60, 0);
  });
});

test.serial("cannot set timeline duration below 0", async (t) => {
  t.throws(() => {
    t.context.player.setTimeline(-1, 0);
  });
});

test.serial("cannot set timeline position below 0", async (t) => {
  t.throws(() => {
    t.context.player.setTimeline(60, -1);
  });
});

test.serial("cannot set timeline position above duration", async (t) => {
  t.throws(() => {
    t.context.player.setTimeline(60, 61);
  });
});

//
// MEDIATYPE PROPERTY TESTS
//
test.serial("cannot set media type to unknown", async (t) => {
  t.throws(() => (t.context.player.mediaType = MediaPlayerMediaType.Unknown));
});

test.serial("can set media type to music", async (t) => {
  t.notThrows(() => (t.context.player.mediaType = MediaPlayerMediaType.Music));
});

test.serial("can get media type before set", async (t) => {
  t.notThrows(() => t.context.player.mediaType);
});

test.serial("media type before set is unknown", async (t) => {
  t.assert(() => t.context.player.mediaType == MediaPlayerMediaType.Unknown);
});

//
// PLAYBACK RATE PROPERTY TESTS
//
test.serial("cannot set  to unknown", async (t) => {
  t.throws(() => (t.context.player.playbackStatus = MediaPlayerPlaybackStatus.Unknown));
});

test.serial("can set playback rate", async (t) => {
  t.notThrows(
    () => (t.context.player.playbackRate = 1)
  );
});

test.serial("can set playback rate to float", async (t) => {
  t.notThrows(() => t.context.player.playbackRate = 1.5);
});

//
// PLAYBACKSTATUS PROPERTY TESTS
//
test.serial("cannot set playback status to unknown", async (t) => {
  t.throws(() => (t.context.player.playbackStatus = MediaPlayerPlaybackStatus.Unknown));
});

test.serial("can set playback status to music", async (t) => {
  t.notThrows(
    () => (t.context.player.playbackStatus = MediaPlayerPlaybackStatus.Playing)
  );
});

test.serial("can get playback status before set", async (t) => {
  t.notThrows(() => t.context.player.playbackStatus);
});

test.serial("playback status before set is unknown", async (t) => {
  t.assert(() => t.context.player.playbackStatus == MediaPlayerPlaybackStatus.Unknown);
});

//
// BUTTON ENABLEMENT PROPERTY TESTS
//
test.serial("can get play button enabled before set", async (t) => {
  t.notThrows(() => t.context.player.playButtonEnabled);
});

test.serial("can set play button enabled", async (t) => {
  t.notThrows(() => (t.context.player.playButtonEnabled = true));
});


test.serial("can get pause button enabled before set", async (t) => {
  t.notThrows(() => t.context.player.pauseButtonEnabled);
});

test.serial("can set pause button enabled", async (t) => {
  t.notThrows(() => (t.context.player.pauseButtonEnabled = true));
});


test.serial("can get stop button enabled before set", async (t) => {
  t.notThrows(() => t.context.player.stopButtonEnabled);
});

test.serial("can set stop button enabled", async (t) => {
  t.notThrows(() => (t.context.player.stopButtonEnabled = true));
});


test.serial("can get previous button enabled before set", async (t) => {
  t.notThrows(() => t.context.player.previousButtonEnabled);
});

test.serial("can set previous button enabled", async (t) => {
  t.notThrows(() => (t.context.player.previousButtonEnabled = true));
});


test.serial("can get next button enabled before set", async (t) => {
  t.notThrows(() => t.context.player.nextButtonEnabled);
});

test.serial("can set next button enabled", async (t) => {
  t.notThrows(() => (t.context.player.nextButtonEnabled = true));
});

//
// TITLE PROPERTY TESTS
//
test.serial("can get title before set", async (t) => {
  t.notThrows(() => t.context.player.title);
});

test.serial("title before set is empty string", async (t) => {
  t.assert(t.context.player.title === "");
});

test.serial("can set title", async (t) => {
  t.notThrows(() => (t.context.player.title = "Test Title"));
});

//
// ARTIST PROPERTY TESTS
//
test.serial("can get artist before set", async (t) => {
  t.notThrows(() => t.context.player.artist);
});

test.serial("artist before set is empty string", async (t) => {
  t.assert(t.context.player.artist === "");
});

test.serial("can set artist", async (t) => {
  t.notThrows(() => (t.context.player.artist = "Test Artist"));
});

//
// ALBUM TITLE PROPERTY TESTS
//
test.serial("can get album title before set", async (t) => {
  t.notThrows(() => t.context.player.albumTitle);
});

test.serial("album title before set is empty string", async (t) => {
  t.assert(t.context.player.albumTitle === "");
});

test.serial("can set album title", async (t) => {
  t.notThrows(() => (t.context.player.albumTitle = "Test Artist"));
});

//
// TRACK ID PROPERTY TESTS
//
test.serial("can get track id before set", async (t) => {
  t.notThrows(() => t.context.player.trackId);
});

test.serial("track id before set is empty string", async (t) => {
  t.assert(t.context.player.trackId === "");
});

test.serial("can set track id", async (t) => {
  t.notThrows(() => (t.context.player.trackId = "TestTrackId"));
});

//
// UPDATE TESTS
//
test.serial("can run update", async (t) => {
  t.notThrows(() => t.context.player.update());
});
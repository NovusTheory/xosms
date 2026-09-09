# Xosms [![npm version](https://badge.fury.io/js/xosms.svg)](https://badge.fury.io/js/xosms)
A cross platform media service library made in Rust for Node to easily integrate with the platforms media service.

## Platforms Supported
- [x] Windows
- [x] MacOS
- [x] Linux (via MPRIS)

## Example Usage
```javascript
import { MediaPlayer, MediaPlayerThumbnail, platformSharesMediaPlayers } from "xosms";

const mediaPlayer = new MediaPlayer("test-xosms", "Test Xosms");
mediaPlayer.title = "Test Title";
mediaPlayer.artist = ["Test Artist"];
mediaPlayer.albumTitle = "Test Album";
mediaPlayer.volume = 1.0;
mediaPlayer.playbackRate = 1.0;
mediaPlayer.minimumPlaybackRate = 0.25;
mediaPlayer.maximumPlaybackRate = 2.0;
mediaPlayer.playbackRate = 1.0;
mediaPlayer.nextButtonEnabled = true;
mediaPlayer.previousButtonEnabled = true;
mediaPlayer.playButtonEnabled = true;
mediaPlayer.pauseButtonEnabled = true;
mediaPlayer.stopButtonEnabled = true;
mediaPlayer.seekEnabled = true;
mediaPlayer.fullscreen = false;
mediaPlayer.thumbnail = await MediaPlayerThumbnail.create("uri", "https://placehold.co/128x128.png?text=1");

// Callbacks can be a Promise and Xosms will wait for your callback to finish before calling the next ready callback.
//  The next callback could be a button press, a position change, etc. Xosms works through events from the media service in order
mediaPlayer.setButtonPressedCallback(async (button) => {
  // Act upon a button from the media service
});
mediaPlayer.setPositionChangedCallback(async (position) => {
  // Set position
});
mediaPlayer.setPositionSeekedCallback(async (seek) => {
  // Seek forward or backward this amount
});
mediaPlayer.setLoopChangedCallback(async (loopType) => {
  // Set loop mode
});
mediaPlayer.setRateChangedCallback(async (rate) => {
  // Set playback rate
});
mediaPlayer.setShuffleChangedCallback(async (shuffle) => {
  // Set shuffle
});
mediaPlayer.setVolumeChangedCallback(async (volume) => {
  // Set volume
})

// You must call update anytime you make changes to the MediaPlayer properties. The platform will only display changes when this is called
await mediaPlayer.update();
// Activates this MediaPlayer which will register it to the platform and start presenting information
await mediaPlayer.activate();

// It is critical that when you are done with the MediaPlayer you call dispose!
// You are at the mercy of the garbage collector if you do not call this to clean up any actively used platform natives
mediaPlayer.dispose();

// Sometimes a platforms media service is singleton per process like macOS. Use this function to detect that
//
// Xosms doesn't prevent you from having multiple MediaPlayer instances but behavior will not work how you think.
//  Calling `activate` will make button presses be passed to that instance
//  The latest `update` call from any MediaPlayer is the information used for the platform
if (platformSharesMediaPlayers()) {
  // Ensure only a single MediaPlayer is used
}
```

# Development
To locally develop and build xosms ensure you have
- rust
- pnpm

Once you have all of the above you can clone the repository and run
- `pnpm`
- `pnpm build`

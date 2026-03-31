# Xosms [![build](https://github.com/NovusTheory/xosms/actions/workflows/CI.yml/badge.svg?branch=dev)](https://github.com/NovusTheory/xosms/actions/workflows/CI.yml) [![npm version](https://badge.fury.io/js/xosms.svg)](https://badge.fury.io/js/xosms)
A cross platform media service library made in Rust for Node to easily and seamelessly integrate with the operating systems media service API.

## Current Platforms Supported
- [x] Windows
- [ ] MacOS
- [x] Linux (via MPRIS)

Even if your platform above isn't currently supported, the beauty of xosms is that it will still compile for it but noop on everything.

## Basic Usage
```javascript
import { MediaPlayer, MediaPlayerMediaType, MediaPlayerPlaybackStatus, MediaPlayerThumbnail, MediaPlayerThumbnailType } from "xosms";

// Create a new MediaPlayer. If the OS permits it you can create as many of these as you like for different media players
const mp = new MediaPlayer("my-media-player", "My Media Player");
mp.playButtonEnabled = true;
mp.pauseButtonEnabled = true;
mp.title = "An Awesome Song";
mp.artist = "An Awesome Artist";
mp.albumTitle = "An Awesome Album";

// Every event in Xosms emits a nullable error as the first argument of the callback. This includes any error data that may have occured.
mp.on("buttonpressed", (err, button) => {
  if (button == "play") {
    // play media
  } else if (button == "pause") {
    // pause media
  }
});

// Activate the MediaPlayer which will begin showing it in the OS. Likewise you can call `deactivate()` to remove it.
mp.activate()
```

# Development
To setup and locally develop and build xosms please ensure you have
- Rust
- Yarn

Once you have all of the above you can clone the repository and run
- `yarn`
- `yarn build` (Compiles the rust natives)
- `yarn build:lib` (Runs tsc)

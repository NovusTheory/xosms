const Xosms = require("../dist/binding.js");
const assert = require("assert");

assert(Xosms, "The expected module is undefined");

function ensureMediaServiceProviderIsCreated() {
    const instance = new Xosms.MediaServiceProvider("xosms-test-1", "Xosms Binding Test");
    assert(instance, "Media service provider was not created")
}

async function ensureCanModifyMediaServiceProviderProperties() {
    const instance = new Xosms.MediaServiceProvider("xosms-test-2", "Xosms Binding Test");
    assert(instance, "Media service provider was not created")
    // Ensures we can modify control
    console.log("Service Enabled:", instance.isEnabled);
    instance.isEnabled = false;
    console.log("Service Enabled:", instance.isEnabled);
    instance.isEnabled = true;
    console.log("Service Enabled:", instance.isEnabled);
    // Ensures we can modify the button enablements
    console.log("Play Button Enabled:", instance.playButtonEnabled);
    instance.playButtonEnabled = false;
    console.log("Play Button Enabled:", instance.playButtonEnabled);
    instance.playButtonEnabled = true;
    console.log("Play Button Enabled:", instance.playButtonEnabled);
    console.log("Pause Button Enabled:", instance.pauseButtonEnabled);
    instance.pauseButtonEnabled = false;
    console.log("Pause Button Enabled:", instance.pauseButtonEnabled);
    instance.pauseButtonEnabled = true;
    console.log("Pause Button Enabled:", instance.pauseButtonEnabled);
    console.log("Previous Button Enabled:", instance.previousButtonEnabled);
    instance.previousButtonEnabled = false;
    console.log("Previous Button Enabled:", instance.previousButtonEnabled);
    instance.previousButtonEnabled = true;
    console.log("Previous Button Enabled:", instance.previousButtonEnabled);
    console.log("Next Button Enabled:", instance.nextButtonEnabled);
    instance.nextButtonEnabled = false;
    console.log("Next Button Enabled:", instance.nextButtonEnabled);
    instance.nextButtonEnabled = true;
    console.log("Next Button Enabled:", instance.nextButtonEnabled);
    // Ensures we can modify the metadata
    console.log("Media Type:", instance.mediaType);
    instance.mediaType = Xosms.MediaType.Music;
    console.log("Media Type:", instance.mediaType);
    console.log("Playback Status:", instance.playbackStatus);
    instance.playbackStatus = Xosms.PlaybackStatus.Stopped;
    console.log("Playback Status:", instance.playbackStatus);
    console.log("Artist:", instance.artist);
    instance.artist = "Artist";
    console.log("Artist:", instance.artist);
    console.log("Album Artist:", instance.albumArtist);
    instance.albumArtist = "Album Artist";
    console.log("Album Artist:", instance.albumArtist);
    console.log("Album Title:", instance.albumTitle);
    instance.albumTitle = "Album Title";
    console.log("Album Title:", instance.albumTitle);
    console.log("Title:", instance.title);
    instance.title = "Song Title";
    console.log("Title:", instance.title);
    console.log("TrackId:", instance.trackId);
    instance.trackId = "TEST";
    console.log("TrackId:", instance.trackId);
    instance.setThumbnail(Xosms.ThumbnailType.Uri, "https://via.placeholder.com/128")
    // Ensures button event attachment works
    console.log("Adding button press callback");
    instance.buttonPressed = (button) => {
        console.log("Pressed button", button)
    };
    console.log("Removing button press callback");
    instance.buttonPressed = null;
}

assert.doesNotThrow(ensureMediaServiceProviderIsCreated, undefined, "ensureMediaServiceProviderIsCreated threw an error")
assert.doesNotThrow(ensureCanModifyMediaServiceProviderProperties, undefined, "ensureCanModifyMediaServiceProviderProperties threw an error")

console.log("Binding tests passed.");
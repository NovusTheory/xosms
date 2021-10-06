const Xosms = require("../dist/binding.js");
const assert = require("assert");

assert(Xosms, "The expected module is undefined");

function ensureMediaServiceProviderIsCreated() {
    const instance = new Xosms.MediaServiceProvider();
    assert(instance, "Media service provider was not created")
}

function ensureCanModifyMediaServiceProviderProperties() {
    const instance = new Xosms.MediaServiceProvider();
    assert(instance, "Media service provider was not created")
    // Ensures we can modify the button enablements
    instance.PlayButtonEnabled = false;
    instance.PlayButtonEnabled = true;
    instance.PauseButtonEnabled = false;
    instance.PauseButtonEnabled = true;
    instance.PreviousTrackEnabled = false;
    instance.PreviousTrackEnabled = true;
    instance.NextTrackEnabled = false;
    instance.NextTrackEnabled = true;
    // Ensures we can modify the metadata
    instance.MediaType = Xosms.MediaType.Music;
    instance.PlaybackStatus = Xosms.PlaybackStatus.Stopped;
    instance.Artist = "Artist";
    instance.AlbumArtist = "Album Artist";
    instance.AlbumTitle = "Album Title";
    instance.Title = "Song Title";
    instance.SetThumbnail(Xosms.ThumbnailType.Uri, "https://via.placeholder.com/128");
    // Ensures button event attachment works
    instance.ButtonPressed = (button) => { };
}

function ensureMediaServiceProviderThrowsOnButtonCallbackModify() {
    const instance = new Xosms.MediaServiceProvider();
    if (!instance) {
        console.log("Media service provider was not created");
        return;
    }
    // Attach button event
    instance.ButtonPressed = (button) => { };
    // Ensure that it throws when trying to modify
    instance.ButtonPressed = null;
}

assert.doesNotThrow(ensureMediaServiceProviderIsCreated, undefined, "ensureMediaServiceProviderIsCreated threw an error")
assert.doesNotThrow(ensureCanModifyMediaServiceProviderProperties, undefined, "ensureCanModifyMediaServiceProviderProperties threw an error")
assert.throws(ensureMediaServiceProviderThrowsOnButtonCallbackModify, undefined, "ensureMediaServiceProviderThrowsOnButtonCallbackModify did not throw an error")

console.log("Binding tests passed.");
// Required since binding ButtonPressed in tests will cause the program to never exit
process.exit();
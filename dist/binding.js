"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ThumbnailType = exports.PlaybackStatus = exports.MediaType = exports.MediaServiceProvider = void 0;
const addon = require('../build/Release/xosms-native');
;
var MediaType;
(function (MediaType) {
    MediaType[MediaType["Unknown"] = 0] = "Unknown";
    MediaType[MediaType["Music"] = 1] = "Music";
    MediaType[MediaType["Video"] = 2] = "Video";
    MediaType[MediaType["Image"] = 3] = "Image";
})(MediaType || (MediaType = {}));
exports.MediaType = MediaType;
var PlaybackStatus;
(function (PlaybackStatus) {
    PlaybackStatus[PlaybackStatus["Closed"] = 0] = "Closed";
    PlaybackStatus[PlaybackStatus["Changing"] = 1] = "Changing";
    PlaybackStatus[PlaybackStatus["Stopped"] = 2] = "Stopped";
    PlaybackStatus[PlaybackStatus["Playing"] = 3] = "Playing";
    PlaybackStatus[PlaybackStatus["Paused"] = 4] = "Paused";
})(PlaybackStatus || (PlaybackStatus = {}));
exports.PlaybackStatus = PlaybackStatus;
var ThumbnailType;
(function (ThumbnailType) {
    ThumbnailType[ThumbnailType["Unknown"] = 0] = "Unknown";
    ThumbnailType[ThumbnailType["File"] = 1] = "File";
    ThumbnailType[ThumbnailType["Uri"] = 2] = "Uri";
})(ThumbnailType || (ThumbnailType = {}));
exports.ThumbnailType = ThumbnailType;
class MediaServiceProvider {
    constructor() {
        // Buttons
        this._playButtonEnabled = true;
        this._pauseButtonEnabled = true;
        this._previousButtonEnabled = true;
        this._nextButtonEnabled = true;
        // Media properties
        this._mediaType = MediaType.Unknown;
        this._playbackStatus = PlaybackStatus.Closed;
        this._title = "";
        this._albumTitle = "";
        this._artist = "";
        this._albumArtist = "";
        this._thumbnail = "";
        this._thumbnailType = ThumbnailType.Unknown;
        // Events
        this._buttonPressed = null;
        switch (process.platform) {
            case "win32":
                this._native = new addon.WindowsMediaService();
                break;
            default:
                this._native = null;
                break;
        }
        this.NativeUpdateButtonEnablement();
    }
    NativeUpdateButtonEnablement() {
        var _a;
        (_a = this._native) === null || _a === void 0 ? void 0 : _a.UpdateButtonEnablement(this._playButtonEnabled, this._pauseButtonEnabled, this._previousButtonEnabled, this._nextButtonEnabled);
    }
    NativeUpdateMediaProperties() {
        var _a;
        (_a = this._native) === null || _a === void 0 ? void 0 : _a.UpdateMediaProperties(this._mediaType, this._playbackStatus, this._title, this._albumTitle, this._artist, this._albumArtist);
    }
    NativeUpdateThumbnail() {
        var _a;
        (_a = this._native) === null || _a === void 0 ? void 0 : _a.UpdateMediaThumbnail(this._thumbnailType, this._thumbnail);
    }
    NativeUpdateEvents() {
        var _a;
        (_a = this._native) === null || _a === void 0 ? void 0 : _a.UpdateEvents(this._buttonPressed);
    }
    // Buttons
    get PlayButtonEnabled() {
        return this._playButtonEnabled;
    }
    set PlayButtonEnabled(enabled) {
        this._playButtonEnabled = enabled;
        this.NativeUpdateButtonEnablement();
    }
    get PauseButtonEnabled() {
        return this._pauseButtonEnabled;
    }
    set PauseButtonEnabled(enabled) {
        this._pauseButtonEnabled = enabled;
        this.NativeUpdateButtonEnablement();
    }
    get PreviousButtonEnabled() {
        return this._previousButtonEnabled;
    }
    set PreviousButtonEnabled(enabled) {
        this._previousButtonEnabled = enabled;
        this.NativeUpdateButtonEnablement();
    }
    get NextButtonEnabled() {
        return this._nextButtonEnabled;
    }
    set NextButtonEnabled(enabled) {
        this._nextButtonEnabled = enabled;
        this.NativeUpdateButtonEnablement();
    }
    // Media properties
    get MediaType() {
        return this._mediaType;
    }
    set MediaType(type) {
        this._mediaType = type;
        this.NativeUpdateMediaProperties();
    }
    get Title() {
        return this._title;
    }
    set Title(title) {
        this._title = title;
        this.NativeUpdateMediaProperties();
    }
    get AlbumTitle() {
        return this._albumTitle;
    }
    set AlbumTitle(albumTitle) {
        this._albumTitle = albumTitle;
        this.NativeUpdateMediaProperties();
    }
    get Artist() {
        return this._artist;
    }
    set Artist(artist) {
        this._artist = artist;
        this.NativeUpdateMediaProperties();
    }
    get AlbumArtist() {
        return this._albumArtist;
    }
    set AlbumArtist(albumArtist) {
        this._albumArtist = albumArtist;
        this.NativeUpdateMediaProperties();
    }
    get Thumbnail() {
        return this._thumbnail;
    }
    get ThumbnailType() {
        return this._thumbnailType;
    }
    SetThumbnail(type, thumbnail) {
        this._thumbnailType = type;
        this._thumbnail = thumbnail;
        this.NativeUpdateThumbnail();
    }
    // Events
    set ButtonPressed(callback) {
        this._buttonPressed = callback;
        this.NativeUpdateEvents();
    }
}
exports.MediaServiceProvider = MediaServiceProvider;

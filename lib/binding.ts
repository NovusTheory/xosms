const native = require('../build/xosms-native');

interface IMediaService
{
    isPlayEnabled(): boolean;
    setIsPlayEnabled(enabled: boolean): void;
}

enum MediaType {
    Unknown = 0,
    Music = 1,
    Video = 2,
    Image = 3
}

enum PlaybackStatus {
    Closed = 0,
    Changing = 1,
    Stopped = 2,
    Playing = 3,
    Paused = 4
}

enum ThumbnailType {
    Unknown = 0,
    File = 1,
    Uri = 2
}

class MediaServiceProvider {
    private _nativeMediaService: IMediaService

    constructor(serviceName: string, identity: string) {
        this._nativeMediaService = native.createMediaService(serviceName, identity);
    }

    // Control
    public get isEnabled() {
        return native.mediaServiceIsEnabled(this._nativeMediaService);
    }

    public set isEnabled(enabled: boolean) {
        native.mediaServiceSetIsEnabled(this._nativeMediaService, enabled);
    }

    // Buttons
    public get playButtonEnabled() {
        return native.mediaServiceIsPlayEnabled(this._nativeMediaService);
    }

    public set playButtonEnabled(enabled: boolean) {
        native.mediaServiceSetIsPlayEnabled(this._nativeMediaService, enabled);
    }

    public get pauseButtonEnabled() {
        return native.mediaServiceIsPauseEnabled(this._nativeMediaService);
    }

    public set pauseButtonEnabled(enabled: boolean) {
        native.mediaServiceSetIsPauseEnabled(this._nativeMediaService, enabled);
    }

    public get previousButtonEnabled() {
        return native.mediaServiceIsPreviousEnabled(this._nativeMediaService);
    }

    public set previousButtonEnabled(enabled: boolean) {
        native.mediaServiceSetIsPreviousEnabled(this._nativeMediaService, enabled);
    }

    public get nextButtonEnabled() {
        return native.mediaServiceIsNextEnabled(this._nativeMediaService);
    }

    public set nextButtonEnabled(enabled: boolean) {
        native.mediaServiceSetIsNextEnabled(this._nativeMediaService, enabled);
    }

    // Media properties
    public get mediaType() {
        return native.mediaServiceGetMediaType(this._nativeMediaService);
    }

    public set mediaType(type: MediaType) {
        native.mediaServiceSetMediaType(this._nativeMediaService, type);
    }

    public get playbackStatus() {
        return native.mediaServiceGetPlaybackStatus(this._nativeMediaService);
    }

    public set playbackStatus(status: PlaybackStatus) {
        native.mediaServiceSetPlaybackStatus(this._nativeMediaService, status);
    }

    public get title() {
        return native.mediaServiceGetTitle(this._nativeMediaService);
    }

    public set title(title: string) {
        native.mediaServiceSetTitle(this._nativeMediaService, title);
    }

    public get albumTitle() {
        return native.mediaServiceGetAlbumTitle(this._nativeMediaService);
    }

    public set albumTitle(albumTitle: string) {
        native.mediaServiceSetAlbumTitle(this._nativeMediaService, albumTitle);
    }

    public get artist() {
        return native.mediaServiceGetArtist(this._nativeMediaService);
    }

    public set artist(artist: string) {
        native.mediaServiceSetArtist(this._nativeMediaService, artist);
    }

    public get albumArtist() {
        return native.mediaServiceGetAlbumArtist(this._nativeMediaService);
    }

    public set albumArtist(albumArtist: string) {
        native.mediaServiceSetAlbumArtist(this._nativeMediaService, albumArtist);
    }

    public get trackId() {
        return native.mediaServiceGetTrackId(this._nativeMediaService);
    }

    public set trackId(trackId: string) {
        native.mediaServiceSetTrackId(this._nativeMediaService, trackId);
    }

    public setThumbnail(type: ThumbnailType, thumbnail: string) {
        native.mediaServiceSetThumbnail(this._nativeMediaService, type, thumbnail);
    }

    // Events
    public set buttonPressed(callback: Function | null) {
        native.mediaServiceSetButtonCallback(this._nativeMediaService, callback)
    }
}

export { MediaServiceProvider, MediaType, PlaybackStatus, ThumbnailType }
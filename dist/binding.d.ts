declare enum MediaType {
    Unknown = 0,
    Music = 1,
    Video = 2,
    Image = 3
}
declare enum PlaybackStatus {
    Closed = 0,
    Changing = 1,
    Stopped = 2,
    Playing = 3,
    Paused = 4
}
declare enum ThumbnailType {
    Unknown = 0,
    File = 1,
    Uri = 2
}
declare class MediaServiceProvider {
    private _nativeMediaService;
    constructor();
    get isEnabled(): boolean;
    set isEnabled(enabled: boolean);
    get playButtonEnabled(): boolean;
    set playButtonEnabled(enabled: boolean);
    get pauseButtonEnabled(): boolean;
    set pauseButtonEnabled(enabled: boolean);
    get previousButtonEnabled(): boolean;
    set previousButtonEnabled(enabled: boolean);
    get nextButtonEnabled(): boolean;
    set nextButtonEnabled(enabled: boolean);
    get mediaType(): MediaType;
    set mediaType(type: MediaType);
    get playbackStatus(): PlaybackStatus;
    set playbackStatus(status: PlaybackStatus);
    get title(): string;
    set title(title: string);
    get albumTitle(): string;
    set albumTitle(albumTitle: string);
    get artist(): string;
    set artist(artist: string);
    get albumArtist(): string;
    set albumArtist(albumArtist: string);
    SetThumbnail(type: ThumbnailType, thumbnail: string): void;
}
export { MediaServiceProvider, MediaType, PlaybackStatus, ThumbnailType };

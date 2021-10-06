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
    private _native;
    private _playButtonEnabled;
    private _pauseButtonEnabled;
    private _previousButtonEnabled;
    private _nextButtonEnabled;
    private _mediaType;
    private _playbackStatus;
    private _title;
    private _albumTitle;
    private _artist;
    private _albumArtist;
    private _thumbnail;
    private _thumbnailType;
    private _buttonPressed;
    constructor();
    private NativeUpdateButtonEnablement;
    private NativeUpdateMediaProperties;
    private NativeUpdateThumbnail;
    private NativeUpdateEvents;
    get PlayButtonEnabled(): boolean;
    set PlayButtonEnabled(enabled: boolean);
    get PauseButtonEnabled(): boolean;
    set PauseButtonEnabled(enabled: boolean);
    get PreviousButtonEnabled(): boolean;
    set PreviousButtonEnabled(enabled: boolean);
    get NextButtonEnabled(): boolean;
    set NextButtonEnabled(enabled: boolean);
    get MediaType(): MediaType;
    set MediaType(type: MediaType);
    get Title(): string;
    set Title(title: string);
    get AlbumTitle(): string;
    set AlbumTitle(albumTitle: string);
    get Artist(): string;
    set Artist(artist: string);
    get AlbumArtist(): string;
    set AlbumArtist(albumArtist: string);
    get Thumbnail(): string;
    get ThumbnailType(): ThumbnailType;
    SetThumbnail(type: ThumbnailType, thumbnail: string): void;
    set ButtonPressed(callback: Function);
}
export { MediaServiceProvider, MediaType, PlaybackStatus, ThumbnailType };

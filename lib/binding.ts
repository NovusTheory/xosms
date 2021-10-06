const addon = require('../build/Release/xosms-native');

// interface IXosmsNative
// {
//     greet(strName: string): string;
// };

// class Xosms {
//     constructor(name: string) {
//         this._addonInstance = new addon.Xosms(name)
//     }

//     greet (strName: string) {
//         return this._addonInstance.greet(strName);
//     }

//     // private members
//     private _addonInstance: IXosmsNative;
// }

// export = Xosms;

interface IMediaService
{
    UpdateButtonEnablement(playButtonEnabled: boolean, pauseButtonEnabled: boolean, previousButtonEnabled: boolean, nextButtonEnabled: boolean): void;
    UpdateMediaProperties(mediaType: MediaType, playbackStatus: PlaybackStatus, title: string, albumTitle: string, artist: string, albumArtist: string): void;
    UpdateMediaThumbnail(thumbnailType: ThumbnailType, thumbnail: string): void;
    UpdateEvents(buttonPressed: Function | null): void;
};

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
    private _native: IMediaService | null

    // Buttons
    private _playButtonEnabled: boolean = true;
    private _pauseButtonEnabled: boolean = true;
    private _previousButtonEnabled: boolean = true;
    private _nextButtonEnabled: boolean = true;
    
    // Media properties
    private _mediaType: MediaType = MediaType.Unknown;
    private _playbackStatus: PlaybackStatus = PlaybackStatus.Closed;
    private _title: string = "";
    private _albumTitle: string = "";
    private _artist: string = "";
    private _albumArtist: string = "";
    private _thumbnail: string = "";
    private _thumbnailType: ThumbnailType = ThumbnailType.Unknown;
    
    // Events
    private _buttonPressed: Function | null = null;

    constructor() {
        switch(process.platform) {
            case "win32":
                this._native = new addon.WindowsMediaService();
                break;
            default:
                this._native = null;
                break;
        }

        this.NativeUpdateButtonEnablement();
    }

    private NativeUpdateButtonEnablement() {
        this._native?.UpdateButtonEnablement(this._playButtonEnabled, this._pauseButtonEnabled, this._previousButtonEnabled, this._nextButtonEnabled);
    }

    private NativeUpdateMediaProperties() {
        this._native?.UpdateMediaProperties(this._mediaType, this._playbackStatus, this._title, this._albumTitle, this._artist, this._albumArtist);
    }

    private NativeUpdateThumbnail() {
        this._native?.UpdateMediaThumbnail(this._thumbnailType, this._thumbnail);
    }

    private NativeUpdateEvents() {
        this._native?.UpdateEvents(this._buttonPressed);
    }

    // Buttons
    public get PlayButtonEnabled() {
        return this._playButtonEnabled;
    }

    public set PlayButtonEnabled(enabled: boolean) {
        this._playButtonEnabled = enabled;
        this.NativeUpdateButtonEnablement();
    }

    public get PauseButtonEnabled() {
        return this._pauseButtonEnabled;
    }

    public set PauseButtonEnabled(enabled: boolean) {
        this._pauseButtonEnabled = enabled;
        this.NativeUpdateButtonEnablement();
    }

    public get PreviousButtonEnabled() {
        return this._previousButtonEnabled;
    }

    public set PreviousButtonEnabled(enabled: boolean) {
        this._previousButtonEnabled = enabled;
        this.NativeUpdateButtonEnablement();
    }

    public get NextButtonEnabled() {
        return this._nextButtonEnabled;
    }

    public set NextButtonEnabled(enabled: boolean) {
        this._nextButtonEnabled = enabled;
        this.NativeUpdateButtonEnablement();
    }

    // Media properties
    public get MediaType() {
        return this._mediaType;
    }

    public set MediaType(type: MediaType) {
        this._mediaType = type;
        this.NativeUpdateMediaProperties();
    }

    public get Title() {
        return this._title;
    }

    public set Title(title: string) {
        this._title = title;
        this.NativeUpdateMediaProperties();
    }

    public get AlbumTitle() {
        return this._albumTitle;
    }

    public set AlbumTitle(albumTitle: string) {
        this._albumTitle = albumTitle;
        this.NativeUpdateMediaProperties();
    }

    public get Artist() {
        return this._artist;
    }

    public set Artist(artist: string) {
        this._artist = artist;
        this.NativeUpdateMediaProperties();
    }

    public get AlbumArtist() {
        return this._albumArtist;
    }

    public set AlbumArtist(albumArtist: string) {
        this._albumArtist = albumArtist;
        this.NativeUpdateMediaProperties();
    }

    public get Thumbnail() {
        return this._thumbnail;
    }

    public get ThumbnailType() {
        return this._thumbnailType;
    }

    public SetThumbnail(type: ThumbnailType, thumbnail: string) {
        this._thumbnailType = type;
        this._thumbnail = thumbnail;
        this.NativeUpdateThumbnail();
    }

    // Events
    public set ButtonPressed(callback: Function) {
        this._buttonPressed = callback;
        this.NativeUpdateEvents();
    }
}

export { MediaServiceProvider, MediaType, PlaybackStatus, ThumbnailType }
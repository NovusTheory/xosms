/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum MediaPlayerThumbnailType {
  Unknown = -1,
  File = 1,
  Uri = 2
}
export const enum MediaPlayerMediaType {
  Unknown = -1,
  Music = 1
}
export const enum MediaPlayerPlaybackStatus {
  Unknown = -1,
  Playing = 1,
  Paused = 2,
  Stopped = 3
}
export class MediaPlayerThumbnail {
  static create(thumbnailType: MediaPlayerThumbnailType, thumbnail: string): Promise<MediaPlayerThumbnail>
  get type(): MediaPlayerThumbnailType
}
export class MediaPlayer {
  constructor(serviceName: string, identity: string)
  /** Activates the MediaPlayer allowing the operating system to see and use it */
  activate(): void
  /** Deactivates the MediaPlayer denying the operating system to see and use it */
  deactivate(): void
  /**
   * Adds an event listener to the MediaPlayer
   *
   * 'buttonpressed' - Emitted when a media services button is pressed
   * 'positionchanged' - Emitted when the media service requests a position change
   * 'positionseeked' - Emitted when the media service requests a forward or backward position seek from current position
   */
  addEventListener(eventName: 'buttonpressed' | 'positionchanged' | 'positionseeked', callback: (...args: any[]) => any): void
  /** Removes an event listener from the MediaPlayer */
  removeEventListener(eventName: 'buttonpressed' | 'positionchanged' | 'positionseeked', callback: (...args: any[]) => any): void
  /**
   * Adds an event listener to the MediaPlayer
   *
   * Alias for addEventListener
   */
  on(eventName: 'buttonpressed' | 'positionchanged' | 'positionseeked', callback: (...args: any[]) => any): void
  /**
   * Removes an event listener from the MediaPlayer
   *
   * Alias for removeEventListener
   */
  off(eventName: 'buttonpressed' | 'positionchanged' | 'positionseeked', callback: (...args: any[]) => any): void
  /** Instructs the media service to update its media information being displayed */
  update(): void
  /** Sets the thumbnail */
  setThumbnail(thumbnail: MediaPlayerThumbnail): void
  /**
   * Sets the timeline data
   *
   * You MUST call this function everytime the position changes in the song. The media service will become out of sync if this is not called enough or cause seeked signals to be emitted to the media service unnecessarily.
   */
  setTimeline(duration: number, position: number): void
  /** Gets the play button enbled state */
  get playButtonEnabled(): boolean
  /** Sets the play button enbled state */
  set playButtonEnabled(enabled: boolean)
  /** Gets the paused button enbled state */
  get pauseButtonEnabled(): boolean
  /** Sets the paused button enbled state */
  set pauseButtonEnabled(enabled: boolean)
  /** Gets the paused button enbled state */
  get stopButtonEnabled(): boolean
  /** Sets the paused button enbled state */
  set stopButtonEnabled(enabled: boolean)
  /** Gets the previous button enbled state */
  get previousButtonEnabled(): boolean
  /** Sets the previous button enbled state */
  set previousButtonEnabled(enabled: boolean)
  /** Gets the next button enbled state */
  get nextButtonEnabled(): boolean
  /** Sets the next button enbled state */
  set nextButtonEnabled(enabled: boolean)
  /** Gets the seek enabled state */
  get seekEnabled(): boolean
  /** Sets the seek enbled state */
  set seekEnabled(enabled: boolean)
  /** Gets the playback rate */
  get playbackRate(): number
  /** Sets the playback rate */
  set playbackRate(playbackRate: number)
  /** Gets the playback status */
  get playbackStatus(): MediaPlayerPlaybackStatus
  /** Sets the playback status */
  set playbackStatus(playbackStatus: MediaPlayerPlaybackStatus)
  /** Gets the media type */
  get mediaType(): MediaPlayerMediaType
  /** Sets the media type */
  set mediaType(mediaType: MediaPlayerMediaType)
  /** Gets the media title */
  get title(): string
  /** Sets the media title */
  set title(title: string)
  /** Gets the media artist */
  get artist(): string
  /** Sets the media artist */
  set artist(artist: string)
  /** Gets the media album title */
  get albumTitle(): string
  /** Sets the media artist */
  set albumTitle(albumTitle: string)
  /** Gets the track id */
  get trackId(): string
  /** Sets the track id */
  set trackId(trackId: string)
}

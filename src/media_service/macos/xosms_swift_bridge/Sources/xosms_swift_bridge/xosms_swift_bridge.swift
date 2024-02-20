import Foundation
import MediaPlayer
import AppKit

let nowPlayingInfoCenter = MPNowPlayingInfoCenter.default()
let remoteCommandCenter = MPRemoteCommandCenter.shared()

var commandsConfigured = false
public func swift_configure_commands() {
    if (commandsConfigured) {
        return
    }
    commandsConfigured = true

    remoteCommandCenter.pauseCommand.addTarget { event in
        rust_remote_command_handler("pause")
        return .success
    }

    remoteCommandCenter.playCommand.addTarget { event in
        rust_remote_command_handler("play")
        return .success
    }

    remoteCommandCenter.stopCommand.addTarget { event in
        rust_remote_command_handler("stop")
        return .success
    }

    remoteCommandCenter.togglePlayPauseCommand.addTarget { event in
        rust_remote_command_handler("playpause")
        return .success
    }

    remoteCommandCenter.nextTrackCommand.addTarget { event in
        rust_remote_command_handler("next")
        return .success
    }
    
    remoteCommandCenter.previousTrackCommand.addTarget { event in
        rust_remote_command_handler("previous")
        return .success
    }
}

public func swift_set_playback_state(state: UInt) {
    nowPlayingInfoCenter.playbackState = MPNowPlayingPlaybackState(rawValue: state)!;
}

public func swift_set_info(info: NowPlayingInfo) {
    var nowPlayingInfo = [String: Any]()
    nowPlayingInfo[MPMediaItemPropertyMediaType] = MPMediaType(rawValue: info.media_type)
    nowPlayingInfo[MPMediaItemPropertyTitle] = info.title.toString()
    nowPlayingInfo[MPMediaItemPropertyArtist] = info.artist.toString()
    if (!info.artwork.toString().isEmpty) {
        if (info.artwork_type == 1) {
            let image = NSImage(contentsOfFile: info.artwork.toString())!
            if #available(macOS 10.13.2, *) {
                nowPlayingInfo[MPMediaItemPropertyArtwork] = MPMediaItemArtwork(boundsSize: image.size) { size in
                    return image
                }
            }
        } else if (info.artwork_type == 2) {
            let image = NSImage(contentsOf: URL(string: info.artwork.toString())!)!
            if #available(macOS 10.13.2, *) {
                nowPlayingInfo[MPMediaItemPropertyArtwork] = MPMediaItemArtwork(boundsSize: image.size) { size in
                    return image
                }
            }
        }
    }
    nowPlayingInfo[MPMediaItemPropertyAlbumArtist] = info.album_artist.toString()
    nowPlayingInfo[MPMediaItemPropertyAlbumTitle] = info.album_title.toString()

    nowPlayingInfoCenter.nowPlayingInfo = nowPlayingInfo
}

public func swift_get_playback_state() -> UInt {
    return nowPlayingInfoCenter.playbackState.rawValue;
}

public func swift_get_info() -> NowPlayingInfo {
    let nowPlayingInfo = nowPlayingInfoCenter.nowPlayingInfo;
    
    if (nowPlayingInfo != nil) {
        var mediaType = MPMediaType.any;
        if (nowPlayingInfo!.index(forKey: MPMediaItemPropertyMediaType) != nil) {
            mediaType = nowPlayingInfo![MPMediaItemPropertyMediaType] as! MPMediaType
        }
        var title = "";
        if (nowPlayingInfo!.index(forKey: MPMediaItemPropertyTitle) != nil) {
            title = nowPlayingInfo![MPMediaItemPropertyTitle] as! String
        }
        var artist = "";
        if (nowPlayingInfo!.index(forKey: MPMediaItemPropertyArtist) != nil) {
            artist = nowPlayingInfo![MPMediaItemPropertyArtist] as! String
        }
        var albumArtist = "";
        if (nowPlayingInfo!.index(forKey: MPNowPlayingInfoPropertyMediaType) != nil) {
            albumArtist = nowPlayingInfo![MPNowPlayingInfoPropertyMediaType] as! String
        }
        var albumTitle = "";
        if (nowPlayingInfo!.index(forKey: MPMediaItemPropertyAlbumTitle) != nil) {
            albumTitle = nowPlayingInfo![MPMediaItemPropertyAlbumTitle] as! String
        }

        return NowPlayingInfo(track_id: "".intoRustString(), media_type: mediaType.rawValue, title: title.intoRustString(), artist: artist.intoRustString(), album_artist: albumArtist.intoRustString(), album_title: albumTitle.intoRustString(), artwork: "".intoRustString(), artwork_type: 0)
    } else {
        return NowPlayingInfo(track_id: "".intoRustString(), media_type: 0, title: "".intoRustString(), artist: "".intoRustString(), album_artist: "".intoRustString(), album_title: "".intoRustString(), artwork: "".intoRustString(), artwork_type: 0)
    }
}

public func swift_set_remote_command_enabled(command: RustStr, enabled: Bool) {
    let command = command.toString();
    switch (command) {
        case "pause":
            remoteCommandCenter.pauseCommand.isEnabled = enabled;
        case "play":
            remoteCommandCenter.playCommand.isEnabled = enabled;
        case "stop":
            remoteCommandCenter.stopCommand.isEnabled = enabled;
        case "playpause":
            remoteCommandCenter.togglePlayPauseCommand.isEnabled = enabled;
        case "next":
            remoteCommandCenter.nextTrackCommand.isEnabled = enabled;
        case "previous":
            remoteCommandCenter.previousTrackCommand.isEnabled = enabled;
        default:
            fatalError("Invalid command provided to swift_set_remote_command_enabled")
    }
}

public func swift_is_remote_command_enabled(command: RustStr) -> Bool {
    let command = command.toString();
    switch (command) {
        case "pause":
            return remoteCommandCenter.pauseCommand.isEnabled;
        case "play":
            return remoteCommandCenter.playCommand.isEnabled;
        case "stop":
            return remoteCommandCenter.stopCommand.isEnabled;
        case "playpause":
            return remoteCommandCenter.togglePlayPauseCommand.isEnabled;
        case "next":
            return remoteCommandCenter.nextTrackCommand.isEnabled;
        case "previous":
            return remoteCommandCenter.previousTrackCommand.isEnabled;
        default:
            fatalError("Invalid command provided to swift_is_remote_command_enabled")
    }
}
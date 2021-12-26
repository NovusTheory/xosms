fn main() {
    println!("cargo:rerun-if-changed=src/build.rs");
    #[cfg(target_os = "windows")]
    windows::build! {
        Windows::Media::{SystemMediaTransportControls, MediaPlaybackStatus, MusicDisplayProperties, SystemMediaTransportControlsDisplayUpdater, MediaPlaybackType, SystemMediaTransportControlsButtonPressedEventArgs, SystemMediaTransportControlsButton},
        Windows::Media::Playback::{MediaPlayer, MediaPlaybackCommandManager},
        Windows::Storage::Streams::RandomAccessStreamReference,
        Windows::Foundation::{Uri, TypedEventHandler, EventRegistrationToken}
    }

    #[cfg(target_os = "macos")]
    {
        let sdk_path_output = std::process::Command::new("xcrun")
            .args(&["--sdk", "macosx", "--show-sdk-path"])
            .output()
            .expect("failed to get sdk path");
        let sdk_path = String::from_utf8_lossy(&sdk_path_output.stdout).into_owned();

        println!("cargo:rustc-link-lib=framework=MediaPlayer");
        let builder = bindgen::Builder::default()
            .rustfmt_bindings(true)
            .header_contents(
                "MediaPlayer.h",
                "#include<MediaPlayer/MPNowPlayingInfoCenter.h>\n#include<MediaPlayer/MPRemoteCommandCenter.h>\n#include<MediaPlayer/MPMediaItem.h>\n#include<MediaPlayer/MPRemoteCommand.h>\n#include<MediaPlayer/MPRemoteCommandEvent.h>\n#import<MediaPlayer/AVFoundation+MPNowPlayingInfoLanguageOptionAdditions.h>",
            )
            .clang_args(&["-isysroot", sdk_path.trim()])
            .block_extern_crate(false)
            .generate_block(true)
            .clang_args(&["-fblocks"])
            .objc_extern_crate(false)
            .clang_args(&["-x", "objective-c"])
            .rustfmt_bindings(true)
            .allowlist_recursively(true)
            .allowlist_type(".*MP.*")
            .allowlist_var(".*MP.*")
            .allowlist_type(".*NSObject.*")
            .allowlist_type(".*NSProxy.*")
            .allowlist_type(".*NSValue.*")
            .allowlist_type(".*NSOrderedSet.*")
            .allowlist_type(".*AV.*");
        /*.blocklist_item("timezone")
        .blocklist_item("objc_object")
        .blocklist_item("HFSPlusCatalogFile")
        .blocklist_item("HFSCatalogFile")
        .blocklist_item("HFSCatalogFolder");*/

        let bindings = builder.generate().expect("unable to generate bindings");

        let out_dir = std::env::var_os("OUT_DIR").unwrap();
        bindings
            .write_to_file(std::path::Path::new(&out_dir).join("macos_bindings.rs"))
            .expect("could not write bindings");
    }
}

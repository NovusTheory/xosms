#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoRepeatModeChangeRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  AutoRepeatModeChangeRequestedEventArgs,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl AutoRepeatModeChangeRequestedEventArgs {
  pub fn RequestedAutoRepeatMode(&self) -> windows_core::Result<MediaPlaybackAutoRepeatMode> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).RequestedAutoRepeatMode)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
}
impl windows_core::RuntimeType for AutoRepeatModeChangeRequestedEventArgs {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, IAutoRepeatModeChangeRequestedEventArgs>();
}
unsafe impl windows_core::Interface for AutoRepeatModeChangeRequestedEventArgs {
  type Vtable = <IAutoRepeatModeChangeRequestedEventArgs as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID =
    <IAutoRepeatModeChangeRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AutoRepeatModeChangeRequestedEventArgs {
  const NAME: &'static str = "Windows.Media.AutoRepeatModeChangeRequestedEventArgs";
}
unsafe impl Send for AutoRepeatModeChangeRequestedEventArgs {}
unsafe impl Sync for AutoRepeatModeChangeRequestedEventArgs {}
windows_core::imp::define_interface!(
  IAutoRepeatModeChangeRequestedEventArgs,
  IAutoRepeatModeChangeRequestedEventArgs_Vtbl,
  0xea137efa_d852_438e_882b_c990109a78f4
);
impl windows_core::RuntimeType for IAutoRepeatModeChangeRequestedEventArgs {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"Windows.Media.IAutoRepeatModeChangeRequestedEventArgs",
  );
}
#[repr(C)]
pub struct IAutoRepeatModeChangeRequestedEventArgs_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub RequestedAutoRepeatMode: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut MediaPlaybackAutoRepeatMode,
  ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  IImageDisplayProperties,
  IImageDisplayProperties_Vtbl,
  0xcd0bc7ef_54e7_411f_9933_f0e98b0a96d2
);
impl windows_core::RuntimeType for IImageDisplayProperties {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.IImageDisplayProperties");
}
#[repr(C)]
pub struct IImageDisplayProperties_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
  IMediaPlayer,
  IMediaPlayer_Vtbl,
  0x381a83cb_6fff_499b_8d64_2885dfc1249e
);
impl windows_core::RuntimeType for IMediaPlayer {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlayer");
}
#[repr(C)]
pub struct IMediaPlayer_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
  IMediaPlayer2,
  IMediaPlayer2_Vtbl,
  0x3c841218_2123_4fc5_9082_2f883f77bdf5
);
impl windows_core::RuntimeType for IMediaPlayer2 {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlayer2");
}
#[repr(C)]
pub struct IMediaPlayer2_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub SystemMediaTransportControls: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  IMusicDisplayProperties,
  IMusicDisplayProperties_Vtbl,
  0x6bbf0c59_d0a0_4d26_92a0_f978e1d18e7b
);
impl windows_core::RuntimeType for IMusicDisplayProperties {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.IMusicDisplayProperties");
}
#[repr(C)]
pub struct IMusicDisplayProperties_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub Title: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub SetTitle: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub AlbumArtist: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub SetAlbumArtist: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub Artist: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub SetArtist: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  IMusicDisplayProperties2,
  IMusicDisplayProperties2_Vtbl,
  0x00368462_97d3_44b9_b00f_008afcefaf18
);
impl windows_core::RuntimeType for IMusicDisplayProperties2 {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.IMusicDisplayProperties2");
}
#[repr(C)]
pub struct IMusicDisplayProperties2_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub AlbumTitle: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub SetAlbumTitle: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub TrackNumber:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
  pub SetTrackNumber:
    unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
  pub Genres: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  IMusicDisplayProperties3,
  IMusicDisplayProperties3_Vtbl,
  0x4db51ac1_0681_4e8c_9401_b8159d9eefc7
);
impl windows_core::RuntimeType for IMusicDisplayProperties3 {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.IMusicDisplayProperties3");
}
#[repr(C)]
pub struct IMusicDisplayProperties3_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub AlbumTrackCount:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
  pub SetAlbumTrackCount:
    unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  IOutputStream,
  IOutputStream_Vtbl,
  0x905a0fe6_bc53_11df_8c49_001e4fc686da
);
impl windows_core::RuntimeType for IOutputStream {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IOutputStream");
}
windows_core::imp::interface_hierarchy!(
  IOutputStream,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl windows_core::RuntimeName for IOutputStream {
  const NAME: &'static str = "Windows.Storage.Streams.IOutputStream";
}
#[repr(C)]
pub struct IOutputStream_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
  IPlaybackPositionChangeRequestedEventArgs,
  IPlaybackPositionChangeRequestedEventArgs_Vtbl,
  0xb4493f88_eb28_4961_9c14_335e44f3e125
);
impl windows_core::RuntimeType for IPlaybackPositionChangeRequestedEventArgs {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"Windows.Media.IPlaybackPositionChangeRequestedEventArgs",
  );
}
#[repr(C)]
pub struct IPlaybackPositionChangeRequestedEventArgs_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub RequestedPlaybackPosition: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut windows_time::TimeSpan,
  ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  IPlaybackRateChangeRequestedEventArgs,
  IPlaybackRateChangeRequestedEventArgs_Vtbl,
  0x2ce2c41f_3cd6_4f77_9ba7_eb27c26a2140
);
impl windows_core::RuntimeType for IPlaybackRateChangeRequestedEventArgs {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"Windows.Media.IPlaybackRateChangeRequestedEventArgs",
  );
}
#[repr(C)]
pub struct IPlaybackRateChangeRequestedEventArgs_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
  IRandomAccessStream,
  IRandomAccessStream_Vtbl,
  0x905a0fe1_bc53_11df_8c49_001e4fc686da
);
impl windows_core::RuntimeType for IRandomAccessStream {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IRandomAccessStream");
}
windows_core::imp::interface_hierarchy!(
  IRandomAccessStream,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl windows_core::RuntimeName for IRandomAccessStream {
  const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStream";
}
#[repr(C)]
pub struct IRandomAccessStream_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
  IRandomAccessStreamReference,
  IRandomAccessStreamReference_Vtbl,
  0x33ee3134_1dd6_4e3a_8067_d1c162e8642b
);
impl windows_core::RuntimeType for IRandomAccessStreamReference {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"Windows.Storage.Streams.IRandomAccessStreamReference",
  );
}
windows_core::imp::interface_hierarchy!(
  IRandomAccessStreamReference,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl windows_core::RuntimeName for IRandomAccessStreamReference {
  const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamReference";
}
#[repr(C)]
pub struct IRandomAccessStreamReference_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
  IRandomAccessStreamReferenceStatics,
  IRandomAccessStreamReferenceStatics_Vtbl,
  0x857309dc_3fbf_4e7d_986f_ef3b1a07a964
);
impl windows_core::RuntimeType for IRandomAccessStreamReferenceStatics {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"Windows.Storage.Streams.IRandomAccessStreamReferenceStatics",
  );
}
#[repr(C)]
pub struct IRandomAccessStreamReferenceStatics_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub CreateFromFile: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub CreateFromUri: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub CreateFromStream: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  IShuffleEnabledChangeRequestedEventArgs,
  IShuffleEnabledChangeRequestedEventArgs_Vtbl,
  0x49b593fe_4fd0_4666_a314_c0e01940d302
);
impl windows_core::RuntimeType for IShuffleEnabledChangeRequestedEventArgs {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"Windows.Media.IShuffleEnabledChangeRequestedEventArgs",
  );
}
#[repr(C)]
pub struct IShuffleEnabledChangeRequestedEventArgs_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub RequestedShuffleEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  IStorageFile,
  IStorageFile_Vtbl,
  0xfa3f6186_4214_428c_a64c_14c9ac7315ea
);
impl windows_core::RuntimeType for IStorageFile {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageFile");
}
windows_core::imp::interface_hierarchy!(
  IStorageFile,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl windows_core::RuntimeName for IStorageFile {
  const NAME: &'static str = "Windows.Storage.IStorageFile";
}
#[repr(C)]
pub struct IStorageFile_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
  IStorageFileStatics,
  IStorageFileStatics_Vtbl,
  0x5984c710_daf2_43c8_8bb4_a4d3eacfd03f
);
impl windows_core::RuntimeType for IStorageFileStatics {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageFileStatics");
}
#[repr(C)]
pub struct IStorageFileStatics_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub GetFileFromPathAsync: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub GetFileFromApplicationUriAsync: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub CreateStreamedFileAsync: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub ReplaceWithStreamedFileAsync: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub CreateStreamedFileFromUriAsync: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub ReplaceWithStreamedFileFromUriAsync: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  IStorageFileStatics2,
  IStorageFileStatics2_Vtbl,
  0x5c76a781_212e_4af9_8f04_740cae108974
);
impl windows_core::RuntimeType for IStorageFileStatics2 {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageFileStatics2");
}
#[repr(C)]
pub struct IStorageFileStatics2_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub GetFileFromPathForUserAsync: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  ISystemMediaTransportControls,
  ISystemMediaTransportControls_Vtbl,
  0x99fa3ff4_1742_42a6_902e_087d41f965ec
);
impl windows_core::RuntimeType for ISystemMediaTransportControls {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.ISystemMediaTransportControls");
}
#[repr(C)]
pub struct ISystemMediaTransportControls_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub PlaybackStatus: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut MediaPlaybackStatus,
  ) -> windows_core::HRESULT,
  pub SetPlaybackStatus:
    unsafe extern "system" fn(*mut core::ffi::c_void, MediaPlaybackStatus) -> windows_core::HRESULT,
  pub DisplayUpdater: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub SoundLevel:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut SoundLevel) -> windows_core::HRESULT,
  pub IsEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
  pub SetIsEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
  pub IsPlayEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
  pub SetIsPlayEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
  pub IsStopEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
  pub SetIsStopEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
  pub IsPauseEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
  pub SetIsPauseEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
  pub IsRecordEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
  pub SetIsRecordEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
  pub IsFastForwardEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
  pub SetIsFastForwardEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
  pub IsRewindEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
  pub SetIsRewindEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
  pub IsPreviousEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
  pub SetIsPreviousEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
  pub IsNextEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
  pub SetIsNextEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
  pub IsChannelUpEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
  pub SetIsChannelUpEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
  pub IsChannelDownEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
  pub SetIsChannelDownEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
  pub ButtonPressed: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut i64,
  ) -> windows_core::HRESULT,
  pub RemoveButtonPressed:
    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
  pub PropertyChanged: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut i64,
  ) -> windows_core::HRESULT,
  pub RemovePropertyChanged:
    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  ISystemMediaTransportControls2,
  ISystemMediaTransportControls2_Vtbl,
  0xea98d2f6_7f3c_4af2_a586_72889808efb1
);
impl windows_core::RuntimeType for ISystemMediaTransportControls2 {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.ISystemMediaTransportControls2");
}
#[repr(C)]
pub struct ISystemMediaTransportControls2_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub AutoRepeatMode: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut MediaPlaybackAutoRepeatMode,
  ) -> windows_core::HRESULT,
  pub SetAutoRepeatMode: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    MediaPlaybackAutoRepeatMode,
  ) -> windows_core::HRESULT,
  pub ShuffleEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
  pub SetShuffleEnabled:
    unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
  pub PlaybackRate:
    unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
  pub SetPlaybackRate:
    unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
  pub UpdateTimelineProperties: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub PlaybackPositionChangeRequested: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut i64,
  ) -> windows_core::HRESULT,
  pub RemovePlaybackPositionChangeRequested:
    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
  pub PlaybackRateChangeRequested: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut i64,
  ) -> windows_core::HRESULT,
  pub RemovePlaybackRateChangeRequested:
    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
  pub ShuffleEnabledChangeRequested: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut i64,
  ) -> windows_core::HRESULT,
  pub RemoveShuffleEnabledChangeRequested:
    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
  pub AutoRepeatModeChangeRequested: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut i64,
  ) -> windows_core::HRESULT,
  pub RemoveAutoRepeatModeChangeRequested:
    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  ISystemMediaTransportControlsButtonPressedEventArgs,
  ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl,
  0xb7f47116_a56f_4dc8_9e11_92031f4a87c2
);
impl windows_core::RuntimeType for ISystemMediaTransportControlsButtonPressedEventArgs {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"Windows.Media.ISystemMediaTransportControlsButtonPressedEventArgs",
  );
}
#[repr(C)]
pub struct ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub Button: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut SystemMediaTransportControlsButton,
  ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  ISystemMediaTransportControlsDisplayUpdater,
  ISystemMediaTransportControlsDisplayUpdater_Vtbl,
  0x8abbc53e_fa55_4ecf_ad8e_c984e5dd1550
);
impl windows_core::RuntimeType for ISystemMediaTransportControlsDisplayUpdater {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"Windows.Media.ISystemMediaTransportControlsDisplayUpdater",
  );
}
#[repr(C)]
pub struct ISystemMediaTransportControlsDisplayUpdater_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub Type: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut MediaPlaybackType,
  ) -> windows_core::HRESULT,
  pub SetType:
    unsafe extern "system" fn(*mut core::ffi::c_void, MediaPlaybackType) -> windows_core::HRESULT,
  pub AppMediaId: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub SetAppMediaId: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub Thumbnail: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub SetThumbnail: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub MusicProperties: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub VideoProperties: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub ImageProperties: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  CopyFromFileAsync: usize,
  pub ClearAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
  pub Update: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  ISystemMediaTransportControlsPropertyChangedEventArgs,
  ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl,
  0xd0ca0936_339b_4cb3_8eeb_737607f56e08
);
impl windows_core::RuntimeType for ISystemMediaTransportControlsPropertyChangedEventArgs {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"Windows.Media.ISystemMediaTransportControlsPropertyChangedEventArgs",
  );
}
#[repr(C)]
pub struct ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
  ISystemMediaTransportControlsStatics,
  ISystemMediaTransportControlsStatics_Vtbl,
  0x43ba380a_eca4_4832_91ab_d415fae484c6
);
impl windows_core::RuntimeType for ISystemMediaTransportControlsStatics {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"Windows.Media.ISystemMediaTransportControlsStatics",
  );
}
#[repr(C)]
pub struct ISystemMediaTransportControlsStatics_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
  ISystemMediaTransportControlsTimelineProperties,
  ISystemMediaTransportControlsTimelineProperties_Vtbl,
  0x5125316a_c3a2_475b_8507_93534dc88f15
);
impl windows_core::RuntimeType for ISystemMediaTransportControlsTimelineProperties {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"Windows.Media.ISystemMediaTransportControlsTimelineProperties",
  );
}
#[repr(C)]
pub struct ISystemMediaTransportControlsTimelineProperties_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub StartTime: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut windows_time::TimeSpan,
  ) -> windows_core::HRESULT,
  pub SetStartTime: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    windows_time::TimeSpan,
  ) -> windows_core::HRESULT,
  pub EndTime: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut windows_time::TimeSpan,
  ) -> windows_core::HRESULT,
  pub SetEndTime: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    windows_time::TimeSpan,
  ) -> windows_core::HRESULT,
  pub MinSeekTime: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut windows_time::TimeSpan,
  ) -> windows_core::HRESULT,
  pub SetMinSeekTime: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    windows_time::TimeSpan,
  ) -> windows_core::HRESULT,
  pub MaxSeekTime: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut windows_time::TimeSpan,
  ) -> windows_core::HRESULT,
  pub SetMaxSeekTime: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    windows_time::TimeSpan,
  ) -> windows_core::HRESULT,
  pub Position: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut windows_time::TimeSpan,
  ) -> windows_core::HRESULT,
  pub SetPosition: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    windows_time::TimeSpan,
  ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  IUriEscapeStatics,
  IUriEscapeStatics_Vtbl,
  0xc1d432ba_c824_4452_a7fd_512bc3bbe9a1
);
impl windows_core::RuntimeType for IUriEscapeStatics {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IUriEscapeStatics");
}
#[repr(C)]
pub struct IUriEscapeStatics_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub UnescapeComponent: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub EscapeComponent: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  IUriRuntimeClass,
  IUriRuntimeClass_Vtbl,
  0x9e365e57_48b2_4160_956f_c7385120bbfc
);
impl windows_core::RuntimeType for IUriRuntimeClass {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IUriRuntimeClass");
}
#[repr(C)]
pub struct IUriRuntimeClass_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
  IUriRuntimeClassFactory,
  IUriRuntimeClassFactory_Vtbl,
  0x44a9796f_723e_4fdf_a218_033e75b0c084
);
impl windows_core::RuntimeType for IUriRuntimeClassFactory {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IUriRuntimeClassFactory");
}
#[repr(C)]
pub struct IUriRuntimeClassFactory_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub CreateUri: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub CreateWithRelativeUri: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUser, IUser_Vtbl, 0xdf9a26c6_e746_4bcd_b5d4_120103c4209b);
impl windows_core::RuntimeType for IUser {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.System.IUser");
}
#[repr(C)]
pub struct IUser_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
  IUserStatics,
  IUserStatics_Vtbl,
  0x155eb23b_242a_45e0_a2e9_3171fc6a7fdd
);
impl windows_core::RuntimeType for IUserStatics {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.System.IUserStatics");
}
#[repr(C)]
pub struct IUserStatics_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub CreateWatcher: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub FindAllAsync: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub FindAllAsyncByType: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    UserType,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub FindAllAsyncByTypeAndStatus: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    UserType,
    UserAuthenticationStatus,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
  pub GetFromId: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  IUserStatics2,
  IUserStatics2_Vtbl,
  0x74a37e11_2eb5_4487_b0d5_2c6790e013e9
);
impl windows_core::RuntimeType for IUserStatics2 {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.System.IUserStatics2");
}
#[repr(C)]
pub struct IUserStatics2_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
  pub GetDefault: unsafe extern "system" fn(
    *mut core::ffi::c_void,
    *mut *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
  IUserWatcher,
  IUserWatcher_Vtbl,
  0x155eb23b_242a_45e0_a2e9_3171fc6a7fbb
);
impl windows_core::RuntimeType for IUserWatcher {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.System.IUserWatcher");
}
#[repr(C)]
pub struct IUserWatcher_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
  IVideoDisplayProperties,
  IVideoDisplayProperties_Vtbl,
  0x5609fdb1_5d2d_4872_8170_45dee5bc2f5c
);
impl windows_core::RuntimeType for IVideoDisplayProperties {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.IVideoDisplayProperties");
}
#[repr(C)]
pub struct IVideoDisplayProperties_Vtbl {
  pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageDisplayProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  ImageDisplayProperties,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl windows_core::RuntimeType for ImageDisplayProperties {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, IImageDisplayProperties>();
}
unsafe impl windows_core::Interface for ImageDisplayProperties {
  type Vtable = <IImageDisplayProperties as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID = <IImageDisplayProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ImageDisplayProperties {
  const NAME: &'static str = "Windows.Media.ImageDisplayProperties";
}
unsafe impl Send for ImageDisplayProperties {}
unsafe impl Sync for ImageDisplayProperties {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MediaPlaybackAutoRepeatMode(pub i32);
impl MediaPlaybackAutoRepeatMode {
  pub const None: Self = Self(0);
  pub const Track: Self = Self(1);
  pub const List: Self = Self(2);
}
impl windows_core::imp::TypeKind for MediaPlaybackAutoRepeatMode {
  type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for MediaPlaybackAutoRepeatMode {
  const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"enum(Windows.Media.MediaPlaybackAutoRepeatMode;i4)",
  );
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.MediaPlaybackAutoRepeatMode");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MediaPlaybackStatus(pub i32);
impl MediaPlaybackStatus {
  pub const Closed: Self = Self(0);
  pub const Changing: Self = Self(1);
  pub const Stopped: Self = Self(2);
  pub const Playing: Self = Self(3);
  pub const Paused: Self = Self(4);
}
impl windows_core::imp::TypeKind for MediaPlaybackStatus {
  type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for MediaPlaybackStatus {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackStatus;i4)");
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.MediaPlaybackStatus");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MediaPlaybackType(pub i32);
impl MediaPlaybackType {
  pub const Unknown: Self = Self(0);
  pub const Music: Self = Self(1);
  pub const Video: Self = Self(2);
  pub const Image: Self = Self(3);
}
impl windows_core::imp::TypeKind for MediaPlaybackType {
  type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for MediaPlaybackType {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackType;i4)");
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.MediaPlaybackType");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaPlayer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  MediaPlayer,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl MediaPlayer {
  pub fn new() -> windows_core::Result<Self> {
    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
  }
  fn IActivationFactory<
    R,
    F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
  >(
    callback: F,
  ) -> windows_core::Result<R> {
    static SHARED: windows_core::imp::FactoryCache<
      MediaPlayer,
      windows_core::imp::IGenericFactory,
    > = windows_core::imp::FactoryCache::new();
    SHARED.call(callback)
  }
  pub fn SystemMediaTransportControls(&self) -> windows_core::Result<SystemMediaTransportControls> {
    let this = &windows_core::Interface::cast::<IMediaPlayer2>(self)?;
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).SystemMediaTransportControls)(
        windows_core::Interface::as_raw(this),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
  }
}
impl windows_core::RuntimeType for MediaPlayer {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlayer>();
}
unsafe impl windows_core::Interface for MediaPlayer {
  type Vtable = <IMediaPlayer as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID = <IMediaPlayer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlayer {
  const NAME: &'static str = "Windows.Media.Playback.MediaPlayer";
}
unsafe impl Send for MediaPlayer {}
unsafe impl Sync for MediaPlayer {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MediaPlayerAudioCategory(pub i32);
impl MediaPlayerAudioCategory {
  pub const Other: Self = Self(0);
  pub const Communications: Self = Self(3);
  pub const Alerts: Self = Self(4);
  pub const SoundEffects: Self = Self(5);
  pub const GameEffects: Self = Self(6);
  pub const GameMedia: Self = Self(7);
  pub const GameChat: Self = Self(8);
  pub const Speech: Self = Self(9);
  pub const Movie: Self = Self(10);
  pub const Media: Self = Self(11);
}
impl windows_core::imp::TypeKind for MediaPlayerAudioCategory {
  type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for MediaPlayerAudioCategory {
  const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"enum(Windows.Media.Playback.MediaPlayerAudioCategory;i4)",
  );
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.MediaPlayerAudioCategory");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MediaPlayerAudioDeviceType(pub i32);
impl MediaPlayerAudioDeviceType {
  pub const Console: Self = Self(0);
  pub const Multimedia: Self = Self(1);
  pub const Communications: Self = Self(2);
}
impl windows_core::imp::TypeKind for MediaPlayerAudioDeviceType {
  type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for MediaPlayerAudioDeviceType {
  const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"enum(Windows.Media.Playback.MediaPlayerAudioDeviceType;i4)",
  );
  const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"Windows.Media.Playback.MediaPlayerAudioDeviceType",
  );
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MusicDisplayProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  MusicDisplayProperties,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl MusicDisplayProperties {
  pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).Title)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| core::mem::transmute(result__))
    }
  }
  pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetTitle)(
        windows_core::Interface::as_raw(self),
        core::mem::transmute_copy(value),
      )
      .ok()
    }
  }
  pub fn AlbumArtist(&self) -> windows_core::Result<windows_core::HSTRING> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).AlbumArtist)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| core::mem::transmute(result__))
    }
  }
  pub fn SetAlbumArtist(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetAlbumArtist)(
        windows_core::Interface::as_raw(self),
        core::mem::transmute_copy(value),
      )
      .ok()
    }
  }
  pub fn Artist(&self) -> windows_core::Result<windows_core::HSTRING> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).Artist)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| core::mem::transmute(result__))
    }
  }
  pub fn SetArtist(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetArtist)(
        windows_core::Interface::as_raw(self),
        core::mem::transmute_copy(value),
      )
      .ok()
    }
  }
  pub fn AlbumTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
    let this = &windows_core::Interface::cast::<IMusicDisplayProperties2>(self)?;
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).AlbumTitle)(
        windows_core::Interface::as_raw(this),
        &mut result__,
      )
      .map(|| core::mem::transmute(result__))
    }
  }
  pub fn SetAlbumTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
    let this = &windows_core::Interface::cast::<IMusicDisplayProperties2>(self)?;
    unsafe {
      (windows_core::Interface::vtable(this).SetAlbumTitle)(
        windows_core::Interface::as_raw(this),
        core::mem::transmute_copy(value),
      )
      .ok()
    }
  }
  pub fn TrackNumber(&self) -> windows_core::Result<u32> {
    let this = &windows_core::Interface::cast::<IMusicDisplayProperties2>(self)?;
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).TrackNumber)(
        windows_core::Interface::as_raw(this),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetTrackNumber(&self, value: u32) -> windows_core::Result<()> {
    let this = &windows_core::Interface::cast::<IMusicDisplayProperties2>(self)?;
    unsafe {
      (windows_core::Interface::vtable(this).SetTrackNumber)(
        windows_core::Interface::as_raw(this),
        value,
      )
      .ok()
    }
  }
  pub fn Genres(
    &self,
  ) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
    let this = &windows_core::Interface::cast::<IMusicDisplayProperties2>(self)?;
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).Genres)(
        windows_core::Interface::as_raw(this),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
  }
  pub fn AlbumTrackCount(&self) -> windows_core::Result<u32> {
    let this = &windows_core::Interface::cast::<IMusicDisplayProperties3>(self)?;
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).AlbumTrackCount)(
        windows_core::Interface::as_raw(this),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetAlbumTrackCount(&self, value: u32) -> windows_core::Result<()> {
    let this = &windows_core::Interface::cast::<IMusicDisplayProperties3>(self)?;
    unsafe {
      (windows_core::Interface::vtable(this).SetAlbumTrackCount)(
        windows_core::Interface::as_raw(this),
        value,
      )
      .ok()
    }
  }
}
impl windows_core::RuntimeType for MusicDisplayProperties {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, IMusicDisplayProperties>();
}
unsafe impl windows_core::Interface for MusicDisplayProperties {
  type Vtable = <IMusicDisplayProperties as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID = <IMusicDisplayProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MusicDisplayProperties {
  const NAME: &'static str = "Windows.Media.MusicDisplayProperties";
}
unsafe impl Send for MusicDisplayProperties {}
unsafe impl Sync for MusicDisplayProperties {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaybackPositionChangeRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  PlaybackPositionChangeRequestedEventArgs,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl PlaybackPositionChangeRequestedEventArgs {
  pub fn RequestedPlaybackPosition(&self) -> windows_core::Result<windows_time::TimeSpan> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).RequestedPlaybackPosition)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
}
impl windows_core::RuntimeType for PlaybackPositionChangeRequestedEventArgs {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, IPlaybackPositionChangeRequestedEventArgs>();
}
unsafe impl windows_core::Interface for PlaybackPositionChangeRequestedEventArgs {
  type Vtable = <IPlaybackPositionChangeRequestedEventArgs as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID =
    <IPlaybackPositionChangeRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlaybackPositionChangeRequestedEventArgs {
  const NAME: &'static str = "Windows.Media.PlaybackPositionChangeRequestedEventArgs";
}
unsafe impl Send for PlaybackPositionChangeRequestedEventArgs {}
unsafe impl Sync for PlaybackPositionChangeRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaybackRateChangeRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  PlaybackRateChangeRequestedEventArgs,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl windows_core::RuntimeType for PlaybackRateChangeRequestedEventArgs {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, IPlaybackRateChangeRequestedEventArgs>();
}
unsafe impl windows_core::Interface for PlaybackRateChangeRequestedEventArgs {
  type Vtable = <IPlaybackRateChangeRequestedEventArgs as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID =
    <IPlaybackRateChangeRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlaybackRateChangeRequestedEventArgs {
  const NAME: &'static str = "Windows.Media.PlaybackRateChangeRequestedEventArgs";
}
unsafe impl Send for PlaybackRateChangeRequestedEventArgs {}
unsafe impl Sync for PlaybackRateChangeRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RandomAccessStreamReference(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  RandomAccessStreamReference,
  windows_core::IUnknown,
  windows_core::IInspectable,
  IRandomAccessStreamReference
);
impl RandomAccessStreamReference {
  pub fn CreateFromFile<P0>(file: P0) -> windows_core::Result<Self>
  where
    P0: windows_core::Param<IStorageFile>,
  {
    Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).CreateFromFile)(
        windows_core::Interface::as_raw(this),
        file.param().abi(),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  pub fn CreateFromUri<P0>(uri: P0) -> windows_core::Result<Self>
  where
    P0: windows_core::Param<Uri>,
  {
    Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).CreateFromUri)(
        windows_core::Interface::as_raw(this),
        uri.param().abi(),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  pub fn CreateFromStream<P0>(stream: P0) -> windows_core::Result<Self>
  where
    P0: windows_core::Param<IRandomAccessStream>,
  {
    Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).CreateFromStream)(
        windows_core::Interface::as_raw(this),
        stream.param().abi(),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  fn IRandomAccessStreamReferenceStatics<
    R,
    F: FnOnce(&IRandomAccessStreamReferenceStatics) -> windows_core::Result<R>,
  >(
    callback: F,
  ) -> windows_core::Result<R> {
    static SHARED: windows_core::imp::FactoryCache<
      RandomAccessStreamReference,
      IRandomAccessStreamReferenceStatics,
    > = windows_core::imp::FactoryCache::new();
    SHARED.call(callback)
  }
}
impl windows_core::RuntimeType for RandomAccessStreamReference {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, IRandomAccessStreamReference>();
}
unsafe impl windows_core::Interface for RandomAccessStreamReference {
  type Vtable = <IRandomAccessStreamReference as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID = <IRandomAccessStreamReference as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RandomAccessStreamReference {
  const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStreamReference";
}
unsafe impl Send for RandomAccessStreamReference {}
unsafe impl Sync for RandomAccessStreamReference {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShuffleEnabledChangeRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  ShuffleEnabledChangeRequestedEventArgs,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl ShuffleEnabledChangeRequestedEventArgs {
  pub fn RequestedShuffleEnabled(&self) -> windows_core::Result<bool> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).RequestedShuffleEnabled)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
}
impl windows_core::RuntimeType for ShuffleEnabledChangeRequestedEventArgs {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, IShuffleEnabledChangeRequestedEventArgs>();
}
unsafe impl windows_core::Interface for ShuffleEnabledChangeRequestedEventArgs {
  type Vtable = <IShuffleEnabledChangeRequestedEventArgs as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID =
    <IShuffleEnabledChangeRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ShuffleEnabledChangeRequestedEventArgs {
  const NAME: &'static str = "Windows.Media.ShuffleEnabledChangeRequestedEventArgs";
}
unsafe impl Send for ShuffleEnabledChangeRequestedEventArgs {}
unsafe impl Sync for ShuffleEnabledChangeRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SoundLevel(pub i32);
impl SoundLevel {
  pub const Muted: Self = Self(0);
  pub const Low: Self = Self(1);
  pub const Full: Self = Self(2);
}
impl windows_core::imp::TypeKind for SoundLevel {
  type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for SoundLevel {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SoundLevel;i4)");
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.SoundLevel");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageFile(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  StorageFile,
  windows_core::IUnknown,
  windows_core::IInspectable,
  IStorageFile
);
impl StorageFile {
  pub fn GetFileFromPathAsync(
    path: &windows_core::HSTRING,
  ) -> windows_core::Result<windows_future::IAsyncOperation<Self>> {
    Self::IStorageFileStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).GetFileFromPathAsync)(
        windows_core::Interface::as_raw(this),
        core::mem::transmute_copy(path),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  pub fn GetFileFromApplicationUriAsync<P0>(
    uri: P0,
  ) -> windows_core::Result<windows_future::IAsyncOperation<Self>>
  where
    P0: windows_core::Param<Uri>,
  {
    Self::IStorageFileStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).GetFileFromApplicationUriAsync)(
        windows_core::Interface::as_raw(this),
        uri.param().abi(),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  pub fn CreateStreamedFileAsync<P1, P2>(
    displaynamewithextension: &windows_core::HSTRING,
    datarequested: P1,
    thumbnail: P2,
  ) -> windows_core::Result<windows_future::IAsyncOperation<Self>>
  where
    P1: windows_core::Param<StreamedFileDataRequestedHandler>,
    P2: windows_core::Param<IRandomAccessStreamReference>,
  {
    Self::IStorageFileStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).CreateStreamedFileAsync)(
        windows_core::Interface::as_raw(this),
        core::mem::transmute_copy(displaynamewithextension),
        datarequested.param().abi(),
        thumbnail.param().abi(),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  pub fn ReplaceWithStreamedFileAsync<P0, P1, P2>(
    filetoreplace: P0,
    datarequested: P1,
    thumbnail: P2,
  ) -> windows_core::Result<windows_future::IAsyncOperation<Self>>
  where
    P0: windows_core::Param<IStorageFile>,
    P1: windows_core::Param<StreamedFileDataRequestedHandler>,
    P2: windows_core::Param<IRandomAccessStreamReference>,
  {
    Self::IStorageFileStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).ReplaceWithStreamedFileAsync)(
        windows_core::Interface::as_raw(this),
        filetoreplace.param().abi(),
        datarequested.param().abi(),
        thumbnail.param().abi(),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  pub fn CreateStreamedFileFromUriAsync<P1, P2>(
    displaynamewithextension: &windows_core::HSTRING,
    uri: P1,
    thumbnail: P2,
  ) -> windows_core::Result<windows_future::IAsyncOperation<Self>>
  where
    P1: windows_core::Param<Uri>,
    P2: windows_core::Param<IRandomAccessStreamReference>,
  {
    Self::IStorageFileStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).CreateStreamedFileFromUriAsync)(
        windows_core::Interface::as_raw(this),
        core::mem::transmute_copy(displaynamewithextension),
        uri.param().abi(),
        thumbnail.param().abi(),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  pub fn ReplaceWithStreamedFileFromUriAsync<P0, P1, P2>(
    filetoreplace: P0,
    uri: P1,
    thumbnail: P2,
  ) -> windows_core::Result<windows_future::IAsyncOperation<Self>>
  where
    P0: windows_core::Param<IStorageFile>,
    P1: windows_core::Param<Uri>,
    P2: windows_core::Param<IRandomAccessStreamReference>,
  {
    Self::IStorageFileStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).ReplaceWithStreamedFileFromUriAsync)(
        windows_core::Interface::as_raw(this),
        filetoreplace.param().abi(),
        uri.param().abi(),
        thumbnail.param().abi(),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  pub fn GetFileFromPathForUserAsync<P0>(
    user: P0,
    path: &windows_core::HSTRING,
  ) -> windows_core::Result<windows_future::IAsyncOperation<Self>>
  where
    P0: windows_core::Param<User>,
  {
    Self::IStorageFileStatics2(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).GetFileFromPathForUserAsync)(
        windows_core::Interface::as_raw(this),
        user.param().abi(),
        core::mem::transmute_copy(path),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  fn IStorageFileStatics<R, F: FnOnce(&IStorageFileStatics) -> windows_core::Result<R>>(
    callback: F,
  ) -> windows_core::Result<R> {
    static SHARED: windows_core::imp::FactoryCache<StorageFile, IStorageFileStatics> =
      windows_core::imp::FactoryCache::new();
    SHARED.call(callback)
  }
  fn IStorageFileStatics2<R, F: FnOnce(&IStorageFileStatics2) -> windows_core::Result<R>>(
    callback: F,
  ) -> windows_core::Result<R> {
    static SHARED: windows_core::imp::FactoryCache<StorageFile, IStorageFileStatics2> =
      windows_core::imp::FactoryCache::new();
    SHARED.call(callback)
  }
}
impl windows_core::RuntimeType for StorageFile {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, IStorageFile>();
}
unsafe impl windows_core::Interface for StorageFile {
  type Vtable = <IStorageFile as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID = <IStorageFile as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageFile {
  const NAME: &'static str = "Windows.Storage.StorageFile";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamedFileDataRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  StreamedFileDataRequest,
  windows_core::IUnknown,
  windows_core::IInspectable,
  IOutputStream
);
impl windows_core::RuntimeType for StreamedFileDataRequest {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, IOutputStream>();
}
unsafe impl windows_core::Interface for StreamedFileDataRequest {
  type Vtable = <IOutputStream as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID = <IOutputStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StreamedFileDataRequest {
  const NAME: &'static str = "Windows.Storage.StreamedFileDataRequest";
}
windows_core::imp::define_interface!(
  StreamedFileDataRequestedHandler,
  StreamedFileDataRequestedHandler_Vtbl,
  0xfef6a824_2fe1_4d07_a35b_b77c50b5f4cc
);
impl windows_core::RuntimeType for StreamedFileDataRequestedHandler {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl StreamedFileDataRequestedHandler {
  pub fn new<
    F: Fn(windows_core::Ref<StreamedFileDataRequest>) -> windows_core::Result<()> + Send + 'static,
  >(
    invoke: F,
  ) -> Self {
    let com = windows_core::imp::DelegateBox::<Self, F>::new(
      &StreamedFileDataRequestedHandlerBox::<F>::VTABLE,
      invoke,
    );
    unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
  }
  pub fn Invoke<P0>(&self, stream: P0) -> windows_core::Result<()>
  where
    P0: windows_core::Param<StreamedFileDataRequest>,
  {
    unsafe {
      (windows_core::Interface::vtable(self).Invoke)(
        windows_core::Interface::as_raw(self),
        stream.param().abi(),
      )
      .ok()
    }
  }
}
#[repr(C)]
pub struct StreamedFileDataRequestedHandler_Vtbl {
  base__: windows_core::IUnknown_Vtbl,
  Invoke: unsafe extern "system" fn(
    this: *mut core::ffi::c_void,
    stream: *mut core::ffi::c_void,
  ) -> windows_core::HRESULT,
}
struct StreamedFileDataRequestedHandlerBox<
  F: Fn(windows_core::Ref<StreamedFileDataRequest>) -> windows_core::Result<()> + Send + 'static,
>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<StreamedFileDataRequest>) -> windows_core::Result<()> + Send + 'static>
  StreamedFileDataRequestedHandlerBox<F>
{
  const VTABLE: StreamedFileDataRequestedHandler_Vtbl = StreamedFileDataRequestedHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl {
      QueryInterface:
        windows_core::imp::DelegateBox::<StreamedFileDataRequestedHandler, F>::QueryInterface,
      AddRef: windows_core::imp::DelegateBox::<StreamedFileDataRequestedHandler, F>::AddRef,
      Release: windows_core::imp::DelegateBox::<StreamedFileDataRequestedHandler, F>::Release,
    },
    Invoke: Self::Invoke,
  };
  unsafe extern "system" fn Invoke(
    this: *mut core::ffi::c_void,
    stream: *mut core::ffi::c_void,
  ) -> windows_core::HRESULT {
    unsafe {
      let this = &mut *(this as *mut *mut core::ffi::c_void
        as *mut windows_core::imp::DelegateBox<StreamedFileDataRequestedHandler, F>);
      (this.invoke)(core::mem::transmute_copy(&stream)).into()
    }
  }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemMediaTransportControls(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  SystemMediaTransportControls,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl SystemMediaTransportControls {
  pub fn PlaybackStatus(&self) -> windows_core::Result<MediaPlaybackStatus> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).PlaybackStatus)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetPlaybackStatus(&self, value: MediaPlaybackStatus) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetPlaybackStatus)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn DisplayUpdater(&self) -> windows_core::Result<SystemMediaTransportControlsDisplayUpdater> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).DisplayUpdater)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
  }
  pub fn SoundLevel(&self) -> windows_core::Result<SoundLevel> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).SoundLevel)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn IsEnabled(&self) -> windows_core::Result<bool> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).IsEnabled)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetIsEnabled)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn IsPlayEnabled(&self) -> windows_core::Result<bool> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).IsPlayEnabled)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetIsPlayEnabled(&self, value: bool) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetIsPlayEnabled)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn IsStopEnabled(&self) -> windows_core::Result<bool> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).IsStopEnabled)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetIsStopEnabled(&self, value: bool) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetIsStopEnabled)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn IsPauseEnabled(&self) -> windows_core::Result<bool> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).IsPauseEnabled)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetIsPauseEnabled(&self, value: bool) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetIsPauseEnabled)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn IsRecordEnabled(&self) -> windows_core::Result<bool> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).IsRecordEnabled)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetIsRecordEnabled(&self, value: bool) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetIsRecordEnabled)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn IsFastForwardEnabled(&self) -> windows_core::Result<bool> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).IsFastForwardEnabled)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetIsFastForwardEnabled(&self, value: bool) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetIsFastForwardEnabled)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn IsRewindEnabled(&self) -> windows_core::Result<bool> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).IsRewindEnabled)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetIsRewindEnabled(&self, value: bool) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetIsRewindEnabled)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn IsPreviousEnabled(&self) -> windows_core::Result<bool> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).IsPreviousEnabled)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetIsPreviousEnabled(&self, value: bool) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetIsPreviousEnabled)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn IsNextEnabled(&self) -> windows_core::Result<bool> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).IsNextEnabled)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetIsNextEnabled(&self, value: bool) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetIsNextEnabled)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn IsChannelUpEnabled(&self) -> windows_core::Result<bool> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).IsChannelUpEnabled)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetIsChannelUpEnabled(&self, value: bool) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetIsChannelUpEnabled)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn IsChannelDownEnabled(&self) -> windows_core::Result<bool> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).IsChannelDownEnabled)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetIsChannelDownEnabled(&self, value: bool) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetIsChannelDownEnabled)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn ButtonPressed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
  where
    F: Fn(
        windows_core::Ref<Self>,
        windows_core::Ref<SystemMediaTransportControlsButtonPressedEventArgs>,
      ) + Send
      + 'static,
  {
    let handler =
      <TypedEventHandler<Self, SystemMediaTransportControlsButtonPressedEventArgs>>::new(
        move |a0, a1| {
          handler(a0, a1);
          Ok(())
        },
      );
    unsafe {
      let mut result__ = core::mem::zeroed();
      let token__ = (windows_core::Interface::vtable(self).ButtonPressed)(
        windows_core::Interface::as_raw(self),
        windows_core::Interface::as_raw(&handler),
        &mut result__,
      )
      .map(|| result__)?;
      Ok(windows_core::EventRevoker::new(
        self.clone(),
        token__,
        windows_core::Interface::vtable(self).RemoveButtonPressed,
      ))
    }
  }
  pub fn PropertyChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
  where
    F: Fn(
        windows_core::Ref<Self>,
        windows_core::Ref<SystemMediaTransportControlsPropertyChangedEventArgs>,
      ) + Send
      + 'static,
  {
    let handler =
      <TypedEventHandler<Self, SystemMediaTransportControlsPropertyChangedEventArgs>>::new(
        move |a0, a1| {
          handler(a0, a1);
          Ok(())
        },
      );
    unsafe {
      let mut result__ = core::mem::zeroed();
      let token__ = (windows_core::Interface::vtable(self).PropertyChanged)(
        windows_core::Interface::as_raw(self),
        windows_core::Interface::as_raw(&handler),
        &mut result__,
      )
      .map(|| result__)?;
      Ok(windows_core::EventRevoker::new(
        self.clone(),
        token__,
        windows_core::Interface::vtable(self).RemovePropertyChanged,
      ))
    }
  }
  pub fn AutoRepeatMode(&self) -> windows_core::Result<MediaPlaybackAutoRepeatMode> {
    let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).AutoRepeatMode)(
        windows_core::Interface::as_raw(this),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetAutoRepeatMode(&self, value: MediaPlaybackAutoRepeatMode) -> windows_core::Result<()> {
    let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
    unsafe {
      (windows_core::Interface::vtable(this).SetAutoRepeatMode)(
        windows_core::Interface::as_raw(this),
        value,
      )
      .ok()
    }
  }
  pub fn ShuffleEnabled(&self) -> windows_core::Result<bool> {
    let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).ShuffleEnabled)(
        windows_core::Interface::as_raw(this),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetShuffleEnabled(&self, value: bool) -> windows_core::Result<()> {
    let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
    unsafe {
      (windows_core::Interface::vtable(this).SetShuffleEnabled)(
        windows_core::Interface::as_raw(this),
        value,
      )
      .ok()
    }
  }
  pub fn PlaybackRate(&self) -> windows_core::Result<f64> {
    let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).PlaybackRate)(
        windows_core::Interface::as_raw(this),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetPlaybackRate(&self, value: f64) -> windows_core::Result<()> {
    let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
    unsafe {
      (windows_core::Interface::vtable(this).SetPlaybackRate)(
        windows_core::Interface::as_raw(this),
        value,
      )
      .ok()
    }
  }
  pub fn UpdateTimelineProperties<P0>(&self, timelineproperties: P0) -> windows_core::Result<()>
  where
    P0: windows_core::Param<SystemMediaTransportControlsTimelineProperties>,
  {
    let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
    unsafe {
      (windows_core::Interface::vtable(this).UpdateTimelineProperties)(
        windows_core::Interface::as_raw(this),
        timelineproperties.param().abi(),
      )
      .ok()
    }
  }
  pub fn PlaybackPositionChangeRequested<F>(
    &self,
    handler: F,
  ) -> windows_core::Result<windows_core::EventRevoker>
  where
    F: Fn(windows_core::Ref<Self>, windows_core::Ref<PlaybackPositionChangeRequestedEventArgs>)
      + Send
      + 'static,
  {
    let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
    let handler =
      <TypedEventHandler<Self, PlaybackPositionChangeRequestedEventArgs>>::new(move |a0, a1| {
        handler(a0, a1);
        Ok(())
      });
    unsafe {
      let mut result__ = core::mem::zeroed();
      let token__ = (windows_core::Interface::vtable(this).PlaybackPositionChangeRequested)(
        windows_core::Interface::as_raw(this),
        windows_core::Interface::as_raw(&handler),
        &mut result__,
      )
      .map(|| result__)?;
      Ok(windows_core::EventRevoker::new(
        this.clone(),
        token__,
        windows_core::Interface::vtable(this).RemovePlaybackPositionChangeRequested,
      ))
    }
  }
  pub fn PlaybackRateChangeRequested<F>(
    &self,
    handler: F,
  ) -> windows_core::Result<windows_core::EventRevoker>
  where
    F: Fn(windows_core::Ref<Self>, windows_core::Ref<PlaybackRateChangeRequestedEventArgs>)
      + Send
      + 'static,
  {
    let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
    let handler =
      <TypedEventHandler<Self, PlaybackRateChangeRequestedEventArgs>>::new(move |a0, a1| {
        handler(a0, a1);
        Ok(())
      });
    unsafe {
      let mut result__ = core::mem::zeroed();
      let token__ = (windows_core::Interface::vtable(this).PlaybackRateChangeRequested)(
        windows_core::Interface::as_raw(this),
        windows_core::Interface::as_raw(&handler),
        &mut result__,
      )
      .map(|| result__)?;
      Ok(windows_core::EventRevoker::new(
        this.clone(),
        token__,
        windows_core::Interface::vtable(this).RemovePlaybackRateChangeRequested,
      ))
    }
  }
  pub fn ShuffleEnabledChangeRequested<F>(
    &self,
    handler: F,
  ) -> windows_core::Result<windows_core::EventRevoker>
  where
    F: Fn(windows_core::Ref<Self>, windows_core::Ref<ShuffleEnabledChangeRequestedEventArgs>)
      + Send
      + 'static,
  {
    let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
    let handler =
      <TypedEventHandler<Self, ShuffleEnabledChangeRequestedEventArgs>>::new(move |a0, a1| {
        handler(a0, a1);
        Ok(())
      });
    unsafe {
      let mut result__ = core::mem::zeroed();
      let token__ = (windows_core::Interface::vtable(this).ShuffleEnabledChangeRequested)(
        windows_core::Interface::as_raw(this),
        windows_core::Interface::as_raw(&handler),
        &mut result__,
      )
      .map(|| result__)?;
      Ok(windows_core::EventRevoker::new(
        this.clone(),
        token__,
        windows_core::Interface::vtable(this).RemoveShuffleEnabledChangeRequested,
      ))
    }
  }
  pub fn AutoRepeatModeChangeRequested<F>(
    &self,
    handler: F,
  ) -> windows_core::Result<windows_core::EventRevoker>
  where
    F: Fn(windows_core::Ref<Self>, windows_core::Ref<AutoRepeatModeChangeRequestedEventArgs>)
      + Send
      + 'static,
  {
    let this = &windows_core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
    let handler =
      <TypedEventHandler<Self, AutoRepeatModeChangeRequestedEventArgs>>::new(move |a0, a1| {
        handler(a0, a1);
        Ok(())
      });
    unsafe {
      let mut result__ = core::mem::zeroed();
      let token__ = (windows_core::Interface::vtable(this).AutoRepeatModeChangeRequested)(
        windows_core::Interface::as_raw(this),
        windows_core::Interface::as_raw(&handler),
        &mut result__,
      )
      .map(|| result__)?;
      Ok(windows_core::EventRevoker::new(
        this.clone(),
        token__,
        windows_core::Interface::vtable(this).RemoveAutoRepeatModeChangeRequested,
      ))
    }
  }
  fn ISystemMediaTransportControlsStatics<
    R,
    F: FnOnce(&ISystemMediaTransportControlsStatics) -> windows_core::Result<R>,
  >(
    callback: F,
  ) -> windows_core::Result<R> {
    static SHARED: windows_core::imp::FactoryCache<
      SystemMediaTransportControls,
      ISystemMediaTransportControlsStatics,
    > = windows_core::imp::FactoryCache::new();
    SHARED.call(callback)
  }
}
impl windows_core::RuntimeType for SystemMediaTransportControls {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, ISystemMediaTransportControls>();
}
unsafe impl windows_core::Interface for SystemMediaTransportControls {
  type Vtable = <ISystemMediaTransportControls as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID = <ISystemMediaTransportControls as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemMediaTransportControls {
  const NAME: &'static str = "Windows.Media.SystemMediaTransportControls";
}
unsafe impl Send for SystemMediaTransportControls {}
unsafe impl Sync for SystemMediaTransportControls {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SystemMediaTransportControlsButton(pub i32);
impl SystemMediaTransportControlsButton {
  pub const Play: Self = Self(0);
  pub const Pause: Self = Self(1);
  pub const Stop: Self = Self(2);
  pub const Record: Self = Self(3);
  pub const FastForward: Self = Self(4);
  pub const Rewind: Self = Self(5);
  pub const Next: Self = Self(6);
  pub const Previous: Self = Self(7);
  pub const ChannelUp: Self = Self(8);
  pub const ChannelDown: Self = Self(9);
}
impl windows_core::imp::TypeKind for SystemMediaTransportControlsButton {
  type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for SystemMediaTransportControlsButton {
  const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
    b"enum(Windows.Media.SystemMediaTransportControlsButton;i4)",
  );
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.SystemMediaTransportControlsButton");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemMediaTransportControlsButtonPressedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  SystemMediaTransportControlsButtonPressedEventArgs,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl SystemMediaTransportControlsButtonPressedEventArgs {
  pub fn Button(&self) -> windows_core::Result<SystemMediaTransportControlsButton> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).Button)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
}
impl windows_core::RuntimeType for SystemMediaTransportControlsButtonPressedEventArgs {
  const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
    Self,
    ISystemMediaTransportControlsButtonPressedEventArgs,
  >();
}
unsafe impl windows_core::Interface for SystemMediaTransportControlsButtonPressedEventArgs {
  type Vtable =
    <ISystemMediaTransportControlsButtonPressedEventArgs as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID =
    <ISystemMediaTransportControlsButtonPressedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemMediaTransportControlsButtonPressedEventArgs {
  const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs";
}
unsafe impl Send for SystemMediaTransportControlsButtonPressedEventArgs {}
unsafe impl Sync for SystemMediaTransportControlsButtonPressedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemMediaTransportControlsDisplayUpdater(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  SystemMediaTransportControlsDisplayUpdater,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl SystemMediaTransportControlsDisplayUpdater {
  pub fn Type(&self) -> windows_core::Result<MediaPlaybackType> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).Type)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetType(&self, value: MediaPlaybackType) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), value)
        .ok()
    }
  }
  pub fn AppMediaId(&self) -> windows_core::Result<windows_core::HSTRING> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).AppMediaId)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| core::mem::transmute(result__))
    }
  }
  pub fn SetAppMediaId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetAppMediaId)(
        windows_core::Interface::as_raw(self),
        core::mem::transmute_copy(value),
      )
      .ok()
    }
  }
  pub fn Thumbnail(&self) -> windows_core::Result<RandomAccessStreamReference> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).Thumbnail)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
  }
  pub fn SetThumbnail<P0>(&self, value: P0) -> windows_core::Result<()>
  where
    P0: windows_core::Param<RandomAccessStreamReference>,
  {
    unsafe {
      (windows_core::Interface::vtable(self).SetThumbnail)(
        windows_core::Interface::as_raw(self),
        value.param().abi(),
      )
      .ok()
    }
  }
  pub fn MusicProperties(&self) -> windows_core::Result<MusicDisplayProperties> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).MusicProperties)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
  }
  pub fn VideoProperties(&self) -> windows_core::Result<VideoDisplayProperties> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).VideoProperties)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
  }
  pub fn ImageProperties(&self) -> windows_core::Result<ImageDisplayProperties> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).ImageProperties)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
  }
  pub fn ClearAll(&self) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).ClearAll)(windows_core::Interface::as_raw(self)).ok()
    }
  }
  pub fn Update(&self) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self)).ok()
    }
  }
}
impl windows_core::RuntimeType for SystemMediaTransportControlsDisplayUpdater {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, ISystemMediaTransportControlsDisplayUpdater>(
    );
}
unsafe impl windows_core::Interface for SystemMediaTransportControlsDisplayUpdater {
  type Vtable = <ISystemMediaTransportControlsDisplayUpdater as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID =
    <ISystemMediaTransportControlsDisplayUpdater as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemMediaTransportControlsDisplayUpdater {
  const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsDisplayUpdater";
}
unsafe impl Send for SystemMediaTransportControlsDisplayUpdater {}
unsafe impl Sync for SystemMediaTransportControlsDisplayUpdater {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemMediaTransportControlsPropertyChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  SystemMediaTransportControlsPropertyChangedEventArgs,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl windows_core::RuntimeType for SystemMediaTransportControlsPropertyChangedEventArgs {
  const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
    Self,
    ISystemMediaTransportControlsPropertyChangedEventArgs,
  >();
}
unsafe impl windows_core::Interface for SystemMediaTransportControlsPropertyChangedEventArgs {
  type Vtable =
    <ISystemMediaTransportControlsPropertyChangedEventArgs as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID =
    <ISystemMediaTransportControlsPropertyChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemMediaTransportControlsPropertyChangedEventArgs {
  const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs";
}
unsafe impl Send for SystemMediaTransportControlsPropertyChangedEventArgs {}
unsafe impl Sync for SystemMediaTransportControlsPropertyChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemMediaTransportControlsTimelineProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  SystemMediaTransportControlsTimelineProperties,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl SystemMediaTransportControlsTimelineProperties {
  pub fn new() -> windows_core::Result<Self> {
    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
  }
  fn IActivationFactory<
    R,
    F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
  >(
    callback: F,
  ) -> windows_core::Result<R> {
    static SHARED: windows_core::imp::FactoryCache<
      SystemMediaTransportControlsTimelineProperties,
      windows_core::imp::IGenericFactory,
    > = windows_core::imp::FactoryCache::new();
    SHARED.call(callback)
  }
  pub fn StartTime(&self) -> windows_core::Result<windows_time::TimeSpan> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).StartTime)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetStartTime(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetStartTime)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn EndTime(&self) -> windows_core::Result<windows_time::TimeSpan> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).EndTime)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetEndTime(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetEndTime)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn MinSeekTime(&self) -> windows_core::Result<windows_time::TimeSpan> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).MinSeekTime)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetMinSeekTime(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetMinSeekTime)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn MaxSeekTime(&self) -> windows_core::Result<windows_time::TimeSpan> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).MaxSeekTime)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetMaxSeekTime(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetMaxSeekTime)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
  pub fn Position(&self) -> windows_core::Result<windows_time::TimeSpan> {
    unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(self).Position)(
        windows_core::Interface::as_raw(self),
        &mut result__,
      )
      .map(|| result__)
    }
  }
  pub fn SetPosition(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
    unsafe {
      (windows_core::Interface::vtable(self).SetPosition)(
        windows_core::Interface::as_raw(self),
        value,
      )
      .ok()
    }
  }
}
impl windows_core::RuntimeType for SystemMediaTransportControlsTimelineProperties {
  const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
    Self,
    ISystemMediaTransportControlsTimelineProperties,
  >();
}
unsafe impl windows_core::Interface for SystemMediaTransportControlsTimelineProperties {
  type Vtable =
    <ISystemMediaTransportControlsTimelineProperties as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID =
    <ISystemMediaTransportControlsTimelineProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemMediaTransportControlsTimelineProperties {
  const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsTimelineProperties";
}
unsafe impl Send for SystemMediaTransportControlsTimelineProperties {}
unsafe impl Sync for SystemMediaTransportControlsTimelineProperties {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedEventHandler<TSender, TResult>(
  windows_core::IUnknown,
  core::marker::PhantomData<TSender>,
  core::marker::PhantomData<TResult>,
)
where
  TSender: windows_core::RuntimeType + 'static,
  TResult: windows_core::RuntimeType + 'static;
unsafe impl<
  TSender: windows_core::RuntimeType + 'static,
  TResult: windows_core::RuntimeType + 'static,
> windows_core::Interface for TypedEventHandler<TSender, TResult>
{
  type Vtable = TypedEventHandler_Vtbl<TSender, TResult>;
  const IID: windows_core::GUID =
    windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static>
  windows_core::RuntimeType for TypedEventHandler<TSender, TResult>
{
  const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
    .push_slice(b"pinterface({9de1c534-6ae1-11e0-84e1-18a905bcc53f}")
    .push_slice(b";")
    .push_other(TSender::SIGNATURE)
    .push_slice(b";")
    .push_other(TResult::SIGNATURE)
    .push_slice(b")");
}
impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static>
  TypedEventHandler<TSender, TResult>
{
  pub fn new<
    F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) -> windows_core::Result<()>
      + Send
      + 'static,
  >(
    invoke: F,
  ) -> Self {
    let com = windows_core::imp::DelegateBox::<Self, F>::new(
      &TypedEventHandlerBox::<TSender, TResult, F>::VTABLE,
      invoke,
    );
    unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
  }
  pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
  where
    P0: windows_core::Param<TSender>,
    P1: windows_core::Param<TResult>,
  {
    unsafe {
      (windows_core::Interface::vtable(self).Invoke)(
        windows_core::Interface::as_raw(self),
        sender.param().abi(),
        args.param().abi(),
      )
      .ok()
    }
  }
}
#[repr(C)]
pub struct TypedEventHandler_Vtbl<TSender, TResult>
where
  TSender: windows_core::RuntimeType + 'static,
  TResult: windows_core::RuntimeType + 'static,
{
  base__: windows_core::IUnknown_Vtbl,
  Invoke: unsafe extern "system" fn(
    this: *mut core::ffi::c_void,
    sender: windows_core::imp::AbiType<TSender>,
    args: windows_core::imp::AbiType<TResult>,
  ) -> windows_core::HRESULT,
  TSender: core::marker::PhantomData<TSender>,
  TResult: core::marker::PhantomData<TResult>,
}
struct TypedEventHandlerBox<
  TSender,
  TResult,
  F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) -> windows_core::Result<()>
    + Send
    + 'static,
>(core::marker::PhantomData<(TSender, TResult, fn() -> F)>)
where
  TSender: windows_core::RuntimeType + 'static,
  TResult: windows_core::RuntimeType + 'static;
impl<
  TSender: windows_core::RuntimeType + 'static,
  TResult: windows_core::RuntimeType + 'static,
  F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) -> windows_core::Result<()>
    + Send
    + 'static,
> TypedEventHandlerBox<TSender, TResult, F>
{
  const VTABLE: TypedEventHandler_Vtbl<TSender, TResult> =
    TypedEventHandler_Vtbl::<TSender, TResult> {
      base__: windows_core::IUnknown_Vtbl {
        QueryInterface:
          windows_core::imp::DelegateBox::<TypedEventHandler<TSender, TResult>, F>::QueryInterface,
        AddRef: windows_core::imp::DelegateBox::<TypedEventHandler<TSender, TResult>, F>::AddRef,
        Release: windows_core::imp::DelegateBox::<TypedEventHandler<TSender, TResult>, F>::Release,
      },
      Invoke: Self::Invoke,
      TSender: core::marker::PhantomData::<TSender>,
      TResult: core::marker::PhantomData::<TResult>,
    };
  unsafe extern "system" fn Invoke(
    this: *mut core::ffi::c_void,
    sender: windows_core::imp::AbiType<TSender>,
    args: windows_core::imp::AbiType<TResult>,
  ) -> windows_core::HRESULT {
    unsafe {
      let this = &mut *(this as *mut *mut core::ffi::c_void
        as *mut windows_core::imp::DelegateBox<TypedEventHandler<TSender, TResult>, F>);
      (this.invoke)(
        core::mem::transmute_copy(&sender),
        core::mem::transmute_copy(&args),
      )
      .into()
    }
  }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uri(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Uri, windows_core::IUnknown, windows_core::IInspectable);
impl Uri {
  pub fn UnescapeComponent(
    tounescape: &windows_core::HSTRING,
  ) -> windows_core::Result<windows_core::HSTRING> {
    Self::IUriEscapeStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).UnescapeComponent)(
        windows_core::Interface::as_raw(this),
        core::mem::transmute_copy(tounescape),
        &mut result__,
      )
      .map(|| core::mem::transmute(result__))
    })
  }
  pub fn EscapeComponent(
    toescape: &windows_core::HSTRING,
  ) -> windows_core::Result<windows_core::HSTRING> {
    Self::IUriEscapeStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).EscapeComponent)(
        windows_core::Interface::as_raw(this),
        core::mem::transmute_copy(toescape),
        &mut result__,
      )
      .map(|| core::mem::transmute(result__))
    })
  }
  pub fn CreateUri(uri: &windows_core::HSTRING) -> windows_core::Result<Self> {
    Self::IUriRuntimeClassFactory(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).CreateUri)(
        windows_core::Interface::as_raw(this),
        core::mem::transmute_copy(uri),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  pub fn CreateWithRelativeUri(
    baseuri: &windows_core::HSTRING,
    relativeuri: &windows_core::HSTRING,
  ) -> windows_core::Result<Self> {
    Self::IUriRuntimeClassFactory(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).CreateWithRelativeUri)(
        windows_core::Interface::as_raw(this),
        core::mem::transmute_copy(baseuri),
        core::mem::transmute_copy(relativeuri),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  fn IUriEscapeStatics<R, F: FnOnce(&IUriEscapeStatics) -> windows_core::Result<R>>(
    callback: F,
  ) -> windows_core::Result<R> {
    static SHARED: windows_core::imp::FactoryCache<Uri, IUriEscapeStatics> =
      windows_core::imp::FactoryCache::new();
    SHARED.call(callback)
  }
  fn IUriRuntimeClassFactory<R, F: FnOnce(&IUriRuntimeClassFactory) -> windows_core::Result<R>>(
    callback: F,
  ) -> windows_core::Result<R> {
    static SHARED: windows_core::imp::FactoryCache<Uri, IUriRuntimeClassFactory> =
      windows_core::imp::FactoryCache::new();
    SHARED.call(callback)
  }
}
impl windows_core::RuntimeType for Uri {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, IUriRuntimeClass>();
}
unsafe impl windows_core::Interface for Uri {
  type Vtable = <IUriRuntimeClass as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID = <IUriRuntimeClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Uri {
  const NAME: &'static str = "Windows.Foundation.Uri";
}
unsafe impl Send for Uri {}
unsafe impl Sync for Uri {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(User, windows_core::IUnknown, windows_core::IInspectable);
impl User {
  pub fn CreateWatcher() -> windows_core::Result<UserWatcher> {
    Self::IUserStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).CreateWatcher)(
        windows_core::Interface::as_raw(this),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  pub fn FindAllAsync()
  -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<Self>>>
  {
    Self::IUserStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).FindAllAsync)(
        windows_core::Interface::as_raw(this),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  pub fn FindAllAsyncByType(
    r#type: UserType,
  ) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<Self>>>
  {
    Self::IUserStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).FindAllAsyncByType)(
        windows_core::Interface::as_raw(this),
        r#type,
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  pub fn FindAllAsyncByTypeAndStatus(
    r#type: UserType,
    status: UserAuthenticationStatus,
  ) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<Self>>>
  {
    Self::IUserStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).FindAllAsyncByTypeAndStatus)(
        windows_core::Interface::as_raw(this),
        r#type,
        status,
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  pub fn GetFromId(nonroamableid: &windows_core::HSTRING) -> windows_core::Result<Self> {
    Self::IUserStatics(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).GetFromId)(
        windows_core::Interface::as_raw(this),
        core::mem::transmute_copy(nonroamableid),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  pub fn GetDefault() -> windows_core::Result<Self> {
    Self::IUserStatics2(|this| unsafe {
      let mut result__ = core::mem::zeroed();
      (windows_core::Interface::vtable(this).GetDefault)(
        windows_core::Interface::as_raw(this),
        &mut result__,
      )
      .and_then(|| windows_core::imp::Type::from_abi(result__))
    })
  }
  fn IUserStatics<R, F: FnOnce(&IUserStatics) -> windows_core::Result<R>>(
    callback: F,
  ) -> windows_core::Result<R> {
    static SHARED: windows_core::imp::FactoryCache<User, IUserStatics> =
      windows_core::imp::FactoryCache::new();
    SHARED.call(callback)
  }
  fn IUserStatics2<R, F: FnOnce(&IUserStatics2) -> windows_core::Result<R>>(
    callback: F,
  ) -> windows_core::Result<R> {
    static SHARED: windows_core::imp::FactoryCache<User, IUserStatics2> =
      windows_core::imp::FactoryCache::new();
    SHARED.call(callback)
  }
}
impl windows_core::RuntimeType for User {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, IUser>();
}
unsafe impl windows_core::Interface for User {
  type Vtable = <IUser as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID = <IUser as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for User {
  const NAME: &'static str = "Windows.System.User";
}
unsafe impl Send for User {}
unsafe impl Sync for User {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UserAuthenticationStatus(pub i32);
impl UserAuthenticationStatus {
  pub const Unauthenticated: Self = Self(0);
  pub const LocallyAuthenticated: Self = Self(1);
  pub const RemotelyAuthenticated: Self = Self(2);
}
impl windows_core::imp::TypeKind for UserAuthenticationStatus {
  type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for UserAuthenticationStatus {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.UserAuthenticationStatus;i4)");
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.System.UserAuthenticationStatus");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UserType(pub i32);
impl UserType {
  pub const LocalUser: Self = Self(0);
  pub const RemoteUser: Self = Self(1);
  pub const LocalGuest: Self = Self(2);
  pub const RemoteGuest: Self = Self(3);
  pub const SystemManaged: Self = Self(4);
}
impl windows_core::imp::TypeKind for UserType {
  type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for UserType {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.UserType;i4)");
  const NAME: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::from_slice(b"Windows.System.UserType");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserWatcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  UserWatcher,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl windows_core::RuntimeType for UserWatcher {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, IUserWatcher>();
}
unsafe impl windows_core::Interface for UserWatcher {
  type Vtable = <IUserWatcher as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID = <IUserWatcher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserWatcher {
  const NAME: &'static str = "Windows.System.UserWatcher";
}
unsafe impl Send for UserWatcher {}
unsafe impl Sync for UserWatcher {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoDisplayProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
  VideoDisplayProperties,
  windows_core::IUnknown,
  windows_core::IInspectable
);
impl windows_core::RuntimeType for VideoDisplayProperties {
  const SIGNATURE: windows_core::imp::ConstBuffer =
    windows_core::imp::ConstBuffer::for_class::<Self, IVideoDisplayProperties>();
}
unsafe impl windows_core::Interface for VideoDisplayProperties {
  type Vtable = <IVideoDisplayProperties as windows_core::Interface>::Vtable;
  const IID: windows_core::GUID = <IVideoDisplayProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VideoDisplayProperties {
  const NAME: &'static str = "Windows.Media.VideoDisplayProperties";
}
unsafe impl Send for VideoDisplayProperties {}
unsafe impl Sync for VideoDisplayProperties {}

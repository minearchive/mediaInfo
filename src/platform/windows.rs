#[cfg(target_os = "windows")]
use {
    crate::{MediaInfo, PlaybackState},
    std::{
        env,
        fs::File,
        io::Write
    },
    windows::{
        core::HSTRING,
        Media::{
            MediaPlaybackAutoRepeatMode,
            Control::{
                GlobalSystemMediaTransportControlsSessionManager,
                GlobalSystemMediaTransportControlsSessionMediaProperties,
                GlobalSystemMediaTransportControlsSessionPlaybackStatus
            }
        },
        Storage::{
            CreationCollisionOption,
            StorageFolder,
            Streams::{
                Buffer,
                DataReader,
                InputStreamOptions
            }
        }
    }
};

#[cfg(target_os = "windows")]
pub fn get_media_info() -> MediaInfo {
    let session = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap();

    let properties = session.TryGetMediaPropertiesAsync().unwrap().get().unwrap();

    MediaInfo::new(
        properties.Title().unwrap().to_string(),
        properties.Artist().unwrap().to_string(),
        properties.AlbumTitle().unwrap().to_string(),
        save_thumbnail_and_get_path(&properties, format!("{}_{}", properties.Title().unwrap().to_string(), properties.AlbumTitle().unwrap().to_string()))
    )
}

#[cfg(target_os = "windows")]
pub fn get_playback_state() -> PlaybackState {
    let session = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap();

    let timeline = &session.GetTimelineProperties().unwrap();
    let state = &session.GetPlaybackInfo().unwrap();

    PlaybackState::new(
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing == state.PlaybackStatus().unwrap(),
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Paused == state.PlaybackStatus().unwrap(),
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Stopped == state.PlaybackStatus().unwrap(),
        timeline.Position().unwrap().Duration,
        timeline.MaxSeekTime().unwrap().Duration,
        state.Controls().unwrap().IsPlayEnabled().unwrap(),
        state.Controls().unwrap().IsPauseEnabled().unwrap(),
        state.Controls().unwrap().IsStopEnabled().unwrap(),
        state.Controls().unwrap().IsRecordEnabled().unwrap(),
        state.Controls().unwrap().IsFastForwardEnabled().unwrap(),
        state.Controls().unwrap().IsRewindEnabled().unwrap(),
        state.Controls().unwrap().IsNextEnabled().unwrap(),
        state.Controls().unwrap().IsPreviousEnabled().unwrap(),
        state.Controls().unwrap().IsChannelUpEnabled().unwrap(),
        state.Controls().unwrap().IsChannelDownEnabled().unwrap(),
        state.Controls().unwrap().IsPlayPauseToggleEnabled().unwrap(),
        state.Controls().unwrap().IsShuffleEnabled().unwrap(),
        state.Controls().unwrap().IsRepeatEnabled().unwrap(),
        state.Controls().unwrap().IsPlaybackRateEnabled().unwrap(),
        state.Controls().unwrap().IsPlaybackPositionEnabled().unwrap(),
    )
}

#[cfg(target_os = "windows")]
pub fn try_play() {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryPlayAsync().expect("Error caused while trying play");
}

#[cfg(target_os = "windows")]
pub fn try_pause() {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryPauseAsync().expect("Error caused while trying pause async");
}

#[cfg(target_os = "windows")]
pub fn try_stop() {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryStopAsync().expect("Error caused while trying stop async");
}

#[cfg(target_os = "windows")]
pub fn try_record() {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryRecordAsync().expect("Error caused while trying record async");
}

#[cfg(target_os = "windows")]
pub fn try_fast_forward() {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryFastForwardAsync().expect("Error caused while trying fast_forward async");
}

#[cfg(target_os = "windows")]
pub fn try_rewind() {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryRewindAsync().expect("Error caused while trying rewind async");
}

#[cfg(target_os = "windows")]
pub fn try_next() {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TrySkipNextAsync().expect("Error caused while trying skip_next async");
}

#[cfg(target_os = "windows")]
pub fn try_previous() {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TrySkipPreviousAsync().expect("Error caused while trying skip_previous_async");
}

#[cfg(target_os = "windows")]
pub fn try_change_channel_up() {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryChangeChannelUpAsync().expect("Error caused while trying change channel_up_async");
}

#[cfg(target_os = "windows")]
pub fn try_change_channel_down() {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryChangeChannelDownAsync().expect("Error caused while trying change channel_down_async");
}

#[cfg(target_os = "windows")]
pub fn try_play_pause_toggle() {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryTogglePlayPauseAsync().expect("Error caused while trying to try to play pause toggle");
}

#[cfg(target_os = "windows")]
pub fn try_change_shuffle(shuffle: bool) {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryChangeShuffleActiveAsync(shuffle).expect("Error caused while trying to try to change shuffle active async");
}

#[cfg(target_os = "windows")]
pub fn try_change_repeat(repeat: MediaPlaybackAutoRepeatMode) {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryChangeAutoRepeatModeAsync(repeat).unwrap().get().expect("Error caused while trying to try to change autorepeat mode async");
}

#[cfg(target_os = "windows")]
pub fn try_change_playback_rate(i: i64) {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryChangePlaybackPositionAsync(i).expect("Error caused while trying to try to change playback position async");
}

#[cfg(target_os = "windows")]
pub fn try_change_playback_position(i: i64) {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryChangePlaybackPositionAsync(i).expect("Error caused while trying to try to change playback position async");
}

//noinspection Annotator
#[cfg(target_os = "windows")]
fn save_thumbnail_and_get_path(
    properties: &GlobalSystemMediaTransportControlsSessionMediaProperties,
    name: String,
) -> String {
    let thumbnail = properties.Thumbnail().unwrap();
    let cache_folder = StorageFolder::GetFolderFromPathAsync(&HSTRING::from(env::temp_dir().to_str().unwrap())).unwrap().get().unwrap();
    let file_name = format!("thumbnail_cache_{}.jpg", name);

    let file = cache_folder
        .CreateFileAsync(&file_name.into(), CreationCollisionOption::ReplaceExisting)
        .unwrap()
        .get()
        .unwrap();

    let input_stream = thumbnail.OpenReadAsync().unwrap().get().unwrap();
    let size = input_stream.Size().unwrap() as u32;

    let buffer = Buffer::Create(size).unwrap();
    input_stream
        .ReadAsync(&buffer, size, InputStreamOptions::None)
        .unwrap()
        .get()
        .unwrap();

    let data_reader = DataReader::FromBuffer(&buffer).unwrap();
    let mut bytes = vec![0u8; size as usize];
    data_reader.ReadBytes(&mut bytes).unwrap();

    File::create(file.Path().unwrap().to_string()).unwrap().write(bytes.as_slice()).unwrap();

    file.Path().unwrap().to_string()
}

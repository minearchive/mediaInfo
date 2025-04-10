use crate::{MediaInfo, PlaybackState};
use std::{
    env,
    fs::File,
    io::Write
};
use windows::{
    core::HSTRING,
    Media::Control::{
        GlobalSystemMediaTransportControlsSessionManager,
        GlobalSystemMediaTransportControlsSessionMediaProperties
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
};
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus;

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

//noinspection Annotator
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

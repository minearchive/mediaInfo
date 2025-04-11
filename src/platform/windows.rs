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
        if state.Controls().unwrap().IsShuffleEnabled().unwrap() { state.IsShuffleActive().unwrap().Value().unwrap() } else { false },
        if state.Controls().unwrap().IsRepeatEnabled().unwrap() { state.AutoRepeatMode().unwrap().Value().unwrap() == MediaPlaybackAutoRepeatMode::Track } else { false },
        if state.Controls().unwrap().IsRepeatEnabled().unwrap() { state.AutoRepeatMode().unwrap().Value().unwrap() == MediaPlaybackAutoRepeatMode::List } else { false },
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
pub fn try_play() -> bool {
    match GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryPlayAsync() {
        Ok(session) => {
            session.get().unwrap()
        },
        Err(_) => {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_pause() -> bool {
    match GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryPauseAsync() {
        Ok(session) => {
            session.get().unwrap()
        },
        Err(_) => {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_stop() -> bool {
    match GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryStopAsync() {
        Ok(session) => {
            session.get().unwrap()
        },
        Err(_) => {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_record() -> bool {
    match GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryRecordAsync() {
        Ok(session) => {
            session.get().unwrap()
        },
        Err(_) => {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_fast_forward() -> bool {
    match GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryFastForwardAsync() {
        Ok(session) => {
            session.get().unwrap()
        },
        Err(_) => {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_rewind() -> bool {
    match GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryRewindAsync() {
        Ok(session) => {
            session.get().unwrap()
        },
        Err(_) => {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_next() -> bool {
    match GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TrySkipNextAsync() {
        Ok(session) => {
            session.get().unwrap()
        },
        Err(_) => {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_previous() -> bool {
    match GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TrySkipPreviousAsync() {
        Ok(session) => {
            session.get().unwrap()
        },
        Err(_) => {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_change_channel_up() -> bool {
    match GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryChangeChannelUpAsync() {
        Ok(session) => {
            session.get().unwrap()
        },
        Err(_) => {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_change_channel_down() -> bool {
    match GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryChangeChannelDownAsync() {
        Ok(session) => {
            session.get().unwrap()
        },
        Err(_) => {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_play_pause_toggle() -> bool {
    match GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryTogglePlayPauseAsync() {
        Ok(session) => {
            session.get().unwrap()
        },
        Err(_) => {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_change_shuffle(shuffle: bool) -> bool {
    match GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryChangeShuffleActiveAsync(shuffle) {
        Ok(session) => {
            session.get().unwrap()
        },
        Err(_) => {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_change_repeat() -> bool {
    match GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryChangeAutoRepeatModeAsync(
        next(GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().GetPlaybackInfo().unwrap().AutoRepeatMode().unwrap().Value().unwrap())
    ) {
        Ok(session) => {
            session.get().unwrap()
        },
        Err(_) => {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_change_playback_rate(i: f64) -> bool {
    match GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryChangePlaybackRateAsync(i) {
        Ok(session) => {
            session.get().unwrap()
        },
        Err(_) => {
            false
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_change_playback_position(i: i64) -> bool {
    match GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap().TryChangePlaybackPositionAsync(i) {
        Ok(session) => {
            session.get().unwrap()
        },
        Err(_) => {
            false
        }
    }
}

//noinspection Annotator
#[cfg(target_os = "windows")]
fn save_thumbnail_and_get_path(
    properties: &GlobalSystemMediaTransportControlsSessionMediaProperties,
    name: String,
) -> String {
    let thumbnail = properties.Thumbnail().unwrap();
    let cache_folder = StorageFolder::GetFolderFromPathAsync(&HSTRING::from(env::temp_dir().to_str().unwrap())).unwrap().get().unwrap();
    let file_name = format!("thumbnail_cache_{}.jpg", name.replace("/", ""));

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

#[cfg(target_os = "windows")]
fn next(mode: MediaPlaybackAutoRepeatMode) -> MediaPlaybackAutoRepeatMode {
    if mode == MediaPlaybackAutoRepeatMode::None {
        MediaPlaybackAutoRepeatMode::Track
    } else if mode == MediaPlaybackAutoRepeatMode::Track {
        MediaPlaybackAutoRepeatMode::List
    } else if mode == MediaPlaybackAutoRepeatMode::List {
        MediaPlaybackAutoRepeatMode::Track
    } else {
        mode
    }
}
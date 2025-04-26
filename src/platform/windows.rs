#[cfg(target_os = "windows")]
use {
    anyhow::{
        Result
    },
    crate::{MediaInfo, PlaybackState},
    std::{
        env,
        fs::File,
        io::Write
    },
    windows::{
        core::HSTRING,
        Media::{
            Control::{
                GlobalSystemMediaTransportControlsSessionManager,
                GlobalSystemMediaTransportControlsSessionMediaProperties,
                GlobalSystemMediaTransportControlsSessionPlaybackStatus
            },
            MediaPlaybackAutoRepeatMode
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
    },
};

#[cfg(target_os = "windows")]
pub fn get_media_info() -> Result<MediaInfo>  {
    if unavailable() {
        Ok(MediaInfo::empty())
    } else {
        let session = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?;

        let properties = session.TryGetMediaPropertiesAsync()?.get()?;

        Ok(
            MediaInfo::new(
                properties.Title()?.to_string(),
                properties.Artist()?.to_string(),
                properties.AlbumTitle()?.to_string(),
                save_thumbnail_and_get_path(&properties, format!("{}_{}", properties.Title()?.to_string(), properties.AlbumTitle()?.to_string()))
            )
        )
    }
}

#[cfg(target_os = "windows")]
pub fn get_playback_state() -> Result<PlaybackState> {
    if unavailable() {
        Ok(PlaybackState::empty())
    } else {
        let session = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?;

        let timeline = &session.GetTimelineProperties()?;
        let state = &session.GetPlaybackInfo()?;

        Ok(
            PlaybackState::new(
                GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing == state.PlaybackStatus()?,
                GlobalSystemMediaTransportControlsSessionPlaybackStatus::Paused == state.PlaybackStatus()?,
                GlobalSystemMediaTransportControlsSessionPlaybackStatus::Stopped == state.PlaybackStatus()?,
                if state.Controls()?.IsShuffleEnabled()? { state.IsShuffleActive()?.Value()? } else { false },
                if state.Controls()?.IsRepeatEnabled()? { state.AutoRepeatMode()?.Value()? == MediaPlaybackAutoRepeatMode::Track } else { false },
                if state.Controls()?.IsRepeatEnabled()? { state.AutoRepeatMode()?.Value()? == MediaPlaybackAutoRepeatMode::List } else { false },
                timeline.Position()?.Duration,
                timeline.MaxSeekTime()?.Duration,
                state.Controls().unwrap().IsPlayEnabled()?,
                state.Controls()?.IsPauseEnabled()?,
                state.Controls()?.IsStopEnabled()?,
                state.Controls()?.IsRecordEnabled()?,
                state.Controls()?.IsFastForwardEnabled()?,
                state.Controls()?.IsRewindEnabled()?,
                state.Controls()?.IsNextEnabled()?,
                state.Controls()?.IsPreviousEnabled()?,
                state.Controls()?.IsChannelUpEnabled()?,
                state.Controls()?.IsChannelDownEnabled()?,
                state.Controls()?.IsPlayPauseToggleEnabled()?,
                state.Controls()?.IsShuffleEnabled()?,
                state.Controls()?.IsRepeatEnabled()?,
                state.Controls()?.IsPlaybackRateEnabled()?,
                state.Controls()?.IsPlaybackPositionEnabled()?,
            )
        )
    }
}

#[cfg(target_os = "windows")]
pub fn try_play() -> Result<bool> {
    if unavailable() {
        Ok(false)
    } else {
        match GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.TryPlayAsync() {
            Ok(session) => Ok(session.get()?),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_pause() -> Result<bool> {
    if unavailable() {
        Ok(false)
    } else {
        match GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.TryPauseAsync() {
            Ok(session) => Ok(session.get()?),
            Err(_) => Ok(false),
        }
    }
}
#[cfg(target_os = "windows")]
pub fn try_stop() -> Result<bool> {
    if unavailable() {
        Ok(false)
    } else {
        match GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.TryStopAsync() {
            Ok(session) => Ok(session.get()?),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_record() -> Result<bool> {
    if unavailable() {
        Ok(false)
    } else {
        match GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.TryRecordAsync() {
            Ok(session) => Ok(session.get()?),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_fast_forward() -> Result<bool> {
    if unavailable() {
        Ok(false)
    } else {
        match GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.TryFastForwardAsync() {
            Ok(session) => Ok(session.get()?),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_rewind() -> Result<bool> {
    if unavailable() {
        Ok(false)
    } else {
        match GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.TryRewindAsync() {
            Ok(session) => Ok(session.get()?),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_next() -> Result<bool> {
    if unavailable() {
        Ok(false)
    } else {
        match GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.TrySkipNextAsync() {
            Ok(session) => Ok(session.get()?),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_previous() -> Result<bool> {
    if unavailable() {
        Ok(false)
    } else {
        match GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.TrySkipPreviousAsync() {
            Ok(session) => Ok(session.get()?),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_change_channel_up() -> Result<bool> {
    if unavailable() {
        Ok(false)
    } else {
        match GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.TryChangeChannelUpAsync() {
            Ok(session) => Ok(session.get()?),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_change_channel_down() -> Result<bool> {
    if unavailable() {
        Ok(false)
    } else {
        match GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.TryChangeChannelDownAsync() {
            Ok(session) => Ok(session.get()?),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_play_pause_toggle() -> Result<bool> {
    if unavailable() {
        Ok(false)
    } else {
        match GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.TryTogglePlayPauseAsync() {
            Ok(session) => Ok(session.get()?),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_change_shuffle(shuffle: bool) -> Result<bool> {
    if unavailable() {
        Ok(false)
    } else {
        match GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.TryChangeShuffleActiveAsync(shuffle) {
            Ok(session) => Ok(session.get()?),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_change_repeat() -> Result<bool> {
    if unavailable() {
        Ok(false)
    } else {
        match GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.TryChangeAutoRepeatModeAsync(
            next(GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.GetPlaybackInfo()?.AutoRepeatMode()?.Value()?)
        ) {
            Ok(session) => Ok(session.get()?),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_change_playback_rate(i: f64) -> Result<bool> {
    if unavailable() {
        Ok(false)
    } else {
        match GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.TryChangePlaybackRateAsync(i) {
            Ok(session) => Ok(session.get()?),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn try_change_playback_position(i: i64) -> Result<bool> {
    if unavailable() {
        Ok(false)
    } else {
        match GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?.GetCurrentSession()?.TryChangePlaybackPositionAsync(i) {
            Ok(session) => Ok(session.get()?),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn unavailable() -> bool {
    GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetSessions().iter().next().is_none()
}

//noinspection Annotator
#[cfg(target_os = "windows")]
fn save_thumbnail_and_get_path(
    properties: &GlobalSystemMediaTransportControlsSessionMediaProperties,
    name: String,
) -> String {
    match properties.Thumbnail() {
        Ok(p) => {
            let cache_folder = StorageFolder::GetFolderFromPathAsync(&HSTRING::from(env::temp_dir().to_str().unwrap())).unwrap().get().unwrap();
            let file_name = format!("thumbnail_cache_{}.jpg", name.replace("/", ""));

            let file = cache_folder
                .CreateFileAsync(&file_name.into(), CreationCollisionOption::ReplaceExisting)
                .unwrap()
                .get()
                .unwrap();

            let input_stream = p.OpenReadAsync().unwrap().get().unwrap();
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
        },
        Err(_) => {
            "".to_string()
        }
    }
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

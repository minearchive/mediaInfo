#[cfg(target_os = "macos")]
use {
    crate::{
        MediaInfo,
        PlaybackState
    },
    objc2_media_player::{
      MPMediaPlayback,
      MPMusicPlaybackState,
      MPMusicPlayerController,
      MPMusicRepeatMode,
      MPMusicShuffleMode
    },
    std::{
        ffi::c_float,
        ops::Deref
    }
};

#[cfg(target_os = "macos")]
pub fn get_media_info() -> MediaInfo {
    unsafe {
        let playing = MPMusicPlayerController::systemMusicPlayer().nowPlayingItem().unwrap();

        MediaInfo::new(
            playing.title().unwrap().deref().to_string(),
            playing.artist().unwrap().deref().to_string(),
            playing.albumTitle().unwrap().deref().to_string(),
            playing.assetURL().unwrap().deref().to_string(),
        )
    }
}

#[cfg(target_os = "macos")]
pub fn get_playback_state() -> PlaybackState {
    unsafe {
        let playing = MPMusicPlayerController::systemMusicPlayer().currentPlaybackTime();
        let state = MPMusicPlayerController::systemMusicPlayer().playbackState();
        let shuffle = MPMusicPlayerController::systemMusicPlayer().shuffleMode();
        let repeat = MPMusicPlayerController::systemMusicPlayer().repeatMode();
        let media_item = MPMusicPlayerController::systemMusicPlayer().nowPlayingItem().unwrap();
        let prepared = MPMusicPlayerController::systemMusicPlayer().isPreparedToPlay();

        PlaybackState::new(
            state == MPMusicPlaybackState::Playing,
            state == MPMusicPlaybackState::Paused,
            state == MPMusicPlaybackState::Stopped,
            shuffle == MPMusicShuffleMode::Albums || shuffle == MPMusicShuffleMode::Songs,
            repeat == MPMusicRepeatMode::One,
            repeat == MPMusicRepeatMode::All,
            playing.round() as i64,
            media_item.playbackDuration().round() as i64,
            prepared,
            prepared,
            prepared,
            false,
            false,
            false,
            prepared,
            prepared,
            false,
            false,
            prepared,
            prepared,
            prepared,
            true,
            true,
        )
    }
}

#[cfg(target_os = "macos")]
pub fn try_play() -> bool {
    unsafe {
        MPMusicPlayerController::systemMusicPlayer().play();
        true
    }
}

#[cfg(target_os = "macos")]
pub fn try_pause() -> bool {
    unsafe {
        MPMusicPlayerController::systemMusicPlayer().pause();
        true
    }
}

#[cfg(target_os = "macos")]
pub fn try_stop() -> bool {
    unsafe {
        MPMusicPlayerController::systemMusicPlayer().stop();
        true
    }
}

#[cfg(target_os = "macos")]
pub fn try_record() -> bool {
    true
}

#[cfg(target_os = "macos")]
pub fn try_fast_forward() -> bool {
    true
}

#[cfg(target_os = "macos")]
pub fn try_rewind() -> bool {
    true
}

#[cfg(target_os = "macos")]
pub fn try_next() -> bool {
    unsafe {
        MPMusicPlayerController::systemMusicPlayer().skipToNextItem();
        true
    }
}

#[cfg(target_os = "macos")]
pub fn try_previous() -> bool {
    unsafe {
        MPMusicPlayerController::systemMusicPlayer().skipToPreviousItem();
        true
    }
}

#[cfg(target_os = "macos")]
pub fn try_change_channel_up() -> bool {
    true
}

#[cfg(target_os = "macos")]
pub fn try_change_channel_down() -> bool {
    true
}

#[cfg(target_os = "macos")]
pub fn try_play_pause_toggle() -> bool {
    unsafe {
        let playing = MPMusicPlayerController::systemMusicPlayer().playbackState() == MPMusicPlaybackState::Playing;
        let pausing = MPMusicPlayerController::systemMusicPlayer().playbackState() == MPMusicPlaybackState::Paused;
        if playing {
            MPMusicPlayerController::systemMusicPlayer().pause();
        } else if pausing {
            MPMusicPlayerController::systemMusicPlayer().play();
        }
        true
    }
}

#[cfg(target_os = "macos")]
pub fn try_change_shuffle() -> bool {
    unsafe {
        MPMusicPlayerController::systemMusicPlayer().setShuffleMode(next_shuffle(MPMusicPlayerController::systemMusicPlayer().shuffleMode()));
        true
    }
}

#[cfg(target_os = "macos")]
pub fn try_change_repeat() -> bool {
    unsafe {
        MPMusicPlayerController::systemMusicPlayer().setRepeatMode(next_repeat(MPMusicPlayerController::systemMusicPlayer().repeatMode()));
        true
    }
}

#[cfg(target_os = "macos")]
pub fn try_change_playback_rate(f: f64) -> bool {
    unsafe {
        MPMusicPlayerController::systemMusicPlayer().setCurrentPlaybackRate(c_float::from(f));
        true
    }
}

#[cfg(target_os = "macos")]
pub fn try_change_playback_position(i: i64) -> bool {
    unsafe {
        MPMusicPlayerController::systemMusicPlayer().setCurrentPlaybackTime(i as f64);
        true
    }
}

#[cfg(target_os = "macos")]
fn next_shuffle(mode: MPMusicShuffleMode) -> MPMusicShuffleMode {
    if mode == MPMusicShuffleMode::Default {
        mode
    } else  if mode == MPMusicShuffleMode::Off {
        return MPMusicShuffleMode::Songs;
    } else if mode == MPMusicShuffleMode::Songs {
        return MPMusicShuffleMode::Albums;
    } else if mode == MPMusicShuffleMode::Albums {
        return MPMusicShuffleMode::Off;
    } else {
        mode
    }
}

#[cfg(target_os = "macos")]
fn next_repeat(mode: MPMusicRepeatMode) -> MPMusicRepeatMode {
    if mode == MPMusicRepeatMode::Default {
        mode
    } else if mode == MPMusicRepeatMode::None {
        return MPMusicRepeatMode::One;
    } else if mode == MPMusicRepeatMode::One {
        return MPMusicRepeatMode::All;
    } else if mode == MPMusicRepeatMode::All {
        return MPMusicRepeatMode::None;
    } else {
        mode
    }
}
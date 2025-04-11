#[cfg(target_os = "linux")]
use {
    crate::{MediaInfo, PlaybackState},
    anyhow::Result,
    mpris::{LoopStatus, PlaybackStatus, PlayerFinder},
    std::time::Duration,
};

#[cfg(target_os = "linux")]
pub fn get_media_info() -> Result<MediaInfo> {
    let player_finder = PlayerFinder::new()?;
    let player = player_finder.find_active()?;
    let metadata = player.get_metadata()?;

    let media_info = MediaInfo::new(
        metadata.title().unwrap_or("").to_string(),
        vec_to_str(metadata.artists().unwrap()),
        metadata.album_name().unwrap_or("").to_string(),
        metadata.art_url().unwrap().to_string(),
    );
    
    Ok(media_info)
}

#[cfg(target_os = "linux")]
pub fn get_playback_state() -> Result<PlaybackState> {
    let player_finder = PlayerFinder::new()?;
    let player = player_finder.find_active()?;

    let metadata = player.get_metadata()?;

    let playback_state = PlaybackState::new(
        player.get_playback_status()? == PlaybackStatus::Playing,
        player.get_playback_status()? == PlaybackStatus::Paused,
        player.get_playback_status()? == PlaybackStatus::Stopped,
        player.get_shuffle()?,
        player.get_loop_status()? == LoopStatus::Track,
        player.get_loop_status()? == LoopStatus::Playlist,
        player.get_position_in_microseconds()?.try_into()?,
        i64::try_from(metadata.length_in_microseconds().unwrap())?,
        player.can_play()?,
        player.can_pause()?,
        player.can_stop()?,
        false, //because linux hasn't this method.
        false, //because linux hasn't this method
        false, //because linux hasn't this method
        player.can_go_next()?,
        player.can_go_previous()?,
        false, //because linux hasn't this method
        false, //because linux hasn't this method
        player.can_pause()? && player.can_play()?,
        player.can_shuffle()?,
        player.can_loop()?,
        player.has_playback_rate()?,
        player.has_position()?
    );

    Ok(playback_state)
}

#[cfg(target_os = "linux")]
pub fn try_play() -> bool {
    PlayerFinder::new()
        .ok()
        .and_then(|f| f.find_active().ok())
        .and_then(|f| f.play().ok())
        .is_some()
}

#[cfg(target_os = "linux")]
pub fn try_pause() -> bool {
    PlayerFinder::new()
        .ok()
        .and_then(|f| f.find_active().ok())
        .and_then(|f| f.pause().ok())
        .is_some()
}

#[cfg(target_os = "linux")]
pub fn try_stop() -> bool {
    PlayerFinder::new()
        .ok()
        .and_then(|f| f.find_active().ok())
        .and_then(|f| f.stop().ok())
        .is_some()
}

#[cfg(target_os = "linux")]
pub fn try_record() -> bool {
    false
}

#[cfg(target_os = "linux")]
pub fn try_fast_forward() -> bool {
    false
}

#[cfg(target_os = "linux")]
pub fn try_rewind() -> bool {
    false
}

#[cfg(target_os = "linux")]
pub fn try_next() -> bool {
    PlayerFinder::new()
        .ok()
        .and_then(|f| f.find_active().ok())
        .and_then(|f| f.next().ok())
        .is_some()
}

#[cfg(target_os = "linux")]
pub fn try_previous() -> bool {
    PlayerFinder::new()
        .ok()
        .and_then(|f| f.find_active().ok())
        .and_then(|f| f.previous().ok())
        .is_some()
}

#[cfg(target_os = "linux")]
pub fn try_change_channel_up() -> bool {
    false
}

#[cfg(target_os = "linux")]
pub fn try_change_channel_down() -> bool {
    false
}

#[cfg(target_os = "linux")]
pub fn try_play_pause_toggle() -> bool {
    PlayerFinder::new()
        .ok()
        .and_then(|f| f.find_active().ok())
        .and_then(|f| f.play_pause().ok())
        .is_some()
}

#[cfg(target_os = "linux")]
pub fn try_change_shuffle() -> bool {
    PlayerFinder::new()
        .ok()
        .and_then(|f| f.find_active().ok())
        .and_then(|p| {
            p.set_shuffle(
                !PlayerFinder::new()
                    .ok()
                    .and_then(|f| f.find_active().ok())
                    .and_then(|p| p.get_shuffle().ok())
                    .unwrap()
            ).ok()
        }).is_some()
}

#[cfg(target_os = "linux")]
pub fn try_change_repeat() -> bool {
    PlayerFinder::new()
        .ok()
        .and_then(|f| f.find_active().ok())
        .and_then(|p| p.set_loop_status(next(p.get_loop_status().unwrap())).ok())
        .is_some()
}

#[cfg(target_os = "linux")]
pub fn try_change_playback_rate(i: f64) -> bool {
    PlayerFinder::new()
        .ok()
        .and_then(|f| f.find_active().ok())
        .and_then(|f| f.set_playback_rate(i).ok())
        .is_some()
}

#[cfg(target_os = "linux")]
pub fn try_change_playback_position(i: i64) -> bool {
    PlayerFinder::new()
        .ok()
        .and_then(|f| f.find_active().ok())
        .and_then(|p| p.set_position(
            PlayerFinder::new().ok().and_then(|f| f.find_active().ok()).unwrap().get_metadata().unwrap().track_id().unwrap(),
            &Duration::from_micros(i as u64)
        ).ok())
        .is_some()
}


#[cfg(target_os = "linux")]
fn vec_to_str(str: Vec<&str>) -> String {
    str.iter()
        .enumerate()
        .map(|(i, _)| str[i])
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(target_os = "linux")]
fn next(loop_status: LoopStatus) -> LoopStatus {
    if loop_status == LoopStatus::None {
        LoopStatus::Track
    } else if loop_status == LoopStatus::Track {
        return LoopStatus::Playlist
    } else if loop_status == LoopStatus::Playlist {
        return LoopStatus::None
    } else {
      return loop_status
    }
}
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
pub fn try_play() {
    PlayerFinder::new().unwrap().find_active().unwrap().play().unwrap();
}

#[cfg(target_os = "linux")]
pub fn try_pause() {
    PlayerFinder::new().unwrap().find_active().unwrap().pause().unwrap();
}

#[cfg(target_os = "linux")]
pub fn try_stop() {
    PlayerFinder::new().unwrap().find_active().unwrap().stop().unwrap();
}

#[cfg(target_os = "linux")]
pub fn try_record() {
    //NOTHING TO DO
}

#[cfg(target_os = "linux")]
pub fn try_fast_forward() {
    //NOTHING TO DO
}

#[cfg(target_os = "linux")]
pub fn try_rewind() {
    //NOTHING TO DO
}

#[cfg(target_os = "linux")]
pub fn try_next() {
    PlayerFinder::new().unwrap().find_active().unwrap().next().unwrap();
}

#[cfg(target_os = "linux")]
pub fn try_previous() {
    PlayerFinder::new().unwrap().find_active().unwrap().previous().unwrap();
}

#[cfg(target_os = "linux")]
pub fn try_change_channel_up() {
    //NOTHING TO DO
}

#[cfg(target_os = "linux")]
pub fn try_change_channel_down() {
    //NOTHING TO DO
}

#[cfg(target_os = "linux")]
pub fn try_play_pause_toggle() {
    PlayerFinder::new().unwrap().find_active().unwrap().play_pause().unwrap();
}

#[cfg(target_os = "linux")]
pub fn try_change_shuffle() {
    PlayerFinder::new().unwrap().find_active().unwrap().set_shuffle(!PlayerFinder::new().unwrap().find_active().unwrap().get_shuffle().unwrap()).unwrap();
}

#[cfg(target_os = "linux")]
pub fn try_change_repeat() {
    PlayerFinder::new().unwrap().find_active().unwrap().set_loop_status(next(PlayerFinder::new().unwrap().find_active().unwrap().get_loop_status().unwrap())).unwrap();
}

#[cfg(target_os = "linux")]
pub fn try_change_playback_rate(i: i64) {
    PlayerFinder::new().unwrap().find_active().unwrap().set_playback_rate(i as f64).unwrap();
}

#[cfg(target_os = "linux")]
pub fn try_change_playback_position(i: i64) {
    PlayerFinder::new().unwrap().find_active().unwrap().set_position(
        PlayerFinder::new().unwrap().find_active().unwrap().get_metadata().unwrap().track_id().unwrap(),
        &Duration::from_micros(i as u64)
    ).unwrap()
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
mod platform;

pub struct MediaInfo {
    title: String,
    artist: String,
    album: String,
    album_art: String,
}

pub struct PlaybackState {
    is_playing: bool,
    is_pausing: bool,
    is_stopped: bool,
    is_shuffling: bool,
    is_repeating_track: bool,
    is_repeating_playlist: bool,
    current_time: i64,
    max_time: i64,
    play_enabled: bool,
    pause_enabled: bool,
    stop_enabled: bool,
    record_enabled: bool,
    fast_forward_enabled: bool,
    rewind_enabled: bool,
    next_enabled: bool,
    previous_enabled: bool,
    channel_up_enabled: bool,
    channel_down_enabled: bool,
    play_pause_toggle_enabled: bool,
    shuffle_enabled: bool,
    repeat_enabled: bool,
    playback_rate_enabled: bool,
    playback_position_enabled: bool,
}

impl MediaInfo {
    fn new(title: String, artist: String, album: String, album_art: String) -> Self {
        Self {
            title,
            artist,
            album,
            album_art,
        }
    }

    fn empty() -> Self {
        Self {
            title: "Unavailable".to_string(),
            artist: "Unavailable".to_string(),
            album: "Unavailable".to_string(),
            album_art: "Unavailable".to_string(),
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{},{},{},{}",
            self.title, self.artist, self.album, self.album_art
        )
    }
}

impl PlaybackState {
    fn new(
        is_playing: bool,
        is_pausing: bool,
        is_stopped: bool,
        is_shuffling: bool,
        is_repeating_track: bool,
        is_repeating_playlist: bool,
        current_time: i64,
        max_time: i64,
        play_enabled: bool,
        pause_enabled: bool,
        stop_enabled: bool,
        record_enabled: bool,
        fast_forward_enabled: bool,
        rewind_enabled: bool,
        next_enabled: bool,
        previous_enabled: bool,
        channel_up_enabled: bool,
        channel_down_enabled: bool,
        play_pause_toggle_enabled: bool,
        shuffle_enabled: bool,
        repeat_enabled: bool,
        playback_rate_enabled: bool,
        playback_position_enabled: bool,
    ) -> Self {
        Self {
            is_playing,
            is_pausing,
            is_stopped,
            is_shuffling,
            is_repeating_track,
            is_repeating_playlist,
            current_time,
            max_time,
            play_enabled,
            pause_enabled,
            stop_enabled,
            record_enabled,
            fast_forward_enabled,
            rewind_enabled,
            next_enabled,
            previous_enabled,
            channel_up_enabled,
            channel_down_enabled,
            play_pause_toggle_enabled,
            shuffle_enabled,
            repeat_enabled,
            playback_rate_enabled,
            playback_position_enabled,
        }
    }

    fn empty() -> Self {
        Self {
            is_playing: false,
            is_pausing: false,
            is_stopped: false,
            is_shuffling: false,
            is_repeating_track: false,
            is_repeating_playlist: false,
            current_time: -1,
            max_time: -1,
            play_enabled: false,
            pause_enabled: false,
            stop_enabled: false,
            record_enabled: false,
            fast_forward_enabled: false,
            rewind_enabled: false,
            next_enabled: false,
            previous_enabled: false,
            channel_up_enabled: false,
            channel_down_enabled: false,
            play_pause_toggle_enabled: false,
            shuffle_enabled: false,
            repeat_enabled: false,
            playback_rate_enabled: false,
            playback_position_enabled: false,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},",
            self.is_playing,
            self.is_pausing,
            self.is_stopped,
            self.is_shuffling,
            self.is_repeating_track,
            self.is_repeating_playlist,
            self.current_time,
            self.max_time,
            self.play_enabled,
            self.pause_enabled,
            self.stop_enabled,
            self.record_enabled,
            self.fast_forward_enabled,
            self.rewind_enabled,
            self.next_enabled,
            self.previous_enabled,
            self.channel_up_enabled,
            self.channel_down_enabled,
            self.play_pause_toggle_enabled,
            self.shuffle_enabled,
            self.repeat_enabled,
            self.playback_rate_enabled,
            self.playback_position_enabled
        )
    }
}

pub fn get_media_info() -> MediaInfo {
    #[cfg(target_os = "windows")]
    {
        platform::windows::get_media_info()
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::get_media_info().unwrap()
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::get_media_info()
    }
}

pub fn get_playback_state() -> PlaybackState {
    #[cfg(target_os = "windows")]
    {
        platform::windows::get_playback_state()
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::get_playback_state().unwrap()
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::get_playback_state()
    }
}

pub fn play() -> bool {
    #[cfg(target_os = "windows")]
    {
        platform::windows::try_play()
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::try_play()
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::try_play()
    }
}

pub fn pause() -> bool {
    #[cfg(target_os = "windows")]
    {
        platform::windows::try_pause()
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::try_pause()
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::try_pause()
    }
}
pub fn stop() -> bool {
    #[cfg(target_os = "windows")]
    {
        platform::windows::try_stop()
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::try_stop()
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::try_stop()
    }
}

pub fn record() -> bool {
    #[cfg(target_os = "windows")]
    {
        platform::windows::try_record()
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::try_record()
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::try_record()
    }
}

pub fn fast_forward() -> bool {
    #[cfg(target_os = "windows")]
    {
        platform::windows::try_fast_forward()
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::try_fast_forward()
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::try_fast_forward()
    }
}

pub fn rewind() -> bool {
    #[cfg(target_os = "windows")]
    {
        platform::windows::try_rewind()
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::try_rewind()
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::try_rewind()
    }
}

pub fn next() -> bool {
    #[cfg(target_os = "windows")]
    {
        platform::windows::try_next()
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::try_next()
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::try_next()
    }
}

pub fn previous() -> bool {
    #[cfg(target_os = "windows")]
    {
        platform::windows::try_previous()
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::try_previous()
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::try_previous()
    }
}

pub fn channel_up() -> bool {
    #[cfg(target_os = "windows")]
    {
        platform::windows::try_change_channel_up()
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::try_change_channel_up()
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::try_change_channel_up()
    }
}

pub fn channel_down() -> bool {
    #[cfg(target_os = "windows")]
    {
        platform::windows::try_change_channel_down()
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::try_change_channel_down()
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::try_change_channel_down()
    }
}

pub fn toggle_play_pause() -> bool {
    #[cfg(target_os = "windows")]
    {
        platform::windows::try_play_pause_toggle()
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::try_play_pause_toggle()
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::try_play_pause_toggle()
    }
}

pub fn shuffle(enable: bool) -> bool {
    #[cfg(target_os = "windows")]
    {
        platform::windows::try_change_shuffle(enable)
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::try_change_shuffle()
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::try_change_shuffle()
    }
}

pub fn repeat() -> bool {
    #[cfg(target_os = "windows")]
    {
        platform::windows::try_change_repeat()
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::try_change_repeat()
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::try_change_repeat()
    }
}

pub fn playback_rate(rate: f64) -> bool {
    #[cfg(target_os = "windows")]
    {
        platform::windows::try_change_playback_rate(rate)
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::try_change_playback_rate(rate)
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::try_change_playback_rate(rate)
    }
}

pub fn playback_position(position: i64) -> bool {
    #[cfg(target_os = "windows")]
    {
        platform::windows::try_change_playback_position(position)
    }

    #[cfg(target_os = "linux")]
    {
        platform::linux::try_change_playback_position(position)
    }

    #[cfg(target_os = "macos")]
    {
        platform::macos::try_change_playback_position(position)
    }
}

pub fn player_available() -> bool {
    #[cfg(target_os = "windows")]
    {
        !platform::windows::unavailable()
    }
}

#[cfg(test)]
mod tests {
    use crate::{platform, MediaInfo, PlaybackState};
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn info_text() {

        #[cfg(target_os = "linux")]
        {
            println!("{}", platform::linux::get_media_info().unwrap().to_string());
            println!("{}", platform::linux::get_playback_state().unwrap().to_string());
        }

        #[cfg(target_os = "windows")]
        {
            println!("{}", platform::windows::get_media_info().to_string());
            println!("{}", platform::windows::get_playback_state().to_string());

            assert_eq!(platform::windows::try_pause(), true);
            sleep(Duration::from_secs(2));
            assert_eq!(platform::windows::try_play(), true);
            sleep(Duration::from_secs(2));
            assert_eq!(platform::windows::try_next(), true);
            sleep(Duration::from_secs(2));
            assert_eq!(platform::windows::try_previous(), true);
            sleep(Duration::from_secs(2));
        }
     }

    #[test]
    fn empty_info_text() {
        println!("{}", MediaInfo::empty().to_string());
        println!("{}", PlaybackState::empty().to_string())
    }
}

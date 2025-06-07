use serde::{Deserialize, Serialize};

mod platform;

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaInfo {
    title: String,
    artist: String,
    album: String,
    album_art: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

pub struct MediaControls {
    //old image path
    pub old_image: String,
}

impl MediaControls {

    pub fn new() -> Self {
        Self {
            old_image: String::new(),
        }
    }

    pub fn get_media_info(&self) -> MediaInfo {
        #[cfg(target_os = "windows")]
        {
            platform::windows::get_media_info(self.old_image.clone()).unwrap()
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

    pub fn get_playback_state(&self) -> PlaybackState {
        #[cfg(target_os = "windows")]
        {
            platform::windows::get_playback_state().unwrap()
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

    pub fn play(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            platform::windows::try_play().unwrap()
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

    pub fn pause(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            platform::windows::try_pause().unwrap()
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

    pub fn stop(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            platform::windows::try_stop().unwrap()
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

    pub fn record(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            platform::windows::try_record().unwrap()
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

    pub fn fast_forward(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            platform::windows::try_fast_forward().unwrap()
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

    pub fn rewind(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            platform::windows::try_rewind().unwrap()
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

    pub fn next(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            platform::windows::try_next().unwrap()
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

    pub fn previous(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            platform::windows::try_previous().unwrap()
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

    pub fn channel_up(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            platform::windows::try_change_channel_up().unwrap()
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

    pub fn channel_down(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            platform::windows::try_change_channel_down().unwrap()
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

    pub fn toggle_play_pause(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            platform::windows::try_play_pause_toggle().unwrap()
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

    pub fn shuffle(&self, enable: bool) -> bool {
        #[cfg(target_os = "windows")]
        {
            platform::windows::try_change_shuffle(enable).unwrap()
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

    pub fn repeat(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            platform::windows::try_change_repeat().unwrap()
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

    pub fn playback_rate(&self, rate: f64) -> bool {
        #[cfg(target_os = "windows")]
        {
            platform::windows::try_change_playback_rate(rate).unwrap()
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

    pub fn playback_position(&self, position: i64) -> bool {
        #[cfg(target_os = "windows")]
        {
            platform::windows::try_change_playback_position(position).unwrap()
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

    pub fn player_available(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            !platform::windows::unavailable()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{MediaControls, MediaInfo, PlaybackState};

    #[test]
    fn info_text() {
        let controls = MediaControls::new();

        let info = controls.get_media_info();
        println!("Media Info: {}", info.to_string());
        let playback_state = controls.get_playback_state();
        println!("Playback State: {}", playback_state.to_string());

        if controls.player_available() {
            println!("Player is available.");
        } else {
            println!("Player is not available.");
        }

     }

    #[test]
    fn empty_info_text() {
        println!("{}", MediaInfo::empty().to_string());
        println!("{}", PlaybackState::empty().to_string())
    }

    #[test]
    fn serialize_info() {
        let info = MediaInfo::new(
            "Title".to_string(),
            "Artist".to_string(),
            "Album".to_string(),
            "Album Art".to_string(),
        );
        let serialized = serde_json::to_string(&info).unwrap();
        println!("Serialized: {}", serialized);
    }
}

mod platform;

struct MediaInfo {
    title: String,
    artist: String,
    album: String,
    album_art: String,
}

struct PlaybackState {
    is_playing: bool,
    is_pausing: bool,
    is_stopped: bool,
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
    fn new(
        title: String,
        artist: String,
        album: String,
        album_art: String,
    ) -> Self {
        Self {
            title,
            artist,
            album,
            album_art,
        }
    }

    fn to_string(self) -> String {
        format!(
            "{},{},{},{}",
            self.title,
            self.artist,
            self.album,
            self.album_art
        )
    }
}

impl PlaybackState {
    fn new(
        is_playing: bool,
        is_pausing: bool,
        is_stopped: bool,
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

    fn to_string(self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},",
            self.is_playing, self.is_pausing, self.is_stopped, self.current_time, self.max_time, self.play_enabled, self.pause_enabled, self.stop_enabled, self.record_enabled, self.fast_forward_enabled, self.rewind_enabled, self.next_enabled, self.previous_enabled, self.channel_up_enabled, self.channel_down_enabled, self.play_pause_toggle_enabled, self.shuffle_enabled, self.repeat_enabled, self.playback_rate_enabled, self.playback_position_enabled
        )
    }
}



#[cfg(test)]
mod tests {
    use crate::platform;

    #[test]
    fn run() {
        println!("media info");
        println!("{}", platform::windows::get_media_info().to_string());

        println!("media playback state");
        println!("{}", platform::windows::get_playback_state().to_string());
    }
}

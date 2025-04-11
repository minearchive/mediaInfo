use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jstring;

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

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeMedia_getMediaInfo(_env: JNIEnv, _class: JClass) -> jstring {
    let mut data = String::new();

    #[cfg(target_os = "windows")]
    {
        data = platform::windows::get_media_info().to_string();
    }

    #[cfg(target_os = "linux")]
    {
        data = platform::linux::get_media_info().unwrap().to_string();
    }

    _env.new_string(&data).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeMedia_getPlaybackState(_env: JNIEnv, _class: JClass) -> jstring {
    let mut data = String::new();

    #[cfg(target_os = "windows")]
    {
        data = platform::windows::get_playback_state().to_string();
    }

    #[cfg(target_os = "linux")]
    {
        data = platform::linux::get_playback_state().unwrap().to_string();
    }

    _env.new_string(&data).unwrap().into_raw()
}

#[cfg(test)]
mod tests {
    use crate::platform;

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
        }
     }
}

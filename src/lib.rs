use jni::objects::JClass;
use jni::sys::{jboolean, jdouble, jlong, jstring, JNI_TRUE};
use jni::JNIEnv;

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

    fn to_string(self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},",
            self.is_playing, self.is_pausing, self.is_stopped, self.is_shuffling, self.is_repeating_track, self.is_repeating_playlist, self.current_time, self.max_time, self.play_enabled, self.pause_enabled, self.stop_enabled, self.record_enabled, self.fast_forward_enabled, self.rewind_enabled, self.next_enabled, self.previous_enabled, self.channel_up_enabled, self.channel_down_enabled, self.play_pause_toggle_enabled, self.shuffle_enabled, self.repeat_enabled, self.playback_rate_enabled, self.playback_position_enabled
        )
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeMedia_getMediaInfo(_env: JNIEnv, _class: JClass) -> jstring {

    #[cfg(target_os = "windows")]
    {
        _env.new_string(platform::windows::get_media_info().to_string()).unwrap().into_raw()
    }

    #[cfg(target_os = "linux")]
    {
        _env.new_string(platform::linux::get_media_info().unwrap().to_string()).unwrap().into_raw()
    }

    #[cfg(target_os = "macos")]
    {
        _env.new_string(platform::macos::get_media_info().to_string()).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeMedia_getPlaybackState(_env: JNIEnv, _class: JClass) -> jstring {

    #[cfg(target_os = "windows")]
    {
        _env.new_string(platform::windows::get_playback_state().to_string()).unwrap().into_raw()
    }

    #[cfg(target_os = "linux")]
    {
        _env.new_string(platform::linux::get_playback_state().unwrap().to_string()).unwrap().into_raw()
    }

    #[cfg(target_os = "macos")]
    {
        _env.new_string(platform::macos::get_playback_state().to_string()).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeController_play(_env: JNIEnv, _class: JClass) -> jboolean {

    #[cfg(target_os = "windows")]
    {
        jboolean::from(platform::windows::try_play())
    }

    #[cfg(target_os = "linux")]
    {
        jboolean::from(platform::linux::try_play())
    }

    #[cfg(target_os = "macos")]
    {
        jboolean::from(platform::macos::try_play())
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeController_pause(_env: JNIEnv, _class: JClass) -> jboolean {
    #[cfg(target_os = "windows")]
    {
        jboolean::from(platform::windows::try_pause())
    }

    #[cfg(target_os = "linux")]
    {
        jboolean::from(platform::linux::try_pause())
    }

    #[cfg(target_os = "macos")]
    {
        jboolean::from(platform::macos::try_pause())
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeController_stop(_env: JNIEnv, _class: JClass) -> jboolean {
    #[cfg(target_os = "windows")]
    {
        jboolean::from(platform::windows::try_stop())
    }

    #[cfg(target_os = "linux")]
    {
        jboolean::from(platform::linux::try_stop())
    }

    #[cfg(target_os = "macos")]
    {
        jboolean::from(platform::macos::try_stop())
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeController_record(_env: JNIEnv, _class: JClass) -> jboolean {
    #[cfg(target_os = "windows")]
    {
        jboolean::from(platform::windows::try_record())
    }

    #[cfg(target_os = "linux")]
    {
        jboolean::from(platform::linux::try_record())
    }

    #[cfg(target_os = "macos")]
    {
        jboolean::from(platform::macos::try_record())
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeController_fastForward(_env: JNIEnv, _class: JClass) -> jboolean {
    #[cfg(target_os = "windows")]
    {
        jboolean::from(platform::windows::try_fast_forward())
    }

    #[cfg(target_os = "linux")]
    {
        jboolean::from(platform::linux::try_fast_forward())
    }

    #[cfg(target_os = "macos")]
    {
        jboolean::from(platform::macos::try_fast_forward())
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeController_rewind(_env: JNIEnv, _class: JClass) -> jboolean {
    #[cfg(target_os = "windows")]
    {
        jboolean::from(platform::windows::try_rewind())
    }

    #[cfg(target_os = "linux")]
    {
        jboolean::from(platform::linux::try_rewind())
    }

    #[cfg(target_os = "macos")]
    {
        jboolean::from(platform::macos::try_rewind())
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeController_next(_env: JNIEnv, _class: JClass) -> jboolean {
    #[cfg(target_os = "windows")]
    {
        jboolean::from(platform::windows::try_next())
    }

    #[cfg(target_os = "linux")]
    {
        jboolean::from(platform::linux::try_next())
    }

    #[cfg(target_os = "macos")]
    {
        jboolean::from(platform::macos::try_next())
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeController_previous(_env: JNIEnv, _class: JClass) -> jboolean {
    #[cfg(target_os = "windows")]
    {
        jboolean::from(platform::windows::try_previous())
    }

    #[cfg(target_os = "linux")]
    {
        jboolean::from(platform::linux::try_previous()  )
    }

    #[cfg(target_os = "macos")]
    {
        jboolean::from(platform::macos::try_previous()  )
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeController_channelUp(_env: JNIEnv, _class: JClass) -> jboolean {
    #[cfg(target_os = "windows")]
    {
        jboolean::from(platform::windows::try_change_channel_up())
    }

    #[cfg(target_os = "linux")]
    {
        jboolean::from(platform::linux::try_change_channel_up())
    }

    #[cfg(target_os = "macos")]
    {
        jboolean::from(platform::macos::try_change_channel_up())
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeController_channelDown(_env: JNIEnv, _class: JClass) -> jboolean {
    #[cfg(target_os = "windows")]
    {
        jboolean::from(platform::windows::try_change_channel_down())
    }

    #[cfg(target_os = "linux")]
    {
        jboolean::from(platform::linux::try_change_channel_down())
    }

    #[cfg(target_os = "macos")]
    {
        jboolean::from(platform::macos::try_change_channel_down())
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeController_togglePlayPause(_env: JNIEnv, _class: JClass) -> jboolean {
    #[cfg(target_os = "windows")]
    {
        jboolean::from(platform::windows::try_play_pause_toggle())
    }

    #[cfg(target_os = "linux")]
    {
        jboolean::from(platform::linux::try_play_pause_toggle())
    }

    #[cfg(target_os = "macos")]
    {
        jboolean::from(platform::macos::try_play_pause_toggle())
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeController_shuffle(_env: JNIEnv, _class: JClass, j_boolean: jboolean) -> jboolean {
    #[cfg(target_os = "windows")]
    {
        jboolean::from(platform::windows::try_change_shuffle(j_boolean == JNI_TRUE))
    }

    #[cfg(target_os = "linux")]
    {
        jboolean::from(platform::linux::try_change_shuffle())
    }

    #[cfg(target_os = "macos")]
    {
        jboolean::from(platform::macos::try_change_shuffle())
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeController_repeat(_env: JNIEnv, _class: JClass) -> jboolean {
    #[cfg(target_os = "windows")]
    {
        jboolean::from(platform::windows::try_change_repeat())
    }

    #[cfg(target_os = "linux")]
    {
        jboolean::from(platform::linux::try_change_repeat())
    }

    #[cfg(target_os = "macos")]
    {
        jboolean::from(platform::macos::try_change_repeat())
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeController_playbackRate(_env: JNIEnv, _class: JClass, j_double: jdouble) -> jboolean {
    #[cfg(target_os = "windows")]
    {
        jboolean::from(platform::windows::try_change_playback_rate(j_double))
    }

    #[cfg(target_os = "linux")]
    {
        jboolean::from(platform::linux::try_change_playback_rate(j_double))
    }

    #[cfg(target_os = "macos")]
    {
        jboolean::from(platform::macos::try_change_playback_rate(j_double))
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_yuzuki_libs_media_NativeController_playbackPosition(_env: JNIEnv, _class: JClass, j_long: jlong) -> jboolean {
    #[cfg(target_os = "windows")]
    {
        jboolean::from(platform::windows::try_change_playback_position(j_long))
    }

    #[cfg(target_os = "linux")]
    {
        jboolean::from(platform::linux::try_change_playback_position(j_long))
    }

    #[cfg(target_os = "macos")]
    {
        jboolean::from(platform::macos::try_change_playback_position(j_long))
    }
}

#[cfg(test)]
mod tests {
    use crate::platform;
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
}

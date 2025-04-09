mod platform;

struct MediaInfo {
    title: String,
    artist: String,
    album: String,
    duration: u32,
    album_art: String,
}

struct PlaybackState {
    is_playing: bool,
    is_pausing: bool,
    is_stopped: bool,
    current_time: u32,
    max_time: u32,
    seekable: bool,
}

impl MediaInfo {
    fn new(
        title: String,
        artist: String,
        album: String,
        duration: u32,
        album_art: String,
    ) -> Self {
        Self {
            title,
            artist,
            album,
            duration,
            album_art,
        }
    }

    fn to_string(self) -> String {
        format!(
            "{},{},{},{},{}",
            self.title,
            self.artist,
            self.album,
            self.duration,
            self.album_art
        )
    }
}

impl PlaybackState {
    fn new(
        is_playing: bool,
        is_pausing: bool,
        is_stopped: bool,
        current_time: u32,
        max_time: u32,
        seekable: bool,
    ) -> Self {
        Self {
            is_playing,
            is_pausing,
            is_stopped,
            current_time,
            max_time,
            seekable,
        }
    }

    fn to_string(self) -> String {
        format!(
            "{},{},{},{},{},{}",
            self.is_playing, self.is_pausing, self.is_stopped, self.current_time, self.max_time, self.seekable
        )
    }
}



#[cfg(test)]
mod tests {

}

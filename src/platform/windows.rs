use windows::Media::Control::{GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager};
use windows::Media::SystemMediaTransportControls;
use crate::MediaInfo;

fn get_media_info() -> MediaInfo {
    let session = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap().GetCurrentSession().unwrap();;

    let properties = session.TryGetMediaPropertiesAsync().unwrap().get().unwrap();

    return MediaInfo::new(
        properties.Title().unwrap(),
        properties.Artist().unwrap(),
        properties.AlbumTitle().unwrap(),
        properties.Duration().unwrap(),
        properties.Thumbnail().unwrap()
    )

}
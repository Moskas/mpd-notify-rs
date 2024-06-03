use crate::song_info::Metadata;
use mpd::State;
use notify_rust::Notification;

#[cfg(not(target_os = "macos"))]
pub fn send_notification(metadata: Metadata, state: State, use_cover: bool) {
  match state {
    State::Play => {
      if use_cover {
        Notification::new()
          .summary("Now playing:")
          .body(&format!(
            "{}\n{}\nfrom {}",
            metadata.artist, metadata.title, metadata.album_name
          ))
          .timeout(5000)
          .image_path(&metadata.album_cover)
          .show()
          .unwrap();
      } else {
        Notification::new()
          .summary("Now playing:")
          .body(&format!(
            "{}\n{}\nfrom {}",
            metadata.artist, metadata.title, metadata.album_name
          ))
          .timeout(5000)
          .show()
          .unwrap();
      }
    }
    State::Pause => {
      Notification::new()
        .summary("MPD status")
        .body("Playback paused")
        .timeout(1000)
        .show()
        .unwrap();
    }
    State::Stop => {
      Notification::new()
        .summary("MPD status")
        .body("Playback stopped")
        .timeout(1000)
        .show()
        .unwrap();
    }
  }
  //println!("{}", metadata.album_cover);
}

#[cfg(target_os = "macos")]
pub fn send_notification(metadata: Metadata, state: mpd::State, use_cover: bool) {
  Notification::new()
    .summary("Now playing:")
    .body(&format!(
      "{}\n{}\nfrom {}",
      metadata.artist, metadata.title, metadata.album_name
    ))
    .timeout(5000)
    .show()
    .unwrap();
}

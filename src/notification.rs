use mpd::State;
use notify_rust::Notification;

#[cfg(not(target_os = "macos"))]
pub fn send_notification(metadata: Vec<String>, state: State) {
    match state {
        State::Play => {
            Notification::new()
                .summary("Now playing:")
                .body(&format!("{}\n{}\nfrom {}", metadata[0], metadata[1], metadata[2])[..])
                .timeout(5000)
                .image_path(&format!("{}", metadata[3])[..]) // finding cover image path WIP
                .show()
                .unwrap();
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
}
#[cfg(target_os = "macos")]
pub fn send_notification(metadata: Vec<String>, state: mpd::State) {
    Notification::new()
        .summary("Now playing:")
        .body(&format!("{}\n{}\nfrom {}", metadata[0], metadata[1], metadata[2])[..])
        .timeout(5000)
        .show()
        .unwrap();
}

use notify_rust::Notification;

#[cfg(not(target_os = "macos"))]
pub fn send_notification(metadata: Vec<String>) {
    Notification::new()
        .summary("Now playing:")
        .body(&format!("{}\n{}\nfrom {}", metadata[0], metadata[1], metadata[2])[..])
        .timeout(5000)
        .image_path(&format!("{}", metadata[3])[..]) // finding cover image path WIP
        .show()
        .unwrap();
}
#[cfg(target_os = "macos")]
pub fn send_notification(metadata: Vec<String>) {
    Notification::new()
        .summary("Now playing:")
        .body(&format!("{}\n{}\nfrom {}", metadata[0], metadata[1], metadata[2])[..])
        .timeout(5000)
        .show()
        .unwrap();
}

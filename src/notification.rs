use notify_rust::Notification;

pub fn send_notification(metadata: Vec<String>) {
    Notification::new()
        .summary("Now playing:")
        .body(
            &format!(
                "{}\n{}\nfrom <i>{}</i>",
                metadata[0], metadata[1], metadata[2]
            )[..],
        )
        .timeout(5000)
        .image_path(&format!("{}", metadata[3])[..]) // finding cover image path WIP
        .show()
        .unwrap();
    //println!("{}", &format!("{}/cover.png", metadata[3])[..])
    //    println!(
    //        "{:?}",
    //        &format!(
    //            "{}{} - {}/cover.png",
    //            album_cover_path, metadata[0], metadata[2]
    //        )[..]
    //    )
}

use mpd::Client;

pub fn get_metadata(music_directory: String, connection: &mut Client) -> Vec<String> {
    let current_song = connection.currentsong().unwrap().unwrap();
    let file_name = current_song.file.split('/').next().unwrap().to_string();

    let metadata = vec![
        get_artist(connection),
        get_title(connection),
        get_albumname(connection),
        format!("{}{}", music_directory, get_album_cover(file_name)),
    ];

    metadata
}

fn get_title(connection: &mut Client) -> String {
    match connection.currentsong().unwrap().unwrap().title {
        Some(title) => title,
        None => "Unknown".to_string(),
    }
}

fn get_artist(connection: &mut Client) -> String {
    connection
        .currentsong()
        .expect("No song in the queue")
        .expect("No metadata is present")
        .artist
        .expect("No artist metadata is present")
}

fn get_albumname(connection: &mut Client) -> String {
    let tags = get_tags(connection);
    for (key, value) in &tags {
        if key == "Album" {
            return value.clone();
        }
    }
    String::new()
}

pub fn get_tags(connection: &mut Client) -> Vec<(String, String)> {
    let current_song = connection
        .currentsong()
        .expect("No song in the queue")
        .expect("No metadata is present");

    current_song.tags.clone()
}

pub fn get_album_cover(album_directory: String) -> String {
    let cover_extension =
        if std::path::Path::new(&format!("{}/cover.png", album_directory)).exists() {
            "png"
        } else {
            "jpg"
        };

    format!("{}/cover.{}", album_directory, cover_extension)
}

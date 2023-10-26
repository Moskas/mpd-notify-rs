use mpd::Client;

pub struct Metadata {
    pub artist: String,
    pub title: String,
    pub album_name: String,
    pub album_cover: String,
}

pub fn get_metadata(music_directory: String, connection: &mut Client) -> Metadata {
    let current_song = connection.currentsong().unwrap().unwrap();
    let album_directory = current_song.file.split('/').next().unwrap().to_string();

    let artist = get_artist(connection);
    let title = get_title(connection);
    let album_name = get_album_name(connection);
    let album_cover = get_album_cover(album_directory.clone(), music_directory.to_string());

    Metadata {
        artist,
        title,
        album_name,
        album_cover,
    }
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

fn get_album_name(connection: &mut Client) -> String {
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

pub fn get_album_cover(album_directory: String, music_directory: String) -> String {
    //println!("{}",album_directory);
    let cover_extension = if std::path::Path::new(&format!("{}{}/cover.png", music_directory, album_directory))
        .exists()
    {
        "png"
    } else {
        "jpg"
    };
    //println!("{}{}/cover.{}", music_directory, album_directory, cover_extension);
    format!("{}{}/cover.{}", music_directory, album_directory, cover_extension)
}
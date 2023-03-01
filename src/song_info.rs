use crate::config;
use mpd::Client;

pub fn get_metadata() -> Vec<String> {
    let config = config::get_config();
    let mut connection = Client::connect(&format!("{}:{}", config.ip, config.port)[..]).unwrap();
    let music_dir: String = config.music_dir;
    let metadata = [
        get_artist(&mut connection),
        get_title(&mut connection),
        get_albumname(&mut connection),
        format!(
            "{}{}",
            music_dir.clone(),
            get_album_cover(
                music_dir.clone(),
                connection
                    .currentsong()
                    .unwrap()
                    .unwrap()
                    .file
                    .split("/")
                    .next()
                    .unwrap()
                    .to_string()
            )
        ),
    ]
    .to_vec();
    metadata
}

fn get_title(connection: &mut Client) -> String {
    match connection.currentsong().unwrap().unwrap().title {
        Some(title) => title,
        None => "Unknown".to_string(),
    }
}

fn get_artist(connection: &mut Client) -> String {
    match connection
        .currentsong()
        .unwrap()
        .unwrap()
        .tags
        .get("Artist")
    {
        Some(artist) => artist.to_string(),
        None => "Unknown".to_string(),
    }
}

fn get_albumname(connection: &mut Client) -> String {
    match connection.currentsong().unwrap().unwrap().tags.get("Album") {
        Some(album_name) => album_name.to_string(),
        None => "Unknown".to_string(),
    }
}

pub fn get_album_cover(music_directory: String, album_directory: String) -> String {
    //println!("{}{}", music_directory, album_directory);
    if std::path::Path::new(&format!("{}{}/cover.png", music_directory, album_directory)[..])
        .exists()
    {
        //print!("png");
        return format!("{}/cover.png", album_directory);
    } else {
        //print!("jpg");
        return format!("{}/cover.jpg", album_directory);
    }
}

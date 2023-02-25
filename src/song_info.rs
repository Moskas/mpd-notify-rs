use mpd::Client;

pub fn get_metadata() -> Vec<String> {
    let mut connection = Client::connect("127.0.0.1:6600").unwrap();
    let music_dir: String = "/home/moskas/Music/".to_string();
    let metadata = [
        get_artist(&mut connection),
        get_title(&mut connection),
        get_albumname(&mut connection),
        format!(
            "{}{}",
            music_dir,
            get_album_cover(
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
    //    println!(
    //        "{}",
    //        format!(
    //            "{}{}",
    //            music_dir,
    //            connection
    //                .currentsong()
    //                .unwrap()
    //                .unwrap()
    //                .file
    //                .split("/")
    //                .next()
    //                .unwrap()
    //        ),
    //    );
    //println!("{:?}", metadata);
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

pub fn get_album_cover(album_direcotry: String) -> String {
    if std::path::Path::new(&format!("{}cover.png", album_direcotry)[..]).exists() {
        //print!("png");
        return format!("{}/cover.png", album_direcotry);
    } else {
        //print!("jpg");
        return format!("{}/cover.jpg", album_direcotry);
    }
}

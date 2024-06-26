use mpd::Client;
use walkdir::WalkDir;

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
  let file_names = ["cover.png", "cover.jpg", "Cover.png", "Cover.jpg"];
  let search_path: WalkDir = WalkDir::new(format!("{}{}", music_directory, album_directory));
  for entry in search_path.into_iter() {
    let temp_entry = entry.unwrap();
    if let Some(file_name) = temp_entry.file_name().to_str() {
      if file_names.contains(&file_name) {
        return format!("{}", temp_entry.path().display());
      }
    }
  }
  String::from("None")
}

use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Deserialize)]
pub struct Mpd {
    pub music_dir: String,
    pub ip: String,
    pub port: String,
    pub use_cover: bool,
}

//#[derive(Deserialize)]
//pub struct Notification {
//    pub use_cover: bool,
//    pub show_status_changes: bool,
//}

pub fn get_config() -> Mpd {
    //println!("{}", std::env::var("HOME").unwrap());
    let config_path: String = format!(
        "{}/.config/mpd-notification/config.toml",
        dirs::home_dir().unwrap().display() // .config in users $HOME directory
    );
    //println!("{}", config_path);
    //println!("{}", read_to_string(config_path).unwrap());
    let config_file: &str = &read_to_string(config_path).unwrap()[..];
    let config: Mpd = toml::from_str(config_file).unwrap();
    //println!(
    //    "Music directory: {}\nMPD server ip: {}:{}",
    //    config.music_dir, config.ip, config.port
    //);
    return config;
}

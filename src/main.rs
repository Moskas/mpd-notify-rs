mod config;
mod notification;
mod song_info;
use mpd::{Client, State};
use std::thread::sleep;
use std::time;

use notification::send_notification;
use song_info::get_metadata;

fn handle_connection(config: &config::MpdConfig, music_dir: &String) {
    if let Ok(connected) = Client::connect(&format!("{}:{}", config.ip, config.port)) {
        let mut connection = connected;
        let mut current_song = String::new();
        let mut current_status = State::Pause;

        loop {
            if let Ok(song) = connection.currentsong() {
                if let Some(song_info) = song {
                    if connection.status().unwrap().state != current_status
                        || get_metadata(music_dir.clone(), &mut connection).title != current_song
                    {
                        current_song = song_info.title.unwrap();
                        match connection.status().expect("Can not get current playback state").state {
                            State::Play => {
                                current_status = State::Play;
                                send_notification(
                                    get_metadata(music_dir.clone(), &mut connection),
                                    State::Play,
                                    config.use_cover,
                                );
                            }
                            State::Stop => {
                                current_status = State::Stop;
                                send_notification(
                                    get_metadata(music_dir.clone(), &mut connection),
                                    State::Stop,
                                    config.use_cover,
                                );
                            }
                            State::Pause => {
                                current_status = State::Pause;
                                send_notification(
                                    get_metadata(music_dir.clone(), &mut connection),
                                    State::Pause,
                                    config.use_cover,
                                );
                            }
                        };
                    }
                }
                sleep(time::Duration::from_millis(500));
            } else {
                sleep(time::Duration::from_millis(500));
            }
        }
    } else {
        eprintln!("Couldn't connect, is mpd daemon running?");
        std::thread::sleep(std::time::Duration::from_millis(5000));
    }
}

fn main() {
    let config_result = config::MpdConfig::from_file();
    let music_dir: String;

    let config = match config_result {
        Ok(cfg) => {
            music_dir = cfg.music_dir.clone();
            cfg
        }
        Err(err) => {
            eprintln!("Error loading configuration: {}", err);
            return;
        }
    };

    loop {
        handle_connection(&config, &music_dir);
    }
}

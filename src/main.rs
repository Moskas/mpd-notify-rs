mod config;
mod notification;
mod song_info;
use mpd::{Client, State};
use std::thread::sleep;
use std::time;

use notification::send_notification;
use song_info::get_metadata;

fn main() {
    let config = config::get_config();
    let music_dir: String = config.music_dir;
    loop {
        match Client::connect(&format!("{}:{}", config.ip, config.port)[..]) {
            Ok(connected) => {
                let mut connection = connected;
                let mut current_song = String::new();
                let mut current_status = State::Pause;
                loop {
                    if connection.currentsong().expect("Queue error") != None {
                        if connection.status().unwrap().state != current_status
                            || get_metadata(music_dir.clone(), &mut connection)[1] != current_song
                        {
                            current_song =
                                connection.currentsong().unwrap().unwrap().title.unwrap();
                            match connection
                                .status()
                                .expect("Can not get current playback state")
                                .state
                            {
                                State::Play => {
                                    current_status = State::Play;
                                    //current_song = connection.currentsong().unwrap().unwrap().title.unwrap();
                                    send_notification(
                                        get_metadata(music_dir.clone(), &mut connection),
                                        State::Play,
                                        config.use_cover,
                                    );
                                }
                                State::Stop => {
                                    current_status = State::Stop;
                                    //println!("Playback stopped");
                                    send_notification(
                                        get_metadata(music_dir.clone(), &mut connection),
                                        State::Stop,
                                        config.use_cover,
                                    );
                                }
                                State::Pause => {
                                    current_status = State::Pause;
                                    //println!("Playback paused");
                                    send_notification(
                                        get_metadata(music_dir.clone(), &mut connection),
                                        State::Pause,
                                        config.use_cover,
                                    );
                                }
                            };
                        };
                        sleep(time::Duration::from_millis(500));
                    } else {
                        sleep(time::Duration::from_millis(500));
                    }
                }
            }
            Err(error) => {
                println!("Couldn't connect, is mpd daemon running? ({})", error);
                std::thread::sleep(std::time::Duration::from_millis(5000));
            }
        }
    }
}

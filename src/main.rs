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
    let mut connection = Client::connect(&format!("{}:{}", config.ip, config.port)[..]).unwrap();
    let mut current_song = String::new();
    let mut current_status = State::Pause;
    loop {
        if connection.currentsong().unwrap() != None {
            if connection.status().unwrap().state != current_status
                || get_metadata()[1] != current_song
            {
                current_song = connection.currentsong().unwrap().unwrap().title.unwrap();
                match connection
                    .status()
                    .expect("Can not get current playback state")
                    .state
                {
                    State::Play => {
                        current_status = State::Play;
                        //current_song = connection.currentsong().unwrap().unwrap().title.unwrap();
                        send_notification(get_metadata(), State::Play);
                    }
                    State::Stop => {
                        current_status = State::Stop;
                        //println!("Playback stopped");
                        send_notification(get_metadata(), State::Stop);
                    }
                    State::Pause => {
                        current_status = State::Pause;
                        //println!("Playback paused");
                        send_notification(get_metadata(), State::Pause);
                    }
                };
            };
            sleep(time::Duration::from_millis(500));
        } else {
            sleep(time::Duration::from_millis(500));
        }
    }
}

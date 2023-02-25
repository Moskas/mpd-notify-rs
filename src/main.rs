mod notification;
use std::thread::sleep;
use std::time;

use notification::send_notification;
mod song_info;
use song_info::get_metadata;

fn main() {
    let mut current_song: Vec<String> = get_metadata();
    loop {
        if get_metadata() != current_song {
            send_notification(get_metadata());
            current_song = get_metadata();
        };
        //println!("{}", current_song[3]);
        sleep(time::Duration::from_millis(500));
    }
}

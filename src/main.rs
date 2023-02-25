mod notification;
use notification::send_notification;
mod song_info;
use song_info::get_metadata;

fn main() {
    send_notification(get_metadata());
}

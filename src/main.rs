mod config;
mod notification;
mod song_info;

use mpd::{Client, Song, State};
use notification::send_notification;
use song_info::get_metadata;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

#[derive(Debug)]
struct PlayerState {
  current_song: String,
  current_status: State,
  last_notification_time: Instant,
}

impl PlayerState {
  fn new() -> Self {
    PlayerState {
      current_song: String::new(),
      current_status: State::Pause,
      last_notification_time: Instant::now(),
    }
  }

  fn update(&mut self, song: &Song, status: State) -> bool {
    let song_title = song.title.clone().unwrap_or_default();
    if song_title != self.current_song || status != self.current_status {
      self.current_song = song_title;
      self.current_status = status;
      let now = Instant::now();
      if now.duration_since(self.last_notification_time) > Duration::from_millis(500) {
        self.last_notification_time = now;
        return true;
      }
    }
    false
  }
}

fn handle_connection(config: &config::MpdConfig, music_dir: &str, running: Arc<AtomicBool>) {
  if let Ok(connected) = Client::connect(&format!("{}:{}", config.ip, config.port)) {
    let mut connection = connected;
    let mut state = PlayerState::new();

    while running.load(Ordering::SeqCst) {
      match connection.currentsong() {
        Ok(Some(song)) => {
          let status = connection.status().unwrap().state;
          if state.update(&song, status) {
            send_notification(
              get_metadata(music_dir.to_string(), &mut connection),
              state.current_status,
              config.use_cover,
            );
          }
        }
        Ok(None) => {
          println!("No song currently playing.");
        }
        Err(err) => {
          eprintln!("Error fetching current song: {}", err);
        }
      }
      sleep(Duration::from_millis(500));
    }
  } else {
    eprintln!(
      "Error connecting to MPD server at {}:{}",
      config.ip, config.port
    );
  }
}

fn main() {
  let config_result = config::MpdConfig::from_file();
  let config = match config_result {
    Ok(cfg) => cfg,
    Err(err) => {
      eprintln!("Error loading configuration: {}", err);
      return;
    }
  };

  let music_dir = config.music_dir.clone();
  let running = Arc::new(AtomicBool::new(true));
  let handle = spawn(move || {
    handle_connection(&config, &music_dir, running);
  });

  handle.join().unwrap();
}

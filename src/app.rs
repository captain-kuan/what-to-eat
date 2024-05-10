use std::error;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::components::Popup;
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub options: Vec<String>,
    pub popup: Popup,
    /// Is the application running?
    pub running: bool,
    /// counter
    pub lucky_num: usize,
    pub end_num: usize,
    start_time: SystemTime,
}

impl Default for App {
    fn default() -> Self {
        Self {
            options: vec![
                "Option1".to_string(),
                "Option2".to_string(),
                "Option3".to_string(),
                "Option4".to_string(),
                "Option5".to_string(),
                "Option6".to_string(),
                "Option7".to_string(),
                "Option8".to_string(),
            ],
            popup: Popup::default(),
            running: true,
            lucky_num: 9,
            end_num: 9,
            start_time: UNIX_EPOCH,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if self.end_num != 9 {
            self.lucky_num = rand::random::<usize>() % 8;
            if let Ok(duration) = SystemTime::now().duration_since(self.start_time) {
                if duration.as_secs() > 3 {
                    self.lucky_num = self.end_num;
                    self.popup.show();
                }
            }
        }
    }
    pub fn start(&mut self) {
        self.popup.hide();
        self.end_num = rand::random::<usize>() % 8;
        self.start_time = SystemTime::now();
    }
    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}

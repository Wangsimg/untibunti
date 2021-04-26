use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub audio_folder_path: String,
    pub messages: Messages,
}

/// The key is the audio file name
type Messages = HashMap<String, Message>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub display_name: String,
    pub volume: f32,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let file_size = file.metadata()?.len();

    if fi
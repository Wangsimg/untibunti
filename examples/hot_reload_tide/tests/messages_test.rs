use hot_reload_tide::messages::{load_config, Config};

#[test]
fn load_config_from_file() {
    let Config {
        audio_folder_path,
        messages,
    } = load_conf
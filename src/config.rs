use dirs::config_dir;
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonConfig {
    pub monitor_name: String,
    pub device_ids: DeviceIds,
    pub lunar_command: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct DeviceIds {
    #[serde(deserialize_with = "from_hex")]
    pub hex: Vec<u16>,
}
fn from_hex<'de, D>(deserializer: D) -> Result<Vec<u16>, D::Error>
where
    D: Deserializer<'de>,
{
    let strings: Vec<&str> = Deserialize::deserialize(deserializer)?;
    let mut response: Vec<u16> = Vec::new();
    for string in strings {
        let without_prefix = string.trim_start_matches("0x");
        let value = u16::from_str_radix(without_prefix, 16).unwrap_or(0);
        if value > 0 {
            response.push(value);
        }
    }
    return Ok(response);
}
pub struct Config {
    pub data: JsonConfig,
    pub config_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let config_dir = config_dir().expect("Failed to get config dir");
        let file_path = Path::new(&config_dir).join("MacEvents/config.json");

        println!("Config path: {}", file_path.to_str().expect("as"));
        let config_data = Config::read_config_file(file_path.clone()).expect("No config");

        let instance = Self {
            config_path: file_path,
            data: config_data,
        };

        return instance;
    }
    fn read_config_file(config_path: PathBuf) -> Option<JsonConfig> {
        let mut file: File = match OpenOptions::new().read(true).open(config_path.clone()) {
            Err(_) => {
                Config::create_empty_config(config_path.clone());
                OpenOptions::new()
                    .read(true)
                    .open(config_path.clone())
                    .expect("Couldn't open empty file")
            }
            Ok(file) => file,
        };
        let mut response = String::new();
        match file.read_to_string(&mut response) {
            Err(_) => Config::create_empty_config(config_path),
            Ok(_) => (),
        }

        let result: JsonConfig = serde_json::from_str(&response).expect("Wrong config!");
        return Some(result);
    }

    fn create_empty_config(config_path: PathBuf) {
        let empty_data = JsonConfig {
            monitor_name: String::new(),
            device_ids: DeviceIds { hex: Vec::new() },
            lunar_command: "lunar".to_string(),
        };
        let data =
            serde_json::to_string_pretty(&empty_data).expect("Failed to serialize empty config");

        if !Path::exists(&config_path.clone()) {
            let parent_path = config_path.parent().expect("No parent dir");
            if !Path::exists(&parent_path) {
                create_dir_all(parent_path).expect("Couldn't create directories to config");
            }
            File::create(config_path.clone()).expect("Couldn't create config");
        }

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(config_path)
            .expect("Couldn't open or create config.json");
        write!(file, "{}", data).expect("Couldn't create config file");
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

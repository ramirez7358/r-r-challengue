use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct Config {
    pub api: Api,
}

#[derive(Clone, Deserialize)]
pub struct Api {
    pub bind: String,
    pub workers: usize,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open("env/env.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config = serde_json::from_str::<Config>(&contents)?;
    Ok(config)
}

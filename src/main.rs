mod helpers;
mod models;

use crate::helpers::{get_env, get_input};
use crate::models::parse_inputs;
use itertools::Itertools;
use std::collections::HashMap;
use std::{error, fs};

fn run() -> Result<(), Box<dyn error::Error>> {
    let badge_input = parse_inputs();
    let output_file = get_env("GITHUB_OUTPUT")?;
    let api_url = get_input("api-url").ok_or("api-url not provided")?;
    let read_write_key = get_input("read-write-key").ok_or("read-write-key not provided")?;
    let (read_key, write_key) = read_write_key
        .split(':')
        .map(|x| x.to_string())
        .collect_tuple::<(String, String)>()
        .ok_or("read-write-key is not in the correct format, needs to be as read_key:write_key")?;

    let client = reqwest::blocking::Client::new();
    let response = client
        .put(format!("{}/badge", api_url))
        .json(&badge_input)
        .basic_auth(read_key, Some(write_key))
        .send()?;
    if !response.status().is_success() {
        return Err(format!(
            "Request failure {}: {}",
            response.status(),
            response.text()?
        )
        .into());
    }
    let json = response.json::<HashMap<String, String>>()?;
    fs::write(
        output_file,
        json.into_iter()
            .map(|(key, value)| format!("{}={}", key.replace('_', "-"), value))
            .collect::<Vec<_>>()
            .join("\n"),
    )?;
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

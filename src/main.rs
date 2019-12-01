use std::error::Error;
use std::io::{self, Read};
use std::process::exit;

use serde_json::Value as JsonValue;
use toml::Value as TomlValue;

fn main() {
    match process() {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }
    }
}

fn process() -> Result<String, Box<dyn Error>> {
    let input = read_stdin_to_string()?;
    let value = toml::from_str::<TomlValue>(&input)?;
    let output = serde_json::to_string_pretty(&toml_to_json(value))?;
    Ok(output)
}

fn read_stdin_to_string() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn toml_to_json(value: toml::Value) -> serde_json::Value {
    match value {
        TomlValue::Boolean(value) => JsonValue::Bool(value),
        TomlValue::String(value) => JsonValue::String(value),
        TomlValue::Integer(value) => value.into(),
        TomlValue::Float(value) => value.into(),
        TomlValue::Datetime(value) => JsonValue::String(value.to_string()),
        TomlValue::Array(value) => value.into_iter().map(toml_to_json).collect(),
        TomlValue::Table(value) => {
            let map = value
                .into_iter()
                .map(|(k, v)| (k, toml_to_json(v)))
                .collect();
            JsonValue::Object(map)
        }
    }
}

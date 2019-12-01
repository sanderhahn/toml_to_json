use std::error::Error;
use std::io::{self, Read, Write};

use serde_json::Value as JsonValue;
use toml::Value as TomlValue;

fn main() -> Result<(), Box<dyn Error>> {
    let text = read_stdin_to_string()?;
    let toml = toml::from_str::<TomlValue>(&text)?;
    let json = toml_to_json(toml);
    let output = serde_json::to_string_pretty(&json)?;
    io::stdout().write_all(output.as_bytes())?;
    Ok(())
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

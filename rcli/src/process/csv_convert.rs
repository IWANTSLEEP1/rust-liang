use std::fs;

use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::opt::OutputFormat;
#[derive(Debug, Serialize, Deserialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut v = Vec::new();
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        v.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&v)?,
        OutputFormat::Yaml => serde_yaml::to_string(&v)?,
        OutputFormat::Toml => toml::to_string(&v)?,
    };
    fs::write(output, content)?;
    Ok(())
}

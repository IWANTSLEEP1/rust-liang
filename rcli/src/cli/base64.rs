use std::{fmt, str::FromStr};

use clap::Parser;

use super::verify_input_file;
#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "base64 encode")]
    Encode(EncodeOpts),

    #[command(name = "decode", about = "base64 decode")]
    Decode(DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct EncodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value="-")]
    pub input: String,
    #[arg(long,value_parser=verify_format, default_value="standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct DecodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value="-")]
    pub input: String,
    #[arg(long,value_parser=verify_format, default_value="standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser, Clone, Copy)]
pub enum Base64Format {
    STANDARD,
    URLSAFE,
}

fn verify_format(fmt: &str) -> Result<Base64Format, anyhow::Error> {
    fmt.parse::<Base64Format>()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::STANDARD),
            "urlsafe" => Ok(Base64Format::URLSAFE),
            other => anyhow::bail!("Invalid format:{}", other),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::STANDARD => "standard",
            Base64Format::URLSAFE => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

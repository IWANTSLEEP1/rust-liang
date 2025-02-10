use std::{path::Path, str::FromStr};

use anyhow::bail;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "show csv, or convert csv to json")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate password")]
    GenPass(GenPassOpts),
}

#[derive(Debug, Parser, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub number: bool,
    #[arg(long, default_value_t = true)]
    pub symbols: bool,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// Name of the person to greet
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long, value_parser=parse_format, default_value = "json")]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // Rust 的 .into() 方法是 Into<T> trait 的快捷调用方式，能够将可转换类型自动转换为目标类型
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("file not found")
    }
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

fn parse_format(fmt: &str) -> Result<OutputFormat, anyhow::Error> {
    fmt.parse::<OutputFormat>()
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            other => bail!("Invalid format:{}", other),
        }
    }
}

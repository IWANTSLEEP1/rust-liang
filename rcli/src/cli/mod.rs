mod base64;
mod csv;
mod genpass;
use std::path::Path;

pub use self::{base64::Base64Format, base64::Base64SubCommand, csv::OutputFormat};
use self::{csv::CsvOpts, genpass::GenPassOpts};
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
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // Rust 的 .into() 方法是 Into<T> trait 的快捷调用方式，能够将可转换类型自动转换为目标类型
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("file not found")
    }
}

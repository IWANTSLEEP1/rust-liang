use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::{fs::File, io::Read};

use crate::cli::Base64Format;

fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input).unwrap())
    };
    Ok(reader)
}

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::STANDARD => STANDARD.encode(&buf),
        Base64Format::URLSAFE => URL_SAFE_NO_PAD.encode(&buf),
    };

    println!("{}", encode);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decode = match format {
        Base64Format::STANDARD => STANDARD.decode(&buf),
        Base64Format::URLSAFE => URL_SAFE_NO_PAD.decode(&buf),
    }?;

    println!("{:?}", String::from_utf8(decode));
    Ok(())
}

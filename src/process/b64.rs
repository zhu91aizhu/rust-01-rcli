use std::io::Read;

use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::cli::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let mut buf = Vec::new();
    let _ = reader.read_to_end(&mut buf);
    let encoded = match format {
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
        Base64Format::Standard => STANDARD.encode(buf),
    };

    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let mut buf = String::new();
    let _ = reader.read_to_string(&mut buf);
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
        Base64Format::Standard => STANDARD.decode(buf)?,
    };

    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>, anyhow::Error> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        Ok(Box::new(std::fs::File::open(input)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fuxtures/b64.txt";
        let format = Base64Format::Standard;
        assert!(process_decode(input, format).is_ok());
    }
}

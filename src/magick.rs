use std::process::{Command, Stdio};
use std::path::Path;
use std::ffi::OsString;
use regex::Regex;
use crate::error::*;

#[derive(Debug)]
pub struct Layer {
    pub index: usize,
    pub width: u32,
    pub height: u32,
    pub color_depth: u32,
}

fn identify(path: &Path) -> Result<String> {
    let child = Command::new("identify")
        .arg("-format")
        .arg("%w %h %z\\n")
        .arg(path)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(Error::Io)?;
    let output = child.wait_with_output().map_err(Error::Io)?;
    let result = String::from_utf8(output.stdout).map_err(Error::BadImageFormat)?;
    Ok(result)
}

fn parse_layers(info: &str) -> Result<Vec<Layer>> {
    let re = Regex::new(r##"(?xi)
        (\d+) \s+ (\d+) \s+ (\d+) \n
    "##).unwrap();

    let mut layers = Vec::new();
    for (index, layer_match) in re.captures_iter(info).enumerate() {
        let width = u32::from_str_radix(&layer_match[1], 10).unwrap();
        let height = u32::from_str_radix(&layer_match[2], 10).unwrap();
        let depth = u32::from_str_radix(&layer_match[3], 10).unwrap();
        layers.push(Layer { index, width, height, color_depth: depth });
    }
    Ok(layers)
}

pub fn layers(path: &Path) -> Result<Vec<Layer>> {
    let info = identify(path)?;
    let layers = parse_layers(&info)?;
    Ok(layers)
}

pub fn convert(path: &Path, layer_index: usize, output: &Path) -> Result<()> {
    let mut arg = OsString::from(path);
    arg.push("[");
    arg.push(OsString::from(layer_index.to_string()));
    arg.push("]");

    let mut child = Command::new("convert")
        .arg(arg)
        .arg("-resize")
        .arg("16x16>")
        .arg(output)
        .spawn()
        .map_err(Error::Io)?;
    child.wait().map_err(Error::Io)?;
    Ok(())
}

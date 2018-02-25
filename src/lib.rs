extern crate regex;
extern crate url;
extern crate reqwest;
extern crate tempfile;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;
extern crate structopt;
extern crate base64;

pub mod error;
mod find_favicon;
mod favicon_url;
mod favicon;
mod download;
mod magick;

use std::path::Path;
use error::*;

fn layer_weight(layer: &magick::Layer) -> (u32, u32, u32, u32) {
    let small = if layer.width <= 16 && layer.height <= 16 { 1 } else { 0 };
    (small, layer.width, layer.height, layer.color_depth)
}

pub fn get_favicon(page_url: &str, output_file: &Path) -> Result<()> {
    let favicon = download::download_favicon(page_url)?;
    let file = favicon.save_to_temporary()?;
    let layers = magick::layers(file.path())?;
    let best = layers.iter().max_by_key(|layer| layer_weight(layer)).ok_or_else(|| BadImage)?;
    magick::convert(file.path(), best.index, output_file)?;
    Ok(())
}

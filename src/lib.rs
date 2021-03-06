mod download;
pub mod error;
mod favicon;
mod favicon_url;
mod find_favicon;
mod magick;

use crate::error::{Error, Result};
use std::path::Path;

fn layer_weight(layer: &magick::Layer) -> (u32, u32, u32, u32) {
    let small = if layer.width <= 16 && layer.height <= 16 {
        1
    } else {
        0
    };
    (small, layer.width, layer.height, layer.color_depth)
}

pub async fn get_favicon(page_url: &str, output_file: &Path) -> Result<()> {
    let favicon = download::download_favicon(page_url).await?;
    let file = favicon.save_to_temporary()?;
    let layers = magick::layers(file.path())?;
    let best = layers
        .iter()
        .max_by_key(|layer| layer_weight(layer))
        .ok_or(Error::BadImage)?;
    magick::convert(file.path(), best.index, output_file)?;
    Ok(())
}

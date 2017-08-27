extern crate regex;
extern crate url;
extern crate reqwest;
extern crate tempfile;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod error;
mod find_favicon;
mod favicon_url;
mod favicon;
mod download;
mod magick;

use std::path::Path;
use structopt::StructOpt;
use error::*;

fn layer_weight(layer: &magick::Layer) -> (u32, u32, u32, u32) {
    let small = if layer.width <= 16 && layer.height <= 16 { 1 } else { 0 };
    (small, layer.width, layer.height, layer.color_depth)
}

#[derive(StructOpt, Debug)]
#[structopt(name = "getfavicon", about = "Downloads a favicon for a given page.")]
struct Opts {
    #[structopt(help = "URL of a page")]
    page_url: String,

    #[structopt(help = "Output file")]
    output_file: String,
}

quick_main!(|| -> Result<()> {
    let args = Opts::from_args();
    let favicon = download::download_favicon(&args.page_url)?;
    let file = favicon.save_to_temporary()?;
    let layers = magick::layers(file.path())?;
    let best = layers.iter().max_by_key(|layer| layer_weight(layer)).expect("Best layer should exist.");
    magick::convert(file.path(), best.index, Path::new(&args.output_file))?;
    Ok(())
});

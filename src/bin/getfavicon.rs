#[macro_use]
extern crate error_chain;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate getfavicon;

use std::path::Path;
use structopt::StructOpt;
use getfavicon::get_favicon;
use getfavicon::error::*;

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
    get_favicon(&args.page_url, Path::new(&args.output_file))?;
    Ok(())
});

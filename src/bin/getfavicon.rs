extern crate failure;
extern crate structopt;
extern crate getfavicon;

use std::path::Path;
use structopt::StructOpt;
use getfavicon::get_favicon;

#[derive(StructOpt, Debug)]
#[structopt(name = "getfavicon", about = "Downloads a favicon for a given page.")]
struct Opts {
    #[structopt(help = "URL of a page")]
    page_url: String,

    #[structopt(help = "Output file")]
    output_file: String,
}

fn main() {
    let args = Opts::from_args();
    let result = get_favicon(&args.page_url, Path::new(&args.output_file));

    if let Err(e) = result {
        println!("{:?}", e);
        ::std::process::exit(1);
    }
}

use getfavicon::get_favicon;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "getfavicon", about = "Downloads a favicon for a given page.")]
struct Opts {
    #[structopt(help = "URL of a page")]
    page_url: String,

    #[structopt(help = "Output file")]
    output_file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), getfavicon::error::Error> {
    let args = Opts::from_args();
    get_favicon(&args.page_url, &args.output_file).await?;
    Ok(())
}

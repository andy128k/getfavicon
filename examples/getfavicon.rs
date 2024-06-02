use getfavicon::get_favicon;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "getfavicon", about = "Downloads a favicon for a given page.")]
struct Opts {
    /// URL of a page
    page_url: String,

    /// Output file
    output_file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), getfavicon::error::Error> {
    let args = Opts::parse();
    get_favicon(&args.page_url, &args.output_file).await?;
    Ok(())
}

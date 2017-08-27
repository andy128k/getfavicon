use std::io::Read;
use reqwest;
use reqwest::Response;
use reqwest::header::ContentType;
use reqwest::mime::{Mime, CHARSET};

use find_favicon;
use favicon_url;
use favicon::Favicon;
use error::*;

fn ensure_status_successfull(response: &Response) -> Result<()> {
    let status = response.status();
    if status.is_success() {
        Ok(())
    } else {
        Err(ErrorKind::UnexpectedStatus(status).into())
    }
}

fn get_mime(response: &Response) -> Option<Mime> {
    response.headers().get::<ContentType>().map(|h| (**h).clone())
}

fn get_charset(mime: Mime) -> Option<String> {
    mime.get_param(CHARSET).map(|ref c| c.to_string())
}

fn fetch_page(page_url: &str) -> Result<String> {
    let mut response = reqwest::get(page_url)?;
    let charset = get_mime(&response).and_then(get_charset).unwrap_or("utf-8".to_string());
    let mut bytes = Vec::<u8>::new();
    response.read_to_end(&mut bytes)?;
    let content = String::from_utf8_lossy(&bytes).to_string();
    Ok(content)
}

fn fetch_favicon(favicon_url: &str) -> Result<Favicon> {
    let mut favicon_response = reqwest::get(favicon_url)?;
    ensure_status_successfull(&favicon_response)?;

    let mut favicon_content = Vec::<u8>::new();
    favicon_response.read_to_end(&mut favicon_content)?;

    let mime = get_mime(&favicon_response);
    let filename = favicon_url::favicon_filename(favicon_url).ok();

    Ok(Favicon { filename, mime, content: favicon_content })
}

pub fn download_favicon(page_url: &str) -> Result<Favicon> {
    let page_content = fetch_page(page_url)?;
    let parsed_url = find_favicon::find_favicon(&page_content);
    let favicon_url = favicon_url::favicon_url(parsed_url.as_ref().map(String::as_str), page_url)?;
    let favicon = fetch_favicon(&favicon_url)?;
    Ok(favicon)
}

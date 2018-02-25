use std::io::Read;
use reqwest;
use reqwest::Response;
use reqwest::header::ContentType;
use reqwest::mime::{Mime, CHARSET};
use url::Url;
use regex::Regex;
use base64;

use find_favicon;
use favicon_url;
use favicon::Favicon;
use error::*;

fn ensure_status_successfull(response: &Response) -> Result<()> {
    let status = response.status();
    if status.is_success() {
        Ok(())
    } else {
        Err(Error::UnexpectedStatus(status))
    }
}

fn get_mime(response: &Response) -> Option<Mime> {
    response.headers().get::<ContentType>().map(|h| (**h).clone())
}

fn get_charset(mime: Mime) -> Option<String> {
    mime.get_param(CHARSET).map(|ref c| c.to_string())
}

fn fetch_page(page_url: &str) -> Result<String> {
    let mut response = reqwest::get(page_url).map_err(Error::Request)?;
    let charset = get_mime(&response).and_then(get_charset).unwrap_or("utf-8".to_string());
    let mut bytes = Vec::<u8>::new();
    response.read_to_end(&mut bytes).map_err(Error::Io)?;
    let content = String::from_utf8_lossy(&bytes).to_string();
    Ok(content)
}

fn fetch_http_favicon(favicon_url: &str) -> Result<Favicon> {
    let mut favicon_response = reqwest::get(favicon_url).map_err(Error::Request)?;
    ensure_status_successfull(&favicon_response)?;

    let mut favicon_content = Vec::<u8>::new();
    favicon_response.read_to_end(&mut favicon_content).map_err(Error::Io)?;

    let mime = get_mime(&favicon_response);
    let filename = favicon_url::favicon_filename(favicon_url).ok();

    Ok(Favicon { filename, mime, content: favicon_content })
}

lazy_static! {
    static ref DATA_URL: Regex = Regex::new(r"([^;]+);base64,(.*)$").unwrap();
}

fn fetch_data_favicon(url: &Url) -> Result<Favicon> {
    let path = url.path();

    let matches = DATA_URL.captures(path).ok_or_else(|| Error::UnsupportedDataURLEncoding)?;
    let mime_str = &matches[1];
    let content_base64 = &matches[2];

    let mime = mime_str.parse::<Mime>().ok();
    let content = base64::decode(content_base64).map_err(Error::BadImageData)?;

    Ok(Favicon { filename: None, mime, content })
}

fn fetch_favicon(favicon_url: &str) -> Result<Favicon> {
    let url = Url::parse(favicon_url).map_err(Error::UrlParse)?;
    match url.scheme() {
        "http" | "https" => fetch_http_favicon(favicon_url),
        "data" => fetch_data_favicon(&url),
        scheme@_ => Err(Error::UnsupportedScheme(scheme.to_owned()))
    }
}

pub fn download_favicon(page_url: &str) -> Result<Favicon> {
    let page_content = fetch_page(page_url)?;
    let parsed_url = find_favicon::find_favicon(&page_content);
    let favicon_url = favicon_url::favicon_url(parsed_url.as_ref().map(String::as_str), page_url)?;
    let favicon = fetch_favicon(&favicon_url)?;
    Ok(favicon)
}

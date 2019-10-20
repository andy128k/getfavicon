use std::io::Read;
use std::str::FromStr;
use reqwest::{self, Response, header::CONTENT_TYPE};
use mime::Mime;
use url::Url;
use regex::Regex;
use lazy_static::lazy_static;

use crate::find_favicon;
use crate::favicon_url;
use crate::favicon::Favicon;
use crate::error::*;

impl std::convert::From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Request(error)
    }
}

fn get_mime(response: &Response) -> Option<Mime> {
    response.headers().get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| Mime::from_str(value).ok())
}

fn fetch_page(page_url: &str) -> Result<String> {
    let content = reqwest::get(page_url)?
        .error_for_status()?
        .text()?;
    Ok(content)
}

fn fetch_http_favicon(favicon_url: &str) -> Result<Favicon> {
    let mut favicon_response = reqwest::get(favicon_url)?
        .error_for_status()?;

    let mime = get_mime(&favicon_response);
    let filename = favicon_url::favicon_filename(favicon_url).ok();

    let mut content = Vec::new();
    favicon_response.read_to_end(&mut content).map_err(Error::Io)?;

    Ok(Favicon { filename, mime, content })
}

fn fetch_data_favicon(url: &Url) -> Result<Favicon> {
    lazy_static! {
        static ref DATA_URL: Regex = Regex::new(r"([^;]+);base64,(.*)$").unwrap();
    }

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
        scheme => Err(Error::UnsupportedScheme(scheme.to_owned()))
    }
}

pub fn download_favicon(page_url: &str) -> Result<Favicon> {
    let page_content = fetch_page(page_url)?;
    let parsed_url = find_favicon::find_favicon(&page_content);
    let favicon_url = favicon_url::favicon_url(parsed_url.as_ref().map(String::as_str), page_url)?;
    let favicon = fetch_favicon(&favicon_url)?;
    Ok(favicon)
}

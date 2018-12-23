use std::convert::AsRef;
use std::io::Write;
use regex::Regex;
use mime::{Mime, IMAGE_JPEG, IMAGE_PNG, IMAGE_GIF};
use lazy_static::lazy_static;
use crate::error::*;

#[derive(Debug)]
pub struct Favicon {
    pub filename: Option<String>,
    pub mime: Option<Mime>,
    pub content: Vec<u8>,
}

lazy_static! {
    static ref SUFFIX_ICO: Regex = Regex::new(r"(?i)\.ico$").unwrap();
    static ref SUFFIX_JPG: Regex = Regex::new(r"(?i)\.(jpg|jpeg)$").unwrap();
    static ref SUFFIX_PNG: Regex = Regex::new(r"(?i)\.png$").unwrap();
    static ref SUFFIX_GIF: Regex = Regex::new(r"(?i)\.gif$").unwrap();

    static ref IMAGE_ICO: Mime = "image/vnd.microsoft.icon".parse::<Mime>().unwrap();
}

fn filename_to_suffix<S: AsRef<str>>(filename: S) -> Option<&'static str> {
    if SUFFIX_ICO.is_match(filename.as_ref()) {
        Some(".ico")
    } else if SUFFIX_JPG.is_match(filename.as_ref()) {
        Some(".jpg")
    } else if SUFFIX_PNG.is_match(filename.as_ref()) {
        Some(".png")
    } else if SUFFIX_GIF.is_match(filename.as_ref()) {
        Some(".gif")
    } else {
        None
    }
}

fn mime_to_suffix(mime: &Mime) -> Option<&'static str> {
    if *mime == *IMAGE_ICO {
        Some(".ico")
    } else if *mime == IMAGE_JPEG {
        Some(".jpg")
    } else if *mime == IMAGE_PNG {
        Some(".png")
    } else if *mime == IMAGE_GIF {
        Some(".gif")
    } else {
        None
    }
}

impl Favicon {
    pub fn save_to_temporary(&self) -> Result<tempfile::NamedTempFile> {
        let mut file = tempfile::Builder::new()
            .prefix("favicon-")
            .suffix(self.suffix())
            .rand_bytes(10)
            .tempfile()
            .map_err(Error::Io)?;
        file.write_all(&self.content)
            .map_err(Error::Io)?;
        Ok(file)
    }

    fn suffix(&self) -> &'static str {
        self.filename.as_ref().and_then(filename_to_suffix)
            .or_else(|| self.mime.as_ref().and_then(mime_to_suffix))
            .unwrap_or(".ico")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_favicon(filename: impl Into<Option<&'static str>>, mime: impl Into<Option<Mime>>) -> Favicon {
        Favicon {
            filename: filename.into().map(|v| v.to_owned()),
            mime: mime.into(),
            content: Vec::new()
        }
    }

    #[test]
    fn test_favicon_suffix_empty() {
        assert_eq!(make_favicon(None, None).suffix(), ".ico");
    }

    #[test]
    fn test_favicon_suffix_png() {
        assert_eq!(make_favicon("favicon.png", None).suffix(), ".png");
    }

    #[test]
    fn test_favicon_suffix_gif() {
        assert_eq!(make_favicon("XXXXXX.GIF", None).suffix(), ".gif");
    }

    #[test]
    fn test_favicon_suffix_unknown() {
        assert_eq!(make_favicon("XXXXXX.YYY", None).suffix(), ".ico");
    }

    #[test]
    fn test_favicon_suffix_mime_jpeg() {
        assert_eq!(make_favicon(None, IMAGE_JPEG).suffix(), ".jpg");
    }

    #[test]
    fn test_favicon_suffix_mime_ico() {
        assert_eq!(make_favicon(None, IMAGE_ICO.clone()).suffix(), ".ico");
    }
}

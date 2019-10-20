use crate::error::*;
use url::Url;

fn favicon_parsed(parsed_opt: Option<&str>) -> Result<String> {
    let parsed = parsed_opt.ok_or(Error::NoLink)?;
    let parsed_url = Url::parse(parsed).map_err(Error::UrlParse)?;
    Ok(parsed_url.into_string())
}

fn favicon_parsed_with_base(parsed_opt: Option<&str>, page_url: &str) -> Result<String> {
    let parsed = parsed_opt.ok_or(Error::NoLink)?;
    let base = Url::parse(page_url).map_err(Error::UrlParse)?;
    let joined = base.join(parsed).map_err(Error::UrlParse)?;
    Ok(joined.into_string())
}

fn favicon_fallback(page_url: &str) -> Result<String> {
    let base = Url::parse(page_url).map_err(Error::UrlParse)?;
    let joined = base.join("/favicon.ico").map_err(Error::UrlParse)?;
    Ok(joined.into_string())
}

pub fn favicon_url(parsed_opt: Option<&str>, page_url: &str) -> Result<String> {
    favicon_parsed(parsed_opt)
        .or_else(|_e| favicon_parsed_with_base(parsed_opt, page_url))
        .or_else(|_e| favicon_fallback(page_url))
}

pub fn favicon_filename(favicon_url: &str) -> Result<String> {
    let url = Url::parse(favicon_url).map_err(Error::UrlParse)?;
    let path = url
        .path_segments()
        .ok_or_else(|| Error::NoPath(favicon_url.to_string()))?;
    let last = path
        .last()
        .ok_or_else(|| Error::NoPath(favicon_url.to_string()))?;
    if !last.is_empty() {
        Ok(last.to_owned())
    } else {
        Err(Error::NoPath(favicon_url.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_favicon_parsed_positive() {
        let base = favicon_parsed(Some("http://example.org/favicon.ico"));
        assert!(base.is_ok());
        assert_eq!(base.unwrap(), "http://example.org/favicon.ico");
    }

    #[test]
    fn test_favicon_parsed_negative() {
        let base = favicon_parsed(Some("favicon.ico"));
        assert!(base.is_err());
    }

    #[test]
    fn test_favicon_parsed_with_base() {
        let base = favicon_parsed_with_base(
            Some("favicon.ico"),
            "http://example.com/page/index.html?a=b",
        );
        assert!(base.is_ok());
        assert_eq!(base.unwrap(), "http://example.com/page/favicon.ico");
    }

    #[test]
    fn test_favicon_parsed_with_base_relative() {
        let base = favicon_parsed_with_base(
            Some("../favicon.ico"),
            "http://example.com/page/index.html?a=b",
        );
        assert!(base.is_ok());
        assert_eq!(base.unwrap(), "http://example.com/favicon.ico");
    }

    #[test]
    fn test_favicon_fallback() {
        let base = favicon_fallback("http://example.com/page/index.html?a=b");
        assert!(base.is_ok());
        assert_eq!(base.unwrap(), "http://example.com/favicon.ico");
    }

    #[test]
    fn test_favicon_url_relative() {
        let base = favicon_url(
            Some("favicon.ico"),
            "http://example.com/page/index.html?a=b",
        );
        assert!(base.is_ok());
        assert_eq!(base.unwrap(), "http://example.com/page/favicon.ico");
    }

    #[test]
    fn test_favicon_url_absolute() {
        let base = favicon_url(
            Some("/favicon.ico"),
            "http://example.org/post/with/some/long/path?and=query&also=presents",
        );
        assert!(base.is_ok());
        assert_eq!(base.unwrap(), "http://example.org/favicon.ico");
    }

    #[test]
    fn test_favicon_url_fallback() {
        let base = favicon_url(
            None,
            "http://example.org/post/with/some/long/path?and=query&also=presents",
        );
        assert!(base.is_ok());
        assert_eq!(base.unwrap(), "http://example.org/favicon.ico");
    }

    #[test]
    fn test_favicon_filename_positive() {
        let filename =
            favicon_filename("http://example.org/post/with/some/long/path?and=query&also=presents");
        assert!(filename.is_ok());
        assert_eq!(filename.unwrap(), "path");
    }

    #[test]
    fn test_favicon_filename_negative() {
        let filename = favicon_filename("http://example.org?fav=icon");
        assert!(filename.is_err());
    }

    #[test]
    fn test_favicon_filename_negative_dir() {
        let filename = favicon_filename("http://example.org/fav/icon/");
        assert!(filename.is_err());
    }
}

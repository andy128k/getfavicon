#[derive(Debug)]
pub enum Error {
    UnexpectedStatus(reqwest::StatusCode),
    UnsupportedScheme(String),
    UnsupportedDataURLEncoding,
    NoLink,
    NoPath(String),
    BadImage,
    BadImageData(base64::DecodeError),
    BadImageFormat(std::string::FromUtf8Error),
    Io(String, std::io::Error),
    UrlParse(url::ParseError),
    Request(reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnexpectedStatus(c) => write!(f, "Unexpected response status: {}.", c),
            Error::UnsupportedScheme(s) => write!(f, "Unsupported scheme: {}.", s),
            Error::UnsupportedDataURLEncoding => write!(f, "Unsupported data URL encoding."),
            Error::NoLink => write!(f, "No link to favicon."),
            Error::NoPath(url) => write!(f, "URL '{}' has no path.", url),
            Error::BadImage => write!(f, "Bad image."),
            Error::BadImageData(e) => write!(f, "Bad image data {}.", e),
            Error::BadImageFormat(e) => write!(f, "Bad image format {}.", e),
            Error::Io(ctx, e) => write!(f, "I/O error {}. Context: {}.", e, ctx),
            Error::UrlParse(e) => write!(f, "URL parse error {}.", e),
            Error::Request(e) => write!(f, "Request error {}.", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::UnexpectedStatus(..) => None,
            Error::UnsupportedScheme(..) => None,
            Error::UnsupportedDataURLEncoding => None,
            Error::NoLink => None,
            Error::NoPath(..) => None,
            Error::BadImage => None,
            Error::BadImageData(e) => Some(e),
            Error::BadImageFormat(e) => Some(e),
            Error::Io(_, e) => Some(e),
            Error::UrlParse(e) => Some(e),
            Error::Request(e) => Some(e),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

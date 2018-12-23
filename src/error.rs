#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Unexpected response status: {}.", _0)]
    UnexpectedStatus(::reqwest::StatusCode),

    #[fail(display = "Unsupported scheme: {}.", _0)]
    UnsupportedScheme(String),

    #[fail(display = "Unsupported data URL encoding.")]
    UnsupportedDataURLEncoding,

    #[fail(display = "No link to favicon.")]
    NoLink,

    #[fail(display = "URL '{}' has no path.", _0)]
    NoPath(String),

    #[fail(display = "Bad image.")]
    BadImage,

    #[fail(display = "Bad image data {}.", _0)]
    BadImageData(#[cause] base64::DecodeError),

    #[fail(display = "Bad image format {}.", _0)]
    BadImageFormat(#[cause] std::string::FromUtf8Error),

    #[fail(display = "I/O error {}.", _0)]
    Io(#[cause] std::io::Error),

    #[fail(display = "URL parse error {}.", _0)]
    UrlParse(#[cause] url::ParseError),

    #[fail(display = "Request error {}.", _0)]
    Request(#[cause] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

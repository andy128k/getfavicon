pub type Error = ::failure::Error;
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
#[fail(display = "Unexpected response status: {}.", _0)]
pub struct UnexpectedStatus(pub ::reqwest::StatusCode);

#[derive(Fail, Debug)]
#[fail(display = "Unsupported scheme: {}.", _0)]
pub struct UnsupportedScheme(pub String);

#[derive(Fail, Debug)]
#[fail(display = "Unsupported data URL encoding.")]
pub struct UnsupportedDataURLEncoding;

#[derive(Fail, Debug)]
#[fail(display = "No link to favicon.")]
pub struct NoLink;

#[derive(Fail, Debug)]
#[fail(display = "URL '{}' has no path.", _0)]
pub struct NoPath(pub String);

error_chain!{
    errors {
        UnexpectedStatus(status: ::reqwest::StatusCode) {
            description("Unexpected response status.")
            display("Unexpected response status: {}.", status)
        }
        UnsupportedScheme(scheme: String) {
            description("Unsupported scheme.")
            display("Unsupported scheme: {}.", scheme)
        }
        UnsupportedDataURLEncoding {
            description("Unsupported data URL encoding.")
        }
        NoLink {
            description("No link to favicon.")
        }
        NoPath(url: String) {
            description("URL has no path.")
            display("URL '{}' has no path.", url)
        }
    }
    foreign_links {
        Io(::std::io::Error);
        Mime(::reqwest::mime::FromStrError);
        Net(::reqwest::Error);
        Utf(::std::string::FromUtf8Error);
        Url(::url::ParseError);
        Base64(::base64::DecodeError);
    }
}

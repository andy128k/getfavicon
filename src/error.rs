error_chain!{
    errors {
        UnexpectedStatus(status: ::reqwest::StatusCode) {
            description("Unexpected response status.")
            display("Unexpected response status: {}.", status)
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
        Net(::reqwest::Error);
        Utf(::std::string::FromUtf8Error);
        Url(::url::ParseError);
    }
}

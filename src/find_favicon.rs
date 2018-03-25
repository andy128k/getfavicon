use regex::{Regex, Captures};

lazy_static! {
    static ref SPACES_RE: Regex = Regex::new(r"\s+").unwrap();

    static ref LINK_RE: Regex = Regex::new(
            r##"(?xi)
                <link\s*(?P<attributes> [^>]* )>
            "##
        ).unwrap();

    static ref ICON_REL_RE: Regex = Regex::new(r##"(?xi)
            \b rel \s* = \s* ['"] ([^'"]*) ['"]
        "##).unwrap();

    static ref HREF_RE: Regex = Regex::new(r##"(?xi)
            \b href \s* = \s* ['"] ([^'"]+) ['"]
        "##).unwrap();
}

fn get_attributes(link_match: Captures) -> Option<&str> {
    link_match.name("attributes").map(|m| m.as_str())
}

fn is_icon(attributes: &str) -> bool {
    if let Some(rel) = ICON_REL_RE.captures(attributes) {
        let keywords = &rel[1];
        SPACES_RE.split(keywords).any(|kw| "icon".eq_ignore_ascii_case(kw))
    } else {
        false
    }
}

fn get_href<'t>(attributes: &str) -> Option<&str> {
    HREF_RE.captures(attributes)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
}

pub fn find_favicon(html: &str) -> Option<String> {
    LINK_RE.captures_iter(html)
        .filter_map(get_attributes)
        .filter(|attributes| is_icon(attributes))
        .filter_map(|attributes| get_href(attributes))
        .nth(0)
        .map(|href| href.to_string())
}

#[cfg(test)]
mod tests {
    use super::find_favicon;

    #[test]
    fn test1() {
        let favicon = find_favicon("<html></html>");
        assert_eq!(favicon, None);
    }

    #[test]
    fn test2() {
        let favicon = find_favicon(r#"
<html>
    <link rel="stylesheet" type="text/css" href="../main.css">
</html>
"#);
        assert_eq!(favicon, None);
    }

    #[test]
    fn test3() {
        let favicon = find_favicon(r#"
<html>
    <link rel="stylesheet" type="text/css" href="../main.css">
    <link rel="shortcut icon" href="https://doc.rust-lang.org/favicon.ico">
</html>
"#);
        assert_eq!(favicon, Some("https://doc.rust-lang.org/favicon.ico".to_string()));
    }

    #[test]
    fn test4() {
        let favicon = find_favicon(r#"
<html>
    <link rel="stylesheet" type="text/css" href="../main.css">
    <link rel="shortcut icon" nohref="https://doc.rust-lang.org/notfavicon.ico">
    <link rel="shortcut icon" href="https://doc.rust-lang.org/favicon.ico">
</html>
"#);
        assert_eq!(favicon, Some("https://doc.rust-lang.org/favicon.ico".to_string()));
    }

    #[test]
    fn test5() {
        let favicon = find_favicon(r#"
<html>
    <link rel="stylesheet" type="text/css" href="../main.css">
    <link rel="apple-touch-icon" href="https://l-stat.livejournal.net/img/apple-touch-icon.png">
    <link rel="shortcut icon" href="https://doc.rust-lang.org/favicon.ico">
</html>
"#);
        assert_eq!(favicon, Some("https://doc.rust-lang.org/favicon.ico".to_string()));
    }
}

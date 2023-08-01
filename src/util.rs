pub fn parse_for_code(url: &mut String) -> Option<String> {
    url.split("?code=")
        .nth(1)
        .and_then(|s| s.split("&").next())
        .map(|s| s.to_owned())
}

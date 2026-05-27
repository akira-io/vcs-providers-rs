pub fn next_cursor(next_url: Option<&str>) -> Option<String> {
    next_url.map(str::to_owned)
}

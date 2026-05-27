use vcs_provider_core::Response;

pub fn next_cursor(response: &Response) -> Option<String> {
    header(response, "x-next-cursor")
        .filter(|cursor| !cursor.is_empty())
        .map(str::to_owned)
        .or_else(|| next_page_header(response))
        .or_else(|| header(response, "link").and_then(next_link_cursor))
}

fn next_page_header(response: &Response) -> Option<String> {
    header(response, "x-next-page")
        .filter(|page| !page.is_empty())
        .map(str::to_owned)
}

fn next_link_cursor(link_header: &str) -> Option<String> {
    link_header.split(',').find_map(next_cursor_from_link)
}

fn next_cursor_from_link(link: &str) -> Option<String> {
    let mut parts = link.split(';');
    let url = parts
        .next()?
        .trim()
        .trim_start_matches('<')
        .trim_end_matches('>');
    let has_next_relation = parts.any(|part| part.trim() == r#"rel="next""#);

    if !has_next_relation {
        return None;
    }

    query_value(url, "cursor").or_else(|| query_value(url, "page"))
}

fn query_value(url: &str, expected_name: &str) -> Option<String> {
    let query = url.split_once('?')?.1.split('#').next()?;

    query.split('&').find_map(|parameter| {
        let (name, value) = parameter.split_once('=')?;

        if name == expected_name {
            return Some(value.to_owned());
        }

        None
    })
}

fn header<'a>(response: &'a Response, expected_name: &str) -> Option<&'a str> {
    response
        .headers()
        .iter()
        .find(|header| header.name().as_str().eq_ignore_ascii_case(expected_name))
        .map(|header| header.value().as_str())
}

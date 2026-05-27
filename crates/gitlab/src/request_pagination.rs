use vcs_provider_core::{PageRequest, RequestUrlBuilder};

pub fn apply_page(request_url: RequestUrlBuilder, page: Option<&PageRequest>) -> RequestUrlBuilder {
    let Some(page) = page else {
        return request_url;
    };

    let request_url = request_url.optional_query_param(
        "per_page",
        page.limit().map(|limit| limit.as_u16().to_string()),
    );
    let Some(cursor) = page.cursor() else {
        return request_url;
    };

    request_url.query_param(cursor_parameter(cursor.as_str()), cursor.as_str())
}

fn cursor_parameter(cursor: &str) -> &str {
    if cursor.chars().all(|character| character.is_ascii_digit()) {
        return "page";
    }

    "cursor"
}

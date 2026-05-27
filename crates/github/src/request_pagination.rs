use vcs_provider_core::{PageRequest, RequestUrlBuilder};

pub fn apply_page(request_url: RequestUrlBuilder, page: Option<&PageRequest>) -> RequestUrlBuilder {
    let Some(page) = page else {
        return request_url;
    };

    request_url
        .optional_query_param(
            "per_page",
            page.limit().map(|limit| limit.as_u16().to_string()),
        )
        .optional_query_param(
            "page",
            page.cursor().map(|cursor| cursor.as_str().to_owned()),
        )
}

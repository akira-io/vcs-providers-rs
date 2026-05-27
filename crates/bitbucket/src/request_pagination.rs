use vcs_provider_core::{PageRequest, RequestUrlBuilder, url};

pub fn apply_page(request_url: RequestUrlBuilder, page: Option<&PageRequest>) -> RequestUrlBuilder {
    let Some(page) = page else {
        return request_url;
    };

    let Some(cursor) = page.cursor() else {
        return request_url.optional_query_param(
            "pagelen",
            page.limit().map(|limit| limit.as_u16().to_string()),
        );
    };

    if cursor.as_str().starts_with("http://") {
        return url(cursor.as_str());
    }

    if cursor.as_str().starts_with("https://") {
        return url(cursor.as_str());
    }

    request_url
        .optional_query_param(
            "pagelen",
            page.limit().map(|limit| limit.as_u16().to_string()),
        )
        .query_param("page", cursor.as_str())
}

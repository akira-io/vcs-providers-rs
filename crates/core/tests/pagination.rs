use vcs_provider_core::{Branch, branch, pagination, repo};

#[test]
fn pagination_request_carries_limit_and_cursor() {
    let page_request = pagination()
        .request()
        .limit(100)
        .cursor("opaque-provider-cursor")
        .build();

    assert_eq!(page_request.limit().map(|limit| limit.as_u16()), Some(100));
    assert_eq!(
        page_request.cursor().map(|cursor| cursor.as_str()),
        Some("opaque-provider-cursor")
    );
}

#[test]
fn pagination_page_carries_items_and_next_cursor() {
    let page = pagination()
        .page([branch("main"), branch("develop")])
        .next("next-page")
        .build();

    assert_eq!(page.items().len(), 2);
    assert!(page.has_next());
    assert_eq!(page.next().map(|cursor| cursor.as_str()), Some("next-page"));

    let page_with_direct_branch = pagination().page([Branch::make("release")]).build();

    assert_eq!(page_with_direct_branch.items().len(), 1);
}

#[test]
fn repository_queries_accept_pagination_request() {
    let page_request = pagination().request().limit(50).build();
    let list_query = repo().query().pagination(page_request.clone()).list();
    let search_query = repo()
        .query()
        .search("vcs")
        .pagination(page_request)
        .search();

    assert_eq!(
        list_query
            .page()
            .and_then(|page| page.limit())
            .map(|limit| limit.as_u16()),
        Some(50)
    );
    assert_eq!(
        search_query
            .page()
            .and_then(|page| page.limit())
            .map(|limit| limit.as_u16()),
        Some(50)
    );
}

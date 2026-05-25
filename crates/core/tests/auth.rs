use vcs_provider_core::{AuthHeaderStyle, AuthKind, auth};

#[test]
fn anonymous_auth_has_no_header() {
    let credential = auth().anonymous();

    assert_eq!(credential.kind(), AuthKind::Anonymous);
    assert_eq!(
        credential.header(AuthHeaderStyle::AuthorizationBearer),
        None
    );
}

#[test]
fn personal_access_token_uses_authorization_bearer_header() {
    let credential = auth().personal_access_token("test-token");
    let header = credential.header(AuthHeaderStyle::AuthorizationBearer);

    assert_eq!(credential.kind(), AuthKind::PersonalAccessToken);
    assert_eq!(
        header.map(|header| (
            header.name().as_str().to_owned(),
            header.value().as_str().to_owned()
        )),
        Some(("authorization".into(), "Bearer test-token".into()))
    );
}

#[test]
fn token_auth_uses_authorization_token_header() {
    let credential = auth().personal_access_token("test-token");
    let header = credential.header(AuthHeaderStyle::AuthorizationToken);

    assert_eq!(
        header.map(|header| (
            header.name().as_str().to_owned(),
            header.value().as_str().to_owned()
        )),
        Some(("authorization".into(), "token test-token".into()))
    );
}

#[test]
fn oauth_auth_uses_custom_header_style() {
    let credential = auth().oauth("test-token");
    let header = credential.header(AuthHeaderStyle::CustomHeader("private-token".into()));

    assert_eq!(credential.kind(), AuthKind::OAuth);
    assert_eq!(
        header.map(|header| (
            header.name().as_str().to_owned(),
            header.value().as_str().to_owned()
        )),
        Some(("private-token".into(), "test-token".into()))
    );
}

#[test]
fn app_installation_and_jwt_report_their_auth_kind() {
    let app_installation = auth().app_installation("test-token");
    let jwt = auth().jwt("test-token");

    assert_eq!(app_installation.kind(), AuthKind::AppInstallation);
    assert_eq!(jwt.kind(), AuthKind::Jwt);
}

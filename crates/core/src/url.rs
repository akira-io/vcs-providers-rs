use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RequestUrl(String);

impl RequestUrl {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn scheme(&self) -> Option<&str> {
        self.value().split_once("://").map(|(scheme, _url)| scheme)
    }

    pub fn domain(&self) -> Option<&str> {
        self.url_without_scheme()
            .split(['/', '?', '#'])
            .next()
            .filter(|domain| !domain.is_empty())
    }

    pub fn path(&self) -> &str {
        let url = self.url_without_scheme();
        let Some(path_start) = url.find('/') else {
            return "";
        };

        let path = &url[path_start..];
        path.split(['?', '#']).next().unwrap_or("")
    }

    pub fn as_str(&self) -> &str {
        self.value()
    }

    fn url_without_scheme(&self) -> &str {
        self.value()
            .split_once("://")
            .map(|(_scheme, url)| url)
            .unwrap_or_else(|| self.value())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestUrlBuilder {
    base_url: String,
    query_params: Vec<(String, String)>,
}

impl RequestUrlBuilder {
    pub fn make(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            query_params: Vec::new(),
        }
    }

    pub fn query_param(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((name.into(), value.into()));
        self
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.base_url = join_path(self.base_url, path.into());
        self
    }

    pub fn path_segments(
        mut self,
        path_segments: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let path = path_segments
            .into_iter()
            .map(|path_segment| encode_url_part(&path_segment.into()))
            .collect::<Vec<_>>()
            .join("/");

        self.base_url = join_path(self.base_url, path);
        self
    }

    pub fn optional_query_param(
        mut self,
        name: impl Into<String>,
        value: Option<impl Into<String>>,
    ) -> Self {
        if let Some(value) = value {
            self.query_params.push((name.into(), value.into()));
        }

        self
    }

    pub fn query_params(
        mut self,
        query_params: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        self.query_params.extend(
            query_params
                .into_iter()
                .map(|(name, value)| (name.into(), value.into())),
        );
        self
    }

    pub fn build(self) -> RequestUrl {
        RequestUrl::make(build_url(self.base_url, self.query_params))
    }
}

fn join_path(base_url: String, path: String) -> String {
    if path.is_empty() {
        return base_url;
    }

    if path.starts_with("http://") {
        return path;
    }

    if path.starts_with("https://") {
        return path;
    }

    format!(
        "{}/{}",
        base_url.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

fn build_url(base_url: String, query_params: Vec<(String, String)>) -> String {
    if query_params.is_empty() {
        return base_url;
    }

    let (url_without_fragment, fragment) = split_fragment(base_url);
    let query = query_params
        .into_iter()
        .map(|(name, value)| format!("{}={}", encode_url_part(&name), encode_url_part(&value)))
        .collect::<Vec<_>>()
        .join("&");

    format!(
        "{}{}{}{}",
        url_without_fragment,
        query_separator(&url_without_fragment),
        query,
        fragment
    )
}

fn split_fragment(url: String) -> (String, String) {
    if let Some(fragment_start) = url.find('#') {
        return (
            url[..fragment_start].to_owned(),
            url[fragment_start..].to_owned(),
        );
    }

    (url, String::new())
}

fn query_separator(url: &str) -> &str {
    if url.ends_with('?') {
        return "";
    }

    if url.ends_with('&') {
        return "";
    }

    if url.contains('?') {
        return "&";
    }

    "?"
}

fn encode_url_part(value: &str) -> String {
    value.bytes().fold(String::new(), |mut encoded, byte| {
        if is_unreserved_url_byte(byte) {
            encoded.push(char::from(byte));
            return encoded;
        }

        encoded.push_str(&format!("%{byte:02X}"));
        encoded
    })
}

fn is_unreserved_url_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~')
}

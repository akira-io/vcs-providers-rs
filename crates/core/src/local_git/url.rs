#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitUrl {
    pub(super) url: String,
}

impl LocalGitUrl {
    pub fn is_github(&self) -> bool {
        self.url.contains("github.com")
    }

    pub fn repo_name(&self) -> Option<String> {
        let name = self.url.trim_end_matches('/').split('/').next_back()?;
        let name = name.trim_end_matches(".git");

        if name.is_empty() {
            return None;
        }

        Some(name.to_owned())
    }
}

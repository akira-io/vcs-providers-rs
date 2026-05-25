use crate::{MissingReleaseRepo, ProvidedReleaseRepo, ReleaseDraft, Repo};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseDraftBuilder<RepoState, TagState> {
    pub(crate) repo: RepoState,
    pub(crate) tag: TagState,
    pub(crate) name: Option<String>,
    pub(crate) body: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingReleaseTag;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedReleaseTag {
    pub(crate) tag: String,
}

impl<TagState> ReleaseDraftBuilder<MissingReleaseRepo, TagState> {
    pub fn repo(self, repo: impl Into<Repo>) -> ReleaseDraftBuilder<ProvidedReleaseRepo, TagState> {
        ReleaseDraftBuilder {
            repo: ProvidedReleaseRepo { repo: repo.into() },
            tag: self.tag,
            name: self.name,
            body: self.body,
        }
    }
}

impl<RepoState> ReleaseDraftBuilder<RepoState, MissingReleaseTag> {
    pub fn tag(self, tag: impl Into<String>) -> ReleaseDraftBuilder<RepoState, ProvidedReleaseTag> {
        ReleaseDraftBuilder {
            repo: self.repo,
            tag: ProvidedReleaseTag { tag: tag.into() },
            name: self.name,
            body: self.body,
        }
    }
}

impl<RepoState, TagState> ReleaseDraftBuilder<RepoState, TagState> {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
}

impl ReleaseDraftBuilder<ProvidedReleaseRepo, ProvidedReleaseTag> {
    pub fn build(self) -> ReleaseDraft {
        self.get()
    }

    pub fn get(self) -> ReleaseDraft {
        ReleaseDraft {
            repo: self.repo.repo,
            tag: self.tag.tag,
            name: self.name,
            body: self.body,
        }
    }
}

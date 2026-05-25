use crate::{Release, ReleasePatch};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleasePatchBuilder {
    release: Release,
    name: Option<String>,
    body: Option<String>,
}

impl ReleasePatchBuilder {
    pub fn make(release: Release) -> Self {
        Self {
            release,
            name: None,
            body: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn build(self) -> ReleasePatch {
        self.get()
    }

    pub fn get(self) -> ReleasePatch {
        ReleasePatch {
            release: self.release,
            name: self.name,
            body: self.body,
        }
    }
}

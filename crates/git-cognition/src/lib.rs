//! Umbrella crate for git-cognition. Re-exports [`git_cognition_core`] unconditionally and exposes
//! provider drivers behind feature flags (`github`, `gitlab`, `bitbucket`).

pub use git_cognition_core::*;

/// GitHub provider driver. Available when the `github` feature is enabled.
#[cfg(feature = "github")]
pub mod github {
    pub use git_cognition_github::*;
}

/// GitLab provider driver. Available when the `gitlab` feature is enabled.
#[cfg(feature = "gitlab")]
pub mod gitlab {
    pub use git_cognition_gitlab::*;
}

/// Bitbucket Cloud provider driver. Available when the `bitbucket` feature is enabled.
#[cfg(feature = "bitbucket")]
pub mod bitbucket {
    pub use git_cognition_bitbucket::*;
}

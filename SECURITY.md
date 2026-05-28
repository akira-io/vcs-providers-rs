# Security Policy

## Supported versions

Security fixes are released against the latest minor version on crates.io. Older minor versions
are not patched once a new minor ships.

| Version    | Supported          |
| ---------- | ------------------ |
| `0.1.x`    | Yes (current)      |
| Older      | No                 |

## Reporting a vulnerability

Please do not file a public GitHub issue for security vulnerabilities.

Report privately through GitHub's
[Security Advisory form](https://github.com/akira-io/git-cognition-rs/security/advisories/new)
on this repository. Include:

- Affected version(s) and feature set.
- A minimal reproduction (code, command, or HTTP exchange).
- The impact you observed and any suspected blast radius.
- Suggested mitigation if you have one.

You will receive an acknowledgement within 72 hours. A coordinated disclosure timeline will be
agreed before any public advisory is published.

## Out of scope

- Vulnerabilities in third-party transports (`reqwest`, TLS backends) reachable only through
  custom application code.
- Misuse of `git` CLI on systems where the binary itself is compromised.
- Bugs in disabled feature paths that cannot be reached without enabling the feature.

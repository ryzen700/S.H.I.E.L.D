# Security Policy

## Purpose of the Policy

The purpose of this Security Policy is to ensure the security of S.H.I.E.L.D Launcher and maintain the trust of users and contributors.

## Who is Affected by the Policy

This policy applies to all members of our project community, including developers, testers, repository administrators, and users.

## Supported Versions

The following branches and versions of S.H.I.E.L.D Launcher are currently maintained with security updates:

| Version / Branch  | Supported          |
| ----------------- | ------------------ |
| `dev`             | :white_check_mark: |
| `feat/rust-port`  | :white_check_mark: |
| 4.x releases      | :white_check_mark: |
| < 4.0.0           | :x:                |

## Development Recommendations

### Best Practices

- Follow secure coding principles.
- Use well-established, maintained libraries and frameworks.
- Regularly audit and update dependencies.
- Conduct thorough testing, including security and input validation checks.

### Unrecommended Practices

- Do not use known vulnerabilities that have not been patched.
- Do not commit sensitive information such as API keys, tokens, or credentials.
- Do not merge pull requests that degrade the security posture of the project.

### User-Generated Content and Integrations

- Ensure that any external content or metadata (e.g., game scraping, custom manifests) does not execute untrusted code.
- Handle user data and local save files securely.

### Community Interaction

- Treat contributors with respect and politeness.
- Do not spam or distribute malicious automated tools.
- Follow responsible disclosure standards.

## Vulnerability Discovery and Reporting

If you discover a security vulnerability in S.H.I.E.L.D Launcher:

- **Do NOT disclose it publicly in an open GitHub issue.**
- Report the vulnerability privately via [GitHub Security Advisories](https://github.com/ryzen700/S.H.I.E.L.D/security/advisories/new) or directly contact the maintainer: **Damir Zaripov** (`eulafrsindychrthadcro@gmail.com`).

### Details to Provide

When reporting a vulnerability, please provide the following details:

- A clear description of the vulnerability and its potential impact.
- Exact steps to reproduce the issue (including Proof-of-Concept code or scripts, if safe).
- Affected version(s), platform (Windows/Linux/macOS), and environment details.
- Suggested remediation or patch, if available.

### Expected Behavior

- We will acknowledge receipt of your vulnerability report within 48 hours.
- We will coordinate a fix, release a security advisory, and publish a patch.
- Credit will be given to the reporter in the release notes upon request.

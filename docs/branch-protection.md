# Branch Protection

Protect both `dev` and `main` after the first pull request workflow run exposes the check names.

## Recommended settings

For each branch, enable:

- Require a pull request before merging.
- Require status checks to pass before merging.
- Require branches to be up to date before merging.
- Require conversation resolution before merging.
- Block force pushes.
- Block deletions.

## Required checks

Require these checks from the `LastBeacon Build` workflow:

- `Main source branch policy`
- `Validate LastBeacon on Windows`
- `Package windows-x64 test on Windows`
- `Package windows-x64 shipping on Windows`

The `Main source branch policy` check allows pull requests into `main` only from `dev` or `hotfix/*` branches. It runs before validation and packaging so invalid `main` pull requests fail quickly.

## Public repository runner safety

This repository uses a self-hosted Windows runner. In GitHub Actions settings, keep approval requirements enabled for outside contributors before running workflows from forks or first-time contributors.

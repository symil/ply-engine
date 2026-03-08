# Contributing to Ply

Thanks for your interest in contributing to Ply!

## Getting started

1. Fork the repo and clone it
2. Pick an issue to work on

## Making changes

- Follow the existing code style. No extra abstractions, no over-engineering.
- Follow the issue. If you think it should be changed, comment on the issue first.
- Test your changes on at least one platform. You can test in your own project or in a new project made with `plyx init`, just reference the local path to your Ply fork in `Cargo.toml`.

## Commit messages

All PRs are squash-merged, so your commit messages during development don't matter. The final squash commit will use these prefixes:

- `feat:` new feature
- `fix:` bug fix
- `optimize:` performance improvement
- `refactor:` restructure without changing behavior
- `rename:` rename something
- `feat&fix:` feature but also a fix
- `docs:` documentation
- `test:` testing of existing code
- `tools:` tooling, build, CI, Github, etc

## Pull requests

- Reference the issue number in the PR
- Keep the PR description to just your solution. The issue already has the details.

## What not to do

- Don't open PRs for issues that don't exist. Open an issue first.
- Don't add dependencies without discussing it in an issue.
- Don't reformat code you didn't change.
- Don't add comments to code that's already clear. `///` documentation is welcome.

## License

Ply is 0BSD licensed. By contributing, you agree that your contributions are released under the same license.

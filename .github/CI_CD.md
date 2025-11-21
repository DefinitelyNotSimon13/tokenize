# CI/CD Pipeline Documentation

This repository uses GitHub Actions for continuous integration and deployment.

## Workflows

### CI Workflow (`.github/workflows/ci.yml`)

Automatically runs on:
- Push to `main` or `master` branches
- Pull requests targeting `main` or `master`

**Jobs:**
1. **Test** - Runs tests on Ubuntu, Windows, and macOS with Rust stable
2. **Lint** - Checks code formatting (`cargo fmt`) and lints code (`cargo clippy`)
3. **Build** - Creates release builds for Linux, Windows, and macOS

### Release Workflow (`.github/workflows/release.yml`)

Automatically runs on:
- Push of version tags (e.g., `v0.1.0`, `v1.2.3`)
- Manual workflow dispatch

**Jobs:**
1. **Create Release** - Creates a GitHub release
2. **Build Release** - Builds binaries for multiple platforms:
   - Linux (x86_64, aarch64)
   - Windows (x86_64)
   - macOS (x86_64, aarch64)
3. **Publish Crate** - Publishes to crates.io (requires `CARGO_TOKEN` secret)

#### Creating a Release

To create a new release:

1. Update version in `Cargo.toml`
2. Commit the changes
3. Create and push a tag:
   ```bash
   git tag v0.1.2
   git push origin v0.1.2
   ```
4. The release workflow will automatically build binaries and create a GitHub release

### Security Audit Workflow (`.github/workflows/security.yml`)

Automatically runs on:
- Daily schedule (00:00 UTC)
- Push/PR that modifies `Cargo.toml` or `Cargo.lock`
- Manual workflow dispatch

**Jobs:**
1. **Security Audit** - Scans dependencies for security vulnerabilities using `cargo-audit`
2. **Dependency Check** - Checks for duplicate dependencies

## Required Secrets

For the release workflow to publish to crates.io, you need to add the following secret:

- `CARGO_TOKEN` - Your crates.io API token
  - Get your token from https://crates.io/settings/tokens
  - Add it to repository secrets at: Settings → Secrets and variables → Actions → New repository secret

If you don't want to publish to crates.io, the workflow will continue with `continue-on-error: true` for that step.

## Status Badges

The README now includes status badges for:
- CI build status
- Security audit status
- License information

## Caching

The CI workflow uses GitHub Actions cache to speed up builds by caching:
- Cargo registry
- Cargo git dependencies
- Build artifacts (`target/` directory)

This significantly reduces build times for subsequent runs.

## Local Testing

Before pushing, you can run the same checks locally:

```bash
# Build the project
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Build release
cargo build --release
```

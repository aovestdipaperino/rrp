# Publishing to crates.io

This guide explains how to publish the `reverse-ssh` crate to crates.io.

## Prerequisites

1. **crates.io Account**
   - Create an account at https://crates.io
   - Verify your email address

2. **API Token**
   ```bash
   # Login to crates.io
   cargo login

   # Or manually set token
   cargo login YOUR_API_TOKEN
   ```

3. **Repository Setup**
   - Ensure all changes are committed
   - Push to GitHub: https://github.com/aovestdipaperino/rrp
   - Create a release tag

## Pre-Publishing Checklist

Before publishing, ensure:

- [ ] All tests pass: `cargo test`
- [ ] Code is formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Documentation builds: `cargo doc --no-deps`
- [ ] Examples work: `cargo run --example localhost_run`
- [ ] README.md is up to date
- [ ] CHANGELOG.md is updated
- [ ] Version number is correct in `Cargo.toml`
- [ ] LICENSE files exist (LICENSE-MIT and LICENSE-APACHE)
- [ ] Repository URL is correct
- [ ] Author email is correct

## Version Numbers

Follow [Semantic Versioning](https://semver.org/):

- `0.1.0` - Initial release
- `0.1.1` - Bug fix (backward compatible)
- `0.2.0` - New features (backward compatible)
- `1.0.0` - First stable release
- `2.0.0` - Breaking changes

Update version in `Cargo.toml`:
```toml
[package]
version = "0.1.0"  # Update this
```

## Dry Run

Test the publish process without actually publishing:

```bash
# Check what would be published
cargo publish --dry-run

# Check package contents
cargo package --list
```

This will:
- Build the package
- Check for errors
- Show what files will be included
- Verify all dependencies resolve

### Review Package Contents

```bash
# Create the package
cargo package

# Extract and inspect
cd target/package
tar -xzf reverse-ssh-0.1.0.crate
ls -la reverse-ssh-0.1.0/
```

Verify these files are included:
- `src/` - Source code
- `examples/` - Example files
- `Cargo.toml` - Package metadata
- `README.md` - Main documentation
- `LICENSE-MIT` - MIT license
- `LICENSE-APACHE` - Apache 2.0 license
- `CHANGELOG.md` - Version history

Files that should be excluded:
- `.git/` - Git directory
- `target/` - Build artifacts
- `.gitignore` - Git configuration
- IDE files

## Publishing Steps

### 1. Update Version

Edit `Cargo.toml`:
```toml
[package]
version = "0.1.0"  # Increment this for new releases
```

### 2. Update CHANGELOG.md

```markdown
## [0.1.0] - 2024-10-29

### Added
- Initial release
- Reverse SSH tunneling functionality
- localhost.run integration
- Automatic URL capture
- Multiple examples

### Fixed
- localhost.run bind address issue
```

### 3. Commit Changes

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "Release v0.1.0"
git push origin main
```

### 4. Create Git Tag

```bash
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0
```

### 5. Publish to crates.io

```bash
# Final check
cargo publish --dry-run

# Publish for real
cargo publish
```

### 6. Create GitHub Release

1. Go to https://github.com/aovestdipaperino/rrp/releases
2. Click "Draft a new release"
3. Choose tag `v0.1.0`
4. Title: "v0.1.0 - Initial Release"
5. Description: Copy from CHANGELOG.md
6. Publish release

## Post-Publishing

### Verify Publication

1. Check on crates.io: https://crates.io/crates/reverse-ssh
2. Wait for docs to build: https://docs.rs/reverse-ssh
3. Test installation:
   ```bash
   cargo install reverse-ssh --example localhost_run
   ```

### Update README Badges

Add badges to README.md:

```markdown
[![Crates.io](https://img.shields.io/crates/v/reverse-ssh.svg)](https://crates.io/crates/reverse-ssh)
[![Documentation](https://docs.rs/reverse-ssh/badge.svg)](https://docs.rs/reverse-ssh)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/aovestdipaperino/rrp#license)
```

### Announce

Consider announcing on:
- Reddit: r/rust
- Twitter/X with #rustlang
- Rust Discord/forums
- Your blog/website

## Troubleshooting

### "crate already exists"

The version already exists on crates.io. You cannot republish the same version. Increment the version number in `Cargo.toml`.

### "failed to verify package tarball"

Run `cargo package` and check for errors. Common issues:
- Missing files
- Incorrect paths in `include`/`exclude`
- Build failures

### "API token not found"

Run `cargo login` again with a fresh token from crates.io.

### Documentation build fails

Test locally:
```bash
cargo doc --no-deps --open
```

Fix any documentation warnings or errors.

### Files not included in package

Update `Cargo.toml`:
```toml
[package]
include = [
    "src/**/*",
    "examples/**/*",
    "README.md",
    "LICENSE-MIT",
    "LICENSE-APACHE",
    "CHANGELOG.md",
]
```

Or use `exclude`:
```toml
[package]
exclude = [
    ".git/**",
    ".github/**",
    "target/**",
]
```

## Yanking a Version

If you need to yank a problematic version:

```bash
# Yank version 0.1.0
cargo yank --version 0.1.0

# Un-yank if needed
cargo yank --version 0.1.0 --undo
```

**Note**: Yanking prevents new projects from using the version, but doesn't break existing users.

## Publishing Updates

For subsequent releases:

1. Make changes
2. Update version in `Cargo.toml`
3. Update `CHANGELOG.md`
4. Commit and push
5. Create tag
6. Publish: `cargo publish`
7. Create GitHub release

## CI/CD (Optional)

### GitHub Actions for Publishing

Create `.github/workflows/publish.yml`:

```yaml
name: Publish

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo publish --token ${{ secrets.CARGO_TOKEN }}
```

Add `CARGO_TOKEN` to GitHub secrets.

## Version History

| Version | Date | Notes |
|---------|------|-------|
| 0.1.0 | 2024-10-29 | Initial release |

## References

- [The Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)

## Questions?

Contact: enzinol@gmail.com

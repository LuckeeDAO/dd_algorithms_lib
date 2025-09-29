# Publishing to crates.io

This guide explains how to publish the `dd_algorithms_lib` to crates.io.

## Prerequisites

1. **Crates.io Account**: Create an account at [crates.io](https://crates.io)
2. **API Token**: Get your API token from your account settings
3. **Cargo Login**: Run `cargo login <your-api-token>`

## Pre-Publication Checklist

### 1. Update Version
```bash
# Update version in Cargo.toml
version = "0.1.0"  # or next version
```

### 2. Update Documentation
- [ ] README.md is complete and accurate
- [ ] All public APIs are documented
- [ ] Examples are working and comprehensive
- [ ] Changelog is updated

### 3. Run Tests
```bash
cargo test
cargo test --examples
```

### 4. Check Package
```bash
cargo check
cargo clippy
cargo fmt
```

### 5. Build for Release
```bash
cargo build --release
```

## Publishing Steps

### 1. Dry Run
```bash
cargo publish --dry-run
```

### 2. Publish
```bash
cargo publish
```

### 3. Verify
- Check the package on [crates.io](https://crates.io/crates/dd_algorithms_lib)
- Test installation: `cargo add dd_algorithms_lib`

## Post-Publication

### 1. Create GitHub Release
- Tag the version: `git tag v0.1.0`
- Push tags: `git push origin v0.1.0`
- Create release on GitHub

### 2. Update Documentation
- Update docs.rs documentation
- Update any external documentation

### 3. Announce
- Share on social media
- Update project README
- Notify users

## Version Management

### Semantic Versioning
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Examples
- `0.1.0` → `0.1.1`: Bug fix
- `0.1.1` → `0.2.0`: New feature
- `0.2.0` → `1.0.0`: Breaking change

## Maintenance

### Regular Updates
- Monitor for issues
- Update dependencies
- Fix bugs
- Add features

### Deprecation Policy
- Mark deprecated APIs with `#[deprecated]`
- Provide migration guides
- Remove deprecated APIs in major versions

## Troubleshooting

### Common Issues

1. **Name Already Taken**
   - Choose a different name
   - Check availability first

2. **Compilation Errors**
   - Fix all warnings
   - Ensure no_std compatibility

3. **Documentation Issues**
   - Check all public APIs are documented
   - Fix broken links

4. **Test Failures**
   - Fix all failing tests
   - Ensure examples compile

### Getting Help
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Crates.io Help](https://crates.io/help)
- [Rust Community](https://users.rust-lang.org/)

## Success Metrics

After publishing, monitor:
- Download counts
- User feedback
- Issue reports
- Feature requests
- Community adoption

## Future Plans

- [ ] Add more governance mechanisms
- [ ] Implement advanced voting systems
- [ ] Add more fair division algorithms
- [ ] Improve performance
- [ ] Add more examples
- [ ] Create tutorials

# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2024-10-29

### Fixed
- **localhost.run compatibility**: Changed bind address from `"0.0.0.0"` to `""` (empty string) in `tcpip_forward()` call. This fixes the "missing _lhr TXT record on 0.0.0.0" error when connecting to localhost.run.
- **Message handling**: Improved multi-line message parsing to correctly display all server messages.
- **Error visibility**: Added ⚠️ emoji indicator for error and warning messages.

### Added
- **Extended data capture**: Implemented `extended_data()` handler to capture stderr messages where localhost.run sends URLs.
- **Shell session**: Automatically opens shell session after port forwarding to receive welcome messages.
- **Emoji indicators**: Server messages now display with emoji indicators:
  - 🔗 for URLs
  - ⚠️ for errors/warnings
  - 📨 for regular messages
- **Command-line options**: Added `--key` and `--port` flags to localhost_run example.
- **Environment variables**: Support for `SSH_KEY` and `LOCAL_PORT` environment variables.
- **Tilde expansion**: Automatically expands `~` in file paths.

### Improved
- **URL extraction**: More aggressive URL detection from server messages.
- **Debug logging**: Better logging messages for troubleshooting.
- **Documentation**: Added URL_CAPTURE.md, SUMMARY.md, and comprehensive examples.

### Initial Features
- Core reverse SSH tunneling functionality
- localhost.run integration with automatic URL capture
- Automatic SSH key detection and generation
- Multiple authentication methods (key-based and password)
- Example implementations:
  - `basic.rs` - Minimal example
  - `localhost_run.rs` - Full-featured localhost.run integration with CLI options
  - `local_test.rs` - Complete testing environment
  - `simple_server.rs` - Test HTTP server with beautiful UI
- Comprehensive documentation:
  - README.md with quick start
  - EXAMPLE_OUTPUT.md showing expected behavior
  - FEATURES.md detailing all capabilities
  - URL_CAPTURE.md explaining the capture mechanism
  - CONTRIBUTING.md for contributors
  - PUBLISHING.md for maintainers

---

## Understanding the Bind Address Fix

### The Problem

When requesting remote port forwarding, the bind address specifies which interface the SSH server should listen on:
- `"0.0.0.0"` means "listen on all interfaces with this specific address"
- `""` (empty string) means "let the server choose the appropriate address"

localhost.run expects an empty string and uses it to determine DNS configuration. When `"0.0.0.0"` is provided, it tries to look up DNS records for that literal address, resulting in the error:
```
missing _lhr TXT record on 0.0.0.0
```

### The Solution

Changed the `tcpip_forward` call in `src/lib.rs`:

```rust
// Before
handle.tcpip_forward("0.0.0.0", self.config.remote_port).await?;

// After
handle.tcpip_forward("", self.config.remote_port).await?;
```

### Impact

This change:
- ✅ Fixes localhost.run compatibility
- ✅ Follows SSH protocol best practices (RFC 4254)
- ✅ Works with standard SSH servers
- ✅ Allows server to choose optimal bind address

### Related Documentation

- [URL_CAPTURE.md](URL_CAPTURE.md) - Detailed explanation of the fix
- [README.md](README.md) - Updated usage instructions
- [EXAMPLE_OUTPUT.md](EXAMPLE_OUTPUT.md) - Expected output with fix

---

## SSH Protocol Reference

From RFC 4254 (SSH Connection Protocol), section 7.1:

> The 'address to bind' and 'port number to bind' specify the IP address
> and port on which connections for forwarding are to be accepted.
>
> ...
>
> A port number of 0 requests that the server allocate the next available port.
>
> A client may request that the server listen on all interfaces by providing
> an empty string as the 'address to bind'.

Key point: **Empty string = server chooses**, which is what localhost.run needs.

---

## Troubleshooting

If you still see the "_lhr TXT record" error:

1. **Update your code**: Make sure you have the latest version
2. **Check the call**: Verify `tcpip_forward` uses `""`
3. **Clean build**: Run `cargo clean && cargo build`
4. **Test connection**: Try the traditional SSH command first:
   ```bash
   ssh -R 80:localhost:8080 localhost.run
   ```

If the traditional SSH command works but the library doesn't, please file an issue with:
- Full error output
- Debug logs (`RUST_LOG=debug`)
- Your configuration
- Operating system

---

## Migration Guide

If you're using the library in your own code and seeing the error:

### Before (0.1.0)
```rust
handle.tcpip_forward("0.0.0.0", port).await?;
```

### After (0.1.1+)
```rust
handle.tcpip_forward("", port).await?;
```

No other changes needed!

---

## Version History

- **v0.1.1** - localhost.run compatibility fix (current)
- **v0.1.0** - Initial release

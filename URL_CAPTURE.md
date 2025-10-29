# URL Capture Mechanism

This document explains how the library captures server-generated URLs from services like localhost.run.

## The Challenge

Services like localhost.run generate a random public URL when you connect. This URL needs to be captured and displayed to the user so they know how to access their service. The challenge is that different SSH servers send this information in different ways.

## How localhost.run Works

When you connect to localhost.run via SSH with remote port forwarding:

1. You authenticate via SSH
2. You request remote port forwarding (`tcpip-forward`) with **empty bind address**
3. localhost.run generates a random subdomain (e.g., `https://abc123.localhost.run`)
4. The server sends this URL back through the SSH connection

### Important: Bind Address

localhost.run requires the bind address to be an **empty string** (`""`), not `"0.0.0.0"`.

```rust
// ✅ Correct - empty string lets server choose address
handle.tcpip_forward("", self.config.remote_port).await?;

// ❌ Wrong - causes "missing _lhr TXT record" error
handle.tcpip_forward("0.0.0.0", self.config.remote_port).await?;
```

The URL can be sent via:
- **Standard output (stdout)** - Regular data channel
- **Standard error (stderr)** - Extended data channel (type 1)
- **Shell welcome message** - When opening a shell session

## Our Implementation

### 1. Extended Data Capture

We implement the `extended_data()` handler to capture stderr messages:

```rust
async fn extended_data(
    &mut self,
    _channel: ChannelId,
    ext: u32,
    data: &[u8],
    _session: &mut client::Session,
) -> Result<(), Self::Error> {
    // Extended data includes stderr (ext == 1)
    // localhost.run sends URL info through stderr
    if let Ok(message) = String::from_utf8(data.to_vec()) {
        info!("Received extended data (type {}): {}", ext, message);
        let _ = self.message_tx.send(message);
    }
    Ok(())
}
```

### 2. Regular Data Capture

We also capture regular stdout data:

```rust
async fn data(
    &mut self,
    _channel: ChannelId,
    data: &[u8],
    _session: &mut client::Session,
) -> Result<(), Self::Error> {
    if let Ok(message) = String::from_utf8(data.to_vec()) {
        debug!("Received data: {}", message);
        let _ = self.message_tx.send(message);
    }
    Ok(())
}
```

### 3. Shell Session

After setting up port forwarding, we open a shell session to trigger welcome messages:

```rust
// Open a shell session to receive server messages
match handle.channel_open_session().await {
    Ok(channel) => {
        info!("Opened shell session to receive server messages");
        // Request a shell - this triggers the server to send welcome messages
        if let Err(e) = channel.request_shell(false).await {
            warn!("Failed to request shell: {}", e);
        }
        // Keep the channel open to receive messages
    }
    Err(e) => {
        warn!("Could not open shell session: {}", e);
    }
}
```

### 4. Message Processing

All captured messages are sent through a channel to the application, where they're processed:

```rust
client.run_with_message_handler(move |message| {
    // Print all server messages
    if !message.trim().is_empty() {
        println!("📨 [Server] {}", message.trim());
    }

    // Look for URLs
    if message_lower.contains("http://") || message_lower.contains("https://") {
        // Extract and display the URL
        // ...
    }
}).await?;
```

## URL Extraction Algorithm

1. **Receive all messages** from both stdout and stderr
2. **Search for "http://" or "https://"** in each message
3. **Extract the full URL** by finding boundaries (whitespace, punctuation)
4. **Validate** that it's a localhost.run URL
5. **Display prominently** once detected
6. **Log all messages** so users can see them even if auto-detection fails

## Debugging

If the URL isn't automatically detected, users can:

1. **Check the console output** - All server messages are logged with 📨 or 🔗 emoji
2. **Wait 10 seconds** - A fallback message appears with instructions
3. **Use traditional SSH** - Compare: `ssh -R 80:localhost:8080 localhost.run`

Example debug output:
```
📨 [Server] Welcome to localhost.run!
🔗 [Server] https://abc123.localhost.run tunnels to localhost:8080
📨 [Server] Press Ctrl-C to stop the tunnel
```

## Why Multiple Capture Methods?

Different SSH servers behave differently:

- **localhost.run** - Sends URL via shell welcome message
- **Other services** - May use stdout, stderr, or custom channels
- **Standard SSH servers** - May not send any special messages

By implementing multiple capture methods, we ensure maximum compatibility.

## Testing the Capture

You can verify the capture mechanism works by:

1. Running with debug logging:
   ```bash
   RUST_LOG=debug cargo run --example localhost_run
   ```

2. Checking for these log messages:
   - `"Opened shell session to receive server messages"`
   - `"Shell requested successfully"`
   - `"Received data: ..."` or `"Received extended data: ..."`

3. Looking for server messages in the output:
   - Lines starting with `📨 [Server]` or `🔗 [Server]`

## Comparison with Traditional SSH

Traditional SSH command:
```bash
ssh -R 80:localhost:8080 localhost.run
```

This works because:
1. OpenSSH client automatically displays stdin/stdout/stderr
2. The terminal shows everything the server sends
3. You see the URL immediately

Our implementation replicates this by:
1. Capturing stdin/stdout/stderr programmatically
2. Processing messages to extract URLs
3. Displaying them in a formatted way

## Future Improvements

Potential enhancements:

1. **Pattern matching** - Support more URL patterns
2. **QR code generation** - Display QR code for mobile access
3. **Clipboard integration** - Auto-copy URL to clipboard
4. **Desktop notifications** - Show system notification with URL
5. **URL validation** - Verify URL is accessible before displaying

## Related Files

- `src/lib.rs` - Core capture implementation (`extended_data()`, `data()` handlers)
- `examples/localhost_run.rs` - URL extraction and display logic
- `EXAMPLE_OUTPUT.md` - Shows expected output with URL capture

## Technical Details

### SSH Channel Types

1. **Session channels** - For shell, exec, subsystem
2. **Forwarded channels** - For port forwarding (our tunneled connections)
3. **Direct channels** - For direct TCP/IP connections

### Data Types

- **Type 0** - Normal data (stdout)
- **Type 1** - Extended data (stderr)
- **Type 2+** - Custom application-specific data

### Message Flow

```
localhost.run server
       ↓
   SSH Protocol
       ↓
  russh library
       ↓
Handler::extended_data() / Handler::data()
       ↓
  Message Channel
       ↓
 run_with_message_handler()
       ↓
   URL Extraction
       ↓
  Console Display
```

## Security Considerations

- **No credentials in URLs** - localhost.run URLs are public but temporary
- **SSL/TLS** - URLs use HTTPS by default
- **Message filtering** - We only extract and display URLs, not sensitive data
- **Logging** - All messages are logged but not persisted to disk

## Troubleshooting Guide

### "missing _lhr TXT record" Error

**Problem**: Error message `missing _lhr TXT record on 0.0.0.0`

**Cause**: Using `"0.0.0.0"` as bind address instead of empty string `""`

**Solution**: This is now fixed in the library. If you see this error with the latest version:
1. Make sure you're using the latest code
2. Check that `tcpip_forward` is called with `""` not `"0.0.0.0"`
3. Restart your connection

### URL Not Captured

**Problem**: Tunnel connects but URL doesn't appear

**Solutions**:
1. Check server messages (📨 lines in output)
2. Look for ⚠️ warning messages indicating errors
3. Wait 10 seconds for fallback message
4. Enable debug logging: `RUST_LOG=debug`
5. Try traditional SSH to compare: `ssh -R 80:localhost:8080 localhost.run`

### Messages Not Received

**Problem**: No server messages appear at all

**Solutions**:
1. Ensure tunnel is actually connected (check for "Reverse tunnel established")
2. Verify shell session opened (look for "Opened shell session" log)
3. Check if server supports shell sessions
4. Try with different SSH server for comparison

### Wrong URL Format

**Problem**: URL detected but formatted incorrectly

**Solutions**:
1. Check the raw server message (📨 output)
2. Verify URL extraction regex in `localhost_run.rs`
3. Report issue with example server message

## Performance Impact

The URL capture mechanism has minimal performance impact:

- **CPU**: Negligible (simple string operations)
- **Memory**: ~1KB per message
- **Latency**: No impact on tunnel performance
- **Network**: No additional requests

Messages are processed asynchronously and don't block tunnel operations.

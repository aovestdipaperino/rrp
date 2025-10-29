# Features Summary

This document outlines the key features of the reverse-ssh library and examples.

## Core Library Features

### 1. Reverse SSH Tunneling
- Establishes reverse port forwarding (remote port forwarding)
- Connects to any SSH server supporting remote port forwarding
- Handles multiple concurrent forwarded connections
- Bidirectional data proxying

### 2. Authentication Support
- **Key-based authentication**: Uses SSH private keys (RSA, Ed25519, etc.)
- **Password authentication**: Supports password-based auth
- Automatic key loading via russh-keys

### 3. Connection Management
- Configurable timeout (default: 1 hour)
- Automatic reconnection handling
- Clean connection shutdown

### 4. Server Message Handling
- Captures all messages from SSH server
- Custom message handler support via `run_with_message_handler()`
- Useful for services that send connection info (like localhost.run)

## localhost_run Example Features

### Command-line Configuration

**Flexible Configuration Options:**
- Command-line arguments: `--key`, `--port`
- Environment variables: `SSH_KEY`, `LOCAL_PORT`
- Default values: `~/.ssh/id_rsa`, port `8080`
- Built-in help: `--help` or `-h`

**Examples:**
```bash
# Use custom SSH key
cargo run --example localhost_run -- --key ~/.ssh/my_custom_key

# Forward different port
cargo run --example localhost_run -- --port 3000

# Combine options
cargo run --example localhost_run -- --key ~/.ssh/my_key --port 3000

# Use environment variables
SSH_KEY=~/.ssh/my_key LOCAL_PORT=3000 cargo run --example localhost_run
```

**Tilde Expansion:**
- Automatically expands `~` to home directory
- Works in both command-line args and env vars
- Example: `~/.ssh/my_key` → `/home/user/.ssh/my_key`

### Automatic SSH Key Management
- **Detection**: Checks for `~/.ssh/id_rsa` automatically
- **Interactive Generation**: Prompts user if key is missing
- **Secure Creation**: Uses system `ssh-keygen` for standard compatibility
- **Proper Permissions**: Sets 0600 on Unix systems

Example prompt:
```
⚠ SSH key not found: /home/user/.ssh/id_rsa

Would you like to generate a new SSH keypair?
This will create /home/user/.ssh/id_rsa and /home/user/.ssh/id_rsa.pub [Y/n]:
```

### Automatic URL Extraction and Display

**What it does:**
- Monitors all server messages in real-time
- Automatically detects and extracts URLs from localhost.run
- Displays the public URL in a prominent, formatted box
- Shows connection time and local service info

**Supported URL formats:**
- `https://*.localhost.run`
- `http://*.localhost.run`
- `https://*.lhr.rocks`
- `https://*.lhr.life`

**Display format:**
```
╔══════════════════════════════════════════════════════╗
║              🌐 TUNNEL ACTIVE 🌐                     ║
╠══════════════════════════════════════════════════════╣
║  Your local service is now accessible at:            ║
║                                                      ║
║  https://abc123def456.localhost.run                  ║
║                                                      ║
║  Local: http://127.0.0.1:8080                        ║
║  Connected in: 2s                                    ║
╚══════════════════════════════════════════════════════╝

✨ Ready to accept connections!
```

**Fallback mechanism:**
If no URL is detected after 5 seconds, shows helpful instructions:
```
╔══════════════════════════════════════════════════════╗
║                   TUNNEL CONNECTED                   ║
╠══════════════════════════════════════════════════════╣
║  If you don't see a URL above, you can also:        ║
║                                                      ║
║  1. Check the terminal output for server messages   ║
║  2. Connect manually via SSH to see the URL:        ║
║     ssh -R 80:localhost:8080 localhost.run          ║
║                                                      ║
║  Note: localhost.run may send the URL via a         ║
║  different mechanism. Watch for connection logs.    ║
╚══════════════════════════════════════════════════════╝
```

### User Experience Enhancements
- Beautiful box-drawing UI with emojis
- Clear status messages at each step
- Color-coded information (via emojis: ✓, ⚠, 🔑, 📡, 🚀, 🌐, ✨)
- Connection timing information
- Real-time server message display

## simple_server Example Features

### Built-in Test HTTP Server
- Standalone server for testing tunnels
- Runs on configurable port (default: 8080)
- Styled HTML response page
- Request logging with counter

### Beautiful Web Interface
- Gradient purple background
- Responsive design
- Shows connection details:
  - Request number
  - HTTP method and path
  - Timestamp
  - Server info
- Instructions for next steps

### Logging
- Connection notifications
- Request details (method, path)
- Response status codes
- Numbered requests for tracking

## local_test Example Features

### Complete Testing Environment
- Integrated HTTP server
- Environment variable configuration
- Flexible SSH server support
- Clear connection instructions

### Configuration via Environment Variables
```bash
SSH_HOST=your-server.com
SSH_USER=your-username
SSH_KEY=~/.ssh/id_rsa
REMOTE_PORT=9999
LOCAL_PORT=8080
```

### Error Handling
- Validates all required configuration
- Helpful error messages
- Graceful failure with instructions

## Technical Implementation

### Asynchronous Architecture
- Built on Tokio async runtime
- Non-blocking I/O
- Concurrent connection handling
- Channel-based message passing

### Protocol Support
- Full SSH protocol via russh
- Remote port forwarding (tcpip-forward)
- Multiple authentication methods
- Session management

### Data Flow
```
Internet → SSH Server → Reverse Tunnel → Library Handler → Local Service
                                               ↓
                                      Message Capture
                                               ↓
                                       URL Extraction
                                               ↓
                                      Console Display
```

### Message Handling System
1. **Client Handler**: Implements russh's `Handler` trait
2. **Message Channel**: MPSC channel for server messages
3. **Data Method**: Captures all incoming data
4. **Custom Handler**: User-provided closure for processing

### URL Extraction Algorithm
1. Monitor all incoming server messages
2. Search for "http" prefix in messages
3. Extract URL until whitespace/delimiter
4. Validate against known patterns
5. Display once (atomic flag prevents duplicates)
6. Include timing information

## Security Features

### Key Management
- Uses system `ssh-keygen` for key generation
- Standard format compatibility
- Proper file permissions (0600)
- Optional passphrase support (currently disabled for ease of use)

### Server Verification
- Public key checking (currently accepts all - configurable)
- Authentication result validation
- Connection timeout protection

### Best Practices
- Avoids storing passwords in code
- Uses environment variables for configuration
- Clear security warnings in documentation
- Separates test/production concerns

## Performance Characteristics

### Resource Usage
- Minimal CPU usage (event-driven)
- Low memory footprint
- Efficient buffer management (8KB buffers)

### Scalability
- Handles multiple concurrent connections
- Async/await prevents blocking
- Channel-based architecture scales well

### Latency
- Direct TCP connection to local service
- Minimal overhead from proxy layer
- Streaming data transfer (no buffering beyond transport)

## Future Enhancement Possibilities

### Potential Features
- [ ] Multiple port forwarding in single connection
- [ ] Bandwidth monitoring and statistics
- [ ] Connection retry logic with exponential backoff
- [ ] Configuration file support (TOML/YAML)
- [ ] TLS/SSL support for local connections
- [ ] Access logging and metrics
- [ ] Web UI for monitoring connections
- [ ] Docker container support
- [ ] Systemd service file generation

### Advanced Features
- [ ] Load balancing across multiple tunnels
- [ ] Rate limiting
- [ ] IP whitelisting/blacklisting
- [ ] Request/response modification
- [ ] Custom authentication plugins
- [ ] SNI-based routing
- [ ] WebSocket support

## Comparison with Alternatives

### vs. Traditional SSH Command
**Traditional:**
```bash
ssh -R 80:localhost:8080 localhost.run
```

**This Library:**
```rust
let mut client = ReverseSshClient::new(config);
client.run().await?;
```

**Advantages:**
- Programmatic control
- Custom message handling
- Automatic URL extraction
- Integration into larger applications
- Better error handling
- Configurable behavior

### vs. ngrok
- Open source vs. proprietary
- No registration required (when using localhost.run)
- Self-hostable
- Rust performance and safety
- Library for embedding in applications

### vs. localtunnel
- More robust protocol (SSH)
- Better authentication
- No separate server needed (can use any SSH server)
- Lower latency

## Use Cases

### Development
- Share local web app with team
- Test webhooks from external services
- Demo work to clients
- Mobile app API testing

### IoT/Embedded
- Access devices behind NAT
- Remote debugging
- Firmware updates
- Sensor data collection

### DevOps
- CI/CD agent connectivity
- Internal service exposure
- Testing production integrations
- Temporary access during incidents

### Education
- Teaching networking concepts
- SSH protocol understanding
- Async Rust programming examples
- Security best practices

## Documentation

### Available Resources
- `README.md` - Main documentation
- `EXAMPLE_OUTPUT.md` - Expected output for all examples
- `FEATURES.md` - This file
- Inline code documentation
- Example comments

### Code Examples
- `basic.rs` - Minimal example
- `localhost_run.rs` - Full-featured localhost.run integration
- `local_test.rs` - Complete testing environment
- `simple_server.rs` - Test HTTP server

## License

MIT or Apache-2.0 (dual licensed for maximum compatibility)

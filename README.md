<div align="center">
  <img src="LOGO.png" alt="reverse-ssh logo" width="200"/>

  # reverse-ssh

  [![Crates.io](https://img.shields.io/crates/v/reverse-ssh.svg)](https://crates.io/crates/reverse-ssh)
  [![Documentation](https://docs.rs/reverse-ssh/badge.svg)](https://docs.rs/reverse-ssh)
  [![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/aovestdipaperino/rrp#license)
  [![Rust](https://img.shields.io/badge/rust-1.87%2B-blue.svg?maxAge=3600)](https://github.com/aovestdipaperino/rrp)
</div>

A Rust library for creating reverse SSH tunnels using the `russh` crate.

## What is Reverse SSH?

Reverse SSH (also called remote port forwarding) allows you to:
- Make a service behind a firewall/NAT accessible from outside
- Connect from a restricted network to an external SSH server
- Have the SSH server forward connections back to your local service

This is particularly useful for:
- Accessing services on machines behind NAT
- Bypassing firewall restrictions
- Remote debugging and development
- IoT device management

## How It Works

```
Internet -> SSH Server:8080 -> [SSH Tunnel] -> Your Local Service:3000
```

1. Your local client connects to an SSH server
2. Client requests the SSH server to listen on a port (e.g., 8080)
3. When someone connects to the SSH server's port 8080, the connection is forwarded through the SSH tunnel
4. Your client receives the connection and proxies it to a local service (e.g., localhost:3000)

## Quick Start

The fastest way to try reverse SSH tunneling:

```bash
# 1. Clone and build
git clone <your-repo-url>
cd reverse-ssh

# 2. Start the test server
cargo run --example simple_server

# 3. In another terminal, test with localhost.run
cargo run --example localhost_run
```

That's it! You'll get a public URL that forwards to your local server.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
reverse-ssh = "0.1.0"
```

## Usage

### Basic Example

```rust
use reverse_ssh::{ReverseSshClient, ReverseSshConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ReverseSshConfig {
        server_addr: "your-server.com".to_string(),
        server_port: 22,
        username: "your-username".to_string(),
        key_path: Some("/path/to/private/key".to_string()),
        password: None,
        remote_port: 8080,
        local_addr: "127.0.0.1".to_string(),
        local_port: 3000,
    };

    let mut client = ReverseSshClient::new(config);
    client.run().await?;

    Ok(())
}
```

### Configuration Options

- `server_addr`: SSH server hostname or IP
- `server_port`: SSH server port (usually 22)
- `username`: SSH username
- `key_path`: Path to private key (for key-based auth)
- `password`: Password (for password-based auth)
- `remote_port`: Port on SSH server to listen on
- `local_addr`: Local address to forward to (usually 127.0.0.1)
- `local_port`: Local port to forward to

### Authentication

You can use either key-based or password authentication:

```rust
// Key-based authentication
let config = ReverseSshConfig {
    // ...
    key_path: Some("/home/user/.ssh/id_rsa".to_string()),
    password: None,
    // ...
};

// Password authentication
let config = ReverseSshConfig {
    // ...
    key_path: None,
    password: Some("your-password".to_string()),
    // ...
};
```

## SSH Server Configuration

For reverse port forwarding to work, your SSH server must allow it. Add this to `/etc/ssh/sshd_config`:

```
GatewayPorts yes
AllowTcpForwarding yes
```

Then restart the SSH service:
```bash
sudo systemctl restart sshd
```

## Examples

This library includes several examples to help you get started:

### 1. Simple Test Server

Start a local HTTP server to test your tunnel:

```bash
cargo run --example simple_server
```

This runs a web server on `localhost:8080` that displays connection information. Perfect for testing!

### 2. localhost.run Integration

Expose your local service to the internet using [localhost.run](https://localhost.run):

```bash
# First, start the test server
cargo run --example simple_server

# In another terminal, create the tunnel (default: port 8080, ~/.ssh/id_rsa)
cargo run --example localhost_run

# Or specify custom SSH key and port
cargo run --example localhost_run -- --key ~/.ssh/my_custom_key --port 3000

# Or use environment variables
SSH_KEY=~/.ssh/my_key LOCAL_PORT=3000 cargo run --example localhost_run

# See all options
cargo run --example localhost_run -- --help
```

**Command-line Options:**
- `--key, -k <path>` - Path to SSH private key (default: `~/.ssh/id_rsa`)
- `--port, -p <port>` - Local port to forward (default: `8080`)
- `--help, -h` - Show help message

**Environment Variables:**
- `SSH_KEY` - Path to SSH private key
- `LOCAL_PORT` - Local port to forward

localhost.run is a free SSH tunneling service (similar to ngrok) that requires no registration.

**How it works:**
```
Internet                    SSH Tunnel                Your Machine
        ↓                        ↓                          ↓
http://xxx.localhost.run → ssh.localhost.run:22 → localhost:8080
```

**Automatic Features:**

1. **SSH Key Generation** - The example will automatically detect if you don't have an SSH key and offer to generate one:

```
⚠ SSH key not found: /home/user/.ssh/id_rsa

Would you like to generate a new SSH keypair?
This will create /home/user/.ssh/id_rsa and /home/user/.ssh/id_rsa.pub [Y/n]:
```

Just press Enter or type 'Y' to generate a keypair automatically.

2. **URL Capture & Display** - The tunnel URL is automatically captured from the server and displayed prominently:

**How it works:**
- Captures messages from both stdout and stderr
- Opens a shell session to receive welcome messages
- Automatically extracts URLs from server messages
- Displays all server messages with emoji indicators:
  - 🔗 Messages containing URLs
  - ⚠️  Error or warning messages
  - 📨 Regular informational messages

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
```

The example automatically parses server messages to extract and display your public URL!

**Alternative**: You can also use the traditional SSH command for comparison:
```bash
ssh -R 80:localhost:8080 localhost.run
```

### 3. Local Testing with Your Own Server

Test with your own SSH server:

```bash
export SSH_HOST=your-server.com
export SSH_USER=your-username
export SSH_KEY=~/.ssh/id_rsa
export REMOTE_PORT=9999
export LOCAL_PORT=8080

cargo run --example local_test
```

This example:
- Starts a built-in HTTP server on port 8080
- Connects to your SSH server
- Sets up reverse forwarding from port 9999 to localhost:8080
- Access it at: `http://your-server.com:9999`

### 4. Basic Example

Minimal example showing the library usage:

```bash
# Edit examples/basic.rs with your SSH server details
cargo run --example basic
```

## Use Cases

1. **Web Development**: Expose a local web server for testing
2. **Remote Access**: Access a service behind a firewall
3. **IoT Devices**: Allow devices behind NAT to be accessible
4. **CI/CD**: Connect build agents in restricted networks

## Security Considerations

- Always use key-based authentication in production
- Verify SSH server public keys (currently accepts any key - update for production)
- Use strong passwords if using password authentication
- Consider using a dedicated SSH server for tunneling
- Monitor and log all connections
- Implement rate limiting if needed

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

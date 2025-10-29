# Examples

This directory contains examples demonstrating how to use the reverse-ssh library.

## Available Examples

### 1. basic.rs
Minimal example showing the core functionality. Requires manual configuration in the source code.

```bash
# Edit the example to configure your SSH server details
cargo run --example basic
```

### 2. localhost_run.rs
Full-featured example for using localhost.run tunneling service. **Recommended for getting started!**

**Features:**
- ✅ Automatic SSH key detection and generation
- ✅ Automatic URL extraction and display
- ✅ Command-line configuration
- ✅ Environment variable support
- ✅ Beautiful console UI

**Usage:**

```bash
# Default: port 8080, ~/.ssh/id_rsa
cargo run --example localhost_run

# Custom SSH key
cargo run --example localhost_run -- --key ~/.ssh/my_custom_key

# Custom port
cargo run --example localhost_run -- --port 3000

# Both custom key and port
cargo run --example localhost_run -- -k ~/.ssh/my_key -p 3000

# Using environment variables
SSH_KEY=~/.ssh/my_key LOCAL_PORT=3000 cargo run --example localhost_run

# Show all options
cargo run --example localhost_run -- --help
```

**What it does:**
1. Checks if SSH key exists (generates if missing)
2. Connects to localhost.run
3. Sets up reverse tunnel
4. Captures and displays the public URL
5. Forwards all connections to your local service

**Expected output:**
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

### 3. local_test.rs
Complete testing environment with built-in HTTP server. Requires your own SSH server.

**Features:**
- Built-in test HTTP server
- Environment variable configuration
- Works with any SSH server

**Usage:**

```bash
# Configure via environment variables
export SSH_HOST=your-server.com
export SSH_USER=your-username
export SSH_KEY=~/.ssh/id_rsa
export REMOTE_PORT=9999
export LOCAL_PORT=8080

# Run the example
cargo run --example local_test

# Access your service at http://your-server.com:9999
```

**What it does:**
1. Starts local HTTP server on specified port
2. Connects to your SSH server
3. Sets up reverse forwarding
4. Displays connection info

### 4. simple_server.rs
Standalone HTTP test server for testing tunnels.

**Features:**
- Beautiful styled web interface
- Request logging and tracking
- Connection details display
- No dependencies needed

**Usage:**

```bash
# Default port 8080
cargo run --example simple_server

# Custom port via environment variable
PORT=3000 cargo run --example simple_server
```

**Use with any tunnel example:**
```bash
# Terminal 1: Start server
cargo run --example simple_server

# Terminal 2: Create tunnel
cargo run --example localhost_run
```

## Quick Start Guide

### Option 1: Using localhost.run (Recommended)

Perfect for quick testing without setting up your own SSH server:

```bash
# Terminal 1: Start test server
cargo run --example simple_server

# Terminal 2: Create tunnel
cargo run --example localhost_run

# You'll see output like:
# https://abc123.localhost.run -> localhost:8080
```

Visit the displayed URL in your browser to see your local service!

### Option 2: Using Your Own SSH Server

If you have an SSH server with remote port forwarding enabled:

```bash
# Configure your server
export SSH_HOST=myserver.com
export SSH_USER=myuser
export SSH_KEY=~/.ssh/id_rsa
export REMOTE_PORT=9999
export LOCAL_PORT=8080

# Run the example
cargo run --example local_test

# Access at http://myserver.com:9999
```

## Common Options Reference

### localhost_run Options

| Flag | Short | Environment Variable | Default | Description |
|------|-------|---------------------|---------|-------------|
| `--key` | `-k` | `SSH_KEY` | `~/.ssh/id_rsa` | SSH private key path |
| `--port` | `-p` | `LOCAL_PORT` | `8080` | Local port to forward |
| `--help` | `-h` | - | - | Show help message |

### local_test Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `SSH_HOST` | Yes | - | SSH server hostname |
| `SSH_USER` | Yes | - | SSH username |
| `SSH_KEY` | No | - | SSH private key path |
| `SSH_PASS` | No | - | SSH password (alternative to key) |
| `REMOTE_PORT` | No | `9999` | Port on SSH server to listen on |
| `LOCAL_PORT` | No | `8080` | Local port to forward to |

### simple_server Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `8080` | Port to listen on |

## Troubleshooting

### "Connection refused to localhost:8080"
**Solution:** Start the test server first:
```bash
cargo run --example simple_server
```

### "SSH key not found"
**Solution:** The localhost_run example will offer to generate one automatically. Just press Enter when prompted.

Or generate manually:
```bash
ssh-keygen -t rsa -f ~/.ssh/id_rsa -N ""
```

### "Authentication failed"
**Solution:** Check your SSH key permissions:
```bash
chmod 600 ~/.ssh/id_rsa
```

### "Permission denied (publickey)" with localhost.run
**Solution:** Make sure your key is in standard RSA format. Regenerate if needed:
```bash
ssh-keygen -t rsa -b 2048 -f ~/.ssh/id_rsa -N ""
```

### "Remote port forwarding failed"
**Solution:** Your SSH server must allow remote port forwarding. Add to `/etc/ssh/sshd_config`:
```
GatewayPorts yes
AllowTcpForwarding yes
```
Then restart SSH: `sudo systemctl restart sshd`

## Tips

1. **Development Workflow:**
   ```bash
   # Keep the test server running in one terminal
   cargo run --example simple_server

   # Use another terminal for the tunnel
   cargo run --example localhost_run
   ```

2. **Multiple Services:**
   ```bash
   # Forward different services on different ports
   cargo run --example localhost_run -- --port 3000  # Terminal 1
   cargo run --example localhost_run -- --port 8080  # Terminal 2
   ```

3. **Custom Keys:**
   ```bash
   # Use different keys for different purposes
   cargo run --example localhost_run -- --key ~/.ssh/localhost_key
   ```

4. **Testing APIs:**
   ```bash
   # Forward your API server
   cargo run --example localhost_run -- --port 3001

   # Test with curl
   curl https://[your-url].localhost.run/api/endpoint
   ```

## Next Steps

- Check out [../README.md](../README.md) for library usage
- See [../EXAMPLE_OUTPUT.md](../EXAMPLE_OUTPUT.md) for expected output
- Read [../FEATURES.md](../FEATURES.md) for detailed feature list

## Need Help?

Run any example with `--help`:
```bash
cargo run --example localhost_run -- --help
```

Or check the source code - each example is well-documented!

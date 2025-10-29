# Example Output

This document shows what to expect when running the reverse-ssh examples.

## localhost_run Example

### Usage

```bash
# Default usage (port 8080, ~/.ssh/id_rsa)
cargo run --example localhost_run

# Custom SSH key
cargo run --example localhost_run -- --key ~/.ssh/my_custom_key

# Custom port
cargo run --example localhost_run -- --port 3000

# Both custom key and port
cargo run --example localhost_run -- --key ~/.ssh/my_key --port 3000

# Using environment variables
SSH_KEY=~/.ssh/my_key LOCAL_PORT=3000 cargo run --example localhost_run

# Show help
cargo run --example localhost_run -- --help
```

### Case 1: SSH Key Already Exists (With URL Display)

```
╔═══════════════════════════════════════════════════════╗
║     localhost.run Reverse SSH Tunnel                 ║
╚═══════════════════════════════════════════════════════╝

This will expose your local service on port 8080 to the internet.
Make sure you have a service running on localhost:8080

For testing, you can start a simple HTTP server:
  • Python: python3 -m http.server 8080
  • Node.js: npx http-server -p 8080
  • Rust: cargo run --example simple_server

✓ Found SSH key: /home/user/.ssh/id_rsa

📡 Connecting to localhost.run...
   Remote port: 80 (HTTP)
   Local service: http://127.0.0.1:8080

🚀 Starting reverse tunnel...
   Once connected, localhost.run will provide a public URL.
   Press Ctrl+C to stop the tunnel.

Expected URL format: https://[random-id].localhost.run
Connecting...

[INFO] Connecting to SSH server ssh.localhost.run:22
[INFO] Successfully authenticated to SSH server
[INFO] Setting up reverse tunnel: server port 80 -> local 127.0.0.1:8080
[INFO] Reverse tunnel established successfully
[INFO] Waiting for forwarded connections...
[Server Message] Welcome to localhost.run!
[Server Message] Your tunnel URL is: https://abc123def456.localhost.run

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

[INFO] New forwarded connection received
[INFO] Connecting to local service 127.0.0.1:8080
[INFO] Connected to local service, starting proxy
```

### Case 2: SSH Key Doesn't Exist (Auto-Generate)

```
╔═══════════════════════════════════════════════════════╗
║     localhost.run Reverse SSH Tunnel                 ║
╚═══════════════════════════════════════════════════════╝

This will expose your local service on port 8080 to the internet.
Make sure you have a service running on localhost:8080

For testing, you can start a simple HTTP server:
  • Python: python3 -m http.server 8080
  • Node.js: npx http-server -p 8080
  • Rust: cargo run --example simple_server

⚠ SSH key not found: /home/user/.ssh/id_rsa

Would you like to generate a new SSH keypair?
This will create /home/user/.ssh/id_rsa and /home/user/.ssh/id_rsa.pub [Y/n]: y

🔑 Generating SSH keypair...
Your identification has been saved in /home/user/.ssh/id_rsa
Your public key has been saved in /home/user/.ssh/id_rsa.pub
✓ Generated SSH keypair:
  Private key: /home/user/.ssh/id_rsa
  Public key: /home/user/.ssh/id_rsa.pub

📡 Connecting to localhost.run...
   Remote port: 80 (HTTP)
   Local service: http://127.0.0.1:8080

🚀 Starting reverse tunnel...
   Once connected, localhost.run will display a public URL.
   Press Ctrl+C to stop the tunnel.

[INFO] Connecting to SSH server ssh.localhost.run:22
[INFO] Successfully authenticated to SSH server
[INFO] Setting up reverse tunnel: server port 80 -> local 127.0.0.1:8080
[INFO] Reverse tunnel established successfully
[INFO] Waiting for forwarded connections...
```

### Case 3: User Declines Key Generation

```
╔═══════════════════════════════════════════════════════╗
║     localhost.run Reverse SSH Tunnel                 ║
╚═══════════════════════════════════════════════════════╝

This will expose your local service on port 8080 to the internet.
Make sure you have a service running on localhost:8080

For testing, you can start a simple HTTP server:
  • Python: python3 -m http.server 8080
  • Node.js: npx http-server -p 8080
  • Rust: cargo run --example simple_server

⚠ SSH key not found: /home/user/.ssh/id_rsa

Would you like to generate a new SSH keypair?
This will create /home/user/.ssh/id_rsa and /home/user/.ssh/id_rsa.pub [Y/n]: n

Error: SSH key is required to connect. Please generate one manually:
  ssh-keygen -t rsa -f /home/user/.ssh/id_rsa -N ""
```

## simple_server Example

```
╔═══════════════════════════════════════════════════════╗
║  Simple HTTP Server for Reverse SSH Testing          ║
╚═══════════════════════════════════════════════════════╝

Server running on: http://127.0.0.1:8080
Press Ctrl+C to stop

Waiting for connections...

[1] Connection from 127.0.0.1:52341
[1] GET /
[1] Response sent (200 OK)
[2] Connection from 127.0.0.1:52342
[2] GET /favicon.ico
[2] Response sent (200 OK)
```

When you access the server through your browser, you'll see a beautiful styled page with:
- 🚀✨ Emoji indicators
- Gradient purple background
- Request information (number, method, path, timestamp)
- Instructions on how the tunnel works
- Next steps for production use

## local_test Example

```bash
export SSH_HOST=myserver.com
export SSH_USER=myuser
export SSH_KEY=~/.ssh/id_rsa
export REMOTE_PORT=9999
export LOCAL_PORT=8080
cargo run --example local_test
```

Output:
```
=== Reverse SSH Tunnel - Local Test ===

Configuration:
  SSH Server: myserver.com
  SSH User: myuser
  Authentication: Private Key
  Remote Port: 9999
  Local Port: 8080

[HTTP Server] Listening on http://127.0.0.1:8080

Starting reverse SSH tunnel...

Once connected, access your service at:
  http://myserver.com:9999

Press Ctrl+C to stop.

[INFO] Connecting to SSH server myserver.com:22
[INFO] Successfully authenticated to SSH server
[INFO] Setting up reverse tunnel: server port 9999 -> local 127.0.0.1:8080
[INFO] Reverse tunnel established successfully
[INFO] Waiting for forwarded connections...
[INFO] New forwarded connection received
[INFO] Connecting to local service 127.0.0.1:8080
[INFO] Connected to local service, starting proxy
[HTTP Server] Connection from 127.0.0.1:52345
[HTTP Server] Request: GET /
[HTTP Server] Response sent (200 OK)
```

## Integration Flow

The typical flow when using these examples together:

```
Terminal 1: Start local server
$ cargo run --example simple_server
[Server starts on localhost:8080]

Terminal 2: Create tunnel
$ cargo run --example localhost_run
[Checks for SSH key]
[Generates key if needed]
[Connects to localhost.run]
[Sets up reverse tunnel]
[Displays public URL]

Browser: Visit public URL
https://abc123.localhost.run
[Request flows through tunnel]
[Reaches local server]
[Response flows back]
[Page displays with connection info]

Terminal 1 shows:
[1] Connection from 127.0.0.1:xxxxx
[1] GET /
[1] Response sent (200 OK)
```

## Success Indicators

When everything is working, you should see:

1. ✓ SSH key found or generated
2. 📡 Connected to SSH server
3. ✓ Authentication successful
4. ✓ Reverse tunnel established
5. 🚀 Waiting for connections
6. Connection logs showing traffic

## Common Issues

### "Connection refused to localhost:8080"
- Make sure your local service is running
- Try: `cargo run --example simple_server` in another terminal

### "Authentication failed"
- Check that your SSH key exists and has correct permissions
- Try regenerating: `ssh-keygen -t rsa -f ~/.ssh/id_rsa -N ""`

### "Permission denied (publickey)"
- The SSH server doesn't accept your key
- For localhost.run, make sure your key is in standard format

### "ssh-keygen not found"
- Install OpenSSH client tools
- On Ubuntu/Debian: `sudo apt-get install openssh-client`
- On macOS: Should be pre-installed

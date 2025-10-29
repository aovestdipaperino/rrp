use anyhow::{Context, Result};
use reverse_ssh::{ReverseSshClient, ReverseSshConfig};
use std::io::{self, Write};
use std::path::Path;
use tracing_subscriber;

/// Example: Expose a local web server to the internet using localhost.run
///
/// localhost.run is a free SSH tunneling service that allows you to expose
/// local services to the internet without any registration or configuration.
///
/// Usage:
/// 1. Start a local web server on port 8080 (e.g., `python3 -m http.server 8080`)
/// 2. Run this example: `cargo run --example localhost_run [OPTIONS]`
/// 3. Access your service via the URL provided by localhost.run
///
/// Options:
///   --key, -k <path>     Path to SSH private key (default: ~/.ssh/id_rsa)
///   --port, -p <port>    Local port to forward (default: 8080)
///   --help, -h           Show this help message
///
/// Environment Variables:
///   SSH_KEY              Path to SSH private key
///   LOCAL_PORT           Local port to forward
///
/// Examples:
///   cargo run --example localhost_run
///   cargo run --example localhost_run --key ~/.ssh/my_key
///   cargo run --example localhost_run --port 3000
///   SSH_KEY=~/.ssh/my_key cargo run --example localhost_run
///
/// Note: This example will automatically generate an SSH keypair if one doesn't exist.

struct Config {
    key_path: String,
    local_port: u16,
}

fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return path.replacen("~", &home, 1);
        }
    }
    path.to_string()
}

fn parse_args() -> Result<Config> {
    let args: Vec<String> = std::env::args().collect();

    // Check for help flag
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("localhost.run Reverse SSH Tunnel");
        println!();
        println!("Usage: {} [OPTIONS]", args[0]);
        println!();
        println!("Options:");
        println!("  --key, -k <path>     Path to SSH private key (default: ~/.ssh/id_rsa)");
        println!("  --port, -p <port>    Local port to forward (default: 8080)");
        println!("  --help, -h           Show this help message");
        println!();
        println!("Environment Variables:");
        println!("  SSH_KEY              Path to SSH private key");
        println!("  LOCAL_PORT           Local port to forward");
        println!();
        println!("Examples:");
        println!("  {} --key ~/.ssh/my_key", args[0]);
        println!("  {} --port 3000", args[0]);
        println!("  SSH_KEY=~/.ssh/my_key {}", args[0]);
        std::process::exit(0);
    }

    // Default values
    let home = std::env::var("HOME")
        .context("HOME environment variable not set")?;
    let mut key_path = format!("{}/.ssh/id_rsa", home);
    let mut local_port: u16 = 8080;

    // Check environment variables first
    if let Ok(env_key) = std::env::var("SSH_KEY") {
        key_path = expand_tilde(&env_key);
    }
    if let Ok(env_port) = std::env::var("LOCAL_PORT") {
        local_port = env_port.parse()
            .context("Invalid LOCAL_PORT environment variable")?;
    }

    // Parse command-line arguments (override env vars)
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--key" | "-k" => {
                if i + 1 >= args.len() {
                    anyhow::bail!("--key requires a path argument");
                }
                key_path = expand_tilde(&args[i + 1]);
                i += 2;
            }
            "--port" | "-p" => {
                if i + 1 >= args.len() {
                    anyhow::bail!("--port requires a port number argument");
                }
                local_port = args[i + 1].parse()
                    .context("Invalid port number")?;
                i += 2;
            }
            arg => {
                anyhow::bail!("Unknown argument: {}. Use --help for usage information.", arg);
            }
        }
    }

    Ok(Config {
        key_path,
        local_port,
    })
}

async fn ensure_ssh_key(key_path: &str) -> Result<String> {
    let path = Path::new(key_path);

    // Check if the key already exists
    if path.exists() {
        println!("✓ Found SSH key: {}", key_path);
        return Ok(key_path.to_string());
    }

    // Key doesn't exist, ask user if we should generate one
    println!("⚠ SSH key not found: {}", key_path);
    println!("\nWould you like to generate a new SSH keypair?");
    print!("This will create {} and {}.pub [Y/n]: ", key_path, key_path);
    io::stdout().flush()?;

    let mut response = String::new();
    io::stdin().read_line(&mut response)?;
    let response = response.trim().to_lowercase();

    if response == "n" || response == "no" {
        anyhow::bail!("SSH key is required to connect. Please generate one manually:\n  ssh-keygen -t rsa -f {} -N \"\"", key_path);
    }

    // Generate the keypair
    println!("\n🔑 Generating SSH keypair...");

    // Create .ssh directory if it doesn't exist
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .context("Failed to create .ssh directory")?;
    }

    // Use ssh-keygen command for compatibility and reliability
    let output = std::process::Command::new("ssh-keygen")
        .arg("-t")
        .arg("rsa")
        .arg("-b")
        .arg("2048")
        .arg("-f")
        .arg(key_path)
        .arg("-N")
        .arg("") // No passphrase
        .arg("-q") // Quiet mode
        .output()
        .context("Failed to run ssh-keygen. Is it installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("ssh-keygen failed: {}", stderr);
    }

    println!("✓ Generated SSH keypair:");
    println!("  Private key: {}", key_path);
    println!("  Public key: {}.pub", key_path);
    println!();

    Ok(key_path.to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command-line arguments
    let args_config = parse_args()?;

    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║     localhost.run Reverse SSH Tunnel                 ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!();
    println!("This will expose your local service on port {} to the internet.", args_config.local_port);
    println!("Make sure you have a service running on localhost:{}\n", args_config.local_port);
    println!("For testing, you can start a simple HTTP server:");
    println!("  • Python: python3 -m http.server {}", args_config.local_port);
    println!("  • Node.js: npx http-server -p {}", args_config.local_port);
    println!("  • Rust: cargo run --example simple_server");
    println!();

    // Check for SSH key or generate one
    let key_path = ensure_ssh_key(&args_config.key_path).await?;

    // Configure connection to localhost.run
    let config = ReverseSshConfig {
        // localhost.run SSH server
        server_addr: "ssh.localhost.run".to_string(),
        server_port: 22,

        // localhost.run typically accepts any username
        username: "localhost".to_string(),

        // Use the key we just ensured exists
        key_path: Some(key_path),
        password: None,

        // Port mapping:
        // - Remote port 80: localhost.run will assign a public URL
        // - Local port 8080: your local service
        remote_port: 80,
        local_addr: "127.0.0.1".to_string(),
        local_port: args_config.local_port,
    };

    println!("📡 Connecting to localhost.run...");
    println!("   Remote port: 80 (HTTP)");
    println!("   Local service: http://127.0.0.1:{}", args_config.local_port);
    println!();

    // Create and run the reverse SSH client
    let mut client = ReverseSshClient::new(config);

    println!("🚀 Starting reverse tunnel...");
    println!("   Once connected, localhost.run will provide a public URL.");
    println!("   Press Ctrl+C to stop the tunnel.");
    println!();

    // Display expected URL format
    println!("Expected URL format: https://[random-id].localhost.run");
    println!("Connecting...");
    println!();

    // Use the custom message handler to capture and display URLs
    let url_displayed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let url_displayed_clone = url_displayed.clone();
    let start_time = std::time::Instant::now();
    let local_port = args_config.local_port;

    // Spawn a task to show a fallback message if URL isn't detected
    let url_displayed_timeout = url_displayed.clone();
    let fallback_port = local_port;
    tokio::spawn(async move {
        // Wait a bit longer for localhost.run to send the URL
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        if !url_displayed_timeout.load(std::sync::atomic::Ordering::SeqCst) {
            println!();
            println!("╔══════════════════════════════════════════════════════╗");
            println!("║                   TUNNEL CONNECTED                   ║");
            println!("╠══════════════════════════════════════════════════════╣");
            println!("║  The URL should have been displayed above.           ║");
            println!("║                                                      ║");
            println!("║  If you don't see it, check the [Server Message]    ║");
            println!("║  logs above for the URL, or try this command:       ║");
            println!("║                                                      ║");
            println!("║  ssh -R 80:localhost:{:<4} localhost.run           ║", fallback_port);
            println!("║                                                      ║");
            println!("║  The tunnel IS active - watch for connection logs.  ║");
            println!("╚══════════════════════════════════════════════════════╝");
            println!();
        }
    });

    client.run_with_message_handler(move |message| {
        // Print any server messages (this helps debug if URL isn't automatically detected)
        // Handle both complete and partial messages
        if !message.is_empty() {
            // Split by lines and print each line separately
            for line in message.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    // Use different formatting for different message types
                    if trimmed.contains("http://") || trimmed.contains("https://") {
                        println!("🔗 [Server] {}", trimmed);
                    } else if trimmed.contains("error") || trimmed.contains("Error") || trimmed.contains("missing") || trimmed.contains("failed") {
                        println!("⚠️  [Server] {}", trimmed);
                    } else {
                        println!("📨 [Server] {}", trimmed);
                    }
                }
            }
        }

        // Try to extract URLs from the message - be aggressive about finding them
        // Look for http/https URLs in the message
        let message_lower = message.to_lowercase();
        if message_lower.contains("http://") || message_lower.contains("https://") {
            // Try to find the URL in the original (non-lowercased) message
            let start_pos = message.find("http://").or_else(|| message.find("https://"));

            if let Some(start) = start_pos {
                // Find the end of the URL (whitespace, newline, or end of string)
                let remaining = &message[start..];
                let end = remaining
                    .find(|c: char| c.is_whitespace() || c == '\n' || c == '\r' || c == ',' || c == ';' || c == ')' || c == ']')
                    .unwrap_or(remaining.len());

                let url = &remaining[..end].trim();

                // Check if it's a localhost.run or related URL
                if url.contains("localhost.run") || url.contains("lhr.rocks") || url.contains("lhr.life") {
                    if !url_displayed_clone.swap(true, std::sync::atomic::Ordering::SeqCst) {
                        let elapsed = start_time.elapsed().as_secs();
                        println!();
                        println!("╔══════════════════════════════════════════════════════╗");
                        println!("║              🌐 TUNNEL ACTIVE 🌐                     ║");
                        println!("╠══════════════════════════════════════════════════════╣");
                        println!("║  Your local service is now accessible at:            ║");
                        println!("║                                                      ║");
                        println!("║  {:<52} ║", url);
                        println!("║                                                      ║");
                        println!("║  Local: http://127.0.0.1:{:<31} ║", local_port);
                        println!("║  Connected in: {}s{:<37}║", elapsed, "");
                        println!("╚══════════════════════════════════════════════════════╝");
                        println!();
                        println!("✨ Ready to accept connections!");
                        println!();
                    }
                }
            }
        }
    }).await?;

    Ok(())
}

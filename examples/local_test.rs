use anyhow::Result;
use reverse_ssh::{ReverseSshClient, ReverseSshConfig};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tracing_subscriber;

/// Example: Local testing of reverse SSH tunnel
///
/// This example demonstrates how to test the reverse SSH tunnel locally:
/// 1. Starts a simple HTTP server on localhost:8080
/// 2. Connects to your SSH server and sets up reverse port forwarding
/// 3. The SSH server will listen on port 9999 and forward to localhost:8080
///
/// Prerequisites:
/// - You need access to an SSH server (e.g., your VPS, AWS EC2, etc.)
/// - The SSH server must allow remote port forwarding (GatewayPorts yes)
/// - You need SSH credentials (private key or password)
///
/// Configuration:
/// Set these environment variables:
/// - SSH_HOST: your SSH server hostname
/// - SSH_USER: your SSH username
/// - SSH_KEY: path to your private key (or use SSH_PASS for password)
/// - REMOTE_PORT: port on SSH server to listen on (default: 9999)
/// - LOCAL_PORT: local service port (default: 8080)

async fn start_simple_http_server(port: u16) -> Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    println!("[HTTP Server] Listening on http://127.0.0.1:{}", port);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("[HTTP Server] Connection from {}", addr);

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            // Read the request
            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let request = String::from_utf8_lossy(&buffer[..n]);
                    println!("[HTTP Server] Request: {}", request.lines().next().unwrap_or(""));

                    // Simple HTTP response
                    let response = format!(
                        "HTTP/1.1 200 OK\r\n\
                         Content-Type: text/html; charset=utf-8\r\n\
                         Connection: close\r\n\
                         \r\n\
                         <!DOCTYPE html>\
                         <html>\
                         <head><title>Reverse SSH Test</title></head>\
                         <body>\
                         <h1>🚀 Reverse SSH Tunnel Working!</h1>\
                         <p>This page is served from localhost:8080</p>\
                         <p>Accessed through reverse SSH tunnel</p>\
                         <p>Time: {}</p>\
                         </body>\
                         </html>",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
                    );

                    let _ = socket.write_all(response.as_bytes()).await;
                }
                _ => {}
            }
        });
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== Reverse SSH Tunnel - Local Test ===\n");

    // Get configuration from environment variables
    let ssh_host = std::env::var("SSH_HOST")
        .unwrap_or_else(|_| {
            eprintln!("Error: SSH_HOST environment variable not set");
            eprintln!("\nUsage:");
            eprintln!("  export SSH_HOST=your-server.com");
            eprintln!("  export SSH_USER=your-username");
            eprintln!("  export SSH_KEY=~/.ssh/id_rsa");
            eprintln!("  export REMOTE_PORT=9999  # optional, default 9999");
            eprintln!("  export LOCAL_PORT=8080   # optional, default 8080");
            eprintln!("  cargo run --example local_test");
            std::process::exit(1);
        });

    let ssh_user = std::env::var("SSH_USER")
        .unwrap_or_else(|_| {
            eprintln!("Error: SSH_USER environment variable not set");
            std::process::exit(1);
        });

    let ssh_key = std::env::var("SSH_KEY")
        .ok();
    let ssh_pass = std::env::var("SSH_PASS")
        .ok();

    if ssh_key.is_none() && ssh_pass.is_none() {
        eprintln!("Error: Either SSH_KEY or SSH_PASS must be set");
        std::process::exit(1);
    }

    let remote_port: u32 = std::env::var("REMOTE_PORT")
        .unwrap_or_else(|_| "9999".to_string())
        .parse()?;

    let local_port: u16 = std::env::var("LOCAL_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()?;

    println!("Configuration:");
    println!("  SSH Server: {}", ssh_host);
    println!("  SSH User: {}", ssh_user);
    println!("  Authentication: {}", if ssh_key.is_some() { "Private Key" } else { "Password" });
    println!("  Remote Port: {}", remote_port);
    println!("  Local Port: {}\n", local_port);

    // Start the local HTTP server in the background
    tokio::spawn(async move {
        if let Err(e) = start_simple_http_server(local_port).await {
            eprintln!("HTTP server error: {}", e);
        }
    });

    // Give the server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Configure the reverse SSH tunnel
    let config = ReverseSshConfig {
        server_addr: ssh_host.clone(),
        server_port: 22,
        username: ssh_user,
        key_path: ssh_key,
        password: ssh_pass,
        remote_port,
        local_addr: "127.0.0.1".to_string(),
        local_port,
    };

    println!("Starting reverse SSH tunnel...");
    println!("\nOnce connected, access your service at:");
    println!("  http://{}:{}\n", ssh_host, remote_port);
    println!("Press Ctrl+C to stop.\n");

    // Create and run the reverse SSH client
    let mut client = ReverseSshClient::new(config);
    client.run().await?;

    Ok(())
}

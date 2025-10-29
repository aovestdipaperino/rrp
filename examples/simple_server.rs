use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// Simple HTTP server for testing reverse SSH tunnels
///
/// This is a standalone HTTP server that you can use to test
/// your reverse SSH tunnel setup.
///
/// Usage:
///   cargo run --example simple_server
///
/// Then in another terminal, run one of the tunnel examples:
///   cargo run --example localhost_run
///   cargo run --example local_test

#[tokio::main]
async fn main() -> Result<()> {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()?;

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║  Simple HTTP Server for Reverse SSH Testing          ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!();
    println!("Server running on: http://127.0.0.1:{}", port);
    println!("Press Ctrl+C to stop");
    println!();
    println!("Waiting for connections...");
    println!();

    let mut request_count = 0;

    loop {
        let (socket, addr) = listener.accept().await?;
        request_count += 1;
        let current_count = request_count;

        println!("[{}] Connection from {}", current_count, addr);

        tokio::spawn(async move {
            let mut socket = socket;
            let mut buffer = [0; 2048];

            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let request = String::from_utf8_lossy(&buffer[..n]);
                    let first_line = request.lines().next().unwrap_or("");
                    let method = first_line.split_whitespace().next().unwrap_or("UNKNOWN");
                    let path = first_line.split_whitespace().nth(1).unwrap_or("/");

                    println!("[{}] {} {}", current_count, method, path);

                    let body = format!(
                        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Reverse SSH Test Server</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
        }}
        .container {{
            background: rgba(255, 255, 255, 0.1);
            border-radius: 10px;
            padding: 30px;
            backdrop-filter: blur(10px);
        }}
        h1 {{
            margin-top: 0;
            font-size: 2.5em;
        }}
        .info {{
            background: rgba(0, 0, 0, 0.2);
            padding: 15px;
            border-radius: 5px;
            margin: 20px 0;
            font-family: monospace;
        }}
        .success {{
            background: rgba(76, 175, 80, 0.3);
            padding: 10px;
            border-left: 4px solid #4CAF50;
            margin: 20px 0;
        }}
        .emoji {{
            font-size: 3em;
            text-align: center;
            margin: 20px 0;
        }}
        code {{
            background: rgba(0, 0, 0, 0.3);
            padding: 2px 6px;
            border-radius: 3px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="emoji">🚀✨</div>
        <h1>Reverse SSH Tunnel Working!</h1>

        <div class="success">
            ✅ Connection successful! Your reverse SSH tunnel is functioning correctly.
        </div>

        <div class="info">
            <strong>Request #{}</strong><br>
            Method: {}<br>
            Path: {}<br>
            Time: {}<br>
            Server: localhost:{}
        </div>

        <h2>How it works:</h2>
        <ol>
            <li>This page is served from <code>localhost:{}</code></li>
            <li>Your reverse SSH client forwards connections from a remote server</li>
            <li>Anyone accessing the remote server reaches this local service</li>
        </ol>

        <h2>Next steps:</h2>
        <ul>
            <li>Replace this server with your actual application</li>
            <li>Configure your SSH server for production use</li>
            <li>Add authentication and security measures</li>
            <li>Monitor your tunnel connections</li>
        </ul>

        <p style="text-align: center; margin-top: 40px; opacity: 0.7;">
            Built with Rust 🦀 and reverse-ssh
        </p>
    </div>
</body>
</html>"#,
                        current_count,
                        method,
                        path,
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                        port,
                        port
                    );

                    let response = format!(
                        "HTTP/1.1 200 OK\r\n\
                         Content-Type: text/html; charset=utf-8\r\n\
                         Content-Length: {}\r\n\
                         Connection: close\r\n\
                         Server: reverse-ssh-test-server\r\n\
                         \r\n\
                         {}",
                        body.len(),
                        body
                    );

                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                        eprintln!("[{}] Error writing response: {}", current_count, e);
                    } else {
                        println!("[{}] Response sent (200 OK)", current_count);
                    }
                }
                Ok(_) => {
                    println!("[{}] Connection closed by client", current_count);
                }
                Err(e) => {
                    eprintln!("[{}] Error reading request: {}", current_count, e);
                }
            }
        });
    }
}

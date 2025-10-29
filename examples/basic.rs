use anyhow::Result;
use reverse_ssh::{ReverseSshClient, ReverseSshConfig};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Configure the reverse SSH connection
    let config = ReverseSshConfig {
        // SSH server to connect to
        server_addr: "your-server.com".to_string(),
        server_port: 22,

        // Authentication
        username: "your-username".to_string(),

        // Use either key-based or password authentication
        key_path: Some("/path/to/your/private/key".to_string()),
        password: None, // Or use password: Some("your-password".to_string())

        // Port mapping: SSH server will listen on remote_port
        // and forward connections to local_addr:local_port
        remote_port: 8080,
        local_addr: "127.0.0.1".to_string(),
        local_port: 3000,
    };

    // Create and run the reverse SSH client
    let mut client = ReverseSshClient::new(config);

    println!("Starting reverse SSH tunnel...");
    println!("Remote port 8080 will forward to local 127.0.0.1:3000");

    client.run().await?;

    Ok(())
}

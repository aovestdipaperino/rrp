use anyhow::{Context, Result};
use russh::client::{self, Handle, Msg};
use russh::keys::*;
use russh::*;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncReadExt;

use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

/// Configuration for the reverse SSH connection
#[derive(Debug, Clone)]
pub struct ReverseSshConfig {
    /// The SSH server address to connect to
    pub server_addr: String,
    /// The SSH server port
    pub server_port: u16,
    /// Username for SSH authentication
    pub username: String,
    /// Private key path for authentication
    pub key_path: Option<String>,
    /// Password for authentication (if not using key)
    pub password: Option<String>,
    /// Remote port to listen on (on the SSH server)
    pub remote_port: u32,
    /// Local address to forward connections to
    pub local_addr: String,
    /// Local port to forward connections to
    pub local_port: u16,
}

/// SSH client handler
struct Client {
    tx: mpsc::UnboundedSender<(Channel<Msg>, String, u32)>,
    message_tx: mpsc::UnboundedSender<String>,
}

#[async_trait::async_trait]
impl client::Handler for Client {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &key::PublicKey,
    ) -> Result<bool, Self::Error> {
        // In production, you should verify the server's public key
        // For now, we accept any key
        Ok(true)
    }

    async fn server_channel_open_forwarded_tcpip(
        &mut self,
        channel: Channel<Msg>,
        connected_address: &str,
        connected_port: u32,
        originator_address: &str,
        originator_port: u32,
        _session: &mut client::Session,
    ) -> Result<(), Self::Error> {
        info!(
            "Server opened forwarded channel: {}:{} -> {}:{}",
        debug!(
            "Forwarded channel: {}:{} -> {}:{}",
            originator_address, originator_port, connected_address, connected_port
        );

        // Send the channel to be handled
        let _ = self
            .tx
            .send((channel, connected_address.to_string(), connected_port));

        Ok(())
    }

    async fn data(
        &mut self,
        _channel: ChannelId,
        data: &[u8],
        _session: &mut client::Session,
    ) -> Result<(), Self::Error> {
        // Convert data to string and send it for processing
        // Don't filter out partial messages - send everything
        if let Ok(message) = String::from_utf8(data.to_vec()) {
            debug!("Received data ({} bytes): {}", data.len(), message);
            let _ = self.message_tx.send(message);
        } else {
            // Log if we received non-UTF8 data
            debug!(
                "Received {} bytes of non-UTF8 data on channel {:?}",
                data.len(),
                _channel
            );
        }
        Ok(())
    }

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
        debug!(
            "Received {} bytes of extended data (type {}) on channel {:?}",
            data.len(),
            ext,
            _channel
        );
        Ok(())
    }
}

impl Client {
    fn new(
        tx: mpsc::UnboundedSender<(Channel<Msg>, String, u32)>,
        message_tx: mpsc::UnboundedSender<String>,
    ) -> Self {
        Self { tx, message_tx }
    }
}

/// Reverse SSH client that establishes a reverse tunnel
pub struct ReverseSshClient {
    config: ReverseSshConfig,
    handle: Option<Handle<Client>>,
}

impl ReverseSshClient {
    /// Create a new reverse SSH client with the given configuration
    pub fn new(config: ReverseSshConfig) -> Self {
        Self {
            config,
            handle: None,
        }
    }

    /// Connect to the SSH server and authenticate
    pub async fn connect(
        &mut self,
        tx: mpsc::UnboundedSender<(Channel<Msg>, String, u32)>,
        message_tx: mpsc::UnboundedSender<String>,
    ) -> Result<()> {
        info!(
            "Connecting to SSH server {}:{}",
            self.config.server_addr, self.config.server_port
        );

        let client_config = client::Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
            ..<_>::default()
        };

        let client_handler = Client::new(tx, message_tx);

        let mut session = client::connect(
            Arc::new(client_config),
            (self.config.server_addr.as_str(), self.config.server_port),
            client_handler,
        )
        .await
        .context("Failed to connect to SSH server")?;

        // Authenticate
        let auth_result = if let Some(key_path) = &self.config.key_path {
            info!("Authenticating with private key: {}", key_path);
            let key_pair = russh_keys::load_secret_key(key_path, None)
                .context("Failed to load private key")?;
            session
                .authenticate_publickey(&self.config.username, Arc::new(key_pair))
                .await
        } else if let Some(password) = &self.config.password {
            info!("Authenticating with password");
            session
                .authenticate_password(&self.config.username, password)
                .await
        } else {
            anyhow::bail!("No authentication method provided (need key_path or password)");
        };

        if !auth_result.context("Authentication failed")? {
            anyhow::bail!("Authentication rejected by server");
        }

        info!("Successfully authenticated to SSH server");
        self.handle = Some(session);
        Ok(())
    }

    /// Set up a reverse port forward (remote port forwarding)
    /// This makes the SSH server listen on a port and forward connections back to us
    pub async fn setup_reverse_tunnel(&mut self) -> Result<()> {
        let handle = self
            .handle
            .as_mut()
            .context("Not connected - call connect() first")?;

        info!(
            "Setting up reverse tunnel: server port {} -> local {}:{}",
            self.config.remote_port, self.config.local_addr, self.config.local_port
        );

        // Request remote port forwarding
        // Use empty string "" instead of "0.0.0.0" - this lets the SSH server choose
        // the bind address. localhost.run requires this format.
        handle
            .tcpip_forward("", self.config.remote_port)
            .await
            .context("Failed to set up remote port forwarding")?;

        info!("Reverse tunnel established successfully");

        // Open a shell session to receive server messages (like the URL from localhost.run)
        // This is important for services that send connection info via shell
        match handle.channel_open_session().await {
            Ok(channel) => {
                info!("Opened shell session to receive server messages");
                // Request a shell - this triggers the server to send welcome messages
                if let Err(e) = channel.request_shell(false).await {
                    warn!("Failed to request shell: {}", e);
                } else {
                    debug!("Shell requested successfully");
                }
                // Don't close the channel - keep it open to receive messages
                // The channel will be kept alive by the handler
            }
            Err(e) => {
                warn!(
                    "Could not open shell session: {} (this may be normal for some servers)",
                    e
                );
            }
        }

        Ok(())
    }

    /// Read server messages (useful for services like localhost.run that send URL info)
    /// This opens a session channel and attempts to read any messages from the server
    #[allow(dead_code)]
    pub async fn read_server_messages(&mut self) -> Result<Vec<String>> {
        let handle = self
            .handle
            .as_mut()
            .context("Not connected - call connect() first")?;

        let mut messages = Vec::new();

        // Try to open a session channel to read any server messages
        match handle.channel_open_session().await {
            Ok(channel) => {
                // Request a shell to trigger server messages
                let _ = channel.request_shell(false).await;

                // Wait a bit for messages to arrive
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

                // Try to read data from the channel
                // Note: This is a simplified approach - in practice, we'd need to
                // handle the channel data in the Handler's data() method

                // Close the channel
                let _ = channel.eof().await;
                let _ = channel.close().await;

                messages.push("Check SSH session output for connection URL".to_string());
            }
            Err(e) => {
                warn!("Could not open session channel: {}", e);
            }
        }

        Ok(messages)
    }

    /// Handle forwarded connections from the SSH server
    pub async fn handle_forwarded_connections(
        &mut self,
        mut rx: mpsc::UnboundedReceiver<(Channel<Msg>, String, u32)>,
    ) -> Result<()> {
        info!("Waiting for forwarded connections...");

        while let Some((channel, _remote_addr, _remote_port)) = rx.recv().await {
            info!("New forwarded connection received");

            // Spawn a task to handle this connection
            let local_addr = self.config.local_addr.clone();
            let local_port = self.config.local_port;

            tokio::spawn(async move {
                if let Err(e) = handle_connection(channel, &local_addr, local_port).await {
                    error!("Error handling connection: {}", e);
                }
            });
        }

        warn!("Connection closed by server");
        Ok(())
    }

    /// Run the reverse SSH client (connect, setup tunnel, and handle connections)
    #[allow(dead_code)]
    pub async fn run(&mut self) -> Result<()> {
        let (tx, rx) = mpsc::unbounded_channel();
        let (message_tx, mut message_rx) = mpsc::unbounded_channel();

        self.connect(tx, message_tx).await?;
        self.setup_reverse_tunnel().await?;

        // Spawn a task to print server messages
        tokio::spawn(async move {
            while let Some(message) = message_rx.recv().await {
                // Print server messages, which may include URLs
                if !message.trim().is_empty() {
                    println!("[Server] {}", message.trim());
                }
            }
        });

        self.handle_forwarded_connections(rx).await?;

        Ok(())
    }

    /// Run the client with custom message handling
    pub async fn run_with_message_handler<F>(&mut self, mut message_handler: F) -> Result<()>
    where
        F: FnMut(String) + Send + 'static,
    {
        let (tx, rx) = mpsc::unbounded_channel();
        let (message_tx, mut message_rx) = mpsc::unbounded_channel();

        self.connect(tx, message_tx).await?;
        self.setup_reverse_tunnel().await?;

        // Spawn a task to handle server messages with custom handler
        tokio::spawn(async move {
            while let Some(message) = message_rx.recv().await {
                message_handler(message);
            }
        });

        self.handle_forwarded_connections(rx).await?;

        Ok(())
    }
}

/// Handle a single forwarded connection by proxying data between SSH channel and local service
async fn handle_connection(channel: Channel<Msg>, local_addr: &str, local_port: u16) -> Result<()> {
async fn handle_connection(
    mut channel: Channel<Msg>,
    local_addr: &str,
    local_port: u16,
) -> Result<()> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    info!("Connecting to local service {}:{}", local_addr, local_port);

    // Connect to the local service
    let local_socket_addr: SocketAddr = format!("{}:{}", local_addr, local_port)
        .parse()
        .context("Invalid local address")?;

    let mut local_stream = TcpStream::connect(local_socket_addr)
        .await
        .context("Failed to connect to local service")?;

    info!("Connected to local service, starting proxy");

    // For now, we'll implement a simple bidirectional proxy
    // Note: The russh Channel API may need adjustment based on actual usage
    info!("Connected to local service, starting bidirectional proxy");

    // Create a simple buffer for reading from local service
    let mut buffer = vec![0u8; 8192];
    // Bidirectional proxy using tokio::select!
    let mut local_buf = vec![0u8; 8192];

    // Read from local and forward to SSH
    loop {
        match local_stream.read(&mut buffer).await {
            Ok(0) => {
                debug!("Local connection closed");
                break;
            }
            Ok(n) => {
                debug!("Read {} bytes from local service", n);
                // Send data through SSH channel
                if let Err(e) = channel.data(&buffer[..n]).await {
                    error!("Failed to send data to SSH channel: {}", e);
                    break;
        tokio::select! {
            // Read from SSH channel and write to local service
            msg = channel.wait() => {
                match msg {
                    Some(russh::ChannelMsg::Data { data }) => {
                        debug!("Received {} bytes from SSH channel", data.len());
                        if let Err(e) = local_stream.write_all(&data).await {
                            error!("Failed to write to local service: {}", e);
                            break;
                        }
                    }
                    Some(russh::ChannelMsg::Eof) => {
                        debug!("Received EOF from SSH channel");
                        let _ = local_stream.shutdown().await;
                        break;
                    }
                    Some(russh::ChannelMsg::Close) => {
                        debug!("SSH channel closed");
                        break;
                    }
                    Some(other) => {
                        debug!("Received other channel message: {:?}", other);
                    }
                    None => {
                        debug!("SSH channel receiver closed");
                        break;
                    }
                }
            }
            Err(e) => {
                error!("Error reading from local service: {}", e);
                break;

            // Read from local service and write to SSH channel
            result = local_stream.read(&mut local_buf) => {
                match result {
                    Ok(0) => {
                        debug!("Local connection closed");
                        break;
                    }
                    Ok(n) => {
                        debug!("Read {} bytes from local service", n);
                        if let Err(e) = channel.data(&local_buf[..n]).await {
                            error!("Failed to send data to SSH channel: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        error!("Error reading from local service: {}", e);
                        break;
                    }
                }
            }
        }
    }

    // Close the channel
    // Close the channel gracefully
    let _ = channel.eof().await;
    let _ = channel.close().await;

    info!("Connection closed");
    info!("Connection proxy closed");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = ReverseSshConfig {
            server_addr: "example.com".to_string(),
            server_port: 22,
            username: "user".to_string(),
            key_path: Some("/path/to/key".to_string()),
            password: None,
            remote_port: 8080,
            local_addr: "127.0.0.1".to_string(),
            local_port: 3000,
        };

        assert_eq!(config.server_addr, "example.com");
        assert_eq!(config.remote_port, 8080);
    }
}

use crate::{
    byte_buf_utils::read_varint,
    config::{self, get_config},
    generators::{generate_code, generate_verify_token},
    map::get_map,
    mojang,
};
use aes::Aes128;
use anyhow::{anyhow, Result};
use bytes::{Buf, BufMut, BytesMut};
use cfb8::Encryptor;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::net::TcpStream;
use tracing::{debug, error, info};
use uuid::Uuid;

#[derive(Debug)]
pub enum NextStateEnum {
    Status,
    Login,
    Unknown,
}

#[derive(Debug)]
pub struct Session {
    pub server_id: String,
    pub proto_ver: Option<usize>,
    pub next_state: NextStateEnum,
    pub nickname: Option<String>,
    pub uuid: Option<Uuid>,
    pub secret: Option<Vec<u8>>, // Shared secret,
    pub verify_token: [u8; 4],
    pub cipher: Option<Encryptor<Aes128>>,
    pub addr: SocketAddr,
}

impl Session {
    pub async fn new(addr: SocketAddr) -> Self {
        let config = get_config().await;

        Self {
            server_id: config.server.config.server_name.clone(),
            proto_ver: None,
            next_state: NextStateEnum::Unknown,
            nickname: None,
            uuid: None,
            secret: None,
            verify_token: generate_verify_token(),
            cipher: None,
            addr,
        }
    }
}

pub struct MinecraftServer {
    pub session: Session,
    pub buffer: BytesMut,
    pub config: &'static config::types::Config,
    pub stream: TcpStream,
    pub keys: Arc<rsa::RsaPrivateKey>,
}

impl MinecraftServer {
    pub async fn new(stream: TcpStream, keys: Arc<rsa::RsaPrivateKey>) -> Result<Self> {
        Ok(Self {
            session: Session::new(stream.peer_addr()?).await,
            buffer: BytesMut::new(),
            config: get_config().await,
            stream,
            keys,
        })
    }

    pub async fn run(&mut self) {
        match self._run().await {
            Ok(_) => info!(
                "Connection from {:?} closed successfully",
                self.session.addr
            ),
            Err(e) => error!("Internal error occurred: {}", e),
        }
    }

    async fn _run(&mut self) -> Result<()> {
        loop {
            self.stream.readable().await?;
            let mut temp_buf = vec![0; 1024];

            match self.stream.try_read(&mut temp_buf) {
                Ok(0) => {
                    debug!("No more data to read. Exiting...");
                    break; // if client disconnected
                }
                Ok(n) => {
                    self.buffer.put_slice(&temp_buf[..n]);
                    self.handle_packet().await?;
                }
                Err(_) => {}
            }
            self.buffer.clear();
        }
        Ok(())
    }

    /**
    Packet handler
    */
    async fn handle_packet(&mut self) -> Result<()> {
        while self.packet_available() {
            let packet_id = read_varint(&mut self.buffer)?;
            debug!("Received packet: {}", packet_id);

            match packet_id {
                0x00 => self.handle_packet_0().await?,
                0x01 => self.handle_packet_1().await?,
                _ => return Err(anyhow!("Received unknown packet")),
            }
        }

        Ok(())
    }

    /**
    Handle packet with id 0
    */
    async fn handle_packet_0(&mut self) -> Result<()> {
        match self.session.next_state {
            NextStateEnum::Status => self.send_status().await?, // Send status response
            NextStateEnum::Login => {
                // Handle login start
                self.handle_login_start()?;
                self.send_encryption().await?;
            }
            NextStateEnum::Unknown => self.handle_handshake().await?, // Handle handshake
        }

        Ok(())
    }

    /**
    Handle packet with id 1
    */
    async fn handle_packet_1(&mut self) -> Result<()> {
        match self.session.next_state {
            NextStateEnum::Status => {
                // Handle ping request
                let payload = self.buffer.get_i64();
                self.send_pong(payload).await?
            }
            NextStateEnum::Login => {
                debug!("Received encryption response");
                self.handle_encryption()?;
                self.auth_client().await?
            }
            NextStateEnum::Unknown => return Err(anyhow!("Received unknown packet")),
        }
        Ok(())
    }

    async fn auth_client(&mut self) -> Result<()> {
        let player_data = mojang::join(&mut self.session, self.keys.clone()).await?;

        let player_data = match player_data {
            Some(x) => x,
            None => {
                self.send_disconnect(self.config.messages.bad_session.clone())
                    .await?;
                return Err(anyhow!("Mojang API error"));
            }
        };
        let map = get_map().await;
        let code = generate_code(6); // Generate 6-digit code

        // Insert client data into hash map
        map.insert(
            code.clone(),
            player_data.clone(),
            Duration::from_secs(self.config.api.code_life_time),
        )
        .await;

        // Disconnect client with code
        self.send_disconnect(
            self.config
                .messages
                .success
                .replace("{{NAME}}", &player_data.name)
                .replace("{{UUID}}", &player_data.id)
                .replace("{{CODE}}", &code),
        )
        .await?;

        info!("Created code {} for {}", code, player_data.name);
        Ok(())
    }

    fn packet_available(&mut self) -> bool {
        if self.buffer.is_empty() {
            return false;
        }
        // Read packet length
        read_varint(&mut self.buffer)
            .map(|x| self.buffer.len() >= x)
            .unwrap_or(false)
    }
}

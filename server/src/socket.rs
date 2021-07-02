use openssl::symm::{self, Cipher};
use tokio::{sync::mpsc, net::TcpStream};
use futures::{stream::SplitStream, StreamExt};
use qrcode::{QrCode, render::unicode::Dense1x2};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
use std::{
    net::SocketAddr,
    sync::{Arc, atomic::{AtomicBool, Ordering}}
};

const KEY_LEN: usize = 16;
const IV_LEN: usize = 12;
const TAG_LEN: usize = 16;
const BASE64_LEN: usize = KEY_LEN * 4 / 3 + 3;

type EncryptionKey = [u8; KEY_LEN];
type Receiver = SplitStream<WebSocketStream<TcpStream>>;

#[derive(Clone)]
pub struct SocketContext {
    event: mpsc::UnboundedSender<tfc::Command>,
    connected: Arc<AtomicBool>,
}

impl SocketContext {
    pub fn new(event: mpsc::UnboundedSender<tfc::Command>) -> Self {
        Self {
            event,
            connected: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn connect(&self, stream: TcpStream, addr: SocketAddr) {
        // TODO: Not sure if this is still an issue
        // I think we need to manually send pings or something
        // https://github.com/seanmonstar/warp/issues/798

        if self.connected.fetch_or(true, Ordering::Acquire) {
            println!("Refused connection from: {}", addr);
            return;
        }

        let ws_stream = match tokio_tungstenite::accept_async(stream).await {
            Ok(s) => s,
            Err(e) => {
                self.connected.store(false, Ordering::Release);
                eprintln!("{}", e);
                return;
            }
        };
        let (_, mut ws_rx) = ws_stream.split();

        println!("Connected to: {}", addr);

        if let Some(encryption_enabled) = self.receive_encryption_flag(&mut ws_rx).await {
            if encryption_enabled {
                self.receive_loop(ws_rx, Some(Self::generate_encryption_key())).await;
            } else {
                self.receive_loop(ws_rx, None).await;
            }
        }

        println!("Disconnected from: {}", addr);

        self.connected.store(false, Ordering::Release);
    }

    async fn receive_encryption_flag(&self, ws_rx: &mut Receiver) -> Option<bool> {
        if let Some(flag) = ws_rx.next().await {
            match flag {
                Ok(f) => {
                    let flag = Self::parse_flag(f);
                    if !flag.is_some() {
                        eprintln!("Invalid encryption flag");
                    }
                    flag
                }
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
            }
        } else {
            None
        }
    }

    fn parse_flag(message: Message) -> Option<bool> {
        if !message.is_binary() {
            return None;
        }
        let data = message.into_data();
        if data.len() != 1 {
            return None;
        }
        match data[0] {
            0 => Some(false),
            1 => Some(true),
            _ => None
        }
    }

    fn generate_encryption_key() -> EncryptionKey {
        let mut key = [0; KEY_LEN];
        let mut base64_key = [0; BASE64_LEN];
        
        openssl::rand::rand_bytes(&mut key).unwrap();
        let len = base64::encode_config_slice(key, base64::STANDARD, &mut base64_key);
        let code = QrCode::new(&base64_key[..len]).unwrap();
        
        println!("{}", code.render::<Dense1x2>()
            .dark_color(Dense1x2::Light)
            .light_color(Dense1x2::Dark)
            .build()
        );
        println!("This is the encryption key.");
        println!("You might want to press enter a few times to hide this after you scan it.");

        key
    }

    async fn receive_loop(&self, mut ws_rx: Receiver, key: Option<EncryptionKey>) {
        while let Some(message) = ws_rx.next().await {
            match message {
                Ok(m) => self.receive(m, &key),
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            }
        }
    }

    fn receive(&self, message: Message, key: &Option<EncryptionKey>) {
        if !message.is_binary() {
            return;
        }

        let bytes = message.into_data();
        let mut bytes = bytes.as_slice();
        if bytes.is_empty() {
            return;
        }

        let plaintext_buffer;

        if let Some(key) = key {
            if bytes.len() < IV_LEN + TAG_LEN {
                eprintln!("Message is too short to be encrypted");
                return;
            }

            let result = symm::decrypt_aead(
                Cipher::aes_128_gcm(),
                key,
                Some(&bytes[..IV_LEN]),
                &[],
                &bytes[IV_LEN..bytes.len() - TAG_LEN],
                &bytes[bytes.len() - TAG_LEN..]
            );

            match result {
                Ok(b) => {
                    plaintext_buffer = b;
                    bytes = plaintext_buffer.as_slice();
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            }
        }

        loop {
            let (command, len) = match tfc::Command::from_bytes(bytes) {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            };

            if self.event.send(command).is_err() {}
            if len < bytes.len() {
                bytes = &bytes[len..];
            } else {
                break;
            }
        }
    }
}

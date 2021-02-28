use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tfc::{Command, CommandBytesError};
use std::fmt::{self, Display, Formatter};

pub enum SocketError {
    Network(tokio::io::Error),
    Data(CommandBytesError),
}

impl Display for SocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SocketError::Network(e) => write!(f, "{}", e),
            SocketError::Data(e) => write!(f, "{}", e),
        }
    }
}

pub struct SocketContext {
    ctx: tfc::Context,
}

const NULL_COMMAND_CODE: u8 = 255;
const _: [u8; 1] = [0; (tfc::CommandCode::COUNT < NULL_COMMAND_CODE) as usize];

impl SocketContext {
    pub fn new(ctx: tfc::Context) -> Self {
        Self { ctx }
    }

    pub async fn handle_stream(&mut self, mut stream: TcpStream) -> Result<(), SocketError> {
        let mut buf = vec![0; 1024];
        let mut required_len = 1;
        let mut filled_len = 0;

        loop {
            let read_len = match stream.read(&mut buf[filled_len..required_len]).await {
                Ok(l) => l,
                Err(e) => return Err(SocketError::Network(e)),
            };
            if read_len == 0 {
                return Ok(())
            }
            filled_len += read_len;

            if filled_len < required_len {
                continue;
            }

            if required_len == 1 && buf[0] == NULL_COMMAND_CODE {
                filled_len = 0;
                continue;
            }

            match Command::from_bytes(&buf[..required_len]) {
                Ok((command, consumed_len)) => {
                    assert_eq!(filled_len, required_len);
                    assert_eq!(required_len, consumed_len);
                    required_len = 1;
                    filled_len = 0;
                    if let Err(e) = command.execute(&mut self.ctx) {
                        println!("Execute: {}", e);
                    }
                }

                Err(CommandBytesError::BufferTooShort(expected_len)) => {
                    required_len = expected_len;
                    if required_len > buf.len() {
                        // The buffer will not grow larger than 65535 + 3 bytes
                        buf.resize(required_len, 0);
                    }
                }

                // TODO: Perhaps try to recover from some errors.
                // An invalid command code is fatal but for the others, we can
                // print an error message and skip over the invalid bytes to
                // continue processing.
                Err(e) => {
                    return Err(SocketError::Data(e));
                }
            }
        }
    }
}

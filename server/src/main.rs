use tokio::sync::mpsc;
use std::net::Ipv4Addr;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use tfc::{Command, CommandBytesError, CommandCode, Context};

enum Disconnection {
    Normal,
    Network(std::io::Error),
    Data(CommandBytesError),
}

const NULL_COMMAND_CODE: u8 = 255;
const _: [u8; 1] = [0; (CommandCode::COUNT < NULL_COMMAND_CODE) as usize];

async fn handle_stream(ch_tx: mpsc::UnboundedSender<Command>, mut stream: TcpStream) -> Disconnection {
    let mut buf = vec![0; 1024];
    let mut required_len = 1;
    let mut filled_len = 0;

    loop {
        let read_len = match stream.read(&mut buf[filled_len..required_len]).await {
            Ok(l) => l,
            Err(e) => return Disconnection::Network(e),
        };
        if read_len == 0 {
            return Disconnection::Normal;
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
                // Send only fails if the receiving end is dropped.
                if ch_tx.send(command).is_err() {}
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
                return Disconnection::Data(e);
            }
        }
    }
}

// If the network connection is abruptly disconnected, e.g. because a cable was
// yanked, there's no FIN packet so the server wouldn't know that a
// disconnection has occurred. Maybe I shouldn't have moved away from
// WebSockets. I didn't quite realise how helpful WebSockets was being for me.
// I'm going to need to have a ping-pong thing.

static CONNECTED: AtomicBool = AtomicBool::new(false);

#[tokio::main(flavor="current_thread")]
async fn main() {
    let port = {
        let args = std::env::args().collect::<Vec<_>>();
        if args.len() == 1 {
            2048
        } else if args.len() == 2 {
            match args[1].parse() {
                Ok(p) => p,
                Err(_) => {
                    println!("Invalid port number: {}", args[1]);
                    return;
                }
            }
        } else {
            println!("  tfc-server\n  tfc-server <port>");
            return;
        }
    };

    let mut ctx = match Context::new() {
        Ok(c) => c,
        Err(e) => {
            println!("Initialize: {}", e);
            return;
        }
    };

    let (ch_tx, mut ch_rx) = mpsc::unbounded_channel::<Command>();

    // We need to decline incoming connections if we're already connected.
    // We need to invoke the methods of tfc::Context on the main thread.
    // We need to await accept() and read() at the same time.

    tokio::spawn(async move {
        let listener = match TcpListener::bind((Ipv4Addr::new(0, 0, 0, 0), port)).await {
            Ok(l) => l,
            Err(e) => {
                println!("Bind: {}", e);
                return;
            }
        };
        println!("Listening on port {}", port);

        loop {
            let (mut stream, addr) = match listener.accept().await {
                Ok(s) => s,
                Err(e) => {
                    println!("Accept: {}", e);
                    continue;
                }
            };

            if CONNECTED.fetch_or(true, Ordering::Relaxed) {
                println!("Declined connection from {}", addr);
                continue;
            } else {
                println!("Connected to {}", addr);
            }

            let ch_tx = ch_tx.clone();
            tokio::spawn(async move {
                match handle_stream(ch_tx, stream).await {
                    Disconnection::Normal => println!("Disconnected"),
                    Disconnection::Network(e) => println!("Disconnected: {}", e),
                    Disconnection::Data(e) => println!("Disconnected: {}", e),
                }
                CONNECTED.store(false, Ordering::Relaxed);
            });
        }
    });

    while let Some(command) = ch_rx.recv().await {
        if let Err(e) = command.execute(&mut ctx) {
            println!("Execute: {}", e);
        }
    }
}

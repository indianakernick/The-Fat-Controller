use std::io::Read;
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use tfc::{Command, CommandBytesError, CommandCode, Context};

enum Disconnection {
    Normal,
    Network(std::io::Error),
    Data(CommandBytesError),
}

const NULL_COMMAND_CODE: u8 = 255;
const _: [u8; 1] = [0; (CommandCode::COUNT < NULL_COMMAND_CODE) as usize];

fn handle_stream(ctx: &mut Context, mut stream: TcpStream) -> Disconnection {
    let mut buf = vec![0; 1024];
    let mut required_len = 1;
    let mut filled_len = 0;

    loop {
        let read_len = match stream.read(&mut buf[filled_len..required_len]) {
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
                if let Err(e) = command.execute(ctx) {
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
                return Disconnection::Data(e);
            }
        }
    }
}

fn main() {
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

    let listener = match TcpListener::bind((Ipv4Addr::new(0, 0, 0, 0), port)) {
        Ok(l) => l,
        Err(e) => {
            println!("Bind: {}", e);
            return;
        }
    };
    println!("Listening on port {}", port);

    loop {
        let (stream, addr) = match listener.accept() {
            Ok(s) => s,
            Err(e) => {
                println!("Accept: {}", e);
                continue;
            }
        };
        println!("Connected to {}", addr);
        match handle_stream(&mut ctx, stream) {
            Disconnection::Normal => println!("Disconnected"),
            Disconnection::Network(e) => println!("Disconnected: {}", e),
            Disconnection::Data(e) => println!("Disconnected: {}", e),
        }
    }
}

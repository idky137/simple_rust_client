// simple_rust_client - main.rs
// use: gains access to a shell running on simple_rust_server.rs over tcp using IP address and port given.
// authers: idkky137
//

use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

fn send_command(command: &str) {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
        stream
            .write_all(command.as_bytes())
            .expect("Error: Failed sending command");

        let mut buffer = Vec::new();

        loop {
            let mut chunk = vec![0; 1024];
            match stream.read(&mut chunk) {
                Ok(size) if size > 0 => {
                    buffer.extend_from_slice(&chunk[..size]);
                    let buffer_len = buffer.len();
                    let eor_flag: &[u8] = &buffer[buffer_len - 2..];
                    if eor_flag == [b'\t', b'\t'] {
                        break;
                    }
                }
                Ok(0) | Ok(_) | Err(_) => {
                    println!("Error: Failed to receive response from server");
                    break;
                }
            }
        }

        let response = String::from_utf8_lossy(&buffer);
        println!("{}", response);
    } else {
        eprintln!("Error: failed to connect to server")
    }
}

fn main() {
    println!(
            " - Enter command:\t--- use \"endsession\" to end connection with server and close program."
        );

    loop {
        let mut command = String::new();

        print!("$$ ");
        io::stdout().flush().expect("Error: failed to flush stdout");

        let _length = std::io::stdin()
            .read_line(&mut command)
            .expect("Error: failed to read command");

        let str_command = command.trim();

        if str_command == "endsession" {
            println!("Ending connection with server and closing program");
            send_command(str_command);
            break;
        } else if str_command == "" {
            println!("No command entered\n");
        } else {
            send_command(str_command);
        }
    }
}

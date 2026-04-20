use signal_hook::{consts::SIGINT, iterator::Signals};
use std::os::unix::net::UnixDatagram;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{env, io, thread};

const SOCK_PATH: &str = "/tmp/scry.sock";

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn open_socket() -> io::Result<UnixDatagram> {
    let socket = UnixDatagram::bind(SOCK_PATH)?;
    socket.set_nonblocking(true)?;
    Ok(socket)
}

fn main() -> std::io::Result<()> {
    let mut buf = vec![0; 100];

    let args: Vec<String> = env::args().collect();

    let run = Arc::new(AtomicBool::new(true));
    let run_clone = run.clone();

    let mut signal = Signals::new([SIGINT])?;
    thread::spawn(move || {
        for sig in signal.forever() {
            println!("Received signal {:?}", sig);
            run_clone.store(false, Ordering::Release);
        }
    });

    let socket = open_socket()?;

    let output = std::process::Command::new(&args[1])
        .envs([
            ("LD_PRELOAD", "./spy/libspy.so"),
            ("SCRY_SOCK_PATH", "/tmp/scry.sock"),
        ])
        .output()?;

    println!("{:?}", std::str::from_utf8(output.stdout.as_slice()));
    println!("Starting while");
    while run.load(Ordering::Acquire) {
        match socket.recv(buf.as_mut_slice()) {
            Ok(size) => {
                let msg = std::str::from_utf8(&buf[..size]).expect("Failed to parse message");
                println!("{}", msg);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No data available, just continue
                continue;
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }
    println!("Exiting...");
    socket.shutdown(std::net::Shutdown::Both)?;
    std::fs::remove_file(SOCK_PATH)?;

    Ok(())
}

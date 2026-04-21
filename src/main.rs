use common::SharedData;
use shared_memory::{ShmemConf, ShmemError};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::os::unix::net::UnixDatagram;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{env, io, thread};

// const SOCK_PATH: &str = "/tmp/scry.sock";

const LIB_PATH: &str = "./target/debug/libspy.so";

// fn open_socket() -> io::Result<UnixDatagram> {
//     let socket = UnixDatagram::bind(SOCK_PATH)?;
//     socket.set_nonblocking(true)?;
//     Ok(socket)
// }

fn open_shmem() -> io::Result<&'static mut SharedData> {
    let shmem_conf = shared_memory::ShmemConf::new()
        .size(4096)
        .os_id("scry_shmem");

    let shmem = match shmem_conf.create() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Unble to create shared memory {e}");
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to create shared memory",
            ));
        }
    };

    let data = unsafe { &mut *(shmem.as_ptr() as *mut SharedData) };

    Ok(data)
}

fn main() -> std::io::Result<()> {
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

    let shmem_data = open_shmem()?;

    let output = std::process::Command::new(&args[1])
        .envs([
            ("LD_PRELOAD", LIB_PATH),
            ("SCRY_SOCK_PATH", "/tmp/scry.sock"),
        ])
        .output()?;

    println!("{:?}", std::str::from_utf8(output.stdout.as_slice()));
    println!("Starting while");
    while run.load(Ordering::Acquire) {
        let event = &shmem_data.buffer[shmem_data.tail.load(Ordering::Acquire) % 1024];
        println!("Event:\n\t size: {}\n\tptr: {:x}", event.size, event.ptr);
        shmem_data.tail.fetch_add(1, Ordering::Release);
    }
    println!("Exiting...");

    Ok(())
}

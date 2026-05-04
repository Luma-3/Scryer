use common::SharedData;
use shared_memory::Shmem;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::{env, io, thread};

// const SOCK_PATH: &str = "/tmp/scry.sock";

const LIB_PATH: &str = "./target/debug/libspy.so";

// fn open_socket() -> io::Result<UnixDatagram> {
//     let socket = UnixDatagram::bind(SOCK_PATH)?;
//     socket.set_nonblocking(true)?;
//     Ok(socket)
// }

pub struct SharedController {
    _shmem: Shmem,
    pub data: &'static mut SharedData,
}

impl SharedController {
    pub fn new() -> io::Result<Self> {
        let shmem_conf = shared_memory::ShmemConf::new()
            .size(size_of::<SharedData>())
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

        Ok(SharedController {
            _shmem: shmem,
            data,
        })
    }
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

    let shmem = SharedController::new()?;

    let output = std::process::Command::new(&args[1])
        .envs([
            ("LD_PRELOAD", LIB_PATH),
            ("SCRY_SOCK_PATH", "/tmp/scry.sock"),
        ])
        .output()?;

    println!("{:?}", std::str::from_utf8(output.stdout.as_slice()));
    println!("Starting while");
    while run.load(Ordering::Acquire) {
        let arg_num = shmem.data.tail.load(Ordering::Acquire) % 1024;
        let event = &shmem.data.buffer[arg_num];
        println!(
            "Event:{}\n\tSize: {}\n\tPtr:{}",
            arg_num,
            event.size.load(Ordering::Acquire),
            event.ptr.load(Ordering::Acquire)
        );
        thread::sleep(Duration::new(1, 0));
        shmem.data.tail.fetch_add(1, Ordering::Release);
    }
    println!("Exiting...");

    Ok(())
}

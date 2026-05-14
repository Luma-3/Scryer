use common::SharedData;
use common::event::EventType;
use shared_memory::Shmem;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::collections::HashMap;
use std::process::Child;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{env, io, thread};

mod alloc_data;

use crate::alloc_data::AllocData;

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

    let mut app = App::new(args)?;
    app.run_loop()?;

    println!("{:?}", app.alloc_data);

    Ok(())
}

#[derive()]
struct App {
    args: Vec<String>,
    run: Arc<AtomicBool>,

    shmem: SharedController,
    child: Child,

    pub alloc_data: HashMap<String, AllocData>,
}

impl App {
    fn new(args: Vec<String>) -> std::io::Result<Self> {
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

        let child = std::process::Command::new(&args[1])
            .envs([
                ("LD_PRELOAD", LIB_PATH),
                ("SCRY_SOCK_PATH", "/tmp/scry.sock"),
            ])
            .spawn()?;

        Ok(Self {
            run,
            args,
            shmem,
            child,
            alloc_data: HashMap::new(),
        })
    }

    pub fn run_loop(&mut self) -> std::io::Result<()> {
        println!("Starting while");
        loop {
            if !self.run.load(Ordering::Acquire) {
                self.child.kill()?;
                println!("Process Killed !");
                break;
            }
            match self.child.try_wait() {
                Ok(Some(status)) => {
                    println!("Process Exiting with code {}", status);
                    break;
                }

                Ok(None) => {
                    if let Some(event) = self.shmem.data.pop() {
                        self.alloc_data.insert(
                            event.ptr.to_string(),
                            AllocData {
                                size: event.size,
                                alloc_type: EventType::from_int(event.event_type as i32)?,
                            },
                        );
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            }
        }
        println!("Exiting...");
        Ok(())
    }
}

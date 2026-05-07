use common::SharedData;
use std::sync::OnceLock;

mod malloc;

fn safe_print(msg: &str) {
    unsafe {
        libc::write(1, msg.as_ptr() as *const libc::c_void, msg.len());
    }
}

static SHMEM_PTR: OnceLock<usize> = OnceLock::new();

pub fn get_shmem() -> Option<&'static mut SharedData> {
    let ptr = SHMEM_PTR.get()?;
    unsafe { Some(&mut *(*ptr as *mut SharedData)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init() {
    // # Safety
    let shmem_conf = shared_memory::ShmemConf::new().os_id("scry_shmem").open();
    if let Ok(shmem) = shmem_conf {
        let ptr = shmem.as_ptr() as usize;

        std::mem::forget(shmem);
        let _ = SHMEM_PTR.set(ptr);
    }

    safe_print("Library loaded !\n");
}

#[cfg(target_os = "linux")]
#[unsafe(link_section = ".init_array")]
pub static INIT: unsafe extern "C" fn() = init;

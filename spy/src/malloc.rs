use std::os::raw::c_void;
use std::sync::atomic::Ordering;

use crate::{SHMEM_PTR, get_shmem};

#[unsafe(no_mangle)]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    unsafe {
        let real_malloc_ptr = libc::dlsym(libc::RTLD_NEXT, b"malloc\0".as_ptr() as *const i8);

        if real_malloc_ptr.is_null() {
            panic!("Failed to find original malloc");
        }

        let msg = "Malloc intercept !\n";
        libc::write(1, msg.as_ptr() as *const c_void, msg.len());

        if let Some(shmem) = get_shmem() {
            let event = &shmem.buffer[shmem.head.load(Ordering::Acquire)];
            event.size.store(size, Ordering::Release);
        }

        let real_malloc: extern "C" fn(usize) -> *mut u8 = std::mem::transmute(real_malloc_ptr);
        real_malloc(size)
    }
}

use std::os::raw::c_void;

use common::event::{AllocEvent, EventType};

use crate::get_shmem;

#[unsafe(no_mangle)]
pub extern "C" fn free(ptr: *mut u8) {
    unsafe {
        let real_free_ptr = libc::dlsym(libc::RTLD_NEXT, c"free".as_ptr() as *const i8);

        if real_free_ptr.is_null() {
            panic!("Failed to find original malloc");
        }

        let msg = "Free intercept !\n";
        libc::write(1, msg.as_ptr() as *const c_void, msg.len());

        let real_free: extern "C" fn(*mut u8) = std::mem::transmute(real_free_ptr);

        real_free(ptr);

        let ptr_val = ptr as usize;

        if let Some(shmem) = get_shmem() {
            let _ = shmem.push(AllocEvent {
                size: 0,
                ptr: ptr_val,
                event_type: EventType::Dealloc as usize,
            });
        }
    }
}

mod malloc;

fn safe_println(msg: &str) {
    unsafe {
        libc::write(1, msg.as_ptr() as *const libc::c_void, msg.len());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init() {
    safe_println("Library loaded !\n");
}

#[cfg(target_os = "linux")]
#[unsafe(link_section = ".init_array")]
pub static INIT: unsafe extern "C" fn() = init;

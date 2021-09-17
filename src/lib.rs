use bindings::Windows::Win32::Foundation::{BOOL, HINSTANCE};
use bindings::Windows::Win32::System::Console::{AllocConsole, FreeConsole};
use bindings::Windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
use bindings::Windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use std::ffi::c_void;
use std::thread;

macro_rules! create_entry_point {
    () => {};
}

fn hack_main_thread() {
    unsafe {
        AllocConsole();
    }
    for i in 0..5000 {
        println!("{}", i);
    }
    unsafe {
        FreeConsole();
    }
}

#[no_mangle]
#[allow(non_snake_case)]
fn DllMain(h_module: HINSTANCE, dw_reason: u32, _: *const c_void) -> BOOL {
    if dw_reason == DLL_PROCESS_ATTACH {
        unsafe {
            DisableThreadLibraryCalls(h_module);
        }
        thread::spawn(|| {
            hack_main_thread();
        });
    }
    BOOL(1)
}

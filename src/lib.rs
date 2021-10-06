use winapi::um::{
        consoleapi::AllocConsole,
        wincon::FreeConsole,
};
use winapi::shared::minwindef::{BOOL, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::DLL_PROCESS_ATTACH;

use std::thread;

fn hack_main_thread() {
    unsafe {
        AllocConsole();
    }

    // YOUR STUNNING CODE'S SUPPOSED TO BE HERE;
    for i in 0..30000 {
        println!("Hello game {}", i);
    }

    unsafe {
        FreeConsole();
    }
}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(h_module: HINSTANCE, dw_reason: u32, _: LPVOID) -> BOOL {
    if dw_reason == DLL_PROCESS_ATTACH {
        unsafe {
            DisableThreadLibraryCalls(h_module);
        }
        thread::spawn(|| {
            hack_main_thread();
        });
    }
    TRUE
}
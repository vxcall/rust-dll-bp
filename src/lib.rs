use bindings::Windows::Win32::Foundation::{CloseHandle, BOOL, HINSTANCE};
use bindings::Windows::Win32::System::Console::{AllocConsole, FreeConsole};
use bindings::Windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
use bindings::Windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use bindings::Windows::Win32::System::Threading::CreateThread;
use bindings::Windows::Win32::System::Threading::THREAD_CREATION_FLAGS;
use std::ffi::c_void;
use std::mem;
use std::ptr;

extern "system" fn fmain(_lpthreadparameter: *mut c_void) -> u32 {
    unsafe {
        AllocConsole();
    }
    for i in 0..10000 {
        println!("{}", i);
    }
    unsafe {
        FreeConsole();
    }
    0
}

#[no_mangle]
#[allow(non_snake_case)]
fn DllMain(h_module: *mut c_void, dw_reason: u32, _lp_reserved: *const c_void) -> BOOL {
    if dw_reason == DLL_PROCESS_ATTACH {
        unsafe {
            DisableThreadLibraryCalls(mem::transmute::<*mut c_void, HINSTANCE>(h_module));
            let h_thread = CreateThread(
                ptr::null_mut(),
                0,
                Some(fmain),
                h_module,
                THREAD_CREATION_FLAGS(0),
                ptr::null_mut(),
            );
            if !h_thread.is_null() {
                CloseHandle(h_thread);
            }
        }
    }
    BOOL(1)
}

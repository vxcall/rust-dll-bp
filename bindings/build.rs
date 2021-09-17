fn main() {
    windows::build!(
        Windows::Win32::System::Threading::CreateThread,
        Windows::Win32::Foundation::BOOL,
        Windows::Win32::Foundation::CloseHandle,
        Windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls,
        Windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH,
        Windows::Win32::Foundation::HINSTANCE,
        Windows::Win32::Foundation::HANDLE,
        Windows::Win32::Security::SECURITY_ATTRIBUTES,
        Windows::Win32::System::SystemServices::DLL_PROCESS_DETACH,
        Windows::Win32::System::Threading::THREAD_CREATION_FLAGS,
        Windows::Win32::System::Console::AllocConsole,
        Windows::Win32::System::Console::FreeConsole,
    )
}

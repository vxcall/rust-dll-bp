use winapi::um::{
    consoleapi::AllocConsole,
    wincon::FreeConsole,
};


// A neat macro from toy-arms crate which eases defining entry point.
toy_arms::create_entrypoint!(hack_main_thread);

// A main function you want to run in the game.
fn hack_main_thread() {

    // YOUR STUNNING CODE'S SUPPOSED TO BE HERE;
    for i in 0..30000 {
        println!("using toy-arms {}", i);
    }

}

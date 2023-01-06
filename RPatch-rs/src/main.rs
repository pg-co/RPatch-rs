#[macro_use]
extern crate windows;

use std::env;
// use std::ptr;

// use windows::Win32::System::Com::*;
// use windows::Win32::System::UpdateAgent::*;

fn main() {
    println!("{}", env::consts::OS); // Prints the current OS
    if env::consts::OS == "linux" {
        println!("Not a Windows-Machine!");
        std::process::exit(-1);
    }
    println!("Starting Service");
    // unsafe {
    //     let _ = windows::Win32::System::Com::CoInitializeEx(::core::option::Option::Some(ptr::null_mut()), COINIT_APARTMENTTHREADED);
    //     // if hr.is_err() {
    //     //     println!("Error CoInit: {}", hr.unwrap_err().message());
    //     //     exit(-1)
    //     // }

    //     let session = ptr::null::<IUpdateSession>();
    //     let _ = windows::Win32::System::Com::CoCreateInstance::<&IUpdateSession, IUpdateSession>(&UpdateSession, &*session, CLSCTX_INPROC_SERVER);
    //     // if hr1.is_err() {
    //     //     println!("Error CoInstance: {}", hr1.unwrap_err().message());
    //     //     exit(-1)
    //     // }
    // }

}

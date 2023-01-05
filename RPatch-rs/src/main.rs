use std::{
    process::exit, ptr::{self}
};

use windows::{
    Win32::System::Com::*,
    Win32::System::UpdateAgent::*,
};

fn main() {
    unsafe {
        let hr = CoInitializeEx(Option::Some(ptr::null_mut()), COINIT_APARTMENTTHREADED);
        if hr.is_err() {
            println!("Error CoInit: {}", hr.unwrap_err().message());
            exit(-1)
        }

        let hr1 = CoCreateInstance::<&IUpdateSession, IUpdateSession>(&UpdateSession, &UpdateSe ssion, CLSCTX_INPROC_SERVER);
        if hr1.is_err() {
            println!("Error CoInit: {}", hr1.unwrap_err().message());
            exit(-1)
        }
    }
}

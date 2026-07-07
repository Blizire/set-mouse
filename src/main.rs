use std::env;
use std::ffi::c_void;
use std::process::ExitCode;

use windows_sys::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_SETMOUSESPEED,
};

const MIN_SPEED: i32 = 1;
const MAX_SPEED: i32 = 20;

fn main() -> ExitCode {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "set-mouse".to_string());

    let Some(arg) = args.next() else {
        eprintln!("Usage: {program} <speed 1-20>");
        return ExitCode::FAILURE;
    };

    let speed: i32 = match arg.parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: '{arg}' is not a valid integer.");
            return ExitCode::FAILURE;
        }
    };

    if !(MIN_SPEED..=MAX_SPEED).contains(&speed) {
        eprintln!("Error: speed must be between {MIN_SPEED} and {MAX_SPEED} (got {speed}).");
        return ExitCode::FAILURE;
    }

    let ok = unsafe {
        SystemParametersInfoW(
            SPI_SETMOUSESPEED,
            0,
            speed as *mut c_void,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        )
    };

    if ok == 0 {
        eprintln!("Error: SystemParametersInfoW failed (last error: {}).", std::io::Error::last_os_error());
        return ExitCode::FAILURE;
    }

    println!("Mouse pointer speed set to {speed}.");
    ExitCode::SUCCESS
}

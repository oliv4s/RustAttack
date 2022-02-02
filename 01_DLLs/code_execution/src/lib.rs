#![cfg(windows)]
use std::ptr::null_mut;
use winapi::um::winuser::MessageBoxA;

#[no_mangle]
#[allow(unused_variables)]

pub extern "C" fn exec() {
    use std::process::Command;
    let output = Command::new("cmd").args(&["/C", "calc.exe"]).output().expect("failed to execute process");
    output.stdout;
}

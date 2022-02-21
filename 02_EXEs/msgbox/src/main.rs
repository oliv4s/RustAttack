#![cfg(windows)]
use std::ptr::null_mut;
use winapi::um::winuser::MessageBoxA;

#[allow(unused_variables)]

pub fn main() {
    unsafe {MessageBoxA(null_mut(),"oliv4s\0".as_ptr() as *const i8,"Rust EXE MsgBox Test\0".as_ptr() as *const i8,0x00004000);}
}

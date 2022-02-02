#![cfg(windows)]
use std::ptr::null_mut;
use winapi::um::winuser::MessageBoxA;

#[no_mangle]
#[allow(unused_variables)]

pub extern "C" fn exec() {
    unsafe {MessageBoxA(null_mut(),"oliv4s\0".as_ptr() as *const i8,"Rust DLL MsgBox Test\0".as_ptr() as *const i8,0x00004000);}
}

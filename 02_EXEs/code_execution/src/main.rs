#![cfg(windows)]
#[allow(unused_variables)]

pub fn main() {
    use std::process::Command;
    let output = Command::new("cmd").args(&["/C", "calc.exe"]).output().expect("failed to execute process");
    output.stdout;
}

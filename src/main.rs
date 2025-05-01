#![allow(non_snake_case)]

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
fn main() {
   windows::main(); 
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
fn main() {
    linux::main()
}

#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "macos")]
fn main() {
    mac::main()
}
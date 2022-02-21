//! Small binary to run shellcode in memory
//!
//! Pass the shellcode file as the first parameter. It will be loaded into memory
//! and get executed.

mod mmap;

use mmap::MMAP;

use std::io::prelude::*;
use std::fs::File;
use std::mem::transmute;

type Shellcode = extern "C" fn();

fn main() -> std::io::Result<()> {
    let filename = match std::env::args().nth(1) {
        Some(f) => f,
        None => {
            println!("Pass filename of shellcode");
            return Ok(());
        }
    };
    let mut memory = MMAP::<0x1000>::new().expect("Failed to allocate memory");
    let mut file = File::open(filename)?;
    let _ = file.read(&mut *memory)?;

    let jump: Shellcode = unsafe { transmute(memory.data) };
    jump();

    Ok(())
}

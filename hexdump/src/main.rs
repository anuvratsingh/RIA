use std::{env, fs::File, io::prelude::*};

const BYTES_PER_LINE: usize = 16;

fn main() {
    let arg1 = env::args().nth(1);
    let fname = arg1.expect("usage: fview FILENAME");

    let mut f = File::open(&fname).expect("Unable to open file.");
    let mut position = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{:08x}]", position);
        for byte in &buffer {
            match *byte {
                0x00 => print!(".   "),
                0xff => print!("## "),
                _ => print!("{:02x}", byte),
            }
        }
        println!("");
        position += BYTES_PER_LINE;
    }
}

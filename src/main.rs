#![feature(io)]

use std::io::BufReadExt;

fn main() {
    let stdin = std::io::stdin();
    let stdin_buf = stdin.lock();
    
    for line_result in stdin_buf.lines() {
        match line_result {
            Ok(line) => println!("Read line {}", line),
            _ => break
        }
    }
    
    // loop {
    //     match stdin_buf.read_line() {
    //         Ok(line) => println!("Read line {}", line),
    //         _ => break
    //     }
    // }
}

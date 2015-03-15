#![feature(io)]

use std::io::BufReadExt;
use std::process::{Command,Stdio};
use std::sync::mpsc::sync_channel;
use std::thread;

fn main() {
    let stdin = std::io::stdin();
    let stdin_buf = stdin.lock();
    
    let comm = Command::new("sh")
        .arg("-c")
        .arg("for i in $(seq 1 3); do sleep 1; echo line $i; done")
        .stdout(Stdio::piped())
        .spawn().unwrap();
    
    let child_buf = std::io::BufReader::new(comm.stdout.unwrap());
    
    for line_result in child_buf.lines() {
        match line_result {
            Ok(line) => println!("Read line {}", line),
            Err(e) => {println!("Error: {}", e); break;}
        }
    }
}

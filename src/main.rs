#![feature(io)]

/**
A basic example of using a Buffered reader on both stdin and a child, and returning whatever happens first.

```bash
for i in $(seq 10 10 30); do echo $i; sleep 2; done | cargo run --release
    [... Compiling ...]
STDIN: 10
CHILD: line 1
STDIN: 20
CHILD: line 2
CHILD: line 3
STDIN: 30
```
*/

use std::io::BufReadExt;
use std::process::{Command,Stdio};
use std::sync::mpsc::sync_channel;
use std::thread;

enum Line {
    Child(String),
    Stdin(String)
}

fn main() {
    let comm = Command::new("sh")
        .arg("-c")
        .arg("for i in $(seq 1 3); do sleep 1; echo line $i; done")
        .stdout(Stdio::piped())
        .spawn().unwrap();
    
    let child_buf = std::io::BufReader::new(comm.stdout.unwrap());
    
    let (tx_stdin, rx) = sync_channel(1);
    let tx_child = tx_stdin.clone();
    
    thread::spawn(move|| {
        for line_result in child_buf.lines() {
            let line = match line_result {
                Ok(line) => Line::Child(line),
                Err(e) => {println!("Child error: {}", e); break;}
            };
            tx_child.send(line).unwrap();
        }
    });
    
    
    thread::spawn(move|| {
        let stdin = std::io::stdin();
        let stdin_buf = stdin.lock();
        
        for line_result in stdin_buf.lines() {
            let line = match line_result {
                Ok(line) => Line::Stdin(line),
                Err(e) => {println!("Error: {}", e); break;}
            };
            tx_stdin.send(line).unwrap();
        }
    });
    
    for line_result in rx.iter() {
        match line_result {
            Line::Child(line) => println!("CHILD: {}", line),
            Line::Stdin(line) => println!("STDIN: {}", line),
        }
    }
}

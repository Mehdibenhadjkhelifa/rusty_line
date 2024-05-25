use std::{
    io::{stdin, stdout, Write},
    process::Command,
};

fn main() {
    loop {
        print!("> ");
        stdout().flush().expect("Error flushing stdout");
        let mut input: String = String::new();
        let command: &str;
        stdin().read_line(&mut input).expect("Error reading input");
        let mut parts = input.trim().split_whitespace();
        command = parts.next().expect("Invalid Command");
        let args = parts;
        let mut child = Command::new(command)
            .args(args)
            .spawn()
            .expect("Error Executing command ");
        child.wait().unwrap();
    }
}

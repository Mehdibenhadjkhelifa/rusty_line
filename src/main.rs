use std::{io::{stdin, stdout, Write}, process::Command};

fn main() {
    loop{
        print!("> ");
        stdout().flush().expect("Error flushing stdout");
        let mut input: String = String::new();
        let mut command: &str = "None";
        match stdin().read_line(&mut input) {
            Ok(_) => {
                command = input.trim();
                println!("Successfully read {}", command);
            }
            Err(error) => println!("Error reading input : {}", error),
        }
        match Command::new(command).spawn() {
            Ok(mut ch) => {println!("Got the child : {:?}", ch);
                let _ = ch.wait();
            }
            Err(err) => println!("Error executing command : {},\nError is : {}", command, err),
        }
    }
}

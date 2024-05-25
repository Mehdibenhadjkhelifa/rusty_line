use std::{
    env, io::{stdin, stdout, Write}, path::Path, process::Command
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
        match command {
            "cd" => {
                // default to '/' as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/",|x| *x);
                let root = Path::new(new_dir) ;
                if let Err(e) = env::set_current_dir(&root){
                    eprintln!("{}",e);
                }
            },
            command =>{
                let mut child = Command::new(command)
                    .args(args)
                    .spawn()
                    .expect("Error Executing command ");
                child.wait().unwrap();
            }
        }
    }
}

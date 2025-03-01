#[allow(unused_imports)]
use std::io::{self, Write};

use codecrafters_shell::ShellCommand;

fn main() {
    loop {
        // Print prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let cmd = ShellCommand::new(&input);

        match cmd.run() {
            Ok(output) => {
                println!("{}", output.trim());
                continue;
            },
            Err(_err) => {
                break;
            }
        }
    }
}

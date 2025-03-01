pub struct Command {
    cmd: String,
    arguments: Vec<String>,
}

impl Command {
    pub fn new(input: &str) -> Command {
        let mut parts = input.split_whitespace();
        let cmd = parts.next().unwrap().to_owned();
        let arguments = parts.map(|s| s.to_owned()).collect();

        Command { cmd, arguments }
    }

    fn args_to_str(&self) -> String {
        self
            .arguments
            .iter()
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn run(&self) -> Option<String> {
        if self.cmd == "exit" {
            return None
        }

        let output = match self.cmd.as_str() {
            "echo" => Some(self.args_to_str()),
            _ => Some(format!("{}: command not found", self.cmd))
        };

        output
    }
}

const CMD_EXIT: &str = "exit";
const CMD_ECHO: &str = "echo";
const CMD_TYPE: &str = "type";
const CMD_ARR: [&str; 3] = [ CMD_EXIT, CMD_ECHO, CMD_TYPE ];

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

    fn args_to_str(&self) -> Result<String, String> {
        let args_string = self
            .arguments
            .iter()
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
            .join(" ");

        if args_string.len() == 0 {
            Err("No arguments provided".to_owned())
        } else {
            Ok(args_string)
        }
    }

    fn get_type(&self) -> Result<String, String> {
        if !CMD_ARR.contains(&self.cmd.as_str()) {
            self.invalid_command()
        } else {
            Ok(format!("{} is a shell builtin", self.cmd))
        }
    }

    fn invalid_command(&self) -> Result<String, String> {
        Err(format!("{}: command not found", self.cmd))
    }

    pub fn run(&self) -> Result<String, String> {
        if self.cmd == CMD_EXIT {
            return Err("0".to_string());
        }

        let output = match self.cmd.as_str() {
            CMD_ECHO => self.args_to_str(),
            CMD_TYPE => self.get_type(),
            _ => self.invalid_command()
        };

        output
    }
}

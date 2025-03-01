use std::os::unix::fs::PermissionsExt;

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

        if args_string.is_empty() {
            Err("No arguments provided".to_owned())
        } else {
            Ok(args_string)
        }
    }

    fn path_dirs() -> Result<Vec<String>, String> {
        std::env::var("PATH")
            .map(|p| p.split(":").map(|s| s.to_string()).collect::<Vec<String>>())
            .map_err(|err| err.to_string())
    }

    fn get_type(&self) -> Result<String, String> {
        let args_string = self.args_to_str()?;

        if CMD_ARR.contains(&args_string.as_str()) {
            return Ok(format!("{} is a shell builtin", args_string))
        }

        for dir in Command::path_dirs()? {
            let x = std::fs::read_dir(&dir).map_err(|e| e.to_string())?;

            for r in x.flatten() {
                let metadata = r.metadata().map_err(|e| e.to_string())?;
                if metadata.is_dir() {
                    continue;
                }

                let permissions = metadata.permissions().mode() & 0o111 != 0;
                let path = r.path().to_str().unwrap().to_owned();
                let split = path.split("/").collect::<Vec<&str>>();
                let file = split.last().unwrap();

                if &args_string == file && permissions {
                    return Ok(format!("{} is {}", args_string, path));
                }
            }
        }

        Ok(format!("{}: not found", args_string))
    }

    fn invalid_command(&self) -> Result<String, String> {
        Ok(format!("{}: command not found", self.cmd))
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

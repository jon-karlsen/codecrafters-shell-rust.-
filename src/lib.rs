use std::{os::unix::fs::PermissionsExt, process::Command};

const CMD_EXIT: &str = "exit";
const CMD_ECHO: &str = "echo";
const CMD_TYPE: &str = "type";
const CMD_ARR: [&str; 3] = [ CMD_EXIT, CMD_ECHO, CMD_TYPE ];

pub struct ShellCommand {
    cmd: String,
    arguments: Vec<String>,
}

impl ShellCommand {
    pub fn new(input: &str) -> ShellCommand {
        let mut parts = input.split_whitespace();
        let cmd = parts.next().unwrap().to_owned();
        let arguments = parts.map(|s| s.to_owned()).collect();

        ShellCommand { cmd, arguments }
    }

    fn args_to_str(&self) -> Result<String, String> {
        let args_string = self
            .arguments
            .iter()
            .map(|x| x.to_string())
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

    fn search_path(search_term: &str) -> Result<String, String> {
        for dir in ShellCommand::path_dirs()? {
            let dir_entries = std::fs::read_dir(&dir).map_err(|e| e.to_string())?;

            for entry in dir_entries.flatten() {
                let metadata = entry.metadata().map_err(|e| e.to_string())?;
                if metadata.is_dir() {
                    continue;
                }

                let permissions = metadata.permissions().mode() & 0o111 != 0;
                let path = entry.path().to_str().unwrap().to_owned();
                let split = path.split("/").collect::<Vec<&str>>();
                let file = split.last().unwrap();

                if *search_term == **file && permissions {
                    return Ok(path);
                }
            }
        }

        Err("No dirs in path".to_string())
    }

    fn get_type(&self) -> Result<String, String> {
        let args_string = self.args_to_str()?;

        if CMD_ARR.contains(&args_string.as_str()) {
            return Ok(format!("{} is a shell builtin", args_string))
        }

        match ShellCommand::search_path(&args_string) {
            Ok(search_result) => {
                let args_string = self.args_to_str()?;
                Ok(format!("{} is {}", args_string, search_result))
            },
            Err(_) => {
                Ok(format!("{}: not found", args_string))
            }
        }
    }

    fn invalid_command(&self) -> Result<String, String> {
        Ok(format!("{}: command not found", self.cmd))
    }

    fn execute_from_path(&self) -> Result<String, String> {
        let args = self.args_to_str()?;

        let output = Command::new(&self.cmd)
            .arg(args)
            .output()
            .map_err(|e| e.to_string())?;

        Ok(format!("{}", String::from_utf8_lossy(&output.stdout)))
    }

    pub fn run(&self) -> Result<String, String> {
        if self.cmd == CMD_EXIT {
            return Err("0".to_string());
        }

        let output = match self.cmd.as_str() {
            CMD_ECHO => self.args_to_str(),
            CMD_TYPE => self.get_type(),
            _ => {
                match ShellCommand::search_path(&self.cmd) {
                    Ok(_) => self.execute_from_path(),
                    _ => self.invalid_command()
                }
            }
        };

        output
    }
}

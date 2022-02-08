use dirs;
use std::{fmt, path::PathBuf};

pub enum ArgsError {
    InvalidArgs,
    NotEnoughArgs,
}

impl ArgsError {
    pub fn as_str(self) -> &'static str {
        match self {
            ArgsError::InvalidArgs => "invalid arguments",
            ArgsError::NotEnoughArgs => "not enough arguments",
        }
    }
}

pub enum Command {
    List { path: PathBuf },
    Remove { file: String },
    Read { file: String },
    Write { file: String, content: String },
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Remove { file } => write!(f, "Command::Remove{{file: {}}}", file),
            Command::List { path } => write!(f, "Command::List{{path: {:?}}}", path),
            Command::Read { file } => write!(f, "Command::Read{{file: {}}}", file),
            Command::Write { file, content } => {
                write!(f, "Command::Write{{file: {}, content: {}}}", file, content)
            }
        }
    }
}

impl Command {
    pub fn new(args: Vec<String>) -> Result<Command, ArgsError> {
        match &args[..] {
            [] => Err(ArgsError::NotEnoughArgs),
            [op] if op == "list" => {
                let notes_dir = dirs::document_dir().unwrap().join("notes");
                Ok(Command::List { path: notes_dir })
            }
            [_] => Err(ArgsError::NotEnoughArgs),
            [op, file_name] if op == "remove" => Ok(Command::Remove {
                file: file_name.to_string(),
            }),
            [op, file_name] if op == "read" => Ok(Command::Read {
                file: file_name.to_string(),
            }),
            [op, _] if op == "write" => Err(ArgsError::NotEnoughArgs),
            [op, file_name, content] if op == "write" => Ok(Command::Write {
                file: file_name.to_string(),
                content: content.to_string(),
            }),
            _ => Err(ArgsError::InvalidArgs),
        }
    }
}

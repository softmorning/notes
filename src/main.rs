use dirs;
use std::{env, fs, io, path, process};

mod utils;

fn main() -> Result<(), io::Error> {
    let mut notes_dir: path::PathBuf = dirs::document_dir().unwrap().join("notes");
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let command = utils::Command::new(args).unwrap_or_else(|err| {
        eprintln!("{}", err.as_str());
        process::exit(1);
    });

    match command {
        utils::Command::List { path } => Ok(pprint_dir(path)?),
        utils::Command::Remove { file: file_name } => {
            notes_dir.push(&file_name);

            fs::remove_file(notes_dir)?;

            println!("Removed {}", file_name);
            Ok(())
        }
        utils::Command::Read { file: file_name } => {
            notes_dir.push(file_name);

            let content = fs::read_to_string(notes_dir)?;
            println!("{}", content);

            Ok(())
        }
        utils::Command::Write {
            file: file_name,
            content,
        } => {
            fs::create_dir_all(&notes_dir)?;
            notes_dir.push(&file_name);

            fs::write(notes_dir, content)?;

            println!("Wrote file '{}'", file_name);

            Ok(())
        }
    }
}

fn pprint_dir(path: path::PathBuf) -> io::Result<()> {
    let dir = fs::read_dir(path)?;

    for item in dir {
        let printable = item?.path();

        println!("{:?}", printable);
    }

    Ok(())
}

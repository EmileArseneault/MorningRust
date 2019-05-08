use std::process::Command;
use std::io;
use std::io::prelude::*;
use std::env::temp_dir;
use std::env;
use std::fs::File;

fn get_env_editor() -> Result<String, env::VarError> {
    let editor = env::var("EDITOR")?;
    Ok(editor)
}

fn choose_editor() -> String {

    let env_editor = get_env_editor();
    let mut editor: String;

    match env_editor {
        Ok(env_editor) => {
            editor = env_editor;
        },
        Err(_) => {
            println!("Set environnement variable $EDITOR to choose default editor");
            println!("You an use the export command like so : export EDITOR=vim");

            // Should test to see what editors are installed on the system
            editor = String::from("nano");
        }
    }

    return editor;
}

pub fn edit_message() -> Result<String, io::Error> {
    let editor = choose_editor();
    let mut file_path = temp_dir();

    file_path.push("editable");
    File::create(&file_path)?;

    match Command::new(editor).arg(&file_path).status() {
        Ok(_) => {},
        Err(error_code) => {
            println!("Opening editor failed with code : {}", error_code);
        }
    }

    let mut message = String::new();
    File::open(file_path)?.read_to_string(&mut message)?;

    message = String::from(message.trim_end());
    Ok(message)
}

pub fn edit_file() {
    let editor = choose_editor();
    // This will be to edit command and reminder file with default editor
}

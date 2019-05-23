use std::fs;
use std::process::Command;
use std::io;
use std::io::Write;
use std::io::ErrorKind;

use super::configuration::Configuration;
use super::editing;

pub fn execute_commands(conf: &Configuration){

    let command_string = fs::read_to_string(conf.command_file());

    match command_string {
        Ok(commands) => {
            // Do not print the section if there is nothing to write
            if !commands.is_empty() {
                println!("------------------- Commands --------------------");
                let output = Command::new("bash").arg(conf.command_file()).output().expect("Cannot execute commands");
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();

                // Source does not exist it is part of the shell, will have to find something
                // std::os::unix::process::parent_id -> u32
            }
        },
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    // No file is present, so just ignore this section
                },
                ErrorKind::PermissionDenied => {
                    println!("------------------- Commands --------------------");
                    println!("Cannot read the file {}, permission denied", 
                             conf.command_file().display());
                }
                _ => {
                    println!("------------------- Commands --------------------");
                    println!("Cannot read command file");
                    println!("{}", e);
                }
            }
        }
    }
}

pub fn edit_command(conf: &Configuration) {
    editing::edit_file(conf.command_file());
}
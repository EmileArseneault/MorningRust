use std::fs;
use std::io::ErrorKind;

use super::configuration::Configuration;
use super::editing;

pub fn print_reminder(conf: &Configuration){

    let reminder_string = fs::read_to_string(conf.reminder_file());

    match reminder_string {
        Ok(reminders) => {
            // Do not print the section if there is nothing to write
            if !reminders.is_empty() {
                println!("------------------ Reminders --------------------");
                println!("{}", reminders);
            }
        },
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    // No file is present, so just ignore this section
                },
                ErrorKind::PermissionDenied => {
                    println!("------------------ Reminders --------------------");
                    println!("Cannot read the file {}, permission denied", 
                             conf.reminder_file().display());
                }
                _ => {
                    println!("------------------ Reminders --------------------");
                    println!("Cannot read reminder file");
                    println!("{}", e);
                }
            }
        }
    }
}

pub fn edit_reminder(conf: &Configuration) {
    editing::edit_file(conf.reminder_file());
}
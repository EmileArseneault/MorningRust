use std::env;
use std::fs;
use std::io;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

extern crate dirs;
extern crate serde;
extern crate serde_json;

const INSTALLED_SCRIPT:   &str = "/usr/bin/";
// Configuration file for installed requires root access at /etc/morning.conf
// use ~/.config/morning.conf or ~/.config/morning/morning.conf or ~/.morning/morning.conf?
const INSTALLED_CONFIG:   &str = "~/.config/morning.conf";
const INSTALLED_COMMAND:  &str = "~/.morning/command";
const INSTALLED_REMINDER: &str = "~/.morning/reminder.txt";
const INSTALLED_HISTORY:  &str = "~/.morning/history.json";

const PORTABLE_CONFIG:    &str = "morning.conf";
const PORTABLE_COMMAND:   &str = "command";
const PORTABLE_REMINDER:  &str = "reminder.txt";
const PORTABLE_HISTORY:   &str = "history.json";


#[derive(Debug)]
pub struct Configuration {
    executing_dir: PathBuf,
    current_dir:   PathBuf,
    home_dir:      PathBuf,
    portable:      bool,
    config_change: bool,
    config_file:   PathBuf,
    config:        ConfigurationContent,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigurationContent {
    command_file:   PathBuf,
    reminder_file:  PathBuf,
    history_file:   PathBuf,
    history_length: u32
}

impl Configuration {

    pub fn new() -> Result<Self, Box<dyn Error>> {

        let executing_dir = find_executing_dir()?;
        let current_dir   = find_current_dir()?;
        let home_dir      = find_home_dir()?;
        let portable      = is_script_portable();
        let mut config_change = false;

        let mut config_file: PathBuf;

        if portable {
            config_file = PathBuf::from(&executing_dir);
            config_file.push(PORTABLE_CONFIG);
        } else {
            config_file = PathBuf::from(INSTALLED_CONFIG);
        }

        let config: ConfigurationContent;

        if !Path::exists(&config_file) {
            println!("No configuration found, creating one");

            // Create new configuration
            config = Configuration::generate_config(portable, &executing_dir, &home_dir);
            config_change = true;
        } else {
            // Read existing configuration
            config = Configuration::read_config(config_file.as_path())?;
        }

        Ok(
            Configuration {
                executing_dir: executing_dir,
                current_dir:   current_dir,
                home_dir:      home_dir,
                portable:      portable,
                config_change: config_change,
                config_file:   config_file,
                config:        config
            }
        )
    }

    fn read_config(config_path: &Path) -> Result<ConfigurationContent, io::Error>{
        let json_str = fs::read_to_string(config_path)?;
        let json_config = serde_json::from_str(&json_str);

        match json_config {
            Ok(config) => {
                Ok(config)
            },
            Err(e) => {
                Err(io::Error::new(io::ErrorKind::Other, "Serde Error"))
            }
        }
    }

    pub fn write_config(&self) -> Result<(), Box<dyn Error>> {

        if self.config_change {
            let json_config = serde_json::to_string(&self.config)?;
            fs::write(&self.config_file, json_config)?;
        }

        Ok(())
    }

    fn generate_config(portable: bool, executing_dir: &PathBuf, home_dir: &PathBuf) -> ConfigurationContent{

        let mut command_file:  PathBuf;
        let mut reminder_file: PathBuf;
        let mut history_file:  PathBuf;

        if portable {
            command_file  = PathBuf::from(executing_dir);
            reminder_file = PathBuf::from(executing_dir);
            history_file  = PathBuf::from(executing_dir);

            command_file.push(PORTABLE_COMMAND);
            reminder_file.push(PORTABLE_REMINDER);
            history_file.push(PORTABLE_HISTORY);

            ConfigurationContent {
                command_file:   command_file,
                reminder_file:  reminder_file,
                history_file:   history_file,
                history_length: 15,
            }

        } else {
            command_file  = PathBuf::from(home_dir);
            reminder_file = PathBuf::from(home_dir);
            history_file  = PathBuf::from(home_dir);

            command_file.push(INSTALLED_COMMAND);
            reminder_file.push(INSTALLED_REMINDER);
            history_file.push(INSTALLED_HISTORY);

            ConfigurationContent {
                command_file:   command_file,
                reminder_file:  reminder_file,
                history_file:   history_file,
                history_length: 15,
            }
        }
    }

    pub fn is_portable(&self) -> bool {
        self.portable
    }

    pub fn history_path(&self) -> &PathBuf {
        &self.config.history_file
    }
}

fn is_script_portable() -> bool {

    let installed_dir: PathBuf = PathBuf::from(INSTALLED_SCRIPT);

    match find_executing_dir() {
        Ok(exd) => {
            if exd == installed_dir {
                false
            } else {
                true
            }
        },
        Err(_) => {
            true
        }
    }
}

fn find_executing_dir() -> Result<PathBuf, Box<dyn Error>> {

    let mut executing_dir = env::current_exe()?
                                .canonicalize()?;

    executing_dir = PathBuf::from(
                        executing_dir.parent()
                                        .ok_or("Cannot find parent")?
                    );

    Ok(executing_dir)
}

fn find_current_dir() -> Result<PathBuf, Box<dyn Error>> {
    Ok(
        env::current_dir()?
            .canonicalize()?
    )
}

fn find_home_dir() -> Result<PathBuf, Box<dyn Error>> {
    Ok(
        dirs::home_dir()
                .ok_or("Cannot find home directory")?
    )
}
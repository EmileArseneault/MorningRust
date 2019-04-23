use std::env;
use std::fs;
use std::io;
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
    config_file:   Option<PathBuf>,
    config:        Option<ConfigurationContent>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigurationContent {
    command_file:   PathBuf,
    reminder_file:  PathBuf,
    history_file:   PathBuf,
    history_length: u32
}

impl Configuration {

    pub fn new() -> Self {

        Configuration {
            executing_dir: Configuration::find_executing_dir(),
            current_dir:   Configuration::find_current_dir(),
            home_dir:      Configuration::find_home_dir(),
            portable:      Configuration::is_script_portable(),
            config_file:   None,
            config:        None
        }
    }

    fn is_script_portable() -> bool {

        let exd:           PathBuf = Configuration::find_executing_dir();
        let installed_dir: PathBuf = PathBuf::from(INSTALLED_SCRIPT);

        if exd == installed_dir {
            false
        } else {
            true
        }
    }

    fn find_executing_dir() -> PathBuf {

        let ret :PathBuf = match env::current_exe() {
            Ok(cwd) => cwd,
            Err(e)  => {
                println!("Error while getting executing directory : {}", e);
                PathBuf::new()
            },
        };

        match ret.canonicalize(){
            Ok(cwd) => {
                // Remove executable name from path
                match cwd.parent() {
                    Some(cwd) => PathBuf::from(cwd),
                    None => {
                        println!("Error while getting parent path");
                        println!("Maybe the executable is in root folder?");
                        PathBuf::new()
                    }
                }
            },
            Err(e)  => {
                println!("Error while canonising path : {}", e);
                PathBuf::new()
            },
        }
    }

    fn find_current_dir() -> PathBuf {

        let ret :PathBuf = match env::current_dir() {
            Ok(exd) => exd,
            Err(e) => {
                println!("Error while getting current directory : {}", e);
                PathBuf::new()
            },
        };

        match ret.canonicalize(){
            Ok(cwd) => cwd,
            Err(e)  => {
                println!("Error while canonising path : {}", e);
                PathBuf::new()
            },
        }
    }

    fn find_home_dir() -> PathBuf {

        match dirs::home_dir() {
            Some(home_dir) => home_dir,
            None           => {
                println!("Error while getting home directory");
                PathBuf::new()
            },
        }
    }

    pub fn initialize(&mut self) -> Result<(), io::Error>{

        let mut config_file: PathBuf;

        if self.portable {
            config_file = PathBuf::from(&self.executing_dir);
            config_file.push(PORTABLE_CONFIG);
        } else {
            config_file = PathBuf::from(INSTALLED_CONFIG);
        }

        println!("Current dir is : {}", self.current_dir.display());
        println!("Executing dir is : {}", self.executing_dir.display());
        println!("");
        println!("Config file is : {}", config_file.display());

        if !Path::exists(&config_file) {

            // Create new configuration
            self.generate_config();
            self.write_config(config_file.as_path())?;
            println!("No configuration found, creating one");

        } else {

            // Read existing configuration
            self.read_config(config_file.as_path())?;
        }

        Ok(())
    }

    fn read_config(&mut self, config_path: &Path) -> Result<(), io::Error>{
        let json_str = fs::read_to_string(config_path)?;
        let json_config = serde_json::from_str(&json_str);
        
        match json_config {
            Ok(config) => {
                self.config = config;
                Ok(())
            },
            Err(e) => {
                Err(io::Error::new(io::ErrorKind::Other, "Serde Error"))
            }
        }
    }

    fn write_config(&self, config_path: &Path) -> Result<(), io::Error> {

        let json_config = serde_json::to_string(&self.config)?;
        fs::write(config_path, json_config)?;

        Ok(())
    }

    fn generate_config(&mut self) {

        let mut command_file:  PathBuf;
        let mut reminder_file: PathBuf;
        let mut history_file:  PathBuf;

        if self.portable {
            command_file  = PathBuf::from(&self.executing_dir);
            reminder_file = PathBuf::from(&self.executing_dir);
            history_file  = PathBuf::from(&self.executing_dir);

            command_file.push(PORTABLE_COMMAND);
            reminder_file.push(PORTABLE_REMINDER);
            history_file.push(PORTABLE_HISTORY);

            self.config = Some(
                ConfigurationContent {
                    command_file:   command_file,
                    reminder_file:  reminder_file,
                    history_file:   history_file,
                    history_length: 15,
                }
            )
        } else {
            command_file  = PathBuf::from(&self.home_dir);
            reminder_file = PathBuf::from(&self.home_dir);
            history_file  = PathBuf::from(&self.home_dir);

            command_file.push(INSTALLED_COMMAND);
            reminder_file.push(INSTALLED_REMINDER);
            history_file.push(INSTALLED_HISTORY);

            self.config = Some(
                ConfigurationContent {
                    command_file:   command_file,
                    reminder_file:  reminder_file,
                    history_file:   history_file,
                    history_length: 15,
                }
            )
        }
    }

    pub fn is_portable(&self) -> bool {
        self.portable
    }

    pub fn get_history_len(&self) -> u32 {
        match &self.config {
            Some(config) => {
                return config.history_length;
            }
            None => {
                return 0;
            }
        }
    }
}
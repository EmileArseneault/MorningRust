use std::path::PathBuf;
use std::env;
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};
//use serde_json::{Result, Value};

extern crate dirs;
extern crate serde;
extern crate serde_json;

const INSTALLED_CONFIG: &str = "/etc/morning.conf";
const INSTALLED_COMMAND: &str = ".morning/command";
const INSTALLED_REMINDER: &str = ".morning/reminder";
const INSTALLED_HISTORY: &str = ".morning/history";

const PORTABLE_CONFIG: &str = "morning.conf";
const PORTABLE_COMMAND: &str = "command";
const PORTABLE_REMINDER: &str = "reminder";
const PORTABLE_HISTORY: &str = "history";


#[derive(Debug)]
pub struct Configuration {
    executing_dir: PathBuf,
    current_dir: PathBuf,
    home_dir: PathBuf,
    portable: bool,
    config_file: Option<PathBuf>,
    command_file: Option<PathBuf>,
    reminder_file: Option<PathBuf>,
    history_file: Option<PathBuf>,
    history_length: Option<u32>
}

// Example of configuration file
//  {
//      "command_file"   : "path/to/file",
//      "reminder_file"  : "path/to/file",
//      "history_file"   : "path/to/file",
//      "history_length" : 10
//  }

#[derive(Serialize, Deserialize)]
struct ConfigurationContent {
    command_file: PathBuf,
    reminder_file: PathBuf,
    history_file: PathBuf,
    history_length: u32
}

impl Configuration {

    pub fn new() -> Self {

        Configuration {
            executing_dir: Configuration::find_executing_dir(),
            current_dir: Configuration::find_current_dir(),
            home_dir: Configuration::find_home_dir(),
            portable: Configuration::installed_or_portable(),
            config_file: None,
            command_file: None,
            reminder_file: None,
            history_file: None,
            history_length: None
        }
    }

    fn installed_or_portable() -> bool {

        let exd: PathBuf = Configuration::find_executing_dir();
        let home_dir: PathBuf = Configuration::find_home_dir();

        if exd == home_dir {
            false
        } else {
            true
        }
    }

    fn find_executing_dir() -> PathBuf {

        let ret :PathBuf = match env::current_dir() {
            Ok(cwd) => cwd,
            Err(e)  => {
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

    fn find_current_dir() -> PathBuf {

        let ret :PathBuf = match env::current_exe() {
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

    pub fn initialize(&mut self) {

        let config_file;

        if self.portable {
            config_file = PORTABLE_CONFIG;
        } else {
            config_file = INSTALLED_CONFIG;
        }

        println!("Config file is : {}", config_file);

        if !Path::exists(&PathBuf::from(config_file)) {
            // Create new configuration
            if self.portable {
                self.config_file   = Some(PathBuf::from(PORTABLE_CONFIG));
                self.command_file  = Some(PathBuf::from(PORTABLE_COMMAND));
                self.reminder_file = Some(PathBuf::from(PORTABLE_REMINDER));
                self.history_file  = Some(PathBuf::from(PORTABLE_HISTORY));
            } else {
                let mut command_file  = PathBuf::from(&self.home_dir);
                let mut reminder_file = PathBuf::from(&self.home_dir);
                let mut history_file  = PathBuf::from(&self.home_dir);

                command_file.push(INSTALLED_COMMAND);
                reminder_file.push(INSTALLED_REMINDER);
                history_file.push(INSTALLED_HISTORY);

                self.config_file   = Some(PathBuf::from(INSTALLED_CONFIG));
                self.command_file  = Some(command_file);
                self.reminder_file = Some(reminder_file);
                self.history_file  = Some(history_file);
            }

            // Write paths to json configuration file
            let config_string = ConfigurationContent
            {
                command_file: match &self.config_file
                {
                    Some(path) => PathBuf::from(path),
                    None => PathBuf::from(""),
                },
                reminder_file: match &self.command_file
                {
                    Some(path) => PathBuf::from(path),
                    None => PathBuf::from(""),
                },
                history_file: match &self.reminder_file
                {
                    Some(path) => PathBuf::from(path),
                    None => PathBuf::from(""),
                },
                history_length: 10,
            };

            let json_config = serde_json::to_string(&config_string);
            //let mut file = fs::File::create();
            //fs::write(self.config_file, json_config);

        } else {
            self.config_file = Some(PathBuf::from(config_file));
            self.open_config();
        }


    }

    fn open_config(&mut self) {

        match &self.config_file {
            Some(path) => {

                let config_string = fs::read_to_string(path).expect("Could not read config file");
                // let config_string = r#"
                // {
                //     "command_file"   : "path/to/file",
                //     "reminder_file"  : "path/to/file",
                //     "history_file"   : "path/to/file",
                //     "history_length" : 10
                // }
                // "#;
                let config: ConfigurationContent = serde_json::from_str(&config_string).expect("Could not convert configuration file to JSON");

                println!("The history length is {}", config.history_length);
                println!("The command file is {}", config.command_file.display());
            },
            None => {
                println!("No config file");
            },
        };
        
    }

    pub fn is_portable(&self) -> bool {
        self.portable
    }
}
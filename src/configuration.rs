use std::path::PathBuf;
use std::env;

extern crate dirs;

#[derive(Debug)]
pub struct Configuration {
    
    executing_dir: PathBuf,
    current_dir: PathBuf,
    portable: bool,
    history_length: Option<u32>
}

impl Configuration {

    pub fn new() -> Self {

        Configuration {
            executing_dir: Configuration::find_executing_dir(),
            current_dir: Configuration::find_current_dir(),
            portable: Configuration::installed_or_portable(),
            history_length: None
        }
    }

    fn installed_or_portable() -> bool {

        let exd: PathBuf = Configuration::find_executing_dir();

        let home_dir :PathBuf = match dirs::home_dir() {
            Some(home_dir) => home_dir,
            None           => {
                println!("Error while getting home directory");
                PathBuf::new()
            },
        };

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

    //pub fn open_config(&self){

    //}

    pub fn is_portable(&self) -> bool{
        self.portable
    }
}
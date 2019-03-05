use std::path::PathBuf;
use std::env;

extern crate dirs;

fn main() {

    if is_portable(){
        println!("--    Morning is in portable mode    --");
    } else {
        println!("-- Morning is installed on the system --");
    }
    println!();


    print_help();
    println!();
    
    let cwd :PathBuf = find_current_dir();
    println!("Current directory : {}", cwd.display());

    let exd :PathBuf = find_executing_dir();
    println!("Executing directory : {}", exd.display());



    open_config();
}

fn is_portable() -> bool {

    let exd :PathBuf = find_executing_dir();

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

fn open_config(){

}

fn print_help(){
    println!();
    println!("Usage : morning [OPTION]...");
	println!("----------------------------");
	println!();
	println!("  -n                 Used to write a message to print to tomorrow.");
	println!("                       Can be used with a positive integer to");
	println!("                       report the message by the number of days.");
	println!("  -w                 Used as a shortcut to 'morning -n 3' to be used");
	println!("                       report the message by the number of days.");
	println!("  -p                 Display past messages by the number of days given.");
	println!("                       Messages have to be in history time interval.");
	println!("                       (config variable historytime >= days given)");
	println!("  -r                 Open reminders file in user's default editor.");
	println!("  -c                 Open commands file in user's default editor.");
	println!();
    println!("  -h,--help          Display this help and exit.");
    println!();
}
mod configuration;
use std::env;

extern crate getopts;

fn main() {

    let conf = configuration::Configuration::new();

    let mut nb_of_days = 0;
    let mut flag_w = false;
    let mut past_days = 0;

    let args: Vec<String> = env::args().collect();

    let mut argparser = getopts::Options::new();
    argparser.optflag("h", "help", "Display this help and exit");
    argparser.optflag("n", "next", "Used to write a message for the next day");
    argparser.optopt("r", "report", "Report message for number of days", "DAYS");
    argparser.optopt("p", "past", "Show past messages for number of days", "DAYS");
    argparser.optflag("c", "command", "Used to write commands to be executed each day");
    argparser.optflag("g", "greeting", "Used to write messages to be shown each day");
    argparser.optflag("t", "todo", "Used to manage to do list");

    let matches = match argparser.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_help(argparser);
        return;
    }

    // Check for only one of the option flags
    let option_array = [matches.opt_present("n"),
                        matches.opt_present("r"),
                        matches.opt_present("p"),
                        matches.opt_present("c"),
                        matches.opt_present("g"),
                        matches.opt_present("t")];
    let mut option_count = 0;

    for i in 0..option_array.len() {
        if option_array[i] == true {
            option_count += 1;
        }
        if option_count > 1 {
            print_help(argparser);
            return;
        }
    }

    // Check option used
    if matches.opt_present("n") {
        nb_of_days = 1;
    } else if matches.opt_present("r"){
        nb_of_days = match matches.opt_str("r"){
            Some(nb_days) => nb_days.parse::<u32>().unwrap(),
            None => 1,
        };
    }
    println!("Write message for {} days:", nb_of_days);
    


    if matches.opt_present("p") {
        
    }

    if matches.opt_present("g") {
        
    }

    if matches.opt_present("c") {
        
    }

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_help(argparser);
        return;
    };

    if conf.is_portable(){
        println!("--    Morning is in portable mode    --");
    } else {
        println!("-- Morning is installed on the system --");
    }
    println!();
    println!();
}

fn print_help(opts: getopts::Options) {
    let brief = format!("Usage: morning [options] ...");
    print!("{}", opts.usage(&brief));
}

// fn print_help(){
//     println!();
//     println!("Usage : morning [OPTION]...");
// 	println!("----------------------------");
// 	println!();
// 	println!("  -n                 Used to write a message to print to tomorrow.");
// 	println!("                       Can be used with a positive integer to");
// 	println!("                       report the message by the number of days.");
// 	println!("  -w                 Used as a shortcut to 'morning -n 3' to be used");
// 	println!("                       report the message by the number of days.");
// 	println!("  -p                 Display past messages by the number of days given.");
// 	println!("                       Messages have to be in history time interval.");
// 	println!("                       (config variable historytime >= days given)");
// 	println!("  -r                 Open reminders file in user's default editor.");
// 	println!("  -c                 Open commands file in user's default editor.");
// 	println!();
//     println!("  -h,--help          Display this help and exit.");
//     println!();
// }
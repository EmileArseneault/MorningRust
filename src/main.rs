mod configuration;

fn main() {

    let conf = configuration::Configuration::new();

    if conf.is_portable(){
        println!("--    Morning is in portable mode    --");
    } else {
        println!("-- Morning is installed on the system --");
    }
    println!();

    print_help();
    println!();
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
mod arguments;
mod configuration;
mod history;

use arguments::ArgParser;
use history::History;

fn main() {

    // Read configuration
    let mut conf = configuration::Configuration::new();
    match conf.initialize(){
        Ok(o)  => {
            println!("Config contains : {}", conf.get_history_len());
        },
        Err(e) => {
            println!("Error while loading config");
            println!("{}", e);
            return;
        }
    }

    if conf.is_portable(){
        println!("--    Morning is in portable mode    --");
    } else {
        println!("-- Morning is installed on the system --");
    }

    let mut history = History::new();
/*     let message = String::from("Something is there");
    let message2 = String::from("Other message");
    history.add_message_now(message);
    history.add_message_now(message2); */
    match history.load_history(){
        Ok(o) => {},
        Err(e) => {
            println!("Error while loading history");
            println!("{}", e);
        }
    };
    history.print_history();
    match conf.history_path(){
        Ok(path) => {
            match history.write_history(path.as_path()) {
                Ok(o) => {},
                Err(e) => {
                    println!("Error while writing history");
                    println!("{}", e);
                }
            }
        },
        Err(e) => {
            println!("No history file in configuration");
        }
    }

    let argparser = ArgParser::new();

    // Parse command line arguments
    match argparser.parser() {
        Ok(action) => {
            match action {
                arguments::Action::Help => {
                    argparser.print_help();
                    return;
                },
                arguments::Action::Message(nb_of_days) => {
                    println!("Message for {} days", nb_of_days);
                },
                arguments::Action::Past(nb_of_days) => {
                    println!("Show past message of {} days", nb_of_days);
                },
                arguments::Action::Command => {
                    println!("Open command file for editing");
                },
                arguments::Action::Reminder => {
                    println!("Open reminder file for editing");
                },
            }
        },
        Err(stuff) => {
            argparser.print_help();
            return;
        }
    }

    println!();
    println!();
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
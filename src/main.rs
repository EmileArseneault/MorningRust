mod arguments;
mod configuration;
mod history;
mod editing;

use arguments::ArgParser;
use configuration::Configuration;
use history::History;

fn main() {

    // Read configuration
    let mut conf =  match Configuration::new() {
        Ok(conf)  => {
            conf
        },
        Err(e) => {
            println!("Error while loading config");
            println!("{}", e);
            return;
        }
    };

    // Print portable header if it is
    if conf.is_portable(){
        println!("----- Morning is in portable mode -----");
    }

    // Load history
    let mut history = History::new();

    match history.load_history(conf.history_path().as_path()) {
        Ok(_) => {},
        Err(e) => {
            println!("Error while loading history");
            println!("{}", e);
            return;
        }
    };

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

                    match history.add_delayed_message(nb_of_days) {
                        Ok(_) => {
                            println!("Message added");
                        },
                        Err(_) => {
                            println!("Could not get message");
                        }
                    }
                },
                arguments::Action::Past(nb_of_days) => {
                    println!("Show past message of {} days", nb_of_days);

                    let message = history.find_message_by_nb_day(nb_of_days * -1);
                    match message {
                        Some(message_string) => {
                            println!("{}", message_string);
                        },
                        None => {
                            println!("No message for this day");
                        }
                    }
                },
                arguments::Action::Command => {
                    editing::edit_file(conf.command_file());
                },
                arguments::Action::Reminder => {
                    editing::edit_file(conf.reminder_file());
                },
                arguments::Action::Morning => {
                    println!("Good Morning !");

                    let message = history.find_message_by_nb_day(0);
                    match message {
                        Some(message_string) => {
                            println!("{}", message_string);
                        },
                        None => {
                            println!("No message for today");
                        }
                    }
                },
            }
        },
        Err(_) => {
            argparser.print_help();
            return;
        }
    }

    // Write history
    match history.write_history(&conf) {
        Ok(_) => {},
        Err(e) => {
            println!("Error while writing history");
            println!("{}", e);
        }
    }

    // Write configuration if changed
    match conf.write_config() {
        Ok(_) => {},
        Err(e) => {
            println!("Error while writing configuration");
            println!("{}", e);
        }
    }
}

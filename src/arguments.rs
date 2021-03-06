use std::env;

extern crate getopts;

#[derive(Debug)]
pub enum Action {
    Help,
    Morning,
    Message(i64),
    Past(i64),
    Command,
    Reminder,
}

pub struct ArgParser{
    help_string: String,
    arg_parser: getopts::Options,
}

impl ArgParser{
    pub fn new() -> ArgParser {

        let mut argparser = getopts::Options::new();
        argparser.optflag("h", "help", "Display this help and exit");
        argparser.optflagopt("n", "next", "Used to write a message for the next day", "DAYS");
        argparser.optflagopt("p", "past", "Show past messages for number of days", "DAYS");
        argparser.optflag("c", "command", "Used to write commands to be executed everyday");
        argparser.optflag("r", "reminder", "Used to write messages to be shown each everyday");

        let brief = format!("Usage: morning [options] ...");

        ArgParser {
            help_string: String::from(argparser.usage(&brief)),
            arg_parser: argparser,
        }
    }

    pub fn print_help(&self) {
        print!("{}", self.help_string);
    }

    pub fn parser(&self) -> Result<Action, getopts::Fail> {

        let args: Vec<String> = env::args().collect();
        let mut action_to_take: Action = Action::Help;

        let matches = match self.arg_parser.parse(&args[1..]) {
            Ok(m) => { m }
            Err(f) => { return Err(f) }
        };

        // If help is demanded print and exit
        if matches.opt_present("h") {
            return Ok(Action::Help);
        }

        // Check for only one of the option flags
        let option_array = [matches.opt_present("n"),
                            matches.opt_present("r"),
                            matches.opt_present("p"),
                            matches.opt_present("c")];
        let mut option_count = 0;

        for i in 0..option_array.len() {
            if option_array[i] == true {
                option_count += 1;
            }
            if option_count > 1 {
                return Ok(Action::Help);
            }
        }

        // If no options were provided, use default behavior
        if option_count == 0 {
            action_to_take = Action::Morning;
        }

        let mut nb_of_days;

        // Check option used
        if matches.opt_present("n") {
            nb_of_days = match matches.opt_str("n"){
                // Unwrap panics if not an i64 so should match for error use ?
                Some(nb_days) => nb_days.parse::<i64>().unwrap(),
                None => 1, 
            };

            action_to_take = Action::Message(nb_of_days);
        }

        if matches.opt_present("p") {
            nb_of_days = match matches.opt_str("p"){
                // Unwrap panics if not an i64 so should match for error use ?
                Some(nb_days) => nb_days.parse::<i64>().unwrap(),
                None => 1, 
            };

            action_to_take = Action::Past(nb_of_days);
        }

        if matches.opt_present("r") {
            action_to_take = Action::Reminder;
        }

        if matches.opt_present("c") {
            action_to_take = Action::Command;
        }

        // Checks if other arguments were not parsed
        if !matches.free.is_empty() {
            println!("Extra arguments were found : {}", matches.free[0].clone());
            action_to_take = Action::Help;
        }

        return Ok(action_to_take);
    }
}

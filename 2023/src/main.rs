mod days;
mod registry;
mod utils;

use clap::{Arg, Command};

const PART: &str = "part";
const FILE: &str = "file";

fn init_app() -> Command {
    let mut app = Command::new("aoc")
        .author("matty-rose <matthewrose153@gmail.com>")
        .version("0.1.0")
        .about("CLI to run Advent of Code 2023 solutions")
        .arg_required_else_help(true);

    for sub in inventory::iter::<registry::DayCommand> {
        app = app.subcommand(
            Command::new(sub.name)
                .arg(
                    Arg::new(PART)
                        .short('p')
                        .required(true)
                        .value_parser(["1", "2"]),
                )
                .arg(Arg::new(FILE).short('f').required(false)),
        )
    }

    app
}

fn main() {
    let app: Command = init_app();
    let matches = app.get_matches();

    // Unwrap of part is ok because its required
    for sub in inventory::iter::<registry::DayCommand> {
        if let Some(matched_sub) = matches.subcommand_matches(sub.name) {
            let part = matched_sub.get_one::<String>(PART).expect("required");
            let file = matched_sub.get_one::<String>(FILE);
            println!("executing {}", &sub.name);
            let _ = &sub.execute(part.as_str(), file.map(|x| x.as_str()));
        }
    }
}

#![feature(iter_next_chunk)]
mod days;
mod registry;
mod utils;

use clap::{App, AppSettings, Arg, SubCommand};

fn init_app() -> App<'static, 'static> {
    let mut app = App::new("aoc")
        .author("matty-rose <matthewrose153@gmail.com>")
        .version("0.1.0")
        .about("CLI to run 2022 advent of code solutions")
        .setting(AppSettings::ArgRequiredElseHelp);

    for sub in inventory::iter::<registry::DayCommand> {
        app = app.subcommand(
            SubCommand::with_name(sub.name)
                .arg(
                    Arg::with_name("part")
                        .short("p")
                        .required(true)
                        .takes_value(true)
                        .possible_values(&["1", "2"]),
                )
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .required(false)
                        .takes_value(true),
                ),
        )
    }

    app
}

fn main() {
    let app: App = init_app();
    let matches = app.get_matches();

    // Unwrap of part is ok because its required
    for sub in inventory::iter::<registry::DayCommand> {
        if let Some(matched_sub) = matches.subcommand_matches(&sub.name) {
            let part = matched_sub.value_of("part").unwrap();
            let file = matched_sub.value_of("file");
            println!("executing {}", &sub.name);
            let _ = &sub.execute(part, file);
        }
    }
}

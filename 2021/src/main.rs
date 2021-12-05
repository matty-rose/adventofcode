mod days;
mod registry;

use clap::{App, AppSettings, SubCommand};

fn init_app() -> App<'static, 'static> {
    let mut app = App::new("aoc")
        .author("matty-rose <matthewrose153@gmail.com>")
        .version("0.1.0")
        .about("CLI to run 2021 advent of code solutions")
        .setting(AppSettings::ArgRequiredElseHelp);

    for sub in inventory::iter::<registry::DayCommand> {
        app = app.subcommand(SubCommand::with_name(&sub.name))
    }

    app
}

fn main() {
    let app: App = init_app();
    let matches = app.get_matches();

    for sub in inventory::iter::<registry::DayCommand> {
        if let Some(_matches) = matches.subcommand_matches(&sub.name) {
            println!("executing {}", &sub.name);
            let executor = &sub.func.func;
            executor();
        }
    }
}

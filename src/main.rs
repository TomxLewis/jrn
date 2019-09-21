extern crate clap;
use clap::{Arg, ArgMatches, App, SubCommand, AppSettings};
use std::io::Write;

use jrn::*;

fn clap_app<'a, 'b>() -> App<'a, 'b> {
    App::new("jrn")
        .version("0.1.0")
        .author("Tom Lewis <tomxlewis@gmail.com")
        .about("Command Line journaling System that Integrates with git for version control.")
        .subcommand(SubCommand::with_name("new")
            .about("create a new jrn entry")
            .arg(Arg::from_usage("-f --from [TEXT] 'the new entries contents'")
                .long_help("creates a new entry with TEXT and the default tags provided by the devices config")
                .takes_value(true))
            .arg(Arg::from_usage("-t, --tags [TAGS] 'tags in the new entry'")
                .takes_value(true)
                .multiple(true)))
        .subcommand((SubCommand::with_name("tags")))
        .subcommand((SubCommand::with_name("list")))
        .subcommand(SubCommand::with_name("config")
            .about("Alters or inquires the current jrn configuration")
            .arg(Arg::with_name("list")
                .help("lists all config options and their values")
                .short("l")
                .long("list")))
}

fn main() {
    //init
    let cfg = Config::find_or_default();
    let mut repo = JrnRepo::init(cfg).expect("Failure init repo");

    //process command line args
    let matches = clap_app().get_matches();

    match matches.subcommand() {
        ("new", Some(args)) => new(args, &mut repo),
        _ => {
            #[cfg(debug_assertions)]
            dbg!(&matches);

            //write out help message lazily if needed
            #[cfg(not(debug_assertions))]
            clap_app().print_help();
        }
    }
}

fn new(args: &ArgMatches, repo: &mut JrnRepo) {
    //text to put in new entry if any
    let text: Option<&str> = args.value_of("from");
    dbg!(&text);

    //tags passed as args to the program
    let tags: Option<Vec<String>> = args.values_of_lossy("tags");
    dbg!(&tags);
}

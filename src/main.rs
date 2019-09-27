extern crate clap;
use clap::{Arg, ArgMatches, App, AppSettings, SubCommand};

use jrn::*;
use simplelog::{SimpleLogger, Config};
use log::LevelFilter;
use std::str::FromStr;

fn clap_app<'a, 'b>() -> App<'a, 'b> {
    App::new("jrn")
        .version("0.1.0")
        .setting(AppSettings::VersionlessSubcommands)
        .author("Tom Lewis <tomxlewis@gmail.com")
        .about("Command Line journaling System that Integrates with git for version control.")
        //DONE
        .subcommand(SubCommand::with_name("list")
            .arg(Arg::from_usage("[FILTER] 'List entries where filename contains FILTER'")
                .default_value(".*")
                .takes_value(true))
            .arg(Arg::from_usage("-n [NUM] 'Limit output to most recent NUM of matching entries")))
        .subcommand(SubCommand::with_name("new")
            .about("Create a new jrn entry")
            //TODO remove optional arg make required, but default to no TAGS
            .arg(Arg::from_usage("-t, --tags [TAGS] 'Tags in the new entry'")
                .takes_value(true)
                .multiple(true))
            .arg(Arg::from_usage("-q --quick 'Don't open editor, just create entry'"))
            .arg(Arg::from_usage("-n --note [TEXT] 'The new entries contents'")
                .long_help("creates a new entry with TEXT and the default tags provided by the devices config")
                .takes_value(true)))
        //TODO
        .arg(Arg::from_usage("-c --config [OPTION]=[VALUE]\"\" 'Set a configuration parameter for this run only'")
            //TODO implement parsing config option=value pairs
            .long_help("See mod jrn::jrn_lib::config::settings for fields and values")
            //TODO document all config options
            .multiple(true)
            .number_of_values(2))
        .subcommand(SubCommand::with_name("tags"))
            //TODO define sub-command "tags"
        .subcommand(SubCommand::with_name("config")
            //TODO implement sub-command "config"
            .about("Alters or inquires the current jrn configuration")
            .arg(Arg::with_name("list")
                .help("Lists all config options and their values")
                .short("l")
                .long("list")))
}

fn main() {
    //choose logging impl
    SimpleLogger::init(LevelFilter::Warn, Config::default()).unwrap();

    //TODO pass any config args to cfg object
    let cfg = Settings::find_or_default().expect("Configuration Parsing Error");
    let ignore = IgnorePatterns::find_or_default();
    let mut repo = JrnRepo::init(cfg, ignore).expect("Failure init repo");

    //process command line args
    let matches = clap_app().get_matches();
    match matches.subcommand() {
        ("new", Some(args)) => new(args, &mut repo),
        ("list", Some(args)) => list(args, &mut repo),
        _ => {
            clap_app().print_help().unwrap();
            println!();
        }
    }
}

fn new(args: &ArgMatches, repo: &mut JrnRepo) {
    //text to put in new entry if any
    let text: Option<&str> = args.value_of("from");
    //tags passed as args to the program
    let tags: Option<Vec<String>> = args.values_of_lossy("tags");
    //should open editor?
    let open_editor: bool = !args.is_present("quick");

    repo.create_entry(tags, text, open_editor).expect("Failed to write entry");
}

fn list(args: &ArgMatches, repo: &mut JrnRepo) {
    let filter: &str = args.value_of("FILTER").unwrap();
    let num: Option<usize> = args.value_of("n").map(|s| usize::from_str(s).unwrap());

    repo.list_entries(filter, num).unwrap()
}

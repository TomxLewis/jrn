extern crate clap;
use clap::{Arg, ArgMatches, App, SubCommand};

use jrn::*;

fn clap_app<'a, 'b>() -> App<'a, 'b> {
    App::new("jrn")
        .version("0.1.0")
        .author("Tom Lewis <tomxlewis@gmail.com")
        .about("Command Line journaling System that Integrates with git for version control.")
        .arg(Arg::from_usage("-c --config [OPTION]=[VALUE]\"\" 'Set a configuration parameter for this run only'")
            //TODO implement parsing config option=value pairs
            .long_help("See mod jrn::jrn_lib::config::settings for fields and values")
            //TODO document all config options
            .multiple(true)
            .number_of_values(2))
        .subcommand(SubCommand::with_name("new")
            //TODO implement sub-command "new"
            .about("create a new jrn entry")
            .arg(Arg::from_usage("-q --quick 'Don't open editor, just create entry'"))
            .arg(Arg::from_usage("-n --note [TEXT] 'the new entries contents'")
                .long_help("creates a new entry with TEXT and the default tags provided by the devices config")
                .takes_value(true))
            .arg(Arg::from_usage("-t, --tags [TAGS] 'tags in the new entry'")
                .takes_value(true)
                .multiple(true)))
        .subcommand(SubCommand::with_name("tags"))
            //TODO define sub-command "tags"
        .subcommand(SubCommand::with_name("list"))
            //TODO implement sub-command "list"
        .subcommand(SubCommand::with_name("config")
            //TODO implement sub-command "config"
            .about("Alters or inquires the current jrn configuration")
            .arg(Arg::with_name("list")
                .help("lists all config options and their values")
                .short("l")
                .long("list")))
}

fn main() {
    //init let cfg = Settings::find_or_default().expect("Configuration Parsing Error");
    //TODO pass any config args to cfg object

    let mut repo = JrnRepo::init(cfg).expect("Failure init repo");

    //process command line args
    let matches = clap_app().get_matches();

    match matches.subcommand() {
        ("new", Some(args)) => new(args, &mut repo),
        _ => {
            #[cfg(debug_assertions)]
            dbg!(&matches);

            //write out help message lazily if needed
            //#[cfg(not(debug_assertions))]
            clap_app().print_help().unwrap();
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

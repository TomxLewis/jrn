extern crate clap;
use clap::{Arg, ArgMatches, App, AppSettings, SubCommand};

use jrn::*;
use simplelog::{SimpleLogger, Config};
use log::LevelFilter;
use std::str::FromStr;

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
        ("new", Some(args)) => handle_new(args, &mut repo),
        ("list", Some(args)) => list_entries(args, &mut repo),
        ("log", _) => log(&repo),
        ("tags", Some(args)) => handle_tags(args, &mut repo),
        _ => {
            clap_app().print_help().unwrap();
            println!();
        }
    }
}

fn clap_app<'a, 'b>() -> App<'a, 'b> {
    App::new("jrn")
        .version("0.1.0")
        .setting(AppSettings::VersionlessSubcommands)
        .author("Tom Lewis <tomxlewis@gmail.com")
        .about("Command Line journaling System that Integrates with git for version control.")
        .subcommand(SubCommand::with_name("list").about("List jrn entries")
            .arg(Arg::from_usage("[FILTER] 'List entries where filename contains FILTER'").default_value(".*"))
            .arg(Arg::from_usage("-n [NUM] 'Limit output to most recent NUM of matching entries"))
            .subcommand(SubCommand::with_name("tags").about("Lists all tags"))
            )
        .subcommand(SubCommand::with_name("new").about("Create a new jrn entry")
            .arg(Arg::from_usage("[TAGS] 'Tags in the new entry'")
                .required(false)
                .takes_value(true)
                .multiple(true)
                )
            .arg(Arg::from_usage("-q --quick 'Don't open editor, just create entry'"))
            .arg(Arg::from_usage("-n --note [TEXT] 'The new entries contents'"))
            )
        .subcommand(SubCommand::with_name("log").about("Logs the most recent entries to stdout"))
        .subcommand(SubCommand::with_name("entries")
            //TODO implement entries subcommand
            .arg(Arg::from_usage("[FILTER] 'Operations will apply to entries that match the FILTER'")
                .long_help("Asks for confirmation on modifying multiple entries, \
                this behavior can be skipped by passing the -f or --force option")))
        .subcommand(SubCommand::with_name("tags").about("Modify and remove tags in the working jrn repository")
            //TODO implement tags subcommand
            .arg(Arg::from_usage("[FILTER] 'Operations will apply to TAGS that match the filter'")
                .long_help("Asks for confirmation on modifying multiple entries, \
                this behavior can be skipped by passing the -f or --force option"))
            .arg(Arg::from_usage("-d --delete 'Deletes selected tag from all entries'"))
            .arg(Arg::from_usage("-r --rename [TEXT] 'Renames selected tag'"))
            .subcommand(SubCommand::with_name("list").about("Lists all tags, and the number of times they appear"))
            )
        //TODO
        .arg(Arg::from_usage("-c --config [OPTION]=[VALUE] 'Set a configuration parameter for this run only'")
            //TODO implement parsing config option=value pairs
            .long_help("See mod jrn::jrn_lib::config::settings for fields and values")
            .multiple(true)
            .number_of_values(2)
            )
        .subcommand(SubCommand::with_name("config")
            //TODO document all config options
            .about("Alters or inquires the current jrn configuration")
            .arg(Arg::with_name("list")
                .help("Lists all config options and their values")
                .short("l")
                .long("list")))
}


fn handle_new(args: &ArgMatches, repo: &mut JrnRepo) {
    //text to put in new entry if any
    let text: Option<&str> = args.value_of("from");
    //tags passed as args to the program
    let tags: Option<Vec<String>> = args.values_of_lossy("TAGS");
    //should open editor?
    let open_editor: bool = !args.is_present("quick");

    repo.create_entry(tags, text, open_editor).expect("Failed to write entry");
}

fn handle_tags(args: &ArgMatches, repo: &mut JrnRepo) {
    dbg!(args);
    //TODO handle push tag
    repo.list_tags();
}

fn list_entries(args: &ArgMatches, repo: &mut JrnRepo) {
    let filter: &str = args.value_of("FILTER").unwrap();
    let num: Option<usize> = args.value_of("n").map(|s| usize::from_str(s).unwrap());
    repo.list_entries(filter, num).unwrap();
}

/// Display the most recent entries to stdout
/// shortcut for jrn list -n=5
fn log(repo: &JrnRepo) {
    let filter = ".*";
    let num = Some(5);
    repo.list_entries(filter, num).unwrap();
}

/// Pushes TAG to last NUM of entries
fn tag_push(tag: &str, num: usize, repo: &mut JrnRepo) {
    repo.push_tag(tag, num);
}


struct NewCommand {
    // FLAGS
    // -----
    // -q --quick
    skip_opening_editor: bool,

    // Positional Arguments
    // --------------------
    // [TAGS] 'Tags in the new entry, defaults to the just the tags in the system and local configs'
    tags: Vec<String>,

    // Optional Arguments
    // ------------------
    // -l --location [LOCATION] 'Location the new entry was created, defaults to '
    location: String,

}

#[cfg(test)]
mod test {
    use assert_cmd::crate_name;
    use assert_cmd::prelude::*;
    use std::process::Command;
    use std::fs::DirEntry;

    fn bin() -> Command {
        Command::cargo_bin(crate_name!()).unwrap()
    }

    #[test]
    fn new_with_tags() {
        let mut cmd = bin();
        cmd.args(&["new", "-q", "Test", "One"]);
        let assert = cmd.assert();
        assert.success();
        let paths: Vec<DirEntry> = std::fs::read_dir(".").unwrap().map(|p| p.unwrap()).collect();
        let file: Option<&DirEntry> = paths.iter().find(|p| p.file_name().to_str().unwrap().contains("Test_One"));
        if let Some(file) = file {
            std::fs::remove_file(file.path()).unwrap();
        }
    }
}

extern crate clap;
use clap::{Arg, App, SubCommand, AppSettings};
use std::io::Write;

use jrn::*;

fn clap_app<'a, 'b>() -> App<'a, 'b> {
    App::new("jrn")
        .version("0.1.0")
        .author("Tom Lewis <tomxlewis@gmail.com")
        .about("Command Line journaling System that Integrates with git for version control.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("tag")
            .help("creates a new entry with TAGS")
            //.required(true)
            .multiple(true)
            .value_name("TAGS")
            .conflicts_with_all(&["entry", "view"]))
        .arg(Arg::with_name("entry")
            .help("creates a new entry with TEXT")
            .long_help("creates a new entry with TEXT and the default tags provided by the devices config")
            .short("e")
            .long("entry")
            .takes_value(true)
            .value_name("TEXT")
            .conflicts_with_all(&["tag", "view"]))
        .arg(Arg::with_name("list")
            .help("concatenates last NUM of entries to stdout")
            .short("l")
            .long("list")
            .takes_value(true)
            .value_name("N")
            .conflicts_with_all(&["tag", "entry"]))
        .subcommand(SubCommand::with_name("config")
            .about("Alters jrn configuration")
            .arg(Arg::with_name("file")
                .value_name("FILE")
                .takes_value(true)))
}

fn main() {
    let app = clap_app();
    let cfg = Config::find_or_default();
    let repo = JrnRepo::init(&cfg);


    let matches = app.get_matches();
}

#[cfg(test)]
mod test {

}
use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let matches = App::new("jrn")
        .author("Tom Lewis")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("new")
                    .arg(Arg::with_name("from")
                         .short("f")
                         .long("from")
                         .takes_value(true)
                         .value_name("TEXT"))
                    .arg(Arg::with_name("tags")
                         .short("t")
                         .long("tags")
                         .takes_value(true)
                         .multiple(true)
                         .value_name("TAGS")))
        .get_matches();
}

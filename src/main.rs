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

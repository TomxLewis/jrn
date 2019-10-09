extern crate clap;
extern crate structopt;
use clap::AppSettings;
use structopt::StructOpt;

use jrn::*;

fn main() {
    let cfg = Settings::find_or_default().expect("Configuration Parsing Error");
    let ignore = IgnorePatterns::find_or_default();
    let mut repo = JrnRepo::init(cfg, ignore).expect("Failure init repo");

    let jrn = Jrn::from_args();

    match jrn {
        Jrn::New {skip_opening_editor, location, tags, } => { 
            repo.create_entry(tags, location, skip_opening_editor).expect("Failure creating entry");
        },
        Jrn::List { pattern, n, } => {
            repo.list_entries(pattern.as_ref(), n).expect("Error listing entries");
        }
        _ => { println!("No subcommand given") },
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    setting(AppSettings::VersionlessSubcommands),
    setting(AppSettings::DisableHelpFlags),
    setting(AppSettings::DisableVersion),
)]
/// the stupid journaling system
/// 
/// command line journaling that integrates with git for version control
enum Jrn {
    #[structopt(setting(AppSettings::DisableHelpFlags))]
    /// Craft a new entry
    /// 
    /// The default behavior of this subcommand is to open the JRN_EDITOR with a blank entry.
    /// 
    /// However if an entry already exists it will be opened for editing.
    New {
        #[structopt(short = "q", long = "quick")]
        /// Don't open the editor, just create the entry
        skip_opening_editor: bool,

        #[structopt(
            short,
            long,
            env = "JRN_LOCATION",
        )]
        /// Location the new entry was created
        location: Option<String>,

        #[structopt(short, long)]
        /// Any tags to associate with the new entry
        tags: Option<Vec<String>>,
    },

    #[structopt(setting(AppSettings::DisableHelpFlags))]
    /// List entries
    List {
        #[structopt(default_value = ".*")]
        /// Only list entries whose filename contains the given pattern
        pattern: String,

        #[structopt(short)]
        /// Limit output to most recent n matched entries
        n: Option<usize>,
    },

    #[structopt(setting(AppSettings::DisableHelpFlags))]
    /// Modifies tags in the working jrn repository
    Tags {
        /// Filter to match tags against
        /// 
        /// All operations will only apply to tags that match the filter
        /// Confirmation will be asked for before modifying multiple entries
        pattern: String,

        #[structopt(short)]
        /// Display all tags and the number of times they appear
        list: bool,

        #[structopt(short, long)]
        /// Delete selected tags from all entries
        delete: bool,

        #[structopt(long)]
        /// Rename the selected tag to new_name
        new_name: Option<String>,
    },

    #[structopt(setting(AppSettings::DisableHelpFlags))]
    /// Alters or inquires the working configuration
    ///
    /// The configuration is pulled from all available git configurations
    /// and any system or local .jrnconfig files
    ///
    /// TODO implement
    Config {
        #[structopt(short, long)]
        /// Lists the mapping of all relevant configuration options to their values
        ///
        /// Relevant git config options will be displayed separate from application config options
        list: bool,
    }
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

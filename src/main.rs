extern crate clap;
extern crate structopt;
use clap::AppSettings;
use structopt::StructOpt;

use jrn::*;

fn main() {
    let cfg = Settings::find_or_default().expect("Configuration Parsing Error");
    let ignore = IgnorePatterns::find_or_default();
    let repo = JrnRepo::init(cfg, ignore).expect("Failure init repo");
    Jrn::build_app().match_on_subcommand(repo);
}

#[derive(Debug, StructOpt)]
#[structopt(
    setting(AppSettings::VersionlessSubcommands),
    setting(AppSettings::DisableVersion),
    setting(AppSettings::SubcommandRequiredElseHelp),
)]
/// the stupid journaling system
/// 
/// command line journaling that integrates with git for version control
enum Jrn {
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
        ///
        /// The location can be pulled from the command line, the environment or the configuration
        /// A locally given location e.g. from the command line will override the environment
        /// or configuration set locations.
        location: Option<String>,

        #[structopt(short, long)]
        /// Any tags to associate with the new entry
        tags: Option<Vec<String>>,
    },

    /// List entries
    List {
        #[structopt(default_value = ".*")]
        /// Only list entries whose filename contains the given pattern
        pattern: String,

        #[structopt(short)]
        /// Limit output to most recent n matched entries
        n: Option<usize>,
    },

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

impl Jrn {
    /// app builder in which to change any configuration settings
    /// not possible to change through structopt
    fn build_app() -> Self {
        let clap_app = Jrn::clap().global_setting(AppSettings::DisableHelpFlags);
        Jrn::from_clap(&clap_app.get_matches())
    }

    fn match_on_subcommand(self, mut repo: JrnRepo) {
        match self {
            Self::New {skip_opening_editor, location, tags, } => {
                repo.create_entry(tags, location, skip_opening_editor).expect("Failure creating entry");
            },
            Self::List { pattern, n, } => {
                repo.list_entries(pattern.as_ref(), n).expect("Error listing entries");
            },
            Self::Tags { pattern: _, list: _, delete: _, new_name: _ } => {
                //TODO
            },
            Self::Config { list: _} => {
                //TODO
            },
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn new() {
    }


}

extern crate clap;
extern crate structopt;
use clap::AppSettings;
use structopt::StructOpt;

use jrn::*;

fn main() {
    let cfg = Settings::find_or_default();
    let ignore = IgnorePatterns::find_or_default();
    let repo = JrnRepo::init(cfg, ignore).expect("Failure init repo");
    Jrn::build_app().match_on_subcommand(repo);
}

#[derive(Debug, StructOpt)]
/// the stupid journaling system
/// 
/// command line journaling that integrates with git for version control
enum Jrn {
    /// Craft a new entry
    /// 
    /// The default behavior of this subcommand is to open the JRN_EDITOR with a blank entry.
    /// If an entry already exists at the current time and location it will be opened
    New {
        #[structopt(short = "q", long = "quick")]
        /// Don't open the editor, just create the entry
        skip_opening_editor: bool,

        #[structopt(short, long, env = "JRN_LOCATION")]
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

    /// Pushes a tag to the last opened entry
    PushTag {
        /// The tag to be pushed
        tag: String,
        /// An identifier of the entry to push to.
        /// Defaults to the last entered entry.
        entry_descriptor: Option<String>,
    },

    /// Modifies tags in the working jrn repository
    ///
    /// TODO specify tags subcommand
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
    /// TODO specify config subcommand
    Config {
        #[structopt(short, long)]
        /// Lists the mapping of all relevant configuration options to their values
        ///
        /// Relevant git config options will be displayed separate from application config options
        list: bool,
    }
}

impl Jrn {
    // app builder in which to change apply any [clap::AppSettings]
    // using this pattern allows a shorter structopt derive
    fn build_app() -> Self {
        let clap_app = Jrn::clap()
            .setting(AppSettings::VersionlessSubcommands)
            .setting(AppSettings::DisableVersion);
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
            Self::PushTag { tag, entry_descriptor } => {
                //TODO use entry_descriptor
                repo.push_tag(&tag, Some(1));
            },
            Self::Tags { .. } => {
                //TODO implement tags subcommand
            },
            Self::Config { .. } => {
                //TODO implement config subcommand
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

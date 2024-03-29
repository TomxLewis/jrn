extern crate clap;
extern crate structopt;
use clap::AppSettings;
use structopt::StructOpt;

use jrn::*;
use simplelog::SimpleLogger;
use log::LevelFilter;

fn main() {
    SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()).unwrap();

    let cfg = Settings::find_or_default();
    log::trace!("configuration successfully loaded");

    let ignore = IgnorePatterns::find_or_default();
    log::trace!("ignored patterns successfully loaded");

    let repo = JrnRepo::init(cfg, ignore).expect("Failure init repo");
    log::trace!("Opening repository at {:?}", &repo.root_path);

    Jrn::build_app()
        .start_loop(repo);
}

#[derive(Debug, StructOpt)]
/// the stupid journal system
///
/// command line journal that integrates with git for version control
enum Jrn {
    /// Craft a new entry
    ///
    /// Open the JRN_EDITOR with a blank entry.
    /// If an entry already exists at the current time and location it will be opened.
    New {
        #[structopt(short = "q", long = "quick")]
        /// Don't open the editor, just create the entry
        skip_edit: bool,

        #[structopt(short, long, env = "JRN_LOCATION")]
        /// Location the new entry was created
        ///
        /// The location can be pulled from the command line, or the environment
        /// The command line will override the environment configs
        ///
        /// If no location is found location will be recorded as 'None'
        location: Option<String>,
        //TODO implement location handling

        /// Any tags to associate with the new entry
        tags: Vec<String>,
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

    #[structopt(alias = "pt")]
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
    /// TODO specify tags command
    Tags {
        #[structopt(default_value = ".*")]
        /// Filter to match tags against
        ///
        /// All operations will only apply to tags that match the filter
        /// Confirmation will be asked for before modifying multiple entries
        pattern: String,

        #[structopt(short, long)]
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
    /// and any yet to be defined configuration
    ///
    /// TODO impl config command
    Config {
        #[structopt(short, long)]
        /// Lists the mapping of all relevant configuration options to their values
        ///
        /// Relevant git config options will be displayed separate from application config options
        list: bool,
    },

    #[structopt(alias = "rm")]
    /// Remove entries or tags
    Remove {
        /// The hash of the entry object to be removed
        ///
        /// if given the literal 'HEAD' will delete only the most recent entry
        entry_hash: Option<String>
    },
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

    fn start_loop(self, repo: JrnRepo) {
        self.match_on_command(repo).expect("Debug");
    }

    fn match_on_command(self, mut repo: JrnRepo) -> Result<(), JrnError>{
        use self::Jrn::*;
        match self {
            New { skip_edit, location, tags } => {
                repo.create_entry(tags, location, skip_edit)?;
            }
            List { pattern, n } => {
                repo.list_entries(pattern.as_ref(), n)?;
            }
            PushTag { tag, entry_descriptor} => {
                repo.push_tag(&tag, entry_descriptor);
            }
            Tags { pattern, list, delete, new_name } => {
                if list {
                    repo.list_tags(&pattern)?;
                }
                //TODO implement tags command
            }
            Config { .. } => {
                //TODO implement config command
            }
            Remove { entry_hash } => {
                match entry_hash {
                    Some(s) => { 
                        if &s == "HEAD" {
                            repo.remove_latest()?;
                        } else {
                            log::info!("TODO impl remove hash");
                            log::info!("Found hash {}", &s);
                        }
                    },
                    None => { 
                        log::info!("TODO display jrn-remove help");
                    }
                }
            }
        }
        Ok(())
    }
}

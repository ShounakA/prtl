use clap::{Parser, arg, Subcommand};

pub const DEFAULT_PRTL_TAG: &str = "default_prtl";
pub const CONFIG_APP_NAME: &str = ".prtl";

/// prtl arguments
/// Usage: prtl <COMMAND>
///
/// Commands:
///   set   
///   get   
///   help  Print this message or the help of the given subcommand(s)
///
/// Options:
///   -h, --help     Print help
///   -V, --version  Print version
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct PortalArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {

    Set {
        /// path to set as last directory.
        path: String,
        
        /// give the directory a tag.
        #[arg(short, default_value = DEFAULT_PRTL_TAG)]
        tag: String,
    },
    
    Get {
        /// A tag representing a directory
        #[arg(default_value = DEFAULT_PRTL_TAG)]
        tag: String
    }
}
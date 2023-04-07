mod args;

use core::fmt;
use std::collections::HashMap;
use std::{fs, io};
use std::path::PathBuf;
use std::io::Write as IoWrite;
use clap::Parser;
use serde_derive::{Deserialize, Serialize};
use serde_with::serde_as;

use args::{Commands, DEFAULT_PRTL_TAG, CONFIG_APP_NAME, PortalArgs};

/// prtl config stores the default_prtl tag as well as all the custom tags
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PortalConfig {
    prtl: String,

    #[serde_as(as = "Vec<(_, _)>")]
    portal_map: HashMap<String, String>
}

#[derive(Debug)]
struct PortalError {
   message: String,
}

impl fmt::Display for PortalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ::std::default::Default for PortalConfig {
   fn default() -> Self { Self { prtl: DEFAULT_PRTL_TAG.to_string(), portal_map: HashMap::new() } }
}

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
fn main() -> Result<(), PortalError> {
   // Load config file
   let mut cfg: PortalConfig = match confy::load(CONFIG_APP_NAME, None) {
      Ok(config) => config,
      Err(_e) => return Err(PortalError { message: format!("Error loading config.") })
   };
   
   let args = PortalArgs::parse();
   let mut stdout = io::stdout();

   match &args.command {
      Commands::Set { path, tag } => {
         let srcdir = PathBuf::from(path);
         let canonical_dir = match fs::canonicalize(srcdir) {
            Ok(path) => path.to_string_lossy().into_owned(),
            Err(_e) => return Err(PortalError { message: format!("Path {} is invalid.", &path) } ),
         };
         cfg.portal_map.insert((&tag).to_string(), canonical_dir);
      },
      Commands::Get { tag } => {
         if let Some(value) = cfg.portal_map.get(tag) {
            writeln!(&mut stdout, "{}", value).ok();
         } 
         else {
            return Err(PortalError { message: format!("Did not find prtl with tag {}", tag)});
         }
      }
   };

   match confy::store(CONFIG_APP_NAME, None, cfg) {
      Ok(()) => Ok({}),
      Err(_e) => Err(PortalError { message: "Failed to save config".to_string()} ),
   }
}

use core::fmt;
use std::error::Error;
use std::{fs, io};
use std::path::PathBuf;
use std::io::Write as IoWrite;

use clap::{Parser, arg, Subcommand};
use serde_derive::{Deserialize, Serialize};


const DEFUALT_PRTL_TAG: &str = "default_prtl";

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct PortalArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {

    Set {
        /// path to set as last directory.
        path: String,
        
        /// give the directory a tag.
        #[arg(short, default_value= DEFUALT_PRTL_TAG)]
        tag: String,
    },
    
    Get {
        /// A tag representing a directory
        #[arg(default_value= DEFUALT_PRTL_TAG)]
        tag: String
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PortalConfig {
    prtl: String,
    prtls: Vec<Portal>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Portal {
   tag: String,
   path: String,
}

#[derive(Debug)]
struct PortalError {
   message: String,
}

impl Error for PortalError {}

impl fmt::Display for PortalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ::std::default::Default for PortalConfig {
   fn default() -> Self { Self { prtl: DEFUALT_PRTL_TAG.to_string(), prtls: Vec::<Portal>::new() } }
}

fn main() -> Result<(), PortalError> {
   let mut cfg: PortalConfig = match confy::load(".prtl", None) {
      Ok(config) => config,
      Err(_e) => return Err(PortalError { message: format!("Error loading config.") })
   };
   
   let args = PortalArgs::parse();
   let mut stdout = io::stdout();

   match &args.command {
      Some(Commands::Set { path, tag }) => {

         let srcdir = PathBuf::from(path);
         let canonical_dir = match fs::canonicalize(srcdir) {
            Ok((path)) => path.to_string_lossy().into_owned(),
            Err(_e) => return Err(PortalError { message: format!("Path {} is invalid.", &path) } ),
         };

         match &cfg.prtls.iter().position(|p| p.tag == tag.to_string()) {
            Some(portal_index) =>  {
               let _ = &cfg.prtls.remove(*portal_index);
            },
            None => ()
         };
         cfg.prtls.insert(0, Portal { tag: (&tag).to_string(), path: canonical_dir });
      },
      Some(Commands::Get { tag }) => {
         let prtl = &cfg.prtls.iter().filter(|&p| p.tag == tag.to_string() ).collect::<Vec::<&Portal>>();
         let prtl_path = match prtl.first() {
            Some(first_prtl) => first_prtl.path.clone(),
            None => "".to_string(),  
         };
         writeln!(&mut stdout, "{}", prtl_path);
      },
      None => ()
   };

   match confy::store(".prtl", None, cfg) {
      Ok(()) => Ok({}),
      Err(_e) => Err(PortalError { message: "Failed to save config".to_string()} ),
   }
}

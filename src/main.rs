mod args;
mod tpl;

use core::fmt;
use std::collections::HashMap;
use std::io::Error;
use std::fs::{OpenOptions, File};
use std::{fs};
use std::path::PathBuf;
use std::io::Write as IoWrite;
use clap::Parser;
use dialoguer::{Select, Input};
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use rust_search::SearchBuilder;
use serde_derive::{Deserialize, Serialize};
use serde_with::serde_as;

use args::{Commands, DEFAULT_PRTL_TAG, CONFIG_APP_NAME, SHELL_TAG_BASH, PortalArgs};

use crate::tpl::PRTL_SHORTHAND_SCRIPT;

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

   let mut cfg: PortalConfig = match confy::load(CONFIG_APP_NAME, None) {
      Ok(config) => config,
      Err(_e) => return Err(PortalError { message: format!("Error loading config.") })
   };
   
   let args = PortalArgs::parse();
   let mut stdout = &Term::stdout();

   match &args.command {
      Commands::Set { path, tag } => {
         let srcdir = PathBuf::from(path);
         let canonical_dir = match fs::canonicalize(srcdir) {
            Ok(path) => path.to_string_lossy().to_string(),
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
      },
      Commands::EzInit { shell } => {
         match shell.as_str() {
            SHELL_TAG_BASH => {
               return setup_bash();
            }, 
            _ => {
               writeln!(stdout, "Usupported shell. Please try and configure manually.").expect("Could not write to terminal. ¯\\_(ツ)_/¯");
            },
         }
      }
   };

   match confy::store(CONFIG_APP_NAME, None, cfg) {
      Ok(()) => Ok({}),
      Err(_e) => Err(PortalError { message: "Failed to save config".to_string()} ),
   }
}

fn setup_bash() -> Result<(), PortalError> {
   // Search for Bash Profile
   let search_input = ".bash";
   let mut search: Vec<String> = SearchBuilder::default()
       .location("~/")
       .search_input(search_input)
       .ignore_case()
       .depth(3)
       .limit(20)
       .hidden()
       .build()
      .collect();

   //Add custom option if the file was not found in search.
   let default: String = "Custom".to_string();
   search.push(default);

   // Show selections in terminal
   let selection = Select::with_theme(&ColorfulTheme::default())
   .items(&search)
   .default(0)
   .interact_on_opt(&Term::stderr()).expect("Should've selected 'Custom' ¯\\_(ツ)_/¯");

   let selected_option = match selection {
      Some(index) => &search[index],
      None => &search[0],
   };
   
   // write the script file here
   let file_to_write = match selected_option.as_str() {
      "Custom" => {
         let custom_file = match Input::<String>::new()
            .with_prompt("Enter the file path to your bash profile")
            .interact_text(){
                Ok(typed_text) => typed_text,
                Err(_) => return Err(PortalError { message: format!("¯\\_(ツ)_/¯ Something went wrong with input")} ),
            };
         let canonical_dir = match fs::canonicalize(&custom_file) {
            Ok(path) => path.to_string_lossy().to_string(),
            Err(_e) => return Err(PortalError { message: format!("Path {} is invalid.", custom_file) } ),
         };
         canonical_dir
      },
      _ => {selected_option.to_string()}
   };

   //
   let mut path = PathBuf::from(&file_to_write);
   path.pop();
   path.push("prtl_shorthand.sh");

   let shorthand_path = path.to_string_lossy().to_string();
   match write_shorthand_file(&shorthand_path) {
      Ok(_) => (),
      Err(_e) => return Err(PortalError{ message: format!("")})
   };
   return match write_to_profile(&file_to_write, shorthand_path){
      Ok(_) => Ok(()),
      Err(_e) => Err(PortalError { message: format!("Failed to write to file. Try manual configuration: https://github.com/ShounakA/prtl#readme") })
   };
}

fn write_to_profile(selected_option: &String, shorthand_path: String) -> Result<(), Error> {
   let mut file = OpenOptions::new()
      .write(true)
      .append(true)
      .open(selected_option)
      .unwrap();

   match writeln!(file, "source {}", shorthand_path) {
      Ok(_) => Ok(()),
      Err(e) => Err(e)
   }
}

fn write_shorthand_file(shorthand_path: &String) -> Result<(), Error> {
   let mut file = File::create(shorthand_path)?;
   write!(file, "{}", PRTL_SHORTHAND_SCRIPT)
}
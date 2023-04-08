mod args;
mod portal;
mod tpl;

use clap::Parser;
use colored::Colorize;
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use rust_search::SearchBuilder;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::io::Write as IoWrite;
use std::path::PathBuf;

use args::{Commands, PortalArgs, SHELL_TAG_BASH};
use portal::{PortalConfig, PortalError};

use crate::tpl::PRTL_SHORTHAND_SCRIPT;

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
    let mut cfg = match PortalConfig::load() {
        Ok(config) => config,
        Err(err) => return Err(err),
    };

    let args = PortalArgs::parse();
    let mut stdout = &Term::stdout();

    match &args.command {
        Commands::Set { path, tag } => {
            let srcdir = PathBuf::from(path);
            let canonical_dir = match fs::canonicalize(srcdir) {
                Ok(path) => path.to_string_lossy().to_string(),
                Err(_e) => return Err(PortalError::new(format!("Path {} is invalid.", &path))),
            };
            cfg.put(tag, canonical_dir);
        }
        Commands::Get { tag } => {
            if let Some(value) = cfg.get(tag) {
                writeln!(&mut stdout, "{}", value).ok();
            } else {
                return Err(PortalError::new(format!(
                    "Did not find prtl with tag {}",
                    tag
                )));
            }
        }
        Commands::EzInit { shell } => match shell.as_str() {
            SHELL_TAG_BASH => {
                return setup_bash();
            }
            _ => {
                writeln!(
                    stdout,
                    "Usupported shell. Please try and configure manually."
                )
                .expect("Could not write to terminal. ¯\\_(ツ)_/¯");
            }
        },
        Commands::List { json } => {
            if *json {
                match serde_json::to_value(&cfg.portal_map) {
                    Ok(m) => {
                        writeln!(stdout, "{}", m.to_string()).unwrap();
                        ()
                    }
                    Err(_) => {
                        return Err(PortalError::new(format!(
                            "{}",
                            "Failed to deseriallize portal map to json"
                        )))
                    }
                };
            } else {
                writeln!(
                    stdout,
                    "{0: <12}🧿 {1} 🧿\n",
                    "",
                    "Your Portals".bold().green()
                )
                .unwrap();
                for key in cfg.portal_map.keys() {
                    match cfg.portal_map.get(key) {
                        Some(p) => {
                            let row =
                                format!("{0: <14}\t{1}\n", key.bold().blue(), p.italic().green());
                            write!(stdout, "{0: <3} {1}", "✨", row).expect("¯\\_(ツ)_/¯");
                        }
                        None => (),
                    };
                }
                writeln!(stdout, "\n{0: <14}🍻 {1} 🍻", "", "Cheers!".bold().green()).unwrap();
            }
        }
    };

    cfg.store()
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
        .interact_on_opt(&Term::stderr())
        .expect("Should've selected 'Custom' ¯\\_(ツ)_/¯");

    let selected_option = match selection {
        Some(index) => &search[index],
        None => &search[0],
    };

    // write the script file here
    let file_to_write = match selected_option.as_str() {
        "Custom" => {
            let custom_file = match Input::<String>::new()
                .with_prompt("Enter the file path to your bash profile")
                .interact_text()
            {
                Ok(typed_text) => typed_text,
                Err(_) => {
                    return Err(PortalError::new(format!(
                        "¯\\_(ツ)_/¯ Something went wrong with input"
                    )))
                }
            };
            let canonical_dir = match fs::canonicalize(&custom_file) {
                Ok(path) => path.to_string_lossy().to_string(),
                Err(_e) => {
                    return Err(PortalError::new(format!(
                        "Path {} is invalid.",
                        custom_file
                    )));
                }
            };
            canonical_dir
        }
        _ => selected_option.to_string(),
    };

    //
    let mut path = PathBuf::from(&file_to_write);
    path.pop();
    path.push("prtl_shorthand.sh");

    let shorthand_path = path.to_string_lossy().to_string();
    match write_shorthand_file(&shorthand_path) {
        Ok(_) => (),
        Err(_e) => return Err(PortalError::new(format!(""))),
    };
    return match write_to_profile(&file_to_write, shorthand_path){
      Ok(_) => Ok(()),
      Err(_e) => Err(PortalError::new(format!("Failed to write to file. Try manual configuration: https://github.com/ShounakA/prtl#readme")))
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
        Err(e) => Err(e),
    }
}

fn write_shorthand_file(shorthand_path: &String) -> Result<(), Error> {
    let mut file = File::create(shorthand_path)?;
    write!(file, "{}", PRTL_SHORTHAND_SCRIPT)
}

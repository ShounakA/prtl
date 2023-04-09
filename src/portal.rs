use core::fmt;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::args::{CONFIG_APP_NAME, DEFAULT_PRTL_TAG};

/// prtl config stores the default_prtl tag as well as all the custom tags
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    prtl: String,

    #[serde_as(as = "Vec<(_, _)>")]
    pub portal_map: HashMap<String, String>,
}

impl Config {
    /// Load prtl config
    pub fn load() -> Result<Config, Error> {
        match confy::load(CONFIG_APP_NAME, None) {
            Ok(config) => Ok(config),
            Err(_e) => return Err(Error::new(format!("Error loading config."))),
        }
    }

    /// Store the current prtl config.
    pub fn store(&self) -> Result<(), Error> {
        match confy::store(CONFIG_APP_NAME, None, self) {
            Ok(()) => Ok({}),
            Err(_e) => Err(Error {
                message: "Failed to save config".to_string(),
            }),
        }
    }

    ///  Put a key-value pair into the Hashmap
    pub fn put(&mut self, tag: String, dir: String) -> Result<(), Error> {
        let srcdir = PathBuf::from(&dir);
        let canonical_dir = match fs::canonicalize(srcdir) {
            Ok(path) => path.to_string_lossy().to_string(),
            Err(_e) => return Err(Error::new(format!("Path {} is invalid.", dir))),
        };
        let _ = self.portal_map.insert(tag, canonical_dir);
        Ok(())
    }

    /// Get copy of portal value (dir) by giving reference to portal key (tag)
    pub fn get(&self, tag: &String) -> Option<String> {
        if let Some(prtl_dir) = self.portal_map.get(tag) {
            return Some(prtl_dir.to_owned());
        } else {
            return None;
        }
    }
}

/// Default prtl binary error
#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    /// Create new prtl error
    pub fn new(message: String) -> Error {
        return Error { message };
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            prtl: DEFAULT_PRTL_TAG.to_string(),
            portal_map: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_portal_config() -> Config {
        Config {
            prtl: String::new(),
            portal_map: HashMap::new(),
        }
    }

    #[test]
    fn test_portal_put_dne() {
        let test_cfg = setup_portal_config();

        let actual = test_cfg.portal_map.get("dne");
        assert!(None == actual);
    }

    #[test]
    fn test_portal_put_valid_path_overwrite() {
        let mut test_cfg = setup_portal_config();

        test_cfg.put("Test".to_string(), "/home".to_string()).ok();
        test_cfg.put("Test".to_string(), "/tmp".to_string()).ok();

        let actual = test_cfg.portal_map.get("Test");
        assert!(Some(&"/tmp".to_string()) == actual);
    }

    #[test]
    fn test_portal_put_valid_path() {
        let mut test_cfg = setup_portal_config();

        test_cfg.put("Test".to_string(), "/home".to_string()).ok();

        let actual = test_cfg.portal_map.get("Test");
        assert!(Some(&"/home".to_string()) == actual);
    }

    #[test]
    fn test_portal_put_invalid_path() {
        let mut test_cfg = setup_portal_config();

        test_cfg.put("Test".to_string(), "asdfasdfasdf".to_string()).ok();
        let actual = test_cfg.portal_map.get("Test");
        assert!(None == actual);
    }

    #[test]
    fn test_portal_get_dne() {
        let test_cfg = setup_portal_config();

        let actual = test_cfg.get(&"dne".to_string());
        assert!(None == actual);
    }

    #[test]
    fn test_portal_get_path() {
        let mut test_cfg = setup_portal_config();

        test_cfg
            .portal_map
            .insert("test".to_string(), "/dir/asdf/asdf/".to_string());
        let actual = test_cfg.get(&"test".to_string());
        assert!(Some("/dir/asdf/asdf/".to_string()) == actual);
    }
}

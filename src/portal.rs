use core::fmt;
use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::args::{CONFIG_APP_NAME, DEFAULT_PRTL_TAG};

/// prtl config stores the default_prtl tag as well as all the custom tags
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PortalConfig {
    prtl: String,

    #[serde_as(as = "Vec<(_, _)>")]
    pub portal_map: HashMap<String, String>,
}

impl PortalConfig {
    pub fn load() -> Result<PortalConfig, PortalError> {
        match confy::load(CONFIG_APP_NAME, None) {
            Ok(config) => Ok(config),
            Err(_e) => return Err(PortalError::new(format!("Error loading config."))),
        }
    }

    pub fn store(&self) -> Result<(), PortalError> {
        match confy::store(CONFIG_APP_NAME, None, self) {
            Ok(()) => Ok({}),
            Err(_e) => Err(PortalError {
                message: "Failed to save config".to_string(),
            }),
        }
    }

    pub fn put(&mut self, tag: &String, dir: String) {
        let _ = self.portal_map.insert(tag.to_string(), dir);
    }

    pub fn get(&self, tag: &String) -> Option<&String> {
        self.portal_map.get(tag)
    }
}

#[derive(Debug)]
pub struct PortalError {
    message: String,
}

impl PortalError {
    pub fn new(message: String) -> PortalError {
        return PortalError { message };
    }
}

impl fmt::Display for PortalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ::std::default::Default for PortalConfig {
    fn default() -> Self {
        Self {
            prtl: DEFAULT_PRTL_TAG.to_string(),
            portal_map: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod test {}

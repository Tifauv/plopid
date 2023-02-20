//! 
/// Loads the configuration from a YAML file.

use crate::config::{Config, ConfigLoader};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, Error as IoError, ErrorKind};
use serde_yaml;


pub struct YamlConfigLoader {
    file : String,
}


impl YamlConfigLoader {
    pub fn new(p_config_file: String) -> YamlConfigLoader {
        YamlConfigLoader {
            file : p_config_file
        }
    }


    /// Reads the content of the configuration file.
    fn read_file(&self) -> io::Result<String> {
        let file = File::open(&self.file)?;
        let mut reader = BufReader::new(file);
        let mut data = String::new();
        reader.read_to_string(&mut data)?;
        Ok(data)
    }
}


impl ConfigLoader for YamlConfigLoader {
    /// Reads the configuration file and parses its content.
    fn load(&self) -> io::Result<Config> {
        let file_content = self.read_file()?;

        match serde_yaml::from_str(&file_content) {
            Ok(decoded) => {
                let config: Config = decoded;
                Ok(config)
            },
            Err(err) => {
                let error_msg = format!("Malformed configuration file '{}':", self.file);
                let desc = err.to_string();
                let details = match err.location() {
                    Some(location) => format!(" [l{}c{}]: {}]", location.line(), location.column(), desc),
                    None              => format!(" [{}]", desc)
                };

                Err(IoError::new(ErrorKind::InvalidData, error_msg + &details))
            }
        }
    }
}

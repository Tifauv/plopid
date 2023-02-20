//! 
/// Loads the configuration from a TOML file.

use crate::config::{Config, ConfigLoader};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, Error as IoError, ErrorKind};
use toml;


pub struct TomlConfigLoader {
    file : String,
}


impl TomlConfigLoader {
    pub fn new(p_config_file: String) -> TomlConfigLoader {
        TomlConfigLoader {
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


impl ConfigLoader for TomlConfigLoader {
    /// Reads the configuration file and parses its content.
    fn load(&self) -> io::Result<Config> {
        let file_content = self.read_file()?;

        match toml::from_str(&file_content) {
            Ok(decoded) => {
                let config: Config = decoded;
                Ok(config)
            },
            Err(err) => {
                let error_msg = format!("Malformed configuration file '{}':", self.file);
                let desc = err.to_string();
                let details = match err.line_col() {
                    Some((line, col)) => format!(" [l{}c{}]: {}]", line, col, desc),
                    None              => format!(" [{}]", desc)
                };

                Err(IoError::new(ErrorKind::InvalidData, error_msg + &details))
            }
        }
    }
}

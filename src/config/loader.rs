use crate::config::data::Config;
use std::io;

/// Defines the common interface to configuration loaders.
pub trait ConfigLoader {
    fn load(&self) -> io::Result<Config>;
}

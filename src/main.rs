#[macro_use] extern crate rocket;
extern crate plopid;

use plopid::config;
use plopid::config::{Config, ConfigLoader, TomlConfigLoader};
use plopid::oidc;
use plopid::web;
use plopid::services::registry;
use log::{info, warn};
use std::io;
use std::sync;


/// Loads the configuration from the given file.
///
/// If the configuration cannot be loaded, the default configuration from
/// `plopid::config::default()` is returned.
fn load_config(p_file: &str) -> Config {
    TomlConfigLoader::new(String::from(p_file)).load()
            .and_then(|c: Config| {
                info!("Configuration read from '{}'", p_file);
                Ok(c)
            })
            .or_else(|e: io::Error| -> io::Result<Config> {
                warn!("Failed to read the configuration from '{}': {}", p_file, e); 
                info!("Using default hardcoded configuration instead.");
                Ok(config::default())
            }).unwrap()
}


#[launch]
fn rocket() -> _ {
	info!("PlopID starting...");

	// Load the configuration from the "config/plopid.toml" file
	let config = load_config("config/plopid.toml");

	// Create the services registry & load the client files
	let mut services = registry::ServiceRegistry::new();
	info!("Service registry is ready, loading service definition files...");
	match services.load_from_directory(&config.clients.registry_dir) {
		Ok(_)  => {
			info!("{} service definition files loaded", services.count_services());
		}
		Err(e) => {
			warn!("Error: {}", e);
		}
	}

    rocket::build()
		.attach(rocket_dyn_templates::Template::fairing())
		.manage(sync::RwLock::new(config))
		.manage(sync::RwLock::new(services))
		.mount("/",            routes![web::base::home])
		.mount("/res",         rocket::fs::FileServer::from("res"))
		.mount("/authn",       routes![web::authn::form::login])
		.mount("/authn",       routes![web::authn::pwd::login_pwd])
		.mount("/.well-known", routes![oidc::discovery::endpoint])
		.mount("/oidc",        routes![oidc::auth::endpoint])
		.mount("/oidc",        routes![oidc::token::endpoint])
		.mount("/oidc",        routes![oidc::userinfo::endpoint])
}

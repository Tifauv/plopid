#[macro_use] extern crate rocket;
extern crate plopid;

use plopid::oidc;
use plopid::authn;
use plopid::services::registry;
use std::sync;

#[launch]
fn rocket() -> _ {
	// Create the services registry & load the client files
	let mut services = registry::ServiceRegistry::new();
	services.load_from_directory("clients");

    rocket::build()
		.attach(rocket_dyn_templates::Template::fairing())
		.manage(sync::RwLock::new(services))
		.mount("/res",         rocket::fs::FileServer::from("res"))
		.mount("/.well-known", routes![oidc::discovery::oidc_discovery])
		.mount("/authn",       routes![authn::form::login])
		.mount("/authn",       routes![authn::pwd::login_pwd])
		.mount("/oidc",        routes![oidc::login::oidc_authz])
}

#[macro_use] extern crate rocket;
extern crate plopid;

use plopid::oidc;
use plopid::authn;
use plopid::clients::registry::{Client, ClientRegistry};
use rocket_dyn_templates::Template;
use std::sync::RwLock;

#[launch]
fn rocket() -> _ {
	// Create the clients registry
	let mut clients = ClientRegistry::new();
	
	// Add some test clients
	clients.register(Client::new(
		String::from("test_client_1"),
		String::from("https://example.org/client1/oidc/callback")));
	clients.register(Client::new(
		String::from("test_client_2"),
		String::from("https://example.org/client2/oidc/callback")));

    rocket::build()
		.attach(Template::fairing())
		.manage(RwLock::new(clients))
		.mount("/.well-known", routes![oidc::discovery::oidc_discovery])
		.mount("/authn", routes![authn::form::login])
		.mount("/authn", routes![authn::pwd::login_pwd])
		.mount("/oidc", routes![oidc::login::oidc_authz])
}

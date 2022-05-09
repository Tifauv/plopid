#[macro_use] extern crate rocket;
extern crate plopid;

use plopid::oidc;
use plopid::authn;
use plopid::clients::registry;
use std::sync;

#[launch]
fn rocket() -> _ {
	// Create the clients registry
	let mut clients = registry::ClientRegistry::new();
	
	// Add some test clients
	clients.add_client(registry::RegisteredClient::new(
		String::from("Application 1"),
		String::from("test_client_1"),
		String::from("https://example.org/client1/oidc/callback")));
	clients.add_client(registry::RegisteredClient::new(
		String::from("Application 2"),
		String::from("test_client_2"),
		String::from("https://example.org/client2/oidc/callback")));

    rocket::build()
		.attach(rocket_dyn_templates::Template::fairing())
		.manage(sync::RwLock::new(clients))
		.mount("/.well-known", routes![oidc::discovery::oidc_discovery])
		.mount("/authn", routes![authn::form::login])
		.mount("/authn", routes![authn::pwd::login_pwd])
		.mount("/oidc", routes![oidc::login::oidc_authz])
}

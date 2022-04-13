#[macro_use] extern crate rocket;
extern crate plopid;

use plopid::oidc::{discovery, login};
use plopid::client::registered_clients::{Client, ClientRegistry};
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
		.manage(RwLock::new(clients))
		.mount("/.well-known", routes![discovery::oidc_discovery])
		.mount("/oidc", routes![login::oidc_authz])
}

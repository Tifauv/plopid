use std::collections::HashMap;
use rand::{Rng};


#[derive(Debug, PartialEq)]
pub struct NewClient {
	name: String,
	redirect_uris: Vec<String>,
}


#[derive(Debug, PartialEq)]
pub struct RegisteredClient {
	id: String,
	name: String,
	redirect_uris: Vec<String>,
	secret: Option<String>,
}

impl RegisteredClient {
	pub fn new(p_name: String, p_client_id: String, p_redirect_uri: String) -> RegisteredClient {
		RegisteredClient {
			id           : p_client_id,
			name         : p_name,
			redirect_uris: vec![ p_redirect_uri ],
			secret       : None,
		}
	}
	
	
	pub fn from_client(p_client: NewClient, p_client_id: String) -> RegisteredClient {
		RegisteredClient {
			id           : p_client_id,
			name         : p_client.name,
			redirect_uris: p_client.redirect_uris,
			secret       : None,
		}
	}
	
	pub fn has_redirect_uri(&self, p_redirect_uri: &String) -> bool {
		self.redirect_uris.contains(p_redirect_uri)
	}
	
	pub fn authn_by_secret(&self, p_client_secret: Option<String>) -> Result<bool, String> {
		match &self.secret {
			None    => Err(format!("Client '{}' was not registered with a secret", &self.name)),
			Some(s) => Ok(s.eq(p_client_secret)),
		}
	}
	
	pub fn has_secret(&self) -> bool {
		return self.secret != None
	}
	
	pub fn is_valid(&self) -> bool {
		self.id.len() > 0 && self.redirect_uris.len() > 0
	}
}


#[derive(Debug, PartialEq)]
pub struct RequestingClient {
	id: String,
	redirect_uri: String,
	secret: Option<String>,
}

impl RequestingClient {
	pub fn new(p_client_id: String, p_redirect_uri: String) -> RequestingClient {
		RequestingClient {
			id          : p_client_id,
			redirect_uri: p_redirect_uri,
			secret      : None,
		}
	}
	
	pub fn id(&self) -> String {
		self.id.clone()
	}
	
	pub fn redirect_uri(&self) -> String {
		self.redirect_uri.clone()
	}
}


pub struct ClientRegistry {
	clients: HashMap<String, RegisteredClient>,
}

impl ClientRegistry {
	pub fn new() -> ClientRegistry {
		ClientRegistry {
			clients: HashMap::new(),
		}
	}
	
	pub fn authenticate(&self, p_requesting_client: &RequestingClient) -> Result<String, String> {
		// Check if the client_id is registered
		match self.get_client(&p_requesting_client.id) {
			None    => Err(format!("There is no registered client with id '{}'", p_requesting_client.id)),
			Some(c) => {
				// TODO Check public key authentication ?
				
				// TODO Check if the client_secret is defined and matches
				if c.has_secret() {
					if !c.authn_by_secret(p_requesting_client.secret) {
						Err(String::from("No client_secret in the request or bad client_secret"))
					}
					// TODO log client authentication
				}
		
				// Check if the redirect_uri matches one of the registered ones
				if !c.has_redirect_uri(&p_requesting_client.redirect_uri) {
					Err(String::from("Redirect URI is not registered for that client"))
				}
				else {
					Ok(c.name.clone())
				}
			}
		}
	}
	
	pub fn get_client(&self, p_client_id: &String) -> Option<&RegisteredClient> {
		self.clients.get(p_client_id)
	}
	
	pub fn is_known(&self, p_client_id: &String) -> bool {
		self.clients.contains_key(p_client_id)
	}
	
	pub fn has_registered_uri(&self, p_client_id: &String, p_redirect_uri: &String) -> Result<bool, &'static str> {
		match self.get_client(p_client_id) {
			None    => Err("Unknown client"),
			Some(c) => Ok(c.has_redirect_uri(p_redirect_uri)),
		}
	}
	
	pub fn authn_by_secret(&self, p_client_id: &String, p_client_secret: &String) -> Result<bool, String> {
		match self.get_client(p_client_id) {
			None    => Err(String::from("Unknown client")),
			Some(c) => c.authn_by_secret(p_client_secret),
		}
	}
	
	pub fn register(&mut self, p_client: NewClient) -> Result<String, String> {
		let client = RegisteredClient::from_client(p_client, self.generate_id());
		self.add_client(client)
	}
	
	pub fn add_client(&mut self, p_client: RegisteredClient) -> Result<String, String> {
		if !p_client.is_valid() {
			Err(String::from("Client is not valid"))
		}
		else if self.is_known(&p_client.id) {
			Err(format!("Client id '{}' is already registered by another", p_client.id))
		}
		else {
			let client_id = p_client.id.clone();
			self.clients.insert(p_client.id.clone(), p_client);
			Ok(client_id)
		}
	}
	
	fn generate_id(&self) -> String {
		loop {
			let id: String = rand::thread_rng()
				.sample_iter(&rand::distributions::Alphanumeric)
				.take(32)
				.map(char::from)
				.collect();
			// Exit if the id is not already used
			if !self.is_known(&id) {
				return id
			}
		}
	}
}


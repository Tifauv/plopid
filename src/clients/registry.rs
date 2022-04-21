use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub struct Client {
	id: String,
	redirect_uris: Vec<String>,
	secret: Option<String>,
}

impl Client {
	pub fn new(p_client_id: String, p_redirect_uri: String) -> Client {
		Client {
			id           : p_client_id,
			redirect_uris: vec![ p_redirect_uri ],
			secret       : None,
		}
	}
	
	pub fn has_redirect_uri(&self, p_redirect_uri: &String) -> bool {
		self.redirect_uris.contains(p_redirect_uri)
	}
	
	pub fn authn_by_secret(&self, p_client_secret: &String) -> Result<bool, &'static str> {
		match &self.secret {
			None    => Err("Client was not registered with a secret"),
			Some(s) => Ok(s.eq(p_client_secret)),
		}
	}
	
	pub fn is_valid(&self) -> bool {
		self.id.len() > 0 && self.redirect_uris.len() > 0
	}
}

pub struct ClientRegistry {
	clients: HashMap<String, Client>,
}

impl ClientRegistry {
	pub fn new() -> ClientRegistry {
		ClientRegistry {
			clients: HashMap::new(),
		}
	}
	
	pub fn get_client(&self, p_client_id: &String) -> Option<&Client> {
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
	
	pub fn authn_by_secret(&self, p_client_id: &String, p_client_secret: &String) -> Result<bool, &'static str> {
		match self.get_client(p_client_id) {
			None    => Err("Unknown client"),
			Some(c) => c.authn_by_secret(p_client_secret),
		}
	}
	
	pub fn register(&mut self, p_client: Client) -> Result<&'static str, &'static str> {
		if !p_client.is_valid() {
			Err("Client is not valid")
		}
		else if self.is_known(&p_client.id) {
			Err("Client already registered with that id")
		}
		else {
			self.clients.insert(p_client.id.clone(), p_client);
			Ok("Client registered")
		}
	}
}


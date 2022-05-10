#[derive(Debug, PartialEq)]
pub struct RequestingClient {
	id: String,
	redirect_uri: String,
	secret: Option<String>,
}

impl RequestingClient {
	pub fn new(p_service_id: String, p_redirect_uri: String) -> RequestingClient {
		RequestingClient {
			id          : p_service_id,
			redirect_uri: p_redirect_uri,
			secret      : None,
		}
	}
	
	pub fn client_id(&self) -> String {
		self.id.clone()
	}
	
	pub fn client_secret(&self) -> Option<&String> {
		self.secret.as_ref()
	}
	
	pub fn redirect_uri(&self) -> String {
		self.redirect_uri.clone()
	}
}

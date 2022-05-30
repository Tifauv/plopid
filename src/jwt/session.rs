use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OIDCSessionClaims {
	pub aud: String,
	pub exp: usize,
	pub iss: String,
	pub client_name: String,
	pub client_id: String,     // OIDC client_id
	pub redirect_uri: String,  // OIDC redirect_uri
	pub scopes: Vec<String>,   // OIDC scopes
	pub state: Option<String>, // OIDC optional state
	pub nonce: Option<String>, // OIDC optional nonce
}

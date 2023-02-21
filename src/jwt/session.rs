use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
use jsonwebtoken::errors::Result;
use serde::{Serialize, Deserialize};
use crate::config;


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


pub fn check_token(p_session_token: &str, p_config: &config::Config) -> Result<TokenData<OIDCSessionClaims>> {
	// Check the session token
	let cfg_jwt_oidc_issuer    = String::from(&p_config.jwt.oidc_issuer);
	let cfg_jwt_internal_key   = p_config.jwt.internal_key.as_bytes();
	let cfg_jwt_authn_audience = String::from(&p_config.jwt.internal_authn_aud);

	let mut validation = Validation::new(Algorithm::HS256);
	validation.set_required_spec_claims(&["exp", "aud", "iss"]);
	validation.set_audience(&[cfg_jwt_authn_audience]);
	validation.set_issuer(&[cfg_jwt_oidc_issuer]);
	
	jsonwebtoken::decode::<OIDCSessionClaims>(
		&p_session_token,
		&DecodingKey::from_secret(cfg_jwt_internal_key),
		&validation)
}

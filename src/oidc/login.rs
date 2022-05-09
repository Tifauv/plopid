use rocket::get;
use rocket::State;
use rocket::http::{Status, ContentType};
use crate::clients::registry::{ClientRegistry, RequestingClient};
use std::sync::RwLock;


#[derive(Debug, PartialEq)]
pub struct OIDCAuthzRequest<'a> {
	scopes: Vec<&'a str>,
	client: &'a RequestingClient,
	state: Option<String>,
	nonce: Option<String>,
}


#[derive(Clone, PartialEq)]
pub enum ResponseType {
	Code,
	IdToken,
	Unsupported,
}

impl ResponseType {
	pub fn from_str(p_response_type: &str) -> ResponseType {
		match p_response_type {
			"code"     => ResponseType::Code,
			"id_token" => ResponseType::IdToken,
			_          => ResponseType::Unsupported,
		}
	}
}

fn check_scope(p_scope: &String) -> bool {
	p_scope.split(" ").collect::<Vec<_>>().contains(&"openid")
}


#[get("/login?<scope>&<response_type>&<client_id>&<redirect_uri>&<state>&<nonce>")]
pub fn oidc_authz(scope: String, response_type: String, client_id: String, redirect_uri: String, state: Option<String>, nonce: Option<String>, p_client_registry: &State<RwLock<ClientRegistry>>) -> (Status, (ContentType, String)) {
	// Check the client
	let client_registry = p_client_registry.read().unwrap();
	let client = RequestingClient::new(client_id, redirect_uri);
	let client_name = match client_registry.authenticate(&client) {
		Err(e) => return (Status::BadRequest, (ContentType::Text, e.to_string())),
		Ok(c)  => c
	};

	// Check the OIDC minimal scope
	if !check_scope(&scope) {
		return (Status::BadRequest, (ContentType::HTML, "Invalid scope".to_string()));
	}

	// Check the response type
	let request = OIDCAuthzRequest {
		client: &client,
		scopes: scope.split(" ").collect::<Vec<_>>(),
		state: state,
		nonce: nonce,
	};
	match ResponseType::from_str(&response_type) {
		ResponseType::Code => {
			let content = oidc_code_flow_authz(request);
			(Status::Ok, (ContentType::HTML, content)) 
		},
		ResponseType::IdToken => {
			let content = oidc_implicit_flow_authz(request);
			(Status::Ok, (ContentType::HTML, content)) 
		},
		ResponseType::Unsupported => {
			(Status::BadRequest, (ContentType::HTML, "Invalid response_type".to_string())) 
		}
	}
}


fn oidc_code_flow_authz(p_request: super::login::OIDCAuthzRequest) -> String {
	format!("<html><body><h1>PlopID IdP server</h1><h2>OIDC Authorization Endpoint</h2><h3>Client</h3><ul><li>client_id: {}</li><li>redirect_uri: {}</li></ul><h3>Request - Code Flow</h3><ul><li>scopes: {:?}</li><li>state: {:?}</li><li>nonce: {:?}</li></ul><h3>Login</h3><form method=\"GET\" action=\"/authn/login\"><input type=\"submit\" value=\"Go to Login page\"/></form></body></html>",
		p_request.client.id(),
		p_request.client.redirect_uri(),
		p_request.scopes,
		p_request.state,
		p_request.nonce)
}


fn oidc_implicit_flow_authz(p_request: super::login::OIDCAuthzRequest) -> String {
	format!("<h1>PlopID IdP server</h1><h2>OIDC Authorization Endpoint</h2><h3>Client</h3><ul><li>client_id: {}</li><li>redirect_uri: {}</li></ul><h3>Request - Implicit Flow</h3><ul><li>scopes: {:?}</li><li>state: {:?}</li><li>nonce: {:?}</li><li><strong>The Implicit Flow is not supported</strong></ul>",
		p_request.client.id(),
		p_request.client.redirect_uri(),
		p_request.scopes,
		p_request.state,
		p_request.nonce)
}

use rocket::get;
use rocket::State;
use rocket::http::{Status, ContentType};
use crate::client::registered_clients::ClientRegistry;
use std::sync::RwLock;


#[derive(Debug, PartialEq)]
struct OIDCAuthzRequest<'a> {
	scopes: Vec<&'a str>,
	client: OIDCRequestingClient,
	state: Option<String>,
	nonce: Option<String>,
}


#[derive(Debug, PartialEq)]
struct OIDCRequestingClient {
	id: String,
	redirect_uri: String,
	secret: Option<String>,
}

impl OIDCRequestingClient {
	pub fn new(p_client_id: String, p_redirect_uri: String) -> OIDCRequestingClient {
		OIDCRequestingClient {
			id          : p_client_id,
			redirect_uri: p_redirect_uri,
			secret      : None,
		}
	}
}


fn check_scope(p_scope: &String) -> bool {
	p_scope.split(" ").collect::<Vec<_>>().contains(&"openid")
}


fn response_type_is_code_flow(p_response_type: &String) -> bool {
	p_response_type.eq("code")
}

fn response_type_is_implicit_flow(p_response_type: &String) -> bool {
	p_response_type.eq("id_token")
}


#[get("/login?<scope>&<response_type>&<client_id>&<redirect_uri>&<state>&<nonce>")]
pub fn oidc_authz(scope: String, response_type: String, client_id: String, redirect_uri: String, state: Option<String>, nonce: Option<String>, p_client_registry: &State<RwLock<ClientRegistry>>) -> (Status, (ContentType, String)) {
	// Check the client
	let client = OIDCRequestingClient::new(client_id, redirect_uri);
	let client_registry = p_client_registry.read().unwrap();
	match client_registry.has_registered_uri(&client.id, &client.redirect_uri) {
		Ok(c)  => if !c {
					return (Status::BadRequest, (ContentType::HTML, "Redirect URI not registered for that client".to_string()));
				},
		Err(m) => return (Status::BadRequest, (ContentType::HTML, m.to_string())),
	}

	// Check the OIDC minimal scope
	if !check_scope(&scope) {
		return (Status::BadRequest, (ContentType::HTML, "Invalid scope".to_string()));
	}

	let request = OIDCAuthzRequest {
		client: client,
		scopes: scope.split(" ").collect::<Vec<_>>(),
		state: state,
		nonce: nonce,
	};

	// Switch to the code flow or implicit flow
	if response_type_is_code_flow(&response_type) {
		let content = oidc_authz_code_flow(request);
		(Status::Ok, (ContentType::HTML, content)) 
	}
	else if response_type_is_implicit_flow(&response_type) {
		let content = oidc_authz_implicit_flow(request);
		(Status::Ok, (ContentType::HTML, content)) 
	}
	else {
		(Status::BadRequest, (ContentType::HTML, "Invalid response_type".to_string())) 
	}
}


fn oidc_authz_code_flow(p_request: OIDCAuthzRequest) -> String {
	format!("<h1>PlopID IdP server</h1><h2>OIDC Authorization Endpoint</h2><h3>Client</h3><ul><li>client_id: {}</li><li>redirect_uri: {}</li></ul><h3>Request - Code Flow</h3><ul><li>scopes: {:?}</li><li>state: {:?}</li><li>nonce: {:?}</li></ul>",
		p_request.client.id,
		p_request.client.redirect_uri,
		p_request.scopes,
		p_request.state,
		p_request.nonce)
}


fn oidc_authz_implicit_flow(p_request: OIDCAuthzRequest) -> String {
	format!("<h1>PlopID IdP server</h1><h2>OIDC Authorization Endpoint</h2><h3>Client</h3><ul><li>client_id: {}</li><li>redirect_uri: {}</li></ul><h3>Request - Implicit Flow</h3><ul><li>scopes: {:?}</li><li>state: {:?}</li><li>nonce: {:?}</li><li><strong>The Implicit Flow is not supported</strong></ul>",
		p_request.client.id,
		p_request.client.redirect_uri,
		p_request.scopes,
		p_request.state,
		p_request.nonce)
}

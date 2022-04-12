use rocket::get;
use rocket::State;
use crate::client::registered_clients::ClientRegistry;
use std::sync::RwLock;


#[derive(Debug, PartialEq)]
struct OIDCAuthzRequest {
	scopes: Vec<String>,
	client: RequestingClient,
	state: String,
}


#[derive(Debug, PartialEq)]
struct RequestingClient {
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
}


fn oidc_check_scope(p_scope: &String) -> bool {
	p_scope.split(" ").collect::<Vec<_>>().contains(&"openid")
}


#[get("/authz?<scope>&response_type=code&<client_id>&<redirect_uri>&<state>")]
pub fn oidc_authz(scope: String, client_id: String, redirect_uri: String, state: String, p_client_registry: &State<RwLock<ClientRegistry>>) -> String {
	let client = RequestingClient::new(client_id, redirect_uri);

	let client_registry = p_client_registry.read().unwrap();
	match client_registry.has_registered_uri(&client.id, &client.redirect_uri) {
		Ok(c)  => if !c {
					return String::from("Redirect URI not registered for that client")
				},
		Err(m) => return String::from(m),
	}
	if !oidc_check_scope(&scope) {
		return String::from("Unknown scope")
	}

	let request = OIDCAuthzRequest {
		client: client,
		scopes: [ scope ].to_vec(),
		state: state,
	};
	format!("OIDC Endpoint : authorization\n  - client_id: {}\n  - redirect_uri: {}\n  - state: {}",
		//request.scopes,
		request.client.id,
		request.client.redirect_uri,
		request.state)
}

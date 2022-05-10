use rocket::get;
use rocket::State;
use rocket::http;
use rocket_dyn_templates::Template;
use std::sync::RwLock;
use crate::services::registry;
use crate::services::client;


#[derive(Debug, PartialEq)]
pub struct OIDCAuthzRequest<'a> {
	scopes: Vec<&'a str>,
	client: &'a client::RequestingClient,
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
pub fn oidc_authz(scope: String, response_type: String, client_id: String, redirect_uri: String, state: Option<String>, nonce: Option<String>, p_service_registry: &State<RwLock<registry::ServiceRegistry>>) -> (http::Status, Template) {
	// Check the client
	let service_registry = p_service_registry.read().unwrap();
	let client = client::RequestingClient::new(client_id, redirect_uri);
	let _client_name = match service_registry.authenticate(&client) {
		Err(_) => return (http::Status::BadRequest, error_page(
				400,
				"Client authentication failure",
				"Either:<ul><li>the client_id has not been registered to this IdP</li> \
				<li>a client_secret has been registered but none was given</li> \
				<li>the given client_secret does not match the registered one</li> \
				<li>the given redirect_uri does not match any of the registered ones for that client</li></ul>")),
		Ok(c)  => c
	};

	// Check the OIDC minimal scope
	if !check_scope(&scope) {
		return (http::Status::BadRequest, error_page(
				400,
				"Invalid scope",
				"The <strong>openid</strong> scope MUST be present for OIDC requests")) 
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
			(http::Status::Ok, content) 
		},
		ResponseType::IdToken => {
			let content = oidc_implicit_flow_authz(request);
			(http::Status::Ok, content) 
		},
		ResponseType::Unsupported => {
			(http::Status::BadRequest, error_page(
				400,
				"Invalid response_type",
				"The given response_type is currently not supported")) 
		}
	}
}


#[derive(serde::Serialize)]
struct ErrorContext<'a> {
	page_name: &'a str,
	error_code: u16,
	error_message: &'a str,
	error_details: &'a str,
}

fn error_page(p_code: u16, p_message: &str, p_details: &str) -> Template {
	Template::render("error", &ErrorContext {
		page_name: "Error",
		error_code: p_code,
		error_message: p_message,
		error_details: p_details,
	})
}


#[derive(serde::Serialize)]
struct AuthzContext<'a> {
	page_name: &'a str,
	client_name: &'a str,
	client_id: String,
	redirect_uri: String,
	request_scopes: Vec<&'a str>,
	request_state: Option<String>,
	request_nonce: Option<String>,
}

fn oidc_code_flow_authz(p_request: super::login::OIDCAuthzRequest) -> Template {
	Template::render("oidc_code_flow", &AuthzContext {
		page_name: "OIDC Authorization",
		client_name: "test",
		client_id: p_request.client.client_id(),
		redirect_uri: p_request.client.redirect_uri(),
		request_scopes: p_request.scopes,
		request_state: p_request.state,
		request_nonce: p_request.nonce,
	})
}


fn oidc_implicit_flow_authz(p_request: super::login::OIDCAuthzRequest) -> Template {
	Template::render("oidc_implicit_flow", &AuthzContext {
		page_name: "OIDC Authorization",
		client_name: "test",
		client_id: p_request.client.client_id(),
		redirect_uri: p_request.client.redirect_uri(),
		request_scopes: p_request.scopes,
		request_state: p_request.state,
		request_nonce: p_request.nonce,
	})
}

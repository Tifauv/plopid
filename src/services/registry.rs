use std::collections::HashMap;
use std::fs;
use std::fmt;
use rand::{Rng};
use crate::services::client::RequestingClient;
use crate::services::service::{NewService, RegisteredService, ServiceSecretError};


pub struct ServiceRegistry {
	services: HashMap<String, RegisteredService>,
}

#[derive(Debug)]
pub enum ServiceAuthenticationError {
    UnknownServiceId,
    UnknownRedirectUri,
    ServiceSecret(ServiceSecretError),
    //ServicePublicKeyError,
}

impl From<ServiceSecretError> for ServiceAuthenticationError {
	fn from(p_secret_error: ServiceSecretError) -> Self {
		ServiceAuthenticationError::ServiceSecret(p_secret_error)
	}
}

impl fmt::Display for ServiceAuthenticationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ServiceAuthenticationError::UnknownServiceId => 
				write!(f, "The service_id is unknown amongst the registered services"),
			ServiceAuthenticationError::UnknownRedirectUri =>
				write!(f, "The redirect_uri does not match any of the registered ones"),
			ServiceAuthenticationError::ServiceSecret(_) =>
				write!(f, "A service_secret was required or the given one did not match"),
		}
	}
}

impl ServiceRegistry {
	pub fn new() -> ServiceRegistry {
		ServiceRegistry {
			services: HashMap::new(),
		}
	}
	
	pub fn load_from_directory(&mut self, p_directory: &str) -> Result<(), std::io::Error> {
		let files = fs::read_dir(p_directory).unwrap();
		for file in files {
			//let service = RegisteredService::from_yaml_file(file);
			match file {
				Ok(f) => {
					let service = RegisteredService::from_yaml_file(f.path().to_str().unwrap().to_string())?;
					self.add_service(service);
				},
				Err(_) => {
				}
			}
		}
		Ok(())
	}
	
	
	pub fn authenticate(&self, p_requesting_client: &RequestingClient) -> Result<String, ServiceAuthenticationError> {
		// Check if the service_id is registered
		match self.get_service(&p_requesting_client.client_id()) {
			None    => Err(ServiceAuthenticationError::UnknownServiceId),
			Some(s) => {
				// TODO Check public key authentication ?
				
				// TODO Check if the service_secret is defined and matches
				s.authn_by_secret(p_requesting_client.client_secret())?;
		
				// Check if the redirect_uri matches one of the registered ones
				if !s.has_redirect_uri(&p_requesting_client.redirect_uri()) {
					Err(ServiceAuthenticationError::UnknownRedirectUri)
				}
				else {
					Ok(s.name())
				}
			}
		}
	}
	
	pub fn get_service(&self, p_service_id: &String) -> Option<&RegisteredService> {
		self.services.get(p_service_id)
	}
	
	pub fn is_service_known(&self, p_service: &RegisteredService) -> bool {
		self.services.contains_key(&p_service.client_id())
	}
	
	pub fn is_client_id_known(&self, p_service_id: &String) -> bool {
		self.services.contains_key(p_service_id)
	}
	
	pub fn register(&mut self, p_service: NewService) -> Result<String, String> {
		let service = RegisteredService::from_service(p_service, self.generate_id());
		self.add_service(service)
	}
	
	pub fn add_service(&mut self, p_service: RegisteredService) -> Result<String, String> {
		if !p_service.is_valid() {
			Err(String::from("Service is not valid"))
		}
		else if self.is_service_known(&p_service) {
			Err(format!("Service id '{}' is already registered by another", p_service.client_id()))
		}
		else {
			let service_id = p_service.client_id();
			self.services.insert(p_service.client_id(), p_service);
			Ok(service_id)
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
			if !self.is_client_id_known(&id) {
				return id
			}
		}
	}
}


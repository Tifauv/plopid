use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};
use serde_yaml;


#[derive(Debug, PartialEq)]
pub struct NewService {
	name: String,
	redirect_uris: Vec<String>,
}


#[derive(Debug)]
pub enum ServiceSecretSuccess {
    NoSecretNeeded,
    SecretMatched,
}

#[derive(Debug)]
pub enum ServiceSecretError {
    NoSecretRegistered,
    NoSecretProvided,
    SecretsDoNotMatch,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisteredService {
	id: String,
	name: String,
	redirect_uris: Vec<String>,
	
	#[serde(skip_serializing_if = "Option::is_none")]
	secret: Option<String>,
}

impl RegisteredService {
	pub fn new(p_name: String, p_service_id: String, p_redirect_uri: String) -> RegisteredService {
		RegisteredService {
			id           : p_service_id,
			name         : p_name,
			redirect_uris: vec![ p_redirect_uri ],
			secret       : None,
		}
	}


	pub fn from_service(p_service: NewService, p_service_id: String) -> RegisteredService {
		RegisteredService {
			id           : p_service_id,
			name         : p_service.name,
			redirect_uris: p_service.redirect_uris,
			secret       : None,
		}
	}


	pub fn from_yaml_file(p_filename: String) -> Result<RegisteredService,std::io::Error> {
		match File::open(p_filename) {
			Ok(mut file) => {
				let mut content = String::new();
				file.read_to_string(&mut content).unwrap();
				
				let service: RegisteredService = serde_yaml::from_str(&content).unwrap();
				Ok(service)
			},
			Err(e) => {
				Err(e)
			}
		}
	}
	
	
	pub fn name(&self) -> String {
		self.name.clone()
	}
	
	
	pub fn client_id(&self) -> String {
		self.id.clone()
	}
	
	
	pub fn has_redirect_uri(&self, p_redirect_uri: &String) -> bool {
		self.redirect_uris.contains(p_redirect_uri)
	}


	pub fn authn_by_secret(&self, p_service_secret: Option<&String>) -> Result<ServiceSecretSuccess, ServiceSecretError> {
		match &self.secret {
			None    => {
				match p_service_secret {
					None    => Ok(ServiceSecretSuccess::NoSecretNeeded),
					Some(_) => Err(ServiceSecretError::NoSecretRegistered),
				}
			},
			Some(s) => {
				match p_service_secret {
					None     => Err(ServiceSecretError::NoSecretProvided),
					Some(s2) => {
						if s.eq(s2) {
							Ok(ServiceSecretSuccess::SecretMatched)
						}
						else {
							Err(ServiceSecretError::SecretsDoNotMatch)
						}
					}
				}
			},
		}
	}


	pub fn has_secret(&self) -> bool {
		return self.secret != None
	}


	pub fn is_valid(&self) -> bool {
		self.id.len() > 0 && self.redirect_uris.len() > 0
	}
}

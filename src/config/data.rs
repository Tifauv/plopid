use serde::Deserialize;

/// This the root of the configuration structure.
///
/// # Examples
///
/// ```
/// use oxyboard::config::{ Config, ServerParams, ClientsParams, JWTParams };
///
/// let cfg = Config {
///         server: ServerParams {
///             ip           : String::from("localhost"),
///             port         : 8080,
///             templates_dir: String::from("templates"),
///         },
///
///         clients: ClientsParams {
///             registry_dir : String::from("clients"),
///         },
///
///         jwt: JWTParams {
///             oidc_issuer        : String::from("https://localhost:8000/oidc"),
///             internal_key       : b"secret"),
///             internal_authn_aud : String::from("https://localhost:8000/authn"),
///         },
/// };
/// assert_eq!(cfg.server.ip,              String::from("localhost"));
/// assert_eq!(cfg.server.port,            8080);
/// assert_eq!(cfg.server.templates_dir,   String::from("templates"));
/// assert_eq!(cfg.clients.regsitry_dir,   String::from("clients"));
/// assert_eq!(cfg.jwt.oidc_issuer,        String::from("https://localhost:8000/oidc"));
/// assert_eq!(cfg.jwt.internal_key,       b"secret"));
/// assert_eq!(cfg.jwt.internal_authn_aud, String::from("https://localhost:8000/authn"));
/// ```
#[derive(Debug,Deserialize)]
pub struct Config {
    pub server  : ServerParams,
    pub clients : ClientsParams,
    pub jwt     : JWTParams,
}


/// The server parameters contain the network parameters (ip, port...)
///
/// # Examples
///
/// ```
/// use oxyboard::config::ServerParams;
///
/// let server_cfg = ServerParams {
///         ip           : String::from("localhost"),
///         port         : 8080,
///         templates_dir: String::from("templates"),
/// };
/// assert_eq!(server_cfg.ip,            String::from("localhost"));
/// assert_eq!(server_cfg.port,          8080);
/// assert_eq!(server_cfg.templates_dir, String::from("templates"));
/// ```
#[derive(Debug,Deserialize)]
pub struct ServerParams {
    pub ip   : String,
    pub port : u16,
    pub templates_dir : String,
}


/// The clients parameters define where and how the clients data are saved.
///
/// # Examples
///
/// ```
/// use oxyboard::config::ClientsParams;
///
/// let clients_cfg = ClientsParams {
///         registry_dir : String::from("clients"),
/// };
/// assert_eq!(clients_cfg.registry_dir, String::from("clients"));
/// ```
#[derive(Debug,Deserialize)]
pub struct ClientsParams {
    pub registry_dir : String,
}


/// The JWT parameters define how the JWT (both internal and external) tokens
/// are generated.
///
/// # Examples
///
/// ```
/// use oxyboard::config::JWTParams;
///
/// let jwt_cfg = JWTParams {
///         oidc_issuer        : String::from("https://localhost:8000/oidc"),
///         internal_key       : b"secret",
///         internal_authn_aud : String::from("https://localhost:8000/authn"),
/// };
/// assert_eq!(jwt_cfg.oidc_issuer,        String::from("https://localhost:8000/oidc"));
/// assert_eq!(jwt_cfg.internal_key,       b"secret");
/// assert_eq!(jwt_cfg.internal_authn_aud, String::from("https://localhost:8000/authn"));
/// ```
#[derive(Debug,Deserialize)]
pub struct JWTParams {
    pub oidc_issuer       : String,
    pub internal_key      : String,
    pub internal_authn_aud: String,
}

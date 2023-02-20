use crate::config::data::{ Config, ServerParams, ClientsParams, JWTParams };

/// Builds a default configuration.
///
/// # Examples
///
/// ```
/// use oxyboard::config;
///
/// let cfg = config::default();
/// assert_eq!(cfg.server.listen_ip,       String::from("127.0.0.1"));
/// assert_eq!(cfg.server.host,            String::from("localhost"));
/// assert_eq!(cfg.server.port,            8000);
/// assert_eq!(cfg.server.templates_dir,   String::from("templates"));
/// assert_eq!(cfg.clients.registry_dir,   String::from("config/clients"));
/// assert_eq!(cfg.jwt.oidc_issuer,        String::from("https://localhost:8000/oidc"));
/// assert_eq!(cfg.jwt.internal_key,       String::from("secret"));
/// assert_eq!(cfg.jwt.internal_auhtn_aud, String::from("https://localhost:8000/authn"));
/// ```
pub fn default() -> Config {
    Config {
        server: ServerParams {
            listen_ip    : String::from("127.0.0.1"),
            host         : String::from("localhost"),
            port         : 8000,
            templates_dir: String::from("templates"),
        },

        clients: ClientsParams {
            registry_dir : String::from("config/clients"),
        },
        
        jwt: JWTParams {
            oidc_issuer        : String::from("https://localhost:8000/oidc"),
            internal_key       : String::from("secret"),
            internal_authn_aud : String::from("https://localhost:8000/authn"),
         },
    }
}

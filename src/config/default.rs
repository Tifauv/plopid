use crate::config::data::{ Config, ServerParams, ClientsParams, JWTParams };

/// Builds a default configuration.
///
/// # Examples
///
/// ```
/// use oxyboard::config;
///
/// let cfg = config::default();
/// assert_eq!(cfg.server.ip,          String::from("localhost"));
/// assert_eq!(cfg.server.port,        8080);
/// assert_eq!(cfg.board.name,         String::from("oxyboard"));
/// assert_eq!(cfg.board.history_size, 512);
/// assert_eq!(cfg.storage.data_dir,   String::from("data"));
/// assert_eq!(cfg.ui.templates_dir,   String::from("templates"));
/// ```
pub fn default() -> Config {
    Config {
        server: ServerParams {
            ip           : String::from("localhost"),
            port         : 8080,
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

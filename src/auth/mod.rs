//! HTTP authentication and authorization.
//!
//! # Examples
//!
//! ```
//! # fn main() -> http_types_rs::Result<()> {
//! #
//! use http_types_rs::Response;
//! use http_types_rs::auth::{AuthenticationScheme, BasicAuth};
//!
//! let username = "nori";
//! let password = "secret_fish!!";
//! let authz = BasicAuth::new(username, password);
//!
//! let mut res = Response::new(200);
//! res.insert_header(&authz, &authz);
//!
//! let authz = BasicAuth::from_headers(res)?.unwrap();
//!
//! assert_eq!(authz.username(), username);
//! assert_eq!(authz.password(), password);
//! #
//! # Ok(()) }
//! ```

mod authentication_scheme;
mod authorization;
mod basic_auth;
mod www_authenticate;

pub use authentication_scheme::AuthenticationScheme;
pub use authorization::Authorization;
pub use basic_auth::BasicAuth;
pub use www_authenticate::WwwAuthenticate;

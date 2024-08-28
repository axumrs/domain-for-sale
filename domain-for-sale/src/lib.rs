mod asset;
pub mod captcha;
pub mod config;
mod err;
pub mod form;
pub mod handler;
pub mod mail;
pub mod resp;
mod state;

pub use asset::*;
pub use err::*;
pub use state::*;

pub type Result<T> = std::result::Result<T, crate::Error>;

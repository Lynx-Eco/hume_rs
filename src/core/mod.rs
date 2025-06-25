//! Core functionality for the Hume SDK

pub mod auth;
pub mod client;
pub mod error;
pub mod http;
pub mod request;
pub mod response;
pub mod retry;
pub mod validation;

pub use auth::{Auth, AuthToken};
pub use client::{HumeClient, HumeClientBuilder};
pub use error::{Error, Result};
pub use request::RequestOptions;
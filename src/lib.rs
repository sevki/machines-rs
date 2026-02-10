//! # Machines-rs
//!
//! A Rust client for the Fly.io Machines API.
//!
//! This crate provides a complete client generated from the official Fly.io Machines API OpenAPI specification.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use machines_rs::client;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a configuration with your API token
//!     let mut config = client::apis::configuration::Configuration::new();
//!     config.base_path = "https://api.machines.dev/v1".to_string();
//!     config.bearer_access_token = Some("your-api-token".to_string());
//!
//!     // List apps
//!     let params = client::apis::apps_api::AppsListParams {
//!         org_slug: "personal".to_string(),
//!         app_role: None,
//!     };
//!
//!     match client::apis::apps_api::apps_list(&config, params).await {
//!         Ok(response) => println!("Apps: {:?}", response),
//!         Err(e) => eprintln!("Error: {:?}", e),
//!     }
//! }
//! ```

// Re-export the generated client
pub use machines_api as client;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_available() {
        // Test that the client module is accessible
        let config = client::apis::configuration::Configuration::new();
        assert_eq!(config.base_path, "https://api.machines.dev/v1");
    }
}

//! Example: List all apps in your organization
//!
//! Usage:
//! ```bash
//! FLY_API_TOKEN=your-token cargo run --example list_apps
//! ```

use machines_rs::client;
use std::env;

#[tokio::main]
async fn main() {
    // Get API token from environment
    let api_token = env::var("FLY_API_TOKEN")
        .expect("FLY_API_TOKEN environment variable is required");

    // Create configuration
    let mut config = client::apis::configuration::Configuration::new();
    config.base_path = "https://api.machines.dev/v1".to_string();
    config.bearer_access_token = Some(api_token);

    // List apps for personal organization
    let params = client::apis::apps_api::AppsListParams {
        org_slug: "personal".to_string(),
        app_role: None,
    };

    println!("Fetching apps...");
    match client::apis::apps_api::apps_list(&config, params).await {
        Ok(response) => {
            println!("Successfully retrieved apps:");
            if let Some(apps) = response.apps {
                for app in apps {
                    if let Some(name) = app.name {
                        println!("  - {}", name);
                    }
                }
            } else {
                println!("No apps found");
            }
        }
        Err(e) => {
            eprintln!("Error fetching apps: {:?}", e);
            std::process::exit(1);
        }
    }
}

# machines-rs

A Rust client library for the [Fly.io Machines API](https://fly.io/docs/machines/api/).

This client is automatically generated from the official [Fly.io Machines API OpenAPI specification](https://docs.machines.dev/swagger/doc.json).

## Features

- Complete coverage of the Fly.io Machines API
- Async/await support using `tokio` and `reqwest`
- Type-safe request and response models
- Comprehensive API documentation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
machines-rs = "0.1.0"
```

## Usage

```rust
use machines_rs::client;

#[tokio::main]
async fn main() {
    // Create a configuration with your API token
    let mut config = client::apis::configuration::Configuration::new();
    config.base_path = "https://api.machines.dev/v1".to_string();
    config.bearer_access_token = Some("your-fly-api-token".to_string());

    // List apps for your organization
    let params = client::apis::apps_api::AppsListParams {
        org_slug: "personal".to_string(),
        app_role: None,
    };

    match client::apis::apps_api::apps_list(&config, params).await {
        Ok(response) => println!("Apps: {:?}", response),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

## API Coverage

The client provides access to the following API groups:

- **Apps API**: Manage Fly.io applications
- **Machines API**: Create, update, and manage machines
- **Volumes API**: Manage persistent volumes
- **Organizations API**: Organization-level operations
- **Platform API**: Platform-level queries and operations

## Regenerating the Client

To regenerate the client from the latest OpenAPI specification:

```bash
npm install
npx @openapitools/openapi-generator-cli generate \
  -i openapi.json \
  -g rust \
  -o generated \
  --package-name machines-api \
  --skip-validate-spec \
  --additional-properties=supportAsync=true,useSingleRequestParameter=true
```

Then apply the necessary fixes for compilation errors (see commit history for details).

## License

Apache 2.0 - see the [Fly.io Machines API documentation](https://fly.io/docs/machines/api/) for more information.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

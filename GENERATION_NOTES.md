# OpenAPI Client Generation Notes

## Overview

This document explains how the Rust client was generated from the Fly.io Machines API OpenAPI specification and what manual fixes were required.

## Generation Process

### 1. Download OpenAPI Specification

```bash
curl -s https://docs.machines.dev/swagger/doc.json -o openapi.json
```

The spec is a Swagger 2.0 specification (170KB) describing the complete Fly.io Machines API.

### 2. Install Code Generator

```bash
npm init -y
npm install --save-dev @openapitools/openapi-generator-cli
```

### 3. Generate Rust Client

```bash
npx @openapitools/openapi-generator-cli generate \
  -i openapi.json \
  -g rust \
  -o generated \
  --package-name machines-api \
  --skip-validate-spec \
  --additional-properties=supportAsync=true,useSingleRequestParameter=true
```

**Note:** `--skip-validate-spec` was required because the OpenAPI spec has some validation issues with path parameters not being properly declared at the path level.

## Manual Fixes Required

The generated code had compilation errors that required manual fixes:

### 1. Missing Path Parameters (apps_api.rs)

**Issue:** Three endpoints were missing path parameter structs and the parameters weren't being used in URL formatting.

**Files Modified:**
- `generated/src/apis/apps_api.rs`

**Fixes Applied:**

#### Added `app_name` field to AppIpAssignmentsCreateParams:
```rust
pub struct AppIpAssignmentsCreateParams {
    pub app_name: String,  // Added
    pub request: models::AssignIpRequest
}
```

#### Added AppIpAssignmentsDeleteParams struct:
```rust
pub struct AppIpAssignmentsDeleteParams {
    pub app_name: String,
    pub ip: String
}
```

#### Added AppIpAssignmentsListParams struct:
```rust
pub struct AppIpAssignmentsListParams {
    pub app_name: String
}
```

#### Fixed URL formatting in three functions:
```rust
// Before:
format!("{}/apps/{app_name}/ip_assignments", configuration.base_path)

// After:
format!("{}/apps/{}/ip_assignments", configuration.base_path, params.app_name)
```

### 2. Duplicate `version` Fields (Multiple Model Files)

**Issue:** Several response models had duplicate `version` fields - one with `rename = "Version"` (DEPRECATED) and one with `rename = "version"`. This caused compilation errors.

**Files Modified:**
- `generated/src/models/app_secrets_update_resp.rs`
- `generated/src/models/delete_app_secret_response.rs`
- `generated/src/models/delete_secretkey_response.rs`
- `generated/src/models/set_app_secret_response.rs`
- `generated/src/models/set_secretkey_response.rs`

**Fix Applied:** Removed the deprecated `Version` field and kept only the `version` field in each struct and its `new()` implementation.

### 3. Enum Variant Naming (main_status_code.rs)

**Issue:** Enum variants didn't follow Rust naming conventions (should be UpperCamelCase).

**File Modified:**
- `generated/src/models/main_status_code.rs`

**Fixes Applied:**
```rust
// Before:
unknown,
capacityErr,

// After:
Unknown,
CapacityErr,
```

Also updated all references in the `Display` implementation and `Default` implementation.

## Known Documentation Issues

The generated markdown documentation files in `generated/docs/` have some inconsistencies (duplicate `version` field documentation, missing path parameter documentation), but these don't affect functionality and were left as-is since they're auto-generated documentation.

## Workspace Structure

The project uses a Cargo workspace:

```toml
[workspace]
members = [".", "generated"]

[package]
name = "machines-rs"

[dependencies]
machines-api = { path = "generated" }
```

This allows the main crate to re-export the generated client while keeping the generated code isolated.

## Regeneration

If you need to regenerate the client from an updated OpenAPI spec:

1. Download the latest spec:
   ```bash
   curl -s https://docs.machines.dev/swagger/doc.json -o openapi.json
   ```

2. Regenerate the client:
   ```bash
   npx @openapitools/openapi-generator-cli generate \
     -i openapi.json \
     -g rust \
     -o generated \
     --package-name machines-api \
     --skip-validate-spec \
     --additional-properties=supportAsync=true,useSingleRequestParameter=true
   ```

3. Apply the manual fixes documented above (or review and apply any new fixes needed)

4. Build and test:
   ```bash
   cargo build
   cargo test
   ```

## Files Not to Commit

The `.gitignore` excludes:
- `node_modules/` - npm dependencies
- `package-lock.json` - npm lock file
- `generated/target/` - Rust build artifacts for generated crate
- `/target` - Rust build artifacts for main crate

## Testing

Run tests with:
```bash
cargo test
```

Build examples with:
```bash
cargo build --example list_apps
```

Run examples with:
```bash
FLY_API_TOKEN=your-token cargo run --example list_apps
```

//! Namespace object operations via the host runtime.
//!
//! Provides functions for uploading, deleting, and listing objects in
//! namespaces, as well as syncing audience access levels. These operations
//! go through the host rather than direct HTTP, so plugins don't need
//! HTTP permissions for server operations.
//!
//! Requires the `namespaces` feature.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde::{Deserialize, Serialize};

use super::*;

/// Metadata for a single object in a namespace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMeta {
    pub key: String,
    #[serde(default)]
    pub audience: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
}

/// Upload an object to a namespace.
pub fn put_object(
    ns_id: &str,
    key: &str,
    bytes: &[u8],
    mime_type: &str,
    audience: &str,
) -> Result<(), String> {
    let input = serde_json::json!({
        "ns_id": ns_id,
        "key": key,
        "body_base64": BASE64.encode(bytes),
        "mime_type": mime_type,
        "audience": audience,
    });
    let result = unsafe { host_namespace_put_object(input.to_string()) }
        .map_err(|e| format!("host_namespace_put_object failed: {e}"))?;
    let parsed: serde_json::Value = serde_json::from_str(&result)
        .map_err(|e| format!("Failed to parse put_object response: {e}"))?;
    if let Some(err) = parsed.get("error").and_then(|v| v.as_str()) {
        return Err(err.to_string());
    }
    Ok(())
}

/// Delete a single object from a namespace.
pub fn delete_object(ns_id: &str, key: &str) -> Result<(), String> {
    let input = serde_json::json!({
        "ns_id": ns_id,
        "key": key,
    });
    let result = unsafe { host_namespace_delete_object(input.to_string()) }
        .map_err(|e| format!("host_namespace_delete_object failed: {e}"))?;
    let parsed: serde_json::Value = serde_json::from_str(&result)
        .map_err(|e| format!("Failed to parse delete_object response: {e}"))?;
    if let Some(err) = parsed.get("error").and_then(|v| v.as_str()) {
        return Err(err.to_string());
    }
    Ok(())
}

/// List all objects in a namespace.
pub fn list_objects(ns_id: &str) -> Result<Vec<ObjectMeta>, String> {
    let input = serde_json::json!({
        "ns_id": ns_id,
    });
    let result = unsafe { host_namespace_list_objects(input.to_string()) }
        .map_err(|e| format!("host_namespace_list_objects failed: {e}"))?;
    serde_json::from_str(&result)
        .map_err(|e| format!("Failed to parse list_objects response: {e}"))
}

/// Sync an audience's access level on the server.
pub fn sync_audience(
    ns_id: &str,
    audience: &str,
    access: &str,
) -> Result<(), String> {
    let input = serde_json::json!({
        "ns_id": ns_id,
        "audience": audience,
        "access": access,
    });
    let result = unsafe { host_namespace_sync_audience(input.to_string()) }
        .map_err(|e| format!("host_namespace_sync_audience failed: {e}"))?;
    let parsed: serde_json::Value = serde_json::from_str(&result)
        .map_err(|e| format!("Failed to parse sync_audience response: {e}"))?;
    if let Some(err) = parsed.get("error").and_then(|v| v.as_str()) {
        return Err(err.to_string());
    }
    Ok(())
}

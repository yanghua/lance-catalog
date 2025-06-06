/*
 * Lance Catalog REST Specification
 *
 * **Lance Catalog** is an OpenAPI specification on top of the storage-based Lance format. It provides an integration point for catalog service like Apache Hive MetaStore (HMS), Apache Gravitino, etc. to store and use Lance tables. To integrate, the catalog service implements a **Lance Catalog Adapter**, which is a REST server that converts the Lance catalog requests to native requests against the catalog service. Different tools can integrate with Lance Catalog using the generated OpenAPI clients in various languages, and invoke operations in Lance Catalog to read, write and manage Lance tables in the integrated catalog services. 
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ErrorResponse : JSON error response model based on [RFC-7807](https://datatracker.ietf.org/doc/html/rfc7807)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// a URI identifier that categorizes the error
    #[serde(rename = "type")]
    pub r#type: String,
    /// a brief, human-readable message about the error
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// HTTP response code, (if present) it must match the actual HTTP code returned by the service
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// a human-readable explanation of the error
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// a URI that identifies the specific occurrence of the error
    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

impl ErrorResponse {
    /// JSON error response model based on [RFC-7807](https://datatracker.ietf.org/doc/html/rfc7807)
    pub fn new(r#type: String) -> ErrorResponse {
        ErrorResponse {
            r#type,
            title: None,
            status: None,
            detail: None,
            instance: None,
        }
    }
}


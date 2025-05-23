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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetNamespaceResponse {
    #[serde(rename = "namespace")]
    pub namespace: String,
    /// Properties stored on the namespace, if supported by the server. If the server does not support namespace properties, it should return null for this field. If namespace properties are supported, but none are set, it should return an empty object.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

impl GetNamespaceResponse {
    pub fn new(namespace: String) -> GetNamespaceResponse {
        GetNamespaceResponse {
            namespace,
            properties: None,
        }
    }
}


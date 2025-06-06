# lance-namespace-apache-client

Lance REST Namespace Specification

- API version: 0.0.1

- Generator version: 7.12.0

**Lance Namespace Specification** is an open specification on top of the storage-based Lance data format  to standardize access to a collection of Lance tables (a.k.a. Lance datasets). It describes how a metadata service like Apache Hive MetaStore (HMS), Apache Gravitino, Unity Namespace, etc. should store and use Lance tables, as well as how ML/AI tools and analytics compute engines (will together be called _\"tools\"_ in this document) should integrate with Lance tables.
A Lance namespace is a centralized repository for discovering, organizing, and managing Lance tables. It can either contain a collection of tables, or a collection of Lance namespaces recursively. It is designed to encapsulates concepts including namespace, catalog, metastore, database, schema, etc. that frequently appear in other similar data systems to allow easy integration with any system of any type of object hierarchy.
In an enterprise environment, typically there is a requirement to store tables in a metadata service  such as Apache Hive MetaStore, Apache Gravitino, Unity Namespace, etc.  for more advanced governance features around access control, auditing, lineage tracking, etc. **Lance REST Namespace** is an OpenAPI protocol that enables reading, writing and managing Lance tables by connecting those metadata services or building a custom metadata server in a standardized way. The detailed OpenAPI specification content can be found in [rest.yaml](./rest.yaml).



*Automatically generated by the [OpenAPI Generator](https://openapi-generator.tech)*

## Requirements

Building the API client library requires:

1. Java 1.8+
2. Maven/Gradle

## Installation

To install the API client library to your local Maven repository, simply execute:

```shell
mvn clean install
```

To deploy it to a remote Maven repository instead, configure the settings of the repository and execute:

```shell
mvn clean deploy
```

Refer to the [OSSRH Guide](http://central.sonatype.org/pages/ossrh-guide.html) for more information.

### Maven users

Add this dependency to your project's POM:

```xml
<dependency>
  <groupId>com.lancedb</groupId>
  <artifactId>lance-namespace-apache-client</artifactId>
  <version>0.0.1</version>
  <scope>compile</scope>
</dependency>
```

### Gradle users

Add this dependency to your project's build file:

```groovy
compile "com.lancedb:lance-namespace-apache-client:0.0.1"
```

### Others

At first generate the JAR by executing:

```shell
mvn clean package
```

Then manually install the following JARs:

- `target/lance-namespace-apache-client-0.0.1.jar`
- `target/lib/*.jar`

## Getting Started

Please follow the [installation](#installation) instruction and execute the following Java code:

```java

import com.lancedb.lance.catalog.client.apache.*;
import com.lancedb.lance.catalog.client.apache.auth.*;
import com.lancedb.lance.catalog.client.apache.model.*;
import com.lancedb.lance.catalog.client.apache.api.NamespaceApi;

public class NamespaceApiExample {

    public static void main(String[] args) {
        ApiClient defaultClient = Configuration.getDefaultApiClient();
        defaultClient.setBasePath("http://localhost:2333");
        
        NamespaceApi apiInstance = new NamespaceApi(defaultClient);
        CreateNamespaceRequest createNamespaceRequest = new CreateNamespaceRequest(); // CreateNamespaceRequest | 
        try {
            GetNamespaceResponse result = apiInstance.createNamespace(createNamespaceRequest);
            System.out.println(result);
        } catch (ApiException e) {
            System.err.println("Exception when calling NamespaceApi#createNamespace");
            System.err.println("Status code: " + e.getCode());
            System.err.println("Reason: " + e.getResponseBody());
            System.err.println("Response headers: " + e.getResponseHeaders());
            e.printStackTrace();
        }
    }
}

```

## Documentation for API Endpoints

All URIs are relative to *http://localhost:2333*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*NamespaceApi* | [**createNamespace**](docs/NamespaceApi.md#createNamespace) | **POST** /v1/namespaces | Create a new namespace
*NamespaceApi* | [**dropNamespace**](docs/NamespaceApi.md#dropNamespace) | **DELETE** /v1/namespaces/{ns} | Drop a namespace
*NamespaceApi* | [**getNamespace**](docs/NamespaceApi.md#getNamespace) | **GET** /v1/namespaces/{ns} | Get information about a namespace
*NamespaceApi* | [**listNamespaces**](docs/NamespaceApi.md#listNamespaces) | **GET** /v1/namespaces | List namespaces
*NamespaceApi* | [**namespaceExists**](docs/NamespaceApi.md#namespaceExists) | **HEAD** /v1/namespaces/{ns} | Check if a namespace exists
*TableApi* | [**getTable**](docs/TableApi.md#getTable) | **GET** /v1/tables/{table} | Get a table from the namespace
*TableApi* | [**registerTable**](docs/TableApi.md#registerTable) | **POST** /v1/table/register | Register a table to a namespace
*TableApi* | [**tableExists**](docs/TableApi.md#tableExists) | **HEAD** /v1/tables/{table} | Check if a table exists


## Documentation for Models

 - [CreateNamespaceRequest](docs/CreateNamespaceRequest.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [GetNamespaceResponse](docs/GetNamespaceResponse.md)
 - [GetTableResponse](docs/GetTableResponse.md)
 - [ListNamespacesResponse](docs/ListNamespacesResponse.md)
 - [RegisterTableRequest](docs/RegisterTableRequest.md)


<a id="documentation-for-authorization"></a>
## Documentation for Authorization

Endpoints do not require authorization.


## Recommendation

It's recommended to create an instance of `ApiClient` per thread in a multithreaded environment to avoid any potential issues.

## Author




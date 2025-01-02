use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

/// A self-contained or composite resource which defines or describes an API or elements of an API.
/// The OpenAPI document MUST contain at least one [paths] field, a [components] field or a [webhooks] field.
/// An OpenAPI document uses and conforms to the OpenAPI Specification.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAPI {
    /// This string MUST be the version number of the OpenAPI Specification that the OpenAPI document uses.
    /// The openapi field SHOULD be used by tooling to interpret the OpenAPI document.
    /// This is not related to the API [InfoObject.version] string.
    pub openapi: String,
    /// Provides metadata about the API. The metadata MAY be used by tooling as required.
    pub info: Option<InfoObject>,
    /// The available paths and operations for the API.
    pub paths: Option<HashMap<String, PathItemObject>>,
    /// The incoming webhooks that MAY be received as part of this API and that the API consumer MAY choose to implement.
    pub webhooks: Option<HashMap<String, ReferenceOr<PathItemObject>>>,
    /// An element to hold various schemas for the document.
    pub components: Option<ComponentsObject>,
    /// A declaration of which security mechanisms can be used across the API.
    /// The list of values includes alternative security requirement objects that can be used.
    /// Only one of the security requirement objects need to be satisfied to authorize a request.
    pub security: Option<Vec<HashMap<String, Vec<String>>>>,
    /// A list of tags used by the document with additional metadata.
    pub tags: Option<Vec<TagObject>>,
    /// Additional external documentation.
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<Vec<ExternalDocumentationObject>>,
}

/// The object provides metadata about the API.
/// The metadata MAY be used by the clients if needed,
/// and MAY be presented in editing or documentation generation tools for convenience.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InfoObject {
    /// The title of the API.
    pub title: String,
    /// A short summary of the API.
    pub summary: Option<String>,
    /// A description of the API.
    pub description: Option<String>,
    /// A URL to the Terms of Service for the API.
    /// This MUST be in the form of a URL.
    #[serde(rename = "termsOfService")]
    pub terms_of_service: Option<String>,
    /// The contact information for the exposed API.
    pub contact: Option<ContactObject>,
    /// The license information for the exposed API.
    pub license: Option<LicenseObject>,
    /// The version of the OpenAPI document.
    pub version: String,
}

/// Contact information for the exposed API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContactObject {
    /// The identifying name of the contact person/organization.
    pub name: Option<String>,
    /// The URL pointing to the contact information.
    /// This MUST be in the form of a URL.
    pub url: Option<String>,
    /// The email address of the contact person/organization.
    /// This MUST be in the form of an email address.
    pub email: Option<String>,
}

/// License information for the exposed API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LicenseObject {
    /// The license name used for the API.
    pub name: String,
    /// An SPDX license expression for the API.
    /// The [identifier] field is mutually exclusive of the [url] field.
    pub identifier: Option<String>,
    /// A URL to the license used for the API.
    /// This MUST be in the form of a URL.
    /// The [url] field is mutually exclusive of the [identifier] field.
    pub url: Option<String>,
}

/// Describes the operations available on a single path.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PathItemObject {
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ReferenceOr<ParameterObject>>>,
}

/// Describes a single API operation on a path.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperationObject {
    /// A list of tags for API documentation control.
    pub tags: Option<Vec<String>>,
    /// A short summary of what the operation does.
    pub summary: Option<String>,
    /// A verbose explanation of the operation behavior.
    pub description: Option<String>,
    /// Additional external documentation for this operation.
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<String>,
    /// Unique string used to identify the operation.
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    /// A list of parameters that are applicable for this operation.
    pub parameters: Option<Vec<ReferenceOr<ParameterObject>>>,
    /// The request body applicable for this operation.
    #[serde(rename = "requestBody")]
    pub request_body: Option<ReferenceOr<RequestBodyObject>>,
    /// The list of possible responses as they are returned from executing this operation.
    pub responses: Option<HashMap<String, ReferenceOr<ResponseObject>>>,
    /// A map of possible out-of band callbacks related to the parent operation.
    pub callbacks:
        Option<HashMap<String, ReferenceOr<HashMap<String, ReferenceOr<PathItemObject>>>>>,
    /// Declares this operation to be deprecated.
    pub deprecated: Option<bool>,
    /// A declaration of which security mechanisms can be used for this operation.
    pub security: Option<Vec<HashMap<String, Vec<String>>>>,
    /// An alternative server array to service this operation.
    pub servers: Option<Vec<ServerObject>>,
}

/// Describes a single operation parameter.
/// A unique parameter is defined by a combination of a name and location.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParameterObject {
    pub name: String,
    pub r#in: String,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    #[serde(rename = "allowEmptyValue")]
    pub allow_empty_value: Option<bool>,
    pub style: Option<String>,
    pub explode: Option<bool>,
    #[serde(rename = "allowReserved")]
    pub allow_reserved: Option<bool>,
    pub schema: Option<SchemaObject>,
    pub example: Option<String>,
    pub examples: Option<HashMap<String, ReferenceOr<ExampleObject>>>,
}

/// Describes a single request body.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestBodyObject {
    /// A brief description of the request body.
    pub description: Option<String>,
    /// The content of the request body.
    /// The key is a media type or media type range and the value describes it.
    /// For requests that match multiple keys, only the most specific key is applicable.
    pub content: HashMap<String, MediaTypeObject>,
    /// Determines if the request body is required in the request. Defaults to `false`.
    pub required: Option<bool>,
}

/// Each Media Type Object provides model and examples for the media type identified by its key.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaTypeObject {
    /// The model defining the content of the request, response, or parameter.
    pub schema: Option<SchemaObject>,
    /// Example of the media type.
    /// The example object SHOULD be in the correct format as specified by the media type.
    pub example: Option<String>,
    /// Examples of the media type.
    /// Each example object SHOULD match the media type and specified model if present.
    pub examples: Option<HashMap<String, ReferenceOr<ExampleObject>>>,
    /// A map between a property name and its encoding information.
    /// The key, being the property name, MUST exist in the model as a property.
    pub encoding: Option<HashMap<String, EncodingObject>>,
}

/// The Schema Object allows the definition of input and output data types.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SchemaObject {
    pub discriminator: Option<DiscriminatorObject>,
    pub xml: Option<XMLObject>,
    pub external_docs: Option<ExternalDocumentationObject>,
    pub example: Option<String>,
}

/// When request bodies or response payloads may be one of a number of different schemas,
/// a discriminator object can be used to aid in serialization, deserialization, and validation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscriminatorObject {
    /// The name of the property in the payload that will hold the discriminator value.
    #[serde(rename = "propertyName")]
    pub property_name: String,
    /// An object to hold mappings between payload values and model names or references.
    pub mapping: HashMap<String, String>,
}

/// A metadata object that allows for more fine-tuned XML model definitions.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XMLObject {
    /// Replaces the name of the element/attribute used for the described model property.
    pub name: Option<String>,
    /// The URI of the namespace definition.
    /// This MUST be in the form of an absolute URI.
    pub namespace: Option<String>,
    /// The prefix to be used for the name.
    pub prefix: Option<String>,
    /// Declares whether the property definition translates to an attribute instead of an element.
    /// Default value is `false`.
    pub attribute: Option<bool>,
    /// Signifies whether the array is `wrapped` (for example, <books><book/><book/></books>)
    /// or `unwrapped` (<book/><book/>).
    /// Default value is `false`.
    pub wrapped: Option<bool>,
}

/// Allows referencing an external resource for extended documentation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalDocumentationObject {
    /// A description of the target documentation.
    pub description: Option<String>,
    /// The URL for the target documentation.
    /// This MUST be in the form of a URL.
    pub url: String,
}

/// Describes either internal or external examples.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExampleObject {
    /// Short description for the example.
    pub summary: Option<String>,
    /// Long description for the example.
    pub description: Option<String>,
    /// Embedded literal example.
    pub value: Option<String>,
    /// A URI that points to the literal example.
    #[serde(rename = "externalValue")]
    pub external_value: Option<String>,
}

/// A single encoding definition applied to a single model property.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncodingObject {
    /// The Content-Type for encoding a specific property.
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    /// A map allowing additional information to be provided as headers, for example Content-Disposition.
    pub headers: Option<HashMap<String, ReferenceOr<HeaderObject>>>,
    /// Describes how a specific property value will be serialized depending on its type.
    pub style: Option<String>,
    /// When this is true, property values of type array or object
    /// generate separate parameters for each value of the array, or key-value-pair of the map.
    pub explode: Option<bool>,
    /// Determines whether the parameter value SHOULD allow reserved characters,
    /// as defined by [RFC3986](https://tools.ietf.org/html/rfc3986#section-2.2) `:/?#[]@!$&'()*+,;=` to be included without percent-encoding.
    #[serde(rename = "allowReserved")]
    pub allow_reserved: Option<bool>,
}

/// The Header Object follows the structure of the Parameter Object with the following changes:
///
/// 1. `name` MUST NOT be specified, it is given in the corresponding headers map.
/// 2. `in` MUST NOT be specified, it is implicitly in header.
/// 3. All traits that are affected by the location MUST be applicable to a location of header.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeaderObject {
    pub description: Option<String>,
    pub required: Option<String>,
    pub deprecated: Option<bool>,
    #[serde(rename = "allowEmptyValue")]
    pub allow_empty_value: Option<bool>,
}

/// Describes a single response from an API Operation,
/// including design-time, static links to operations based on the response.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseObject {
    /// A description of the response.
    pub description: String,
    /// Maps a header name to its definition.
    pub headers: Option<HashMap<String, ReferenceOr<HeaderObject>>>,
    /// A map containing descriptions of potential response payloads.
    pub content: Option<HashMap<String, MediaTypeObject>>,
    /// A map of operations links that can be followed from the response.
    pub links: Option<HashMap<String, ReferenceOr<LinkObject>>>,
}

/// The Link object represents a possible design-time link for a response.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkObject {
    /// A relative or absolute URI reference to an OAS operation.
    #[serde(rename = "operationRef")]
    pub operation_ref: Option<String>,
    /// The name of an existing, resolvable OAS operation, as defined with a unique operationId.
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    /// A map representing parameters to pass to an operation
    /// as specified with `operationId` or identified via `operationRef`.
    pub parameters: Option<HashMap<String, String>>,
    /// A literal value or {expression} to use as a request body when calling the target operation.
    #[serde(rename = "requestBody")]
    pub request_body: Option<String>,
    /// A description of the link.
    pub description: Option<String>,
    /// A server object to be used by the target operation.
    pub server: Option<ServerObject>,
}

/// An object representing a Server.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerObject {
    /// A URL to the target host.
    pub url: String,
    /// An optional string describing the host designated by the URL.
    pub description: Option<String>,
    /// A map between a variable name and its value.
    pub variables: Option<HashMap<String, ServerVariableObject>>,
}

/// An object representing a Server Variable for server URL template substitution.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerVariableObject {
    /// An enumeration of string values to be used if the substitution options are from a limited set.
    /// The array MUST NOT be empty.
    pub r#enum: Option<Vec<String>>,
    /// The default value to use for substitution, which SHALL be sent if an alternate value is not supplied.
    pub default: String,
    /// An optional description for the server variable.
    pub description: Option<String>,
}

/// Holds a set of reusable objects for different aspects of the OAS.
/// All objects defined within the components object will have no effect on the API
/// unless they are explicitly referenced from properties outside the components object.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComponentsObject {
    pub schemas: Option<HashMap<String, SchemaObject>>,
    pub responses: Option<HashMap<String, ReferenceOr<ResponseObject>>>,
    pub parameters: Option<HashMap<String, ReferenceOr<ParameterObject>>>,
    pub examples: Option<HashMap<String, ReferenceOr<ExampleObject>>>,
    #[serde(rename = "requestBodies")]
    pub request_bodies: Option<HashMap<String, ReferenceOr<RequestBodyObject>>>,
    pub headers: Option<HashMap<String, ReferenceOr<HeaderObject>>>,
    #[serde(rename = "securitySchemes")]
    pub security_schemes: Option<HashMap<String, ReferenceOr<SecurityScheme>>>,
    pub links: Option<HashMap<String, ReferenceOr<LinkObject>>>,
    pub callbacks:
        Option<HashMap<String, ReferenceOr<HashMap<String, ReferenceOr<PathItemObject>>>>>,
    #[serde(rename = "pathItems")]
    pub path_items: Option<HashMap<String, ReferenceOr<PathItemObject>>>,
}

/// Defines a security scheme that can be used by the operations.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum SecurityScheme {
    #[serde(rename = "apiKey")]
    ApiKey {
        /// The name of the header, query or cookie parameter to be used.
        name: String,
        /// The location of the API key. Valid values are "query", "header" or "cookie".
        r#in: String,
        /// A description for security scheme.
        description: Option<String>,
    },
    #[serde(rename = "http")]
    Http {
        /// The name of the HTTP Authorization scheme to be used in the
        /// [Authorization header as defined in RFC7235](https://tools.ietf.org/html/rfc7235#section-5.1).
        scheme: String,
        /// A hint to the client to identify how the bearer token is formatted.
        #[serde(rename = "bearerFormat")]
        bearer_format: Option<String>,
        /// A description for security scheme.
        description: Option<String>,
    },
    #[serde(rename = "mutualTLS")]
    MutualTLS {
        /// A description for security scheme.
        description: Option<String>,
    },
    #[serde(rename = "oauth2")]
    OAuth2 {
        /// An object containing configuration information for the flow types supported.
        flows: OAuthFlowsObject,
        /// A description for security scheme.
        description: Option<String>,
    },
    #[serde(rename = "openIdConnect")]
    OpenIDConnect {
        /// OpenId Connect URL to discover OAuth2 configuration values.
        /// This MUST be in the form of a URL.
        #[serde(rename = "openIdConnectUrl")]
        open_id_connect_url: String,
        /// A description for security scheme.
        description: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OAuthFlowsObject {
    pub implicit: OAuthImplicitFlow,
    pub password: OAuthPasswordFlow,
    #[serde(rename = "clientCredentials")]
    pub client_credentials: OAuthClientCredentialsFlow,
    #[serde(rename = "authorizationCode")]
    pub authorization_code: OAuthAuthorizationCodeFlow,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OAuthImplicitFlow {
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: String,
    #[serde(rename = "refreshUrl")]
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OAuthPasswordFlow {
    #[serde(rename = "tokenUrl")]
    pub token_url: String,
    #[serde(rename = "refreshUrl")]
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OAuthClientCredentialsFlow {
    #[serde(rename = "tokenUrl")]
    pub token_url: String,
    #[serde(rename = "refreshUrl")]
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OAuthAuthorizationCodeFlow {
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: String,
    #[serde(rename = "tokenUrl")]
    pub token_url: String,
    #[serde(rename = "refreshUrl")]
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagObject {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentationObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ReferenceOr<T> {
    Value(T),
    Reference(ReferenceObject),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReferenceObject {
    #[serde(rename = "$ref")]
    pub reference: String,
    pub summary: Option<String>,
    pub description: Option<String>,
}

impl OpenAPI {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let file =
            File::open(path).map_err(|error| anyhow!("Could not load OpenAPI schema. {error}"))?;
        serde_yaml::from_reader(file).map_err(Into::into)
    }
}

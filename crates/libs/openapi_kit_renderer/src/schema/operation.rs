use crate::schema::parameter::OpenAPIParameter;
use crate::schema::request::OpenAPIRequest;
use crate::schema::resolve_schema;
use crate::schema::response::OpenAPIResponse;
use indexmap::IndexMap;
use openapi_kit_schema::{ParameterSchemaOrContent, ReferenceOr, RequestBody};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAPIOperation {
    pub operation_id: Option<String>,
    pub parameters: Vec<OpenAPIParameter>,
    pub request: Option<OpenAPIRequest>,
    pub response: IndexMap<String, OpenAPIResponse>,
}

impl From<openapi_kit_schema::Operation> for OpenAPIOperation {
    fn from(value: openapi_kit_schema::Operation) -> Self {
        OpenAPIOperation {
            operation_id: value.operation_id,
            parameters: convert_parameters(value.parameters),
            request: value.request_body.and_then(convert_request),
            response: convert_response(value.responses),
        }
    }
}

fn convert_parameters(
    parameters: Vec<ReferenceOr<openapi_kit_schema::Parameter>>,
) -> Vec<OpenAPIParameter> {
    let mut output = Vec::new();

    for ref_or_parameter in parameters {
        match ref_or_parameter {
            ReferenceOr::Reference { reference: _ } => {
                continue;
            }
            ReferenceOr::Item(parameter) => {
                let location = match parameter {
                    openapi_kit_schema::Parameter::Header { .. } => "header",
                    openapi_kit_schema::Parameter::Cookie { .. } => "cookie",
                    openapi_kit_schema::Parameter::Path { .. } => "path",
                    openapi_kit_schema::Parameter::Query { .. } => "query",
                }
                .to_string();

                let data = parameter.parameter_data();

                output.push(OpenAPIParameter {
                    r#in: location,
                    kind: match data.format {
                        ParameterSchemaOrContent::Content(_) => {
                            continue;
                        }
                        ParameterSchemaOrContent::Schema(ref_or_schema) => {
                            resolve_schema(ref_or_schema)
                        }
                    },
                    name: data.name,
                });
            }
        }
    }

    output
}

fn convert_response(
    responses_object: openapi_kit_schema::Responses,
) -> IndexMap<String, OpenAPIResponse> {
    let mut responses = IndexMap::new();

    for (code, response) in responses_object.responses {
        if let ReferenceOr::Item(response) = response {
            for (media_type, media) in response.content {
                if let Some(ref_or_schema) = media.schema {
                    responses.insert(
                        code.to_string(),
                        OpenAPIResponse {
                            kind: resolve_schema(ref_or_schema),
                            media: media_type.to_string(),
                        },
                    );
                }
            }
        } else {
            println!("Currently no support for $ref in response.");
        }
    }

    responses
}

fn convert_request(ref_or_request: ReferenceOr<RequestBody>) -> Option<OpenAPIRequest> {
    if let ReferenceOr::Item(request) = ref_or_request {
        for (media_type, media) in request.content {
            if let Some(ref_or_schema) = media.schema {
                return Some(OpenAPIRequest {
                    kind: resolve_schema(ref_or_schema),
                    media: media_type.to_string(),
                });
            }
        }
    } else {
        println!("Currently no support for $ref in response.");
    }

    None
}

use utoipa::openapi::{
    Content, HeaderBuilder, Info, ObjectBuilder, OpenApi, OpenApiBuilder, Ref, Required, Type,
    path::{
        HttpMethod, OperationBuilder, ParameterBuilder, ParameterIn, PathItemBuilder, PathsBuilder,
    },
    response::{ResponseBuilder, ResponsesBuilder},
    schema::{AdditionalProperties, ComponentsBuilder, SchemaType},
};

use crate::{
    APPLICATION_DID, APPLICATION_DID_RESOLUTION, APPLICATION_JSON, MAX_ACCEPT_HEADER_BYTES,
    MAX_ACCEPT_MEDIA_RANGES, MAX_RESOLUTION_QUERY_BYTES, MAX_RESOLUTION_QUERY_NAME_BYTES,
    MAX_RESOLUTION_QUERY_PARAMETERS, MAX_RESOLUTION_QUERY_VALUE_BYTES,
};

const DOCUMENT_SCHEMA: &str = "DidDocument";
const RESOLUTION_RESULT_SCHEMA: &str = "DidResolutionResult";

/// Build the OpenAPI 3.1 document for the fixed, mount-relative `GET /{did}` route.
///
/// The returned document describes only the transport contract owned by this
/// crate. Consumers remain responsible for mounting the router, publishing the
/// document, and choosing any UI, server, authentication, or deployment policy.
pub fn did_resolver_http_openapi() -> OpenApi {
    let operation = OperationBuilder::new()
        .summary(Some("Resolve a DID"))
        .description(Some(format!(
            "Resolve one DID through the injected resolver. The aggregate `Accept` header is limited to {MAX_ACCEPT_HEADER_BYTES} bytes and {MAX_ACCEPT_MEDIA_RANGES} media ranges. The raw query is limited to {MAX_RESOLUTION_QUERY_BYTES} bytes, {MAX_RESOLUTION_QUERY_PARAMETERS} parameters, {MAX_RESOLUTION_QUERY_NAME_BYTES} bytes per decoded name, and {MAX_RESOLUTION_QUERY_VALUE_BYTES} bytes per decoded value. Unknown query names are retained as bounded extension options."
        )))
        .operation_id(Some("resolveDid"))
        .parameter(string_parameter(
            "did",
            ParameterIn::Path,
            Required::True,
            "DID path value to resolve.",
        ))
        .parameter(string_parameter(
            "Accept",
            ParameterIn::Header,
            Required::False,
            "Requested response media ranges. Supported representations are `application/did`, `application/json`, and `application/did-resolution`; absence defaults to `application/did`.",
        ))
        .parameter(boolean_query_parameter(
            "expandRelativeUrls",
            "Whether relative DID document URLs should be expanded.",
        ))
        .parameter(boolean_query_parameter(
            "noCache",
            "Whether cached resolution results should be bypassed.",
        ))
        .parameter(string_parameter(
            "versionId",
            ParameterIn::Query,
            Required::False,
            "Requested DID document version identifier.",
        ))
        .parameter(string_parameter(
            "versionTime",
            ParameterIn::Query,
            Required::False,
            "Requested DID document version time as an RFC 3339 timestamp.",
        ))
        .responses(
            ResponsesBuilder::new()
                .response("200", success_response())
                .response("400", error_response("The DID or resolution options are invalid."))
                .response("404", error_response("The DID was not found."))
                .response("406", error_response("No requested representation is available."))
                .response("410", error_response("The DID is deactivated."))
                .response("500", error_response("Resolution failed internally."))
                .response("501", error_response("The DID method is not supported.")),
        )
        .build();

    OpenApiBuilder::new()
        .info(Info::new(
            "Identus DID Resolution HTTP",
            env!("CARGO_PKG_VERSION"),
        ))
        .paths(
            PathsBuilder::new()
                .path(
                    "/{did}",
                    PathItemBuilder::new()
                        .operation(HttpMethod::Get, operation)
                        .build(),
                )
                .build(),
        )
        .components(Some(
            ComponentsBuilder::new()
                .schema(DOCUMENT_SCHEMA, did_document_schema())
                .schema(RESOLUTION_RESULT_SCHEMA, resolution_result_schema())
                .build(),
        ))
        .build()
}

fn string_parameter(
    name: &str,
    parameter_in: ParameterIn,
    required: Required,
    description: &str,
) -> utoipa::openapi::path::Parameter {
    ParameterBuilder::new()
        .name(name)
        .parameter_in(parameter_in)
        .required(required)
        .description(Some(description))
        .schema(Some(ObjectBuilder::new().schema_type(Type::String)))
        .build()
}

fn boolean_query_parameter(name: &str, description: &str) -> utoipa::openapi::path::Parameter {
    ParameterBuilder::new()
        .name(name)
        .parameter_in(ParameterIn::Query)
        .required(Required::False)
        .description(Some(description))
        .schema(Some(ObjectBuilder::new().schema_type(Type::Boolean)))
        .build()
}

fn success_response() -> utoipa::openapi::response::Response {
    ResponseBuilder::new()
        .description("DID document or complete DID Resolution result.")
        .header("Vary", vary_header())
        .content(
            APPLICATION_DID,
            Content::new(Some(Ref::from_schema_name(DOCUMENT_SCHEMA))),
        )
        .content(
            APPLICATION_JSON,
            Content::new(Some(Ref::from_schema_name(DOCUMENT_SCHEMA))),
        )
        .content(
            APPLICATION_DID_RESOLUTION,
            Content::new(Some(Ref::from_schema_name(RESOLUTION_RESULT_SCHEMA))),
        )
        .build()
}

fn error_response(description: &str) -> utoipa::openapi::response::Response {
    ResponseBuilder::new()
        .description(description)
        .header("Vary", vary_header())
        .content(
            APPLICATION_DID_RESOLUTION,
            Content::new(Some(Ref::from_schema_name(RESOLUTION_RESULT_SCHEMA))),
        )
        .build()
}

fn vary_header() -> utoipa::openapi::header::Header {
    HeaderBuilder::new()
        .schema(ObjectBuilder::new().schema_type(Type::String))
        .description(Some("Always `Accept`."))
        .build()
}

fn free_form_object() -> ObjectBuilder {
    ObjectBuilder::new()
        .schema_type(Type::Object)
        .additional_properties(Some(AdditionalProperties::FreeForm(true)))
}

fn did_document_schema() -> ObjectBuilder {
    free_form_object()
        .description(Some(
            "Extensible W3C DID document. Method-specific and extension properties are preserved.",
        ))
        .property("id", ObjectBuilder::new().schema_type(Type::String))
        .required("id")
}

fn resolution_result_schema() -> ObjectBuilder {
    ObjectBuilder::new()
        .schema_type(Type::Object)
        .description(Some("W3C DID Resolution result envelope."))
        .property("didResolutionMetadata", free_form_object())
        .property(
            "didDocument",
            free_form_object().schema_type(
                [Type::Object, Type::Null]
                    .into_iter()
                    .collect::<SchemaType>(),
            ),
        )
        .property("didDocumentMetadata", free_form_object())
        .required("didResolutionMetadata")
        .required("didDocument")
        .required("didDocumentMetadata")
}

#[cfg(test)]
mod tests {
    use super::did_resolver_http_openapi;
    use crate::{APPLICATION_DID, APPLICATION_DID_RESOLUTION, APPLICATION_JSON};

    #[test]
    fn openapi_document_is_deterministic_and_bounded_to_get_resolution() {
        let first = serde_json::to_value(did_resolver_http_openapi()).expect("serialize OpenAPI");
        let second = serde_json::to_value(did_resolver_http_openapi()).expect("serialize OpenAPI");

        assert_eq!(first, second);
        assert_eq!(first["openapi"], "3.1.0");
        assert_eq!(
            first["paths"]
                .as_object()
                .expect("paths")
                .keys()
                .collect::<Vec<_>>(),
            vec!["/{did}"]
        );
        assert!(first["paths"]["/{did}"]["get"].is_object());
        assert!(first["paths"]["/{did}"]["post"].is_null());

        let parameters = first["paths"]["/{did}"]["get"]["parameters"]
            .as_array()
            .expect("parameters");
        let actual = parameters
            .iter()
            .map(|parameter| {
                (
                    parameter["name"].as_str().expect("parameter name"),
                    parameter["in"].as_str().expect("parameter location"),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            actual,
            vec![
                ("did", "path"),
                ("Accept", "header"),
                ("expandRelativeUrls", "query"),
                ("noCache", "query"),
                ("versionId", "query"),
                ("versionTime", "query"),
            ]
        );

        let responses = first["paths"]["/{did}"]["get"]["responses"]
            .as_object()
            .expect("responses");
        assert_eq!(
            responses.keys().map(String::as_str).collect::<Vec<_>>(),
            vec!["200", "400", "404", "406", "410", "500", "501"]
        );
        for response in responses.values() {
            assert_eq!(
                response["headers"]["Vary"]["description"],
                "Always `Accept`."
            );
        }
        assert!(responses["200"]["content"][APPLICATION_DID].is_object());
        assert!(responses["200"]["content"][APPLICATION_JSON].is_object());
        assert!(responses["200"]["content"][APPLICATION_DID_RESOLUTION].is_object());
        for status in ["400", "404", "406", "410", "500", "501"] {
            assert!(responses[status]["content"][APPLICATION_DID_RESOLUTION].is_object());
        }

        let description = first["paths"]["/{did}"]["get"]["description"]
            .as_str()
            .expect("operation description");
        for expected in ["8192", "32", "256", "4096", "extension options"] {
            assert!(description.contains(expected), "missing `{expected}`");
        }
    }
}

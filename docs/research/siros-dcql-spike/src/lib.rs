#![forbid(unsafe_code)]

use siros_dcql::DcqlQuery;

/// Research-only upper bound applied before the candidate sees verifier JSON.
pub const MAX_QUERY_BYTES: usize = 16 * 1024;

/// Candidate-free outcome used to prove that upstream types need not escape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuerySummary {
    /// Number of credential queries after bounded parsing.
    pub credential_queries: usize,
    /// Number of credential-set constraints after bounded parsing.
    pub credential_sets: usize,
}

/// Bounded and redacted adapter errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterError {
    /// Input exceeded [`MAX_QUERY_BYTES`] before candidate parsing.
    QueryTooLarge,
    /// Input was not UTF-8 or the candidate rejected its JSON/query shape.
    InvalidQuery,
}

/// Parse through the candidate without exposing its models or diagnostics.
///
/// # Errors
///
/// Returns a category-only error. It never includes verifier-controlled bytes
/// or identifiers.
pub fn parse_query(input: &[u8]) -> Result<QuerySummary, AdapterError> {
    if input.len() > MAX_QUERY_BYTES {
        return Err(AdapterError::QueryTooLarge);
    }
    let text = core::str::from_utf8(input).map_err(|_| AdapterError::InvalidQuery)?;
    let query = DcqlQuery::from_json(text).map_err(|_| AdapterError::InvalidQuery)?;
    Ok(QuerySummary {
        credential_queries: query.credentials.len(),
        credential_sets: query.credential_sets.as_ref().map_or(0, Vec::len),
    })
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};
    use siros_dcql::{
        Credential, DcqlQuery, ExactFormat, PathComponent, PathError, execute, resolve_json,
    };

    use super::{AdapterError, MAX_QUERY_BYTES, QuerySummary, parse_query};

    struct JsonCredential {
        id: &'static str,
        format: &'static str,
        claims: Value,
        holder_bound: bool,
    }

    impl Credential for JsonCredential {
        fn id(&self) -> &str {
            self.id
        }

        fn format(&self) -> &str {
            self.format
        }

        fn claim(&self, path: &[PathComponent]) -> Result<Vec<Value>, PathError> {
            resolve_json(&self.claims, path).map(|values| values.into_iter().cloned().collect())
        }

        fn has_cryptographic_holder_binding(&self) -> bool {
            self.holder_bound
        }
    }

    #[test]
    fn bounded_adapter_returns_candidate_free_summary() {
        let input = br#"{"credentials":[{"id":"pid","format":"dc+sd-jwt","meta":{}}]}"#;
        assert_eq!(
            parse_query(input),
            Ok(QuerySummary {
                credential_queries: 1,
                credential_sets: 0,
            })
        );
    }

    #[test]
    fn rejects_before_candidate_parsing_and_redacts_errors() {
        let oversized = vec![b'x'; MAX_QUERY_BYTES + 1];
        assert_eq!(parse_query(&oversized), Err(AdapterError::QueryTooLarge));

        let canary = "VERIFIER-CONTROLLED-CANARY";
        let invalid = format!(r#"{{"credentials":[{{"id":"{canary}"}}]}}"#);
        let error = parse_query(invalid.as_bytes()).expect_err("missing format must fail");
        assert_eq!(error, AdapterError::InvalidQuery);
        assert!(!format!("{error:?}").contains(canary));
    }

    #[test]
    fn engine_selects_only_a_complete_bound_credential() {
        let query = DcqlQuery::from_json(
            r#"{
                "credentials":[{
                    "id":"pid",
                    "format":"dc+sd-jwt",
                    "meta":{},
                    "claims":[{"id":"given_name","path":["given_name"]}]
                }]
            }"#,
        )
        .expect("fixed query is valid");
        let credentials = [
            JsonCredential {
                id: "complete",
                format: "dc+sd-jwt",
                claims: json!({"given_name": "Erika"}),
                holder_bound: true,
            },
            JsonCredential {
                id: "missing-claim",
                format: "dc+sd-jwt",
                claims: json!({"family_name": "Mustermann"}),
                holder_bound: true,
            },
            JsonCredential {
                id: "unbound",
                format: "dc+sd-jwt",
                claims: json!({"given_name": "Erika"}),
                holder_bound: false,
            },
        ];

        let result = execute(&query, &credentials, &ExactFormat);
        assert!(result.satisfiable);
        let candidates = &result.query("pid").expect("query result exists").candidates;
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].credential_id, "complete");
        assert_eq!(candidates[0].claims.len(), 1);
    }

    #[test]
    fn tolerant_candidate_semantics_are_explicit_mismatches() {
        let missing_meta =
            DcqlQuery::from_json(r#"{"credentials":[{"id":"pid","format":"dc+sd-jwt"}]}"#)
                .expect("candidate deliberately defaults missing meta");
        assert!(missing_meta.credentials[0].meta.is_empty());

        let non_conforming_identifier = DcqlQuery::from_json(
            r#"{"credentials":[{"id":"identifier.with.dot","format":"dc+sd-jwt","meta":{}}]}"#,
        )
        .expect("candidate deliberately tolerates identifier grammar");
        assert_eq!(
            non_conforming_identifier.credentials[0].id,
            "identifier.with.dot"
        );
    }
}

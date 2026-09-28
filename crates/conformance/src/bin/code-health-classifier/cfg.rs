//! Three-valued cfg and cfg_attr evaluation.

use syn::{
    Attribute, LitBool, Meta, MetaList, Token,
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    token::Comma,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Truth {
    True,
    False,
    Unknown,
}

impl Truth {
    fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::False, _) | (_, Self::False) => Self::False,
            (Self::True, Self::True) => Self::True,
            _ => Self::Unknown,
        }
    }

    fn not(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Unknown => Self::Unknown,
        }
    }
}

pub(super) fn normalized_ident(ident: &syn::Ident) -> String {
    let rendered = ident.to_string();
    rendered.strip_prefix("r#").unwrap_or(&rendered).to_owned()
}

fn path_ident(meta: &Meta) -> Option<String> {
    let Meta::Path(path) = meta else {
        return None;
    };
    let ident = path.get_ident()?;
    Some(normalized_ident(ident))
}

enum CfgPredicate {
    Boolean(bool),
    Meta(Meta),
}

impl Parse for CfgPredicate {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(LitBool) {
            return input
                .parse::<LitBool>()
                .map(|value| Self::Boolean(value.value));
        }
        input.parse().map(Self::Meta)
    }
}

fn predicate_truth_for_test(predicate: &CfgPredicate, test_enabled: bool) -> Truth {
    match predicate {
        CfgPredicate::Boolean(true) => Truth::True,
        CfgPredicate::Boolean(false) => Truth::False,
        CfgPredicate::Meta(meta) => cfg_truth_for_test(meta, test_enabled),
    }
}

pub(super) fn cfg_truth_for_test(meta: &Meta, test_enabled: bool) -> Truth {
    match meta {
        Meta::Path(_) => match path_ident(meta).as_deref() {
            Some("test") if test_enabled => Truth::True,
            Some("test" | "false") => Truth::False,
            Some("true") => Truth::True,
            _ => Truth::Unknown,
        },
        Meta::NameValue(_) => Truth::Unknown,
        Meta::List(list) => {
            let Some(name) = list.path.get_ident().map(normalized_ident) else {
                return Truth::Unknown;
            };
            let parser = Punctuated::<CfgPredicate, Comma>::parse_terminated;
            let Ok(values) = parser.parse2(list.tokens.clone()) else {
                return Truth::Unknown;
            };
            match name.as_str() {
                "all" => values.iter().fold(Truth::True, |result, value| {
                    result.and(predicate_truth_for_test(value, test_enabled))
                }),
                "any" => {
                    let mut unknown = false;
                    for value in &values {
                        match predicate_truth_for_test(value, test_enabled) {
                            Truth::True => return Truth::True,
                            Truth::Unknown => unknown = true,
                            Truth::False => {}
                        }
                    }
                    if unknown {
                        Truth::Unknown
                    } else {
                        Truth::False
                    }
                }
                "not" if values.len() == 1 => {
                    predicate_truth_for_test(&values[0], test_enabled).not()
                }
                _ => Truth::Unknown,
            }
        }
    }
}

fn meta_inclusion_for_test(meta: &Meta, test_enabled: bool) -> Truth {
    let Some(name) = meta.path().get_ident().map(normalized_ident) else {
        return Truth::True;
    };
    match (name.as_str(), meta) {
        ("cfg", Meta::List(list)) => syn::parse2::<CfgPredicate>(list.tokens.clone())
            .map_or(Truth::Unknown, |value| {
                predicate_truth_for_test(&value, test_enabled)
            }),
        ("cfg_attr", Meta::List(list)) => {
            let Ok((predicate, values)) = cfg_attr_parts(list, test_enabled) else {
                return Truth::Unknown;
            };
            if predicate == Truth::False {
                return Truth::True;
            }
            let applied = values.iter().fold(Truth::True, |result, value| {
                result.and(meta_inclusion_for_test(value, test_enabled))
            });
            match predicate {
                Truth::True => applied,
                Truth::False => Truth::True,
                Truth::Unknown if applied == Truth::True => Truth::True,
                Truth::Unknown => Truth::Unknown,
            }
        }
        _ => Truth::True,
    }
}

pub(super) fn cfg_attr_parts(
    list: &MetaList,
    test_enabled: bool,
) -> syn::Result<(Truth, Vec<Meta>)> {
    let parser = |input: ParseStream<'_>| {
        let predicate: CfgPredicate = input.parse()?;
        let mut applied = Vec::new();
        while !input.is_empty() {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
            applied.push(input.parse()?);
        }
        Ok((predicate_truth_for_test(&predicate, test_enabled), applied))
    };
    parser.parse2(list.tokens.clone())
}

pub(super) fn attributes_inclusion(attributes: &[Attribute]) -> Truth {
    attributes_inclusion_for_test(attributes, false)
}

pub(super) fn attributes_inclusion_for_test(attributes: &[Attribute], test_enabled: bool) -> Truth {
    attributes.iter().fold(Truth::True, |result, attribute| {
        result.and(meta_inclusion_for_test(&attribute.meta, test_enabled))
    })
}

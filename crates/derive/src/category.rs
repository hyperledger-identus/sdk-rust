//! Inner-type classification for `#[derive(Newtype)]`.

/// The three newtype categories the derive dispatches on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Category {
    /// Owned string (`String`).
    Str,
    /// Owned bytes (`Vec<u8>`).
    Bytes,
    /// A numeric primitive (`u8`..`u128`, `i8`..`i128`, `usize`/`isize`,
    /// `f32`/`f64`).
    Num,
}

/// Classify `ty` into a category, or emit a compile error listing the
/// supported inner types.
pub(crate) fn classify(ty: &syn::Type) -> syn::Result<Category> {
    if let syn::Type::Path(tp) = ty
        && tp.qself.is_none()
        && let Some(seg) = tp.path.segments.last()
    {
        return match seg.ident.to_string().as_str() {
            "String" => Ok(Category::Str),
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64"
            | "i128" | "isize" | "f32" | "f64" => Ok(Category::Num),
            "Vec" => {
                if is_vec_u8(seg) {
                    Ok(Category::Bytes)
                } else {
                    Err(syn::Error::new_spanned(
                        ty,
                        "Newtype `Vec` inner type must be `Vec<u8>`",
                    ))
                }
            }
            _ => Err(unrecognised(ty)),
        };
    }
    Err(unrecognised(ty))
}

fn is_vec_u8(seg: &syn::PathSegment) -> bool {
    let syn::PathArguments::AngleBracketed(ab) = &seg.arguments else {
        return false;
    };
    ab.args.len() == 1 && matches!(&ab.args[0], syn::GenericArgument::Type(t) if is_u8(t))
}

fn is_u8(ty: &syn::Type) -> bool {
    if let syn::Type::Path(tp) = ty {
        tp.qself.is_none() && tp.path.segments.len() == 1 && tp.path.segments[0].ident == "u8"
    } else {
        false
    }
}

fn unrecognised(ty: &syn::Type) -> syn::Error {
    syn::Error::new_spanned(
        ty,
        "Newtype inner type is not supported; expected `String`, `Vec<u8>`, \
        or a numeric primitive (`u8`..`u128`, `i8`..`i128`, `usize`, \
        `isize`, `f32`, `f64`)",
    )
}

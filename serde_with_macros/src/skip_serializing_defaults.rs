use crate::field_has_attribute;
use quote::quote;
use syn::{parse_quote, Field, Type};

fn underlying_type(type_: &Type) -> Option<String> {
    match type_ {
        Type::Path(syn::TypePath { qself: None, path })
            if path.leading_colon.is_none() && path.segments.len() == 1 =>
        {
            Some(path.segments[0].ident.to_string())
        }
        _ => None,
    }
}

/// Add the skip_serializing_if annotation to each field of the struct
pub fn skip_serializing_defaults_add_attr_to_field(
    field: &mut Field,
    enabled_types: &[String],
) -> Result<(), String> {
    let has_skip_serializing_if = field_has_attribute(field, "serde", "skip_serializing_if");
    // Remove the `serialize_always` attribute
    let mut has_always_attr = false;
    field.attrs.retain(|attr| {
        let has_attr = attr.path().is_ident("serialize_always");
        has_always_attr |= has_attr;
        !has_attr
    });
    // Error on conflicting attributes
    if has_always_attr && has_skip_serializing_if {
        let mut msg = r#"The attributes `serialize_always` and `serde(skip_serializing_if = "...")` cannot be used on the same field"#.to_string();
        if let Some(ident) = &field.ident {
            msg += ": `";
            msg += &ident.to_string();
            msg += "`";
        }
        msg += ".";
        return Err(msg);
    }

    let Some(supported_type) = underlying_type(&field.ty).filter(|type_| enabled_types.contains(type_)) else {
        if has_always_attr {
            return Err("`serialize_always` may only be used on fields of type enabled types.".into());
        }
        return Ok(())
    };
    // Do nothing if `skip_serializing_if` or `serialize_always` is already present
    if has_skip_serializing_if || has_always_attr {
        return Ok(());
    }

    let check_path = match supported_type.as_str() {
        "Option" => "Option::is_none",
        "Vec" => "Vec::is_empty",
        _ => "serde_with::is_default_value",
    };
    let check_path = quote!(#check_path);

    // Add the `skip_serializing_if` attribute
    let attr = parse_quote!(
        #[serde(default, skip_serializing_if = #check_path)]
    );
    field.attrs.push(attr);

    Ok(())
}

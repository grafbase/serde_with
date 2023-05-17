use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with_macros::minify_variant_names;

#[minify_variant_names]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
enum EnumWithLongVariantNames {
    LongVariantName { field_name: usize },
    AnotherLongVariantName { field_name: usize },
    AnotherLongishVariantName { field_name: usize },
}

#[test]
fn test_minify_variant_names() {
    let input = EnumWithLongVariantNames::LongVariantName { field_name: 1 };
    let result = serde_json::to_value(&input).unwrap();
    assert_eq!(
        json!({
            "l": { "field_name": 1 }
        }),
        result
    );
    assert_eq!(input, serde_json::from_value(result).unwrap());

    let input = EnumWithLongVariantNames::AnotherLongVariantName { field_name: 1 };
    let result = serde_json::to_value(&input).unwrap();
    assert_eq!(
        json!({
            "a": { "field_name": 1 }
        }),
        result
    );
    assert_eq!(input, serde_json::from_value(result).unwrap());

    let input = EnumWithLongVariantNames::AnotherLongishVariantName { field_name: 1 };
    let result = serde_json::to_value(&input).unwrap();
    assert_eq!(
        json!({
            "A": { "field_name": 1 }
        }),
        result
    );
    assert_eq!(input, serde_json::from_value(result).unwrap());
}

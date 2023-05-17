use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with_macros::minify_field_names;

#[minify_field_names]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct StructWithLongFieldNames {
    long_field_name: usize,
    another_field_name: usize,
    another_field_name_that_shares_a_prefix_with_the_previous_one: usize,
    #[serde(rename = "preserved_field_name")]
    preserved_field_name: usize,
}

#[test]
fn test_minify_field_names() {
    let expected = json!({
        "l": 1,
        "a": 2,
        "A": 3,
        "preserved_field_name": 4,
    });
    let input = StructWithLongFieldNames {
        long_field_name: 1,
        another_field_name: 2,
        another_field_name_that_shares_a_prefix_with_the_previous_one: 3,
        preserved_field_name: 4,
    };
    let res = serde_json::to_value(&input).unwrap();
    assert_eq!(expected, res);
    assert_eq!(input, serde_json::from_value(res).unwrap());
}

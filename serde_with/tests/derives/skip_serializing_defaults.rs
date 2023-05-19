use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with_macros::skip_serializing_defaults;

macro_rules! test {
    ($fn:ident, $struct:ident) => {
        #[test]
        fn $fn() {
            let expected = json!({});
            let data = $struct {
                a: None,
                b: None,
                c: None,
                d: None,
            };
            let res = serde_json::to_value(&data).unwrap();
            assert_eq!(expected, res);
            assert_eq!(data, serde_json::from_value(res).unwrap());
        }
    };
}

#[serde_with::skip_serializing_defaults(Option)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct DataBasic {
    a: Option<String>,
    b: Option<String>,
    c: Option<String>,
    d: Option<String>,
}
test!(test_basic, DataBasic);

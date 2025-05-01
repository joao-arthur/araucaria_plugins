use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Value {
    None,
    U64(u64),
    I64(i64),
    F64(f64),
    USize(usize),
    ISize(isize),
    Bool(bool),
    Str(String),
    Arr(Vec<Value>),
    Obj(BTreeMap<String, Value>),
}

pub fn to_value(value: araucaria::value::Value) -> Value {
    match value {
        araucaria::value::Value::None => Value::None,
        araucaria::value::Value::U64(value) => Value::U64(value),
        araucaria::value::Value::I64(value) => Value::I64(value),
        araucaria::value::Value::F64(value) => Value::F64(value),
        araucaria::value::Value::USize(value) => Value::USize(value),
        araucaria::value::Value::ISize(value) => Value::ISize(value),
        araucaria::value::Value::Bool(value) => Value::Bool(value),
        araucaria::value::Value::Str(value) => Value::Str(value),
        araucaria::value::Value::Arr(value) => Value::Arr(value.into_iter().map(to_value).collect()),
        araucaria::value::Value::Obj(value) => Value::Obj(value.into_iter().map(|(k, v)| (k.clone(), to_value(v))).collect()),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{Value, to_value};

    #[test]
    fn araucaria_value_to_value() {
        assert_eq!(to_value(araucaria::value::Value::None), Value::None);
        assert_eq!(to_value(araucaria::value::Value::U64(12)), Value::U64(12));
        assert_eq!(to_value(araucaria::value::Value::I64(-34)), Value::I64(-34));
        assert_eq!(to_value(araucaria::value::Value::F64(-64.5)), Value::F64(-64.5));
        assert_eq!(to_value(araucaria::value::Value::USize(84)), Value::USize(84));
        assert_eq!(to_value(araucaria::value::Value::ISize(-79)), Value::ISize(-79));
    }

    #[test]
    fn serialize_value() {
        assert_eq!(serde_json::to_string(&Value::None).unwrap(), r#""None""#.to_string());
        assert_eq!(serde_json::to_string(&Value::U64(12)).unwrap(), r#"{"U64":12}"#.to_string());
        assert_eq!(serde_json::to_string(&Value::I64(-34)).unwrap(), r#"{"I64":-34}"#.to_string());
        assert_eq!(serde_json::to_string(&Value::F64(-64.5)).unwrap(), r#"{"F64":-64.5}"#.to_string());
        assert_eq!(serde_json::to_string(&Value::USize(84)).unwrap(), r#"{"USize":84}"#.to_string());
        assert_eq!(serde_json::to_string(&Value::ISize(-79)).unwrap(), r#"{"ISize":-79}"#.to_string());
    }
}

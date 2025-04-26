use std::collections::BTreeMap;

use araucaria::{validation, value::Value};

pub fn value_from_json(value: &serde_json::Value) -> Value {
    match value {
        serde_json::Value::Number(num) => {
            if let Some(num) = num.as_u64() {
                return Value::U64(num);
            }
            if let Some(num) = num.as_i64() {
                return Value::I64(num);
            }
            if let Some(num) = num.as_f64() {
                return Value::F64(num);
            }
            Value::None
        }
        serde_json::Value::Bool(bool) => Value::Bool(*bool),
        serde_json::Value::String(str) => Value::Str(str.clone()),
        serde_json::Value::Array(arr) => Value::Arr(arr.iter().map(|item| value_from_json(item)).collect()),
        serde_json::Value::Object(obj) => {
            let mut result: BTreeMap<String, Value> = BTreeMap::new();
            for (key, item) in obj {
                result.insert(key.clone(), value_from_json(item));
            }
            Value::Obj(result)
        }
        serde_json::Value::Null => Value::None,
    }
}

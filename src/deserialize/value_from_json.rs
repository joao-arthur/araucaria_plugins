use std::collections::BTreeMap;

use araucaria::value::Value;

pub fn value_from_json(json: &serde_json::Value) -> Value {
    match json {
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
        serde_json::Value::Array(arr) => Value::Arr(arr.iter().map(value_from_json).collect()),
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use araucaria::value::Value;

    use super::value_from_json;

    #[test]
    fn value_from_json_number() {
        let json_u64 = serde_json::Value::Number(serde_json::Number::from_u128(192).unwrap());
        let json_i64_pos = serde_json::Value::Number(serde_json::Number::from_i128(192).unwrap());
        let json_i64_neg = serde_json::Value::Number(serde_json::Number::from_i128(-192).unwrap());
        let json_f64_pos = serde_json::Value::Number(serde_json::Number::from_f64(192.0).unwrap());
        let json_f64_neg = serde_json::Value::Number(serde_json::Number::from_f64(-192.0).unwrap());
        let json_f64_pos_float = serde_json::Value::Number(serde_json::Number::from_f64(192.5).unwrap());
        let json_f64_neg_float = serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap());

        assert_eq!(value_from_json(&json_u64), Value::U64(192));
        assert_eq!(value_from_json(&json_i64_pos), Value::U64(192));
        assert_eq!(value_from_json(&json_i64_neg), Value::I64(-192));
        assert_eq!(value_from_json(&json_f64_pos), Value::F64(192.0));
        assert_eq!(value_from_json(&json_f64_neg), Value::F64(-192.0));
        assert_eq!(value_from_json(&json_f64_pos_float), Value::F64(192.5));
        assert_eq!(value_from_json(&json_f64_neg_float), Value::F64(-192.5));
    }

    #[test]
    fn value_from_json_bool() {
        assert_eq!(value_from_json(&serde_json::Value::Bool(false)), Value::Bool(false));
        assert_eq!(value_from_json(&serde_json::Value::Bool(true)), Value::Bool(true));
    }

    #[test]
    fn value_from_json_string() {
        assert_eq!(value_from_json(&serde_json::Value::String("Naruto".into())), Value::Str("Naruto".into()));
        assert_eq!(value_from_json(&serde_json::Value::String("chuck@gmail.com".into())), Value::Str("chuck@gmail.com".into()));
        assert_eq!(value_from_json(&serde_json::Value::String("2025-04-26".into())), Value::Str("2025-04-26".into()));
        assert_eq!(value_from_json(&serde_json::Value::String("16:55".into())), Value::Str("16:55".into()));
        assert_eq!(value_from_json(&serde_json::Value::String("2025-04-26T16:55Z".into())), Value::Str("2025-04-26T16:55Z".into()));
    }

    #[test]
    fn value_from_json_null() {
        assert_eq!(value_from_json(&serde_json::Value::Null), Value::None);
    }

    #[test]
    fn value_from_json_and_schema_obj() {
        let value = Value::Obj(BTreeMap::from([
            ("u64".into(), Value::U64(27)),
            ("i64".into(), Value::I64(-28)),
            ("f64".into(), Value::F64(-29.5)),
            ("bool".into(), Value::Bool(true)),
            ("str".into(), Value::Str("The king will come".into())),
        ]));
        let mut json_map = serde_json::Map::new();
        json_map.insert("u64".into(), serde_json::Value::Number(serde_json::Number::from_u128(27).unwrap()));
        json_map.insert("i64".into(), serde_json::Value::Number(serde_json::Number::from_i128(-28).unwrap()));
        json_map.insert("f64".into(), serde_json::Value::Number(serde_json::Number::from_f64(-29.5).unwrap()));
        json_map.insert("bool".into(), serde_json::Value::Bool(true));
        json_map.insert("str".into(), serde_json::Value::String("The king will come".into()));
        let json_value = serde_json::Value::Object(json_map);
        assert_eq!(value_from_json(&json_value), value);
    }

    #[test]
    fn value_from_json_and_schema_arr() {
        let json_value = serde_json::Value::Array(vec![
            serde_json::Value::Number(serde_json::Number::from_u128(27).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_i128(-28).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(-29.5).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_u128(30).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_i128(-31).unwrap()),
        ]);
        let value = Value::Arr(vec![Value::U64(27), Value::I64(-28), Value::F64(-29.5), Value::U64(30), Value::I64(-31)]);
        assert_eq!(value_from_json(&json_value), value);
    }
}

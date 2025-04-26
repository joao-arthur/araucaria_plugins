use std::collections::BTreeMap;

use araucaria::{validation::Validation, value::Value};

pub fn value_from_json_value(value: &serde_json::Value, validation: Option<&Validation>) -> Value {
    match value {
        serde_json::Value::Number(num) => {
            if let Some(Validation::U64(_)) = validation {
                if let Some(num) = num.as_u64() {
                    return Value::U64(num);
                }
            }
            if let Some(Validation::I64(_)) = validation {
                if let Some(num) = num.as_i64() {
                    return Value::I64(num);
                }
            }
            if let Some(Validation::F64(_)) = validation {
                if let Some(num) = num.as_f64() {
                    return Value::F64(num);
                }
            }
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
        serde_json::Value::Array(arr) => Value::Arr(arr.iter().map(|item| value_from_json_value(item, None)).collect()),
        serde_json::Value::Object(obj) => {
            let mut result: BTreeMap<String, Value> = BTreeMap::new();
            for (key, item) in obj {
                if let Some(Validation::Obj(obj_validation)) = validation {
                    let fff = key.clone();
                    result.insert(key.clone(), value_from_json_value(item, obj_validation.validation.get(&fff)));
                } else {
                    result.insert(key.clone(), value_from_json_value(item, None));
                }
            }
            Value::Obj(result)
        }
        serde_json::Value::Null => Value::None,
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use araucaria::{
        validation::{F64Validation, I64Validation, ObjValidation, U64Validation, Validation},
        value::Value,
    };

    use super::value_from_json_value;

    #[test]
    fn test_serde_json() {
        assert_eq!(serde_json::from_str::<serde_json::Value>("192168").unwrap().as_u64(), Some(192168));
        assert_eq!(serde_json::from_str::<serde_json::Value>("192168").unwrap().as_i64(), Some(192168));
        assert_eq!(serde_json::from_str::<serde_json::Value>("192168").unwrap().as_f64(), Some(192168.0));

        assert_eq!(serde_json::from_str::<serde_json::Value>("-192168").unwrap().as_u64(), None);
        assert_eq!(serde_json::from_str::<serde_json::Value>("-192168").unwrap().as_i64(), Some(-192168));
        assert_eq!(serde_json::from_str::<serde_json::Value>("-192168").unwrap().as_f64(), Some(-192168.0));

        assert_eq!(serde_json::from_str::<serde_json::Value>("-192168.5").unwrap().as_u64(), None);
        assert_eq!(serde_json::from_str::<serde_json::Value>("-192168.5").unwrap().as_i64(), None);
        assert_eq!(serde_json::from_str::<serde_json::Value>("-192168.5").unwrap().as_f64(), Some(-192168.5));
    }

    #[test]
    fn test_value_from_json_value_primites() {
        assert_eq!(value_from_json_value(&serde_json::Value::Null, None), Value::None);
        assert_eq!(value_from_json_value(&serde_json::Value::Bool(false), None), Value::Bool(false));
        assert_eq!(value_from_json_value(&serde_json::Value::String("ingeniosus homo est".into()), None), Value::from("ingeniosus homo est"));
        assert_eq!(value_from_json_value(&serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()), None), Value::U64(192_168));
        assert_eq!(value_from_json_value(&serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()), None), Value::I64(-192_168));
        assert_eq!(value_from_json_value(&serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap()), None), Value::F64(-192.5));
    }

    #[test]
    fn test_value_from_json_value_arr() {
        assert_eq!(
            value_from_json_value(
                &serde_json::Value::Array(vec![
                    serde_json::Value::Null,
                    serde_json::Value::Bool(false),
                    serde_json::Value::String("ingeniosus homo est".into()),
                    serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()),
                    serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()),
                    serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap())
                ]),
                None
            ),
            Value::Arr(vec![
                Value::None,
                Value::Bool(false),
                Value::from("ingeniosus homo est"),
                Value::U64(192_168),
                Value::I64(-192_168),
                Value::F64(-192.5)
            ])
        );
    }

    #[test]
    fn test_value_from_json_value_obj() {
        let mut map = serde_json::Map::new();
        map.insert("null".into(), serde_json::Value::Null);
        map.insert("bool".into(), serde_json::Value::Bool(false));
        map.insert("string".into(), serde_json::Value::String("ingeniosus homo est".into()));
        map.insert("u64".into(), serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()));
        map.insert("i64".into(), serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()));
        map.insert("f64".into(), serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap()));
        let value = serde_json::Value::Object(map);
        assert_eq!(
            value_from_json_value(&value, None),
            Value::from([
                ("null".into(), Value::None),
                ("bool".into(), Value::Bool(false)),
                ("string".into(), Value::from("ingeniosus homo est")),
                ("u64".into(), Value::U64(192_168)),
                ("i64".into(), Value::I64(-192_168)),
                ("f64".into(), Value::F64(-192.5)),
            ])
        );
    }

    #[test]
    fn test_value_from_json_value_without_validation() {
        let mut map = serde_json::Map::new();
        map.insert("u64".into(), serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()));
        map.insert("i64".into(), serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()));
        map.insert("f64".into(), serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap()));
        let value = serde_json::Value::Object(map);
        assert_eq!(
            value_from_json_value(&value, None),
            Value::from([("u64".into(), Value::U64(192_168)), ("i64".into(), Value::I64(-192_168)), ("f64".into(), Value::F64(-192.5))])
        );
    }

    #[test]
    fn test_value_from_json_value_without_same_validation() {
        let validation = Validation::Obj(ObjValidation::default().validation(BTreeMap::from([
            ("u64".into(), Validation::U64(U64Validation::default())),
            ("i64".into(), Validation::I64(I64Validation::default())),
            ("f64".into(), Validation::F64(F64Validation::default())),
        ])));
        let mut map = serde_json::Map::new();
        map.insert("u64".into(), serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()));
        map.insert("i64".into(), serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()));
        map.insert("f64".into(), serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap()));
        let value = serde_json::Value::Object(map);
        assert_eq!(
            value_from_json_value(&value, Some(&validation)),
            Value::from([("u64".into(), Value::U64(192_168)), ("i64".into(), Value::I64(-192_168)), ("f64".into(), Value::F64(-192.5))])
        );
    }

    #[test]
    fn test_value_from_json_value_u64() {
        let validation = Validation::Obj(ObjValidation::default().validation(BTreeMap::from([
            ("num_1".into(), Validation::U64(U64Validation::default())),
            ("num_2".into(), Validation::U64(U64Validation::default())),
            ("num_3".into(), Validation::U64(U64Validation::default())),
        ])));
        let mut map = serde_json::Map::new();
        map.insert("num_1".into(), serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()));
        map.insert("num_2".into(), serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()));
        map.insert("num_3".into(), serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap()));
        let value = serde_json::Value::Object(map);
        assert_eq!(
            value_from_json_value(&value, Some(&validation)),
            Value::from([("num_1".into(), Value::U64(192_168)), ("num_2".into(), Value::I64(-192_168)), ("num_3".into(), Value::F64(-192.5))])
        );
    }

    #[test]
    fn test_value_from_json_value_i64() {
        let validation = Validation::Obj(ObjValidation::default().validation(BTreeMap::from([
            ("num_1".into(), Validation::I64(I64Validation::default())),
            ("num_2".into(), Validation::I64(I64Validation::default())),
            ("num_3".into(), Validation::I64(I64Validation::default())),
        ])));
        let mut map = serde_json::Map::new();
        map.insert("num_1".into(), serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()));
        map.insert("num_2".into(), serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()));
        map.insert("num_3".into(), serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap()));
        let value = serde_json::Value::Object(map);
        assert_eq!(
            value_from_json_value(&value, Some(&validation)),
            Value::from([("num_1".into(), Value::I64(192_168)), ("num_2".into(), Value::I64(-192_168)), ("num_3".into(), Value::F64(-192.5))])
        );
    }

    #[test]
    fn test_value_from_json_value_f64() {
        let validation = Validation::Obj(ObjValidation::default().validation(BTreeMap::from([
            ("num_1".into(), Validation::F64(F64Validation::default())),
            ("num_2".into(), Validation::F64(F64Validation::default())),
            ("num_3".into(), Validation::F64(F64Validation::default())),
        ])));
        let mut map = serde_json::Map::new();
        map.insert("num_1".into(), serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()));
        map.insert("num_2".into(), serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()));
        map.insert("num_3".into(), serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap()));
        let value = serde_json::Value::Object(map);
        assert_eq!(
            value_from_json_value(&value, Some(&validation)),
            Value::from([("num_1".into(), Value::F64(192_168.0)), ("num_2".into(), Value::F64(-192_168.0)), ("num_3".into(), Value::F64(-192.5))])
        );
    }

    #[test]
    fn test_value_from_json_value_nested_obj() {
        let validation = Validation::Obj(ObjValidation::default().validation(BTreeMap::from([(
            "lvl_1".into(),
            Validation::Obj(ObjValidation::default().validation(BTreeMap::from([
                (
                    "lvl_2".into(),
                    Validation::Obj(ObjValidation::default().validation(BTreeMap::from([
                        (
                            "lvl_3".into(),
                            Validation::Obj(
                                ObjValidation::default().validation(BTreeMap::from([("num".into(), Validation::I64(I64Validation::default()))])),
                            ),
                        ),
                        ("num".into(), Validation::U64(U64Validation::default())),
                    ]))),
                ),
                ("num".into(), Validation::I64(I64Validation::default())),
            ]))),
        )])));

        let mut map_level_3 = serde_json::Map::new();
        map_level_3.insert("num".into(), serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()));
        let mut map_level_2 = serde_json::Map::new();
        map_level_2.insert("num".into(), serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()));
        map_level_2.insert("lvl_3".into(), serde_json::Value::Object(map_level_3));
        let mut map_level_1 = serde_json::Map::new();
        map_level_1.insert("num".into(), serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap()));
        map_level_1.insert("lvl_2".into(), serde_json::Value::Object(map_level_2));
        let mut map = serde_json::Map::new();
        map.insert("lvl_1".into(), serde_json::Value::Object(map_level_1));
        let value = serde_json::Value::Object(map);

        assert_eq!(
            value_from_json_value(&value, None),
            Value::from([(
                ("lvl_1".into()),
                Value::from([
                    (
                        ("lvl_2".into()),
                        Value::from([
                            (("lvl_3".into()), Value::from([(("num".into()), Value::U64(192_168))])),
                            (("num".into()), Value::I64(-192_168)),
                        ])
                    ),
                    (("num".into()), Value::F64(-192.5)),
                ])
            )]),
        );
        assert_eq!(
            value_from_json_value(&value, Some(&validation)),
            Value::from([(
                ("lvl_1".into()),
                Value::from([
                    (
                        ("lvl_2".into()),
                        Value::from([
                            (("lvl_3".into()), Value::from([(("num".into()), Value::I64(192_168))])),
                            (("num".into()), Value::I64(-192_168)),
                        ])
                    ),
                    (("num".into()), Value::F64(-192.5)),
                ])
            )]),
        );
    }
}

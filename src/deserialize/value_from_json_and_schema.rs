use std::collections::BTreeMap;

use araucaria::{
    validation::{EnumValues, Validation},
    value::Value,
};

fn internal_value_from_json_value_and_schema(value: &serde_json::Value, validation: Option<&Validation>) -> Value {
    match value {
        serde_json::Value::Number(num) => {
            if let Some(Validation::U64(_)) = validation {
                if let Some(u64_num) = num.as_u64() {
                    return Value::U64(u64_num);
                }
            }
            if let Some(Validation::I64(_)) = validation {
                if let Some(i64_num) = num.as_i64() {
                    return Value::I64(i64_num);
                }
            }
            if let Some(Validation::F64(_)) = validation {
                if let Some(f64_num) = num.as_f64() {
                    return Value::F64(f64_num);
                }
            }
            if let Some(Validation::USize(_)) = validation {
                if let Some(u64_num) = num.as_u64() {
                    if let Ok(usize_num) = usize::try_from(u64_num) {
                        return Value::USize(usize_num);
                    }
                }
            }
            if let Some(Validation::ISize(_)) = validation {
                if let Some(i64_num) = num.as_i64() {
                    if let Ok(isize_num) = isize::try_from(i64_num) {
                        return Value::ISize(isize_num);
                    }
                }
            }
            if let Some(Validation::Enum(v)) = validation {
                match v.values {
                    EnumValues::USize(_) => {
                        if let Some(u64_num) = num.as_u64() {
                            if let Ok(usize_num) = usize::try_from(u64_num) {
                                return Value::USize(usize_num);
                            }
                        }
                    }
                    EnumValues::ISize(_) => {
                        if let Some(i64_num) = num.as_i64() {
                            if let Ok(isize_num) = isize::try_from(i64_num) {
                                return Value::ISize(isize_num);
                            }
                        }
                    }
                    EnumValues::Str(_) => {
                        return Value::None;
                    }
                }
                if let Some(i64_num) = num.as_i64() {
                    if let Ok(isize_num) = isize::try_from(i64_num) {
                        return Value::ISize(isize_num);
                    }
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
        serde_json::Value::Array(arr) => Value::Arr(arr.iter().map(|item| internal_value_from_json_value_and_schema(item, None)).collect()),
        serde_json::Value::Object(obj) => {
            let mut result: BTreeMap<String, Value> = BTreeMap::new();
            for (key, item) in obj {
                if let Some(Validation::Obj(obj_validation)) = validation {
                    let fff = key.clone();
                    result.insert(key.clone(), internal_value_from_json_value_and_schema(item, obj_validation.validation.get(&fff)));
                } else {
                    result.insert(key.clone(), internal_value_from_json_value_and_schema(item, None));
                }
            }
            Value::Obj(result)
        }
        serde_json::Value::Null => Value::None,
    }
}

pub fn value_from_json_and_schema(value: &serde_json::Value, validation: &Validation) -> Value {
    internal_value_from_json_value_and_schema(value, Some(validation))
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use araucaria::{
        validation::{F64Validation, I64Validation, ObjValidation, U64Validation, Validation},
        value::Value,
    };

    use super::value_from_json_and_schema;

    #[test]
    fn value_from_json_value_and_schema_u64() {
        let v = Validation::U64(U64Validation::default());
        let json_u64 = serde_json::Value::Number(serde_json::Number::from_u128(192).unwrap());
        let json_i64_pos = serde_json::Value::Number(serde_json::Number::from_i128(192).unwrap());

        let json_f64_pos = serde_json::Value::Number(serde_json::Number::from_f64(192.0).unwrap());

        let json_i64_neg = serde_json::Value::Number(serde_json::Number::from_i128(-192).unwrap());
        let json_f64_neg = serde_json::Value::Number(serde_json::Number::from_f64(-192.0).unwrap());
        let json_f64_pos_float = serde_json::Value::Number(serde_json::Number::from_f64(192.5).unwrap());
        let json_f64_neg_float = serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap());

        assert_eq!(value_from_json_and_schema(&json_u64, &v), Value::U64(192));
        assert_eq!(value_from_json_and_schema(&json_i64_pos, &v), Value::U64(192));

        assert_eq!(value_from_json_and_schema(&json_f64_pos, &v), Value::F64(192.0)); // Not worth to fix

        assert_eq!(value_from_json_and_schema(&json_i64_neg, &v), Value::I64(-192));
        assert_eq!(value_from_json_and_schema(&json_f64_neg, &v), Value::F64(-192.0));
        assert_eq!(value_from_json_and_schema(&json_f64_pos_float, &v), Value::F64(192.5));
        assert_eq!(value_from_json_and_schema(&json_f64_neg_float, &v), Value::F64(-192.5));
    }

    #[test]
    fn value_from_json_value_and_schema_i64() {
        let v = Validation::I64(I64Validation::default());
        let json_u64 = serde_json::Value::Number(serde_json::Number::from_u128(192).unwrap());
        let json_i64_pos = serde_json::Value::Number(serde_json::Number::from_i128(192).unwrap());
        let json_i64_neg = serde_json::Value::Number(serde_json::Number::from_i128(-192).unwrap());

        let json_f64_pos = serde_json::Value::Number(serde_json::Number::from_f64(192.0).unwrap());
        let json_f64_neg = serde_json::Value::Number(serde_json::Number::from_f64(-192.0).unwrap());

        let json_f64_pos_float = serde_json::Value::Number(serde_json::Number::from_f64(192.5).unwrap());
        let json_f64_neg_float = serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap());

        assert_eq!(value_from_json_and_schema(&json_u64, &v), Value::I64(192));
        assert_eq!(value_from_json_and_schema(&json_i64_pos, &v), Value::I64(192));
        assert_eq!(value_from_json_and_schema(&json_i64_neg, &v), Value::I64(-192));

        assert_eq!(value_from_json_and_schema(&json_f64_pos, &v), Value::F64(192.0)); // Not worth to fix
        assert_eq!(value_from_json_and_schema(&json_f64_neg, &v), Value::F64(-192.0)); // Not worth to fix

        assert_eq!(value_from_json_and_schema(&json_f64_pos_float, &v), Value::F64(192.5));
        assert_eq!(value_from_json_and_schema(&json_f64_neg_float, &v), Value::F64(-192.5));
    }

    #[test]
    fn value_from_json_value_and_schema_f64() {
        let v = Validation::F64(F64Validation::default());
        let json_u64 = serde_json::Value::Number(serde_json::Number::from_u128(192).unwrap());
        let json_i64_pos = serde_json::Value::Number(serde_json::Number::from_i128(192).unwrap());

        let json_f64_pos = serde_json::Value::Number(serde_json::Number::from_f64(192.0).unwrap());

        let json_i64_neg = serde_json::Value::Number(serde_json::Number::from_i128(-192).unwrap());
        let json_f64_neg = serde_json::Value::Number(serde_json::Number::from_f64(-192.0).unwrap());
        let json_f64_pos_float = serde_json::Value::Number(serde_json::Number::from_f64(192.5).unwrap());
        let json_f64_neg_float = serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap());

        assert_eq!(value_from_json_and_schema(&json_u64, &v), Value::F64(192.0));
        assert_eq!(value_from_json_and_schema(&json_i64_pos, &v), Value::F64(192.0));
        assert_eq!(value_from_json_and_schema(&json_f64_pos, &v), Value::F64(192.0));

        assert_eq!(value_from_json_and_schema(&json_i64_neg, &v), Value::F64(-192.0));
        assert_eq!(value_from_json_and_schema(&json_f64_neg, &v), Value::F64(-192.0));
        assert_eq!(value_from_json_and_schema(&json_f64_pos_float, &v), Value::F64(192.5));
        assert_eq!(value_from_json_and_schema(&json_f64_neg_float, &v), Value::F64(-192.5));
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
            value_from_json_and_schema(&value, &validation),
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
            value_from_json_and_schema(&value, &validation),
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
            value_from_json_and_schema(&value, &validation),
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
            value_from_json_and_schema(&value, &validation),
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
            value_from_json_and_schema(&value, &validation),
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

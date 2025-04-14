use std::collections::HashMap;

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
            return Value::None;
        }
        serde_json::Value::Bool(bool) => Value::Bool(*bool),
        serde_json::Value::String(str) => Value::Str(str.clone()),
        serde_json::Value::Array(arr) => Value::Arr(arr.into_iter().map(|item| value_from_json_value(item, None)).collect()),
        serde_json::Value::Object(obj) => {
            let mut result: HashMap<String, Value> = HashMap::new();
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
    use std::collections::HashMap;

    use araucaria::{
        validation::{num_f::NumFValidation, num_i::NumIValidation, num_u::NumUValidation, ObjValidation, Validation},
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
        assert_eq!(
            value_from_json_value(&serde_json::Value::String(String::from("ingeniosus homo est")), None),
            Value::Str(String::from("ingeniosus homo est"))
        );
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
                    serde_json::Value::String(String::from("ingeniosus homo est")),
                    serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()),
                    serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()),
                    serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap())
                ]),
                None
            ),
            Value::Arr(vec![
                Value::None,
                Value::Bool(false),
                Value::Str(String::from("ingeniosus homo est")),
                Value::U64(192_168),
                Value::I64(-192_168),
                Value::F64(-192.5)
            ])
        );
    }

    #[test]
    fn test_value_from_json_value_obj() {
        let mut map = serde_json::Map::new();
        map.insert(String::from("null"), serde_json::Value::Null);
        map.insert(String::from("bool"), serde_json::Value::Bool(false));
        map.insert(String::from("string"), serde_json::Value::String(String::from("ingeniosus homo est")));
        map.insert(String::from("num_u"), serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()));
        map.insert(String::from("num_i"), serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()));
        map.insert(String::from("num_f"), serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap()));
        let value = serde_json::Value::Object(map);
        assert_eq!(
            value_from_json_value(&value, None),
            Value::from([
                (String::from("null"), Value::None),
                (String::from("bool"), Value::Bool(false)),
                (String::from("string"), Value::Str(String::from("ingeniosus homo est"))),
                (String::from("num_u"), Value::U64(192_168)),
                (String::from("num_i"), Value::I64(-192_168)),
                (String::from("num_f"), Value::F64(-192.5)),
            ])
        );
    }

    #[test]
    fn test_value_from_json_value_without_validation() {
        let mut map = serde_json::Map::new();
        map.insert(String::from("num_u"), serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()));
        map.insert(String::from("num_i"), serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()));
        map.insert(String::from("num_f"), serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap()));
        let value = serde_json::Value::Object(map);
        assert_eq!(
            value_from_json_value(&value, None),
            Value::from([
                (String::from("num_u"), Value::U64(192_168)),
                (String::from("num_i"), Value::I64(-192_168)),
                (String::from("num_f"), Value::F64(-192.5)),
            ])
        );
    }

    #[test]
    fn test_value_from_json_value_without_same_validation() {
        let validation = Validation::Obj(ObjValidation::default().validation(HashMap::from([
            (String::from("num_u"), Validation::U64(NumUValidation::default())),
            (String::from("num_i"), Validation::I64(NumIValidation::default())),
            (String::from("num_f"), Validation::F64(NumFValidation::default())),
        ])));
        let mut map = serde_json::Map::new();
        map.insert(String::from("num_u"), serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()));
        map.insert(String::from("num_i"), serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()));
        map.insert(String::from("num_f"), serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap()));
        let value = serde_json::Value::Object(map);
        assert_eq!(
            value_from_json_value(&value, Some(&validation)),
            Value::from([
                (String::from("num_u"), Value::U64(192_168)),
                (String::from("num_i"), Value::I64(-192_168)),
                (String::from("num_f"), Value::F64(-192.5)),
            ])
        );
    }

    #[test]
    fn test_value_from_json_value_num_u() {
        let validation = Validation::Obj(ObjValidation::default().validation(HashMap::from([
            (String::from("num_1"), Validation::U64(NumUValidation::default())),
            (String::from("num_2"), Validation::U64(NumUValidation::default())),
            (String::from("num_3"), Validation::U64(NumUValidation::default())),
        ])));
        let mut map = serde_json::Map::new();
        map.insert(String::from("num_1"), serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()));
        map.insert(String::from("num_2"), serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()));
        map.insert(String::from("num_3"), serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap()));
        let value = serde_json::Value::Object(map);
        assert_eq!(
            value_from_json_value(&value, Some(&validation)),
            Value::from([
                (String::from("num_1"), Value::U64(192_168)),
                (String::from("num_2"), Value::I64(-192_168)),
                (String::from("num_3"), Value::F64(-192.5)),
            ])
        );
    }

    #[test]
    fn test_value_from_json_value_num_i() {
        let validation = Validation::Obj(ObjValidation::default().validation(HashMap::from([
            (String::from("num_1"), Validation::I64(NumIValidation::default())),
            (String::from("num_2"), Validation::I64(NumIValidation::default())),
            (String::from("num_3"), Validation::I64(NumIValidation::default())),
        ])));
        let mut map = serde_json::Map::new();
        map.insert(String::from("num_1"), serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()));
        map.insert(String::from("num_2"), serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()));
        map.insert(String::from("num_3"), serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap()));
        let value = serde_json::Value::Object(map);
        assert_eq!(
            value_from_json_value(&value, Some(&validation)),
            Value::from([
                (String::from("num_1"), Value::I64(192_168)),
                (String::from("num_2"), Value::I64(-192_168)),
                (String::from("num_3"), Value::F64(-192.5)),
            ])
        );
    }

    #[test]
    fn test_value_from_json_value_num_f() {
        let validation = Validation::Obj(ObjValidation::default().validation(HashMap::from([
            (String::from("num_1"), Validation::F64(NumFValidation::default())),
            (String::from("num_2"), Validation::F64(NumFValidation::default())),
            (String::from("num_3"), Validation::F64(NumFValidation::default())),
        ])));
        let mut map = serde_json::Map::new();
        map.insert(String::from("num_1"), serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()));
        map.insert(String::from("num_2"), serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()));
        map.insert(String::from("num_3"), serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap()));
        let value = serde_json::Value::Object(map);
        assert_eq!(
            value_from_json_value(&value, Some(&validation)),
            Value::from([
                (String::from("num_1"), Value::F64(192_168.0)),
                (String::from("num_2"), Value::F64(-192_168.0)),
                (String::from("num_3"), Value::F64(-192.5)),
            ])
        );
    }

    #[test]
    fn test_value_from_json_value_nested_obj() {
        let validation =
            Validation::Obj(ObjValidation::default().validation(HashMap::from([(
                String::from("lvl_1"),
                Validation::Obj(ObjValidation::default().validation(HashMap::from(
                    [
                        (
                            String::from("lvl_2"),
                            Validation::Obj(
                                ObjValidation::default().validation(
                                    HashMap::from(
                                        [
                                            (
                                                String::from("lvl_3"),
                                                Validation::Obj(
                                                    ObjValidation::default().validation(HashMap::from([(
                                                        String::from("num"),
                                                        Validation::I64(NumIValidation::default()),
                                                    )])),
                                                ),
                                            ),
                                            (String::from("num"), Validation::U64(NumUValidation::default())),
                                        ],
                                    ),
                                ),
                            ),
                        ),
                        (String::from("num"), Validation::I64(NumIValidation::default())),
                    ],
                ))),
            )])));

        let mut map_level_3 = serde_json::Map::new();
        map_level_3.insert(String::from("num"), serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap()));
        let mut map_level_2 = serde_json::Map::new();
        map_level_2.insert(String::from("num"), serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap()));
        map_level_2.insert(String::from("lvl_3"), serde_json::Value::Object(map_level_3));
        let mut map_level_1 = serde_json::Map::new();
        map_level_1.insert(String::from("num"), serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap()));
        map_level_1.insert(String::from("lvl_2"), serde_json::Value::Object(map_level_2));
        let mut map = serde_json::Map::new();
        map.insert(String::from("lvl_1"), serde_json::Value::Object(map_level_1));
        let value = serde_json::Value::Object(map);

        assert_eq!(
            value_from_json_value(&value, None),
            Value::from([(
                (String::from("lvl_1")),
                Value::from([
                    (
                        (String::from("lvl_2")),
                        Value::from([
                            ((String::from("lvl_3")), Value::from([((String::from("num")), Value::U64(192_168)),])),
                            ((String::from("num")), Value::I64(-192_168)),
                        ])
                    ),
                    ((String::from("num")), Value::F64(-192.5)),
                ])
            ),]),
        );
        assert_eq!(
            value_from_json_value(&value, Some(&validation)),
            Value::from([(
                (String::from("lvl_1")),
                Value::from([
                    (
                        (String::from("lvl_2")),
                        Value::from([
                            ((String::from("lvl_3")), Value::from([((String::from("num")), Value::I64(192_168)),])),
                            ((String::from("num")), Value::I64(-192_168)),
                        ])
                    ),
                    ((String::from("num")), Value::F64(-192.5)),
                ])
            ),]),
        );
    }
}

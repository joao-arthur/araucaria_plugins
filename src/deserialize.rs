use std::collections::HashMap;

use araucaria::value::Value;

pub fn value_from_json_value(value: &serde_json::Value) -> Value {
    match value {
        serde_json::Value::Number(num) => {
            if let Some(num) = num.as_u64() { 
                return Value::NumU(num);
            }
            if let Some(num) = num.as_i64() {
                return Value::NumI(num);
            }
            if let Some(num) = num.as_f64() { 
                return Value::NumF(num);
            }
            return Value::None
        },
        serde_json::Value::Bool(bool) => Value::Bool(*bool),
        serde_json::Value::String(str) => Value::Str(str.clone()),
        serde_json::Value::Array(arr) => Value::Arr(arr.into_iter().map(value_from_json_value).collect()),
        serde_json::Value::Object(obj) => {
            let mut result: HashMap<String, Value> = HashMap::new();
            for (key, item) in obj {
                result.insert(key.clone(), value_from_json_value(item));
            }
            Value::Obj(result)
        },
        serde_json::Value::Null => Value::None,
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use araucaria::validation::{num_f::NumFValidation, num_i::NumIValidation, num_u::NumUValidation, ObjValidation, Validation};

    use super::*;

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
        assert_eq!(value_from_json_value(&serde_json::Value::Null), Value::None);
        assert_eq!(value_from_json_value(&serde_json::Value::Bool(false)), Value::Bool(false));
        assert_eq!(value_from_json_value(&serde_json::Value::String(String::from("ingeniosus homo est"))), Value::Str(String::from("ingeniosus homo est")));
        assert_eq!(value_from_json_value(&serde_json::Value::Number(serde_json::Number::from_u128(192_168).unwrap())), Value::NumU(192_168));
        assert_eq!(value_from_json_value(&serde_json::Value::Number(serde_json::Number::from_i128(-192_168).unwrap())), Value::NumI(-192_168));
        assert_eq!(value_from_json_value(&serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap())), Value::NumF(-192.5));
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
                ])
            ),
            Value::Arr(vec![
                Value::None,
                Value::Bool(false),
                Value::Str(String::from("ingeniosus homo est")),
                Value::NumU(192_168),
                Value::NumI(-192_168),
                Value::NumF(-192.5)
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
        assert_eq!(value_from_json_value(&serde_json::Value::Object(map)),
            Value::from([
                (String::from("null"), Value::None),
                (String::from("bool"), Value::Bool(false)),
                (String::from("string"), Value::Str(String::from("ingeniosus homo est"))),
                (String::from("num_u"), Value::NumU(192_168)),
                (String::from("num_i"), Value::NumI(-192_168)),
                (String::from("num_f"), Value::NumF(-192.5)),
            ])
        );
    }

    #[test]
    fn test_value_from_json_value_nested_obj() {
        let validation = Validation::Obj(ObjValidation::default().validation(HashMap::from([
            ("lvl_1", Validation::Obj(ObjValidation::default().validation(HashMap::from([
                ("lvl_2", Validation::Obj(ObjValidation::default().validation(HashMap::from([
                    ("lvl_3", Validation::Obj(ObjValidation::default().validation(HashMap::from([
                        ("num", Validation::NumI(NumIValidation::default()))
                    ])))),
                    ("num", Validation::NumU(NumUValidation::default()))
                ])))),
                ("num", Validation::NumF(NumFValidation::default()))
            ])))),
        ])));

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

        assert_eq!(value_from_json_value(&value),
            Value::from([
                ((String::from("lvl_1")), Value::from([
                    ((String::from("lvl_2")), Value::from([
                        ((String::from("lvl_3")), Value::from([
                            ((String::from("num")), Value::NumU(192_168)),
                        ])),
                        ((String::from("num")), Value::NumI(-192_168)),
                    ])),
                    ((String::from("num")), Value::NumF(-192.5)),
                ])),
            ]),
        );
    }
}

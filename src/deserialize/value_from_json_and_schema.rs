use std::collections::BTreeMap;

use araucaria::{
    schema::{EnumValues, Schema},
    value::Value,
};

fn internal_value_from_json_and_schema(json: &serde_json::Value, schema: Option<&Schema>) -> Value {
    match json {
        serde_json::Value::Number(num) => {
            if let Some(Schema::U64(_)) = schema {
                if let Some(u64_num) = num.as_u64() {
                    return Value::U64(u64_num);
                }
            }
            if let Some(Schema::I64(_)) = schema {
                if let Some(i64_num) = num.as_i64() {
                    return Value::I64(i64_num);
                }
            }
            if let Some(Schema::F64(_)) = schema {
                if let Some(f64_num) = num.as_f64() {
                    return Value::F64(f64_num);
                }
            }
            if let Some(Schema::USize(_)) = schema {
                if let Some(u64_num) = num.as_u64() {
                    if let Ok(usize_num) = usize::try_from(u64_num) {
                        return Value::USize(usize_num);
                    }
                }
            }
            if let Some(Schema::ISize(_)) = schema {
                if let Some(i64_num) = num.as_i64() {
                    if let Ok(isize_num) = isize::try_from(i64_num) {
                        return Value::ISize(isize_num);
                    }
                }
            }
            if let Some(Schema::Enum(v)) = schema {
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
                    EnumValues::Str(_) => {}
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
        serde_json::Value::Array(arr) => Value::Arr(arr.iter().map(|item| internal_value_from_json_and_schema(item, None)).collect()),
        serde_json::Value::Object(obj) => {
            let mut result: BTreeMap<String, Value> = BTreeMap::new();
            for (key, item) in obj {
                if let Some(Schema::Obj(obj_validation)) = schema {
                    let fff = key.clone();
                    result.insert(key.clone(), internal_value_from_json_and_schema(item, obj_validation.validation.get(&fff)));
                } else {
                    result.insert(key.clone(), internal_value_from_json_and_schema(item, None));
                }
            }
            Value::Obj(result)
        }
        serde_json::Value::Null => Value::None,
    }
}

pub fn value_from_json_and_schema(json: &serde_json::Value, schema: &Schema) -> Value {
    internal_value_from_json_and_schema(json, Some(schema))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use araucaria::{
        schema::{
            BoolSchema, DateSchema, DateTimeSchema, EmailSchema, EnumSchema, F64Schema, I64Schema, ISizeSchema, ObjSchema, Schema, StrSchema,
            TimeSchema, U64Schema, USizeSchema,
        },
        value::Value,
    };

    use super::value_from_json_and_schema;

    #[test]
    fn value_from_json_and_schema_u64() {
        let v = Schema::U64(U64Schema::default());
        let json_u64 = serde_json::Value::Number(serde_json::Number::from_u128(192).unwrap());
        let json_i64_pos = serde_json::Value::Number(serde_json::Number::from_i128(192).unwrap());

        let json_f64_pos = serde_json::Value::Number(serde_json::Number::from_f64(192.0).unwrap());
        let json_i64_neg = serde_json::Value::Number(serde_json::Number::from_i128(-192).unwrap());
        let json_f64_neg = serde_json::Value::Number(serde_json::Number::from_f64(-192.0).unwrap());
        let json_f64_pos_float = serde_json::Value::Number(serde_json::Number::from_f64(192.5).unwrap());
        let json_f64_neg_float = serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap());

        assert_eq!(value_from_json_and_schema(&json_u64, &v), Value::U64(192));
        assert_eq!(value_from_json_and_schema(&json_i64_pos, &v), Value::U64(192));

        assert_eq!(value_from_json_and_schema(&json_f64_pos, &v), Value::F64(192.0));
        assert_eq!(value_from_json_and_schema(&json_i64_neg, &v), Value::I64(-192));
        assert_eq!(value_from_json_and_schema(&json_f64_neg, &v), Value::F64(-192.0));
        assert_eq!(value_from_json_and_schema(&json_f64_pos_float, &v), Value::F64(192.5));
        assert_eq!(value_from_json_and_schema(&json_f64_neg_float, &v), Value::F64(-192.5));
    }

    #[test]
    fn value_from_json_and_schema_i64() {
        let v = Schema::I64(I64Schema::default());
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

        assert_eq!(value_from_json_and_schema(&json_f64_pos, &v), Value::F64(192.0));
        assert_eq!(value_from_json_and_schema(&json_f64_neg, &v), Value::F64(-192.0));
        assert_eq!(value_from_json_and_schema(&json_f64_pos_float, &v), Value::F64(192.5));
        assert_eq!(value_from_json_and_schema(&json_f64_neg_float, &v), Value::F64(-192.5));
    }

    #[test]
    fn value_from_json_and_schema_f64() {
        let v = Schema::F64(F64Schema::default());
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
    fn value_from_json_and_schema_usize() {
        let v = Schema::USize(USizeSchema::default());
        let json_u64 = serde_json::Value::Number(serde_json::Number::from_u128(192).unwrap());
        let json_i64_pos = serde_json::Value::Number(serde_json::Number::from_i128(192).unwrap());

        let json_f64_pos = serde_json::Value::Number(serde_json::Number::from_f64(192.0).unwrap());
        let json_i64_neg = serde_json::Value::Number(serde_json::Number::from_i128(-192).unwrap());
        let json_f64_neg = serde_json::Value::Number(serde_json::Number::from_f64(-192.0).unwrap());
        let json_f64_pos_float = serde_json::Value::Number(serde_json::Number::from_f64(192.5).unwrap());
        let json_f64_neg_float = serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap());

        assert_eq!(value_from_json_and_schema(&json_u64, &v), Value::USize(192));
        assert_eq!(value_from_json_and_schema(&json_i64_pos, &v), Value::USize(192));

        assert_eq!(value_from_json_and_schema(&json_f64_pos, &v), Value::F64(192.0));
        assert_eq!(value_from_json_and_schema(&json_i64_neg, &v), Value::I64(-192));
        assert_eq!(value_from_json_and_schema(&json_f64_neg, &v), Value::F64(-192.0));
        assert_eq!(value_from_json_and_schema(&json_f64_pos_float, &v), Value::F64(192.5));
        assert_eq!(value_from_json_and_schema(&json_f64_neg_float, &v), Value::F64(-192.5));
    }

    #[test]
    fn value_from_json_and_schema_isize() {
        let v = Schema::ISize(ISizeSchema::default());
        let json_u64 = serde_json::Value::Number(serde_json::Number::from_u128(192).unwrap());
        let json_i64_pos = serde_json::Value::Number(serde_json::Number::from_i128(192).unwrap());
        let json_i64_neg = serde_json::Value::Number(serde_json::Number::from_i128(-192).unwrap());

        let json_f64_pos = serde_json::Value::Number(serde_json::Number::from_f64(192.0).unwrap());
        let json_f64_neg = serde_json::Value::Number(serde_json::Number::from_f64(-192.0).unwrap());
        let json_f64_pos_float = serde_json::Value::Number(serde_json::Number::from_f64(192.5).unwrap());
        let json_f64_neg_float = serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap());

        assert_eq!(value_from_json_and_schema(&json_u64, &v), Value::ISize(192));
        assert_eq!(value_from_json_and_schema(&json_i64_pos, &v), Value::ISize(192));
        assert_eq!(value_from_json_and_schema(&json_i64_neg, &v), Value::ISize(-192));

        assert_eq!(value_from_json_and_schema(&json_f64_pos, &v), Value::F64(192.0));
        assert_eq!(value_from_json_and_schema(&json_f64_neg, &v), Value::F64(-192.0));
        assert_eq!(value_from_json_and_schema(&json_f64_pos_float, &v), Value::F64(192.5));
        assert_eq!(value_from_json_and_schema(&json_f64_neg_float, &v), Value::F64(-192.5));
    }

    #[test]
    fn value_from_json_and_schema_enum_usize() {
        let enum_values: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let v = Schema::Enum(EnumSchema::from(enum_values));
        let json_u64 = serde_json::Value::Number(serde_json::Number::from_u128(192).unwrap());
        let json_i64_pos = serde_json::Value::Number(serde_json::Number::from_i128(192).unwrap());

        let json_f64_pos = serde_json::Value::Number(serde_json::Number::from_f64(192.0).unwrap());
        let json_i64_neg = serde_json::Value::Number(serde_json::Number::from_i128(-192).unwrap());
        let json_f64_neg = serde_json::Value::Number(serde_json::Number::from_f64(-192.0).unwrap());
        let json_f64_pos_float = serde_json::Value::Number(serde_json::Number::from_f64(192.5).unwrap());
        let json_f64_neg_float = serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap());

        assert_eq!(value_from_json_and_schema(&json_u64, &v), Value::USize(192));
        assert_eq!(value_from_json_and_schema(&json_i64_pos, &v), Value::USize(192));

        assert_eq!(value_from_json_and_schema(&json_f64_pos, &v), Value::F64(192.0));
        assert_eq!(value_from_json_and_schema(&json_i64_neg, &v), Value::I64(-192));
        assert_eq!(value_from_json_and_schema(&json_f64_neg, &v), Value::F64(-192.0));
        assert_eq!(value_from_json_and_schema(&json_f64_pos_float, &v), Value::F64(192.5));
        assert_eq!(value_from_json_and_schema(&json_f64_neg_float, &v), Value::F64(-192.5));
    }

    #[test]
    fn value_from_json_and_schema_enum_isize() {
        let enum_values: Vec<isize> = vec![0, -1, -2, -3, -4, -5];
        let v = Schema::Enum(EnumSchema::from(enum_values));
        let json_u64 = serde_json::Value::Number(serde_json::Number::from_u128(192).unwrap());
        let json_i64_pos = serde_json::Value::Number(serde_json::Number::from_i128(192).unwrap());
        let json_i64_neg = serde_json::Value::Number(serde_json::Number::from_i128(-192).unwrap());

        let json_f64_pos = serde_json::Value::Number(serde_json::Number::from_f64(192.0).unwrap());
        let json_f64_neg = serde_json::Value::Number(serde_json::Number::from_f64(-192.0).unwrap());
        let json_f64_pos_float = serde_json::Value::Number(serde_json::Number::from_f64(192.5).unwrap());
        let json_f64_neg_float = serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap());

        assert_eq!(value_from_json_and_schema(&json_u64, &v), Value::ISize(192));
        assert_eq!(value_from_json_and_schema(&json_i64_pos, &v), Value::ISize(192));
        assert_eq!(value_from_json_and_schema(&json_i64_neg, &v), Value::ISize(-192));

        assert_eq!(value_from_json_and_schema(&json_f64_pos, &v), Value::F64(192.0));
        assert_eq!(value_from_json_and_schema(&json_f64_neg, &v), Value::F64(-192.0));
        assert_eq!(value_from_json_and_schema(&json_f64_pos_float, &v), Value::F64(192.5));
        assert_eq!(value_from_json_and_schema(&json_f64_neg_float, &v), Value::F64(-192.5));
    }

    #[test]
    fn value_from_json_and_schema_enum_string() {
        let enum_values: Vec<String> = vec!["APPLE".into(), "MELON".into(), "TOMATO".into(), "ORANGE".into(), "PEACH".into()];
        let v = Schema::Enum(EnumSchema::from(enum_values));
        let json_u64 = serde_json::Value::Number(serde_json::Number::from_u128(192).unwrap());
        let json_i64_pos = serde_json::Value::Number(serde_json::Number::from_i128(192).unwrap());
        let json_i64_neg = serde_json::Value::Number(serde_json::Number::from_i128(-192).unwrap());
        let json_f64_pos = serde_json::Value::Number(serde_json::Number::from_f64(192.0).unwrap());
        let json_f64_neg = serde_json::Value::Number(serde_json::Number::from_f64(-192.0).unwrap());
        let json_f64_pos_float = serde_json::Value::Number(serde_json::Number::from_f64(192.5).unwrap());
        let json_f64_neg_float = serde_json::Value::Number(serde_json::Number::from_f64(-192.5).unwrap());
        assert_eq!(value_from_json_and_schema(&json_u64, &v), Value::U64(192));
        assert_eq!(value_from_json_and_schema(&json_i64_pos, &v), Value::U64(192));
        assert_eq!(value_from_json_and_schema(&json_i64_neg, &v), Value::I64(-192));
        assert_eq!(value_from_json_and_schema(&json_f64_pos, &v), Value::F64(192.0));
        assert_eq!(value_from_json_and_schema(&json_f64_neg, &v), Value::F64(-192.0));
        assert_eq!(value_from_json_and_schema(&json_f64_pos_float, &v), Value::F64(192.5));
        assert_eq!(value_from_json_and_schema(&json_f64_neg_float, &v), Value::F64(-192.5));
    }

    #[test]
    fn value_from_json_and_schema_bool() {
        let v = Schema::Bool(BoolSchema::default());
        assert_eq!(value_from_json_and_schema(&serde_json::Value::Bool(false), &v), Value::Bool(false));
        assert_eq!(value_from_json_and_schema(&serde_json::Value::Bool(true), &v), Value::Bool(true));
    }

    #[test]
    fn value_from_json_and_schema_string() {
        let v = Schema::Str(StrSchema::default());
        assert_eq!(value_from_json_and_schema(&serde_json::Value::String("Naruto".into()), &v), Value::Str("Naruto".into()));
        assert_eq!(value_from_json_and_schema(&serde_json::Value::String("chuck@gmail.com".into()), &v), Value::Str("chuck@gmail.com".into()));
        assert_eq!(value_from_json_and_schema(&serde_json::Value::String("2025-04-26".into()), &v), Value::Str("2025-04-26".into()));
        assert_eq!(value_from_json_and_schema(&serde_json::Value::String("16:55".into()), &v), Value::Str("16:55".into()));
        assert_eq!(value_from_json_and_schema(&serde_json::Value::String("2025-04-26T16:55Z".into()), &v), Value::Str("2025-04-26T16:55Z".into()));
    }

    #[test]
    fn value_from_json_and_schema_null() {
        let v = Schema::U64(U64Schema::default());
        assert_eq!(value_from_json_and_schema(&serde_json::Value::Null, &v), Value::None);
    }

    #[test]
    fn value_from_json_and_schema_obj() {
        let usize_values: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let isize_values: Vec<isize> = vec![0, -1, -2, -3, -4, -5];
        let string_values: Vec<String> = vec!["APPLE".into(), "MELON".into(), "TOMATO".into(), "ORANGE".into(), "PEACH".into()];
        let schema = Schema::Obj(ObjSchema::from(BTreeMap::from([
            ("u64".into(), Schema::U64(U64Schema::default())),
            ("i64".into(), Schema::I64(I64Schema::default())),
            ("f64".into(), Schema::F64(F64Schema::default())),
            ("usize".into(), Schema::USize(USizeSchema::default())),
            ("isize".into(), Schema::ISize(ISizeSchema::default())),
            ("bool".into(), Schema::Bool(BoolSchema::default())),
            ("str".into(), Schema::Str(StrSchema::default())),
            ("email".into(), Schema::Email(EmailSchema::default())),
            ("date".into(), Schema::Date(DateSchema::default())),
            ("time".into(), Schema::Time(TimeSchema::default())),
            ("datetime".into(), Schema::DateTime(DateTimeSchema::default())),
            ("enum_usize".into(), Schema::Enum(EnumSchema::from(usize_values))),
            ("enum_isize".into(), Schema::Enum(EnumSchema::from(isize_values))),
            ("enum_str".into(), Schema::Enum(EnumSchema::from(string_values))),
        ])));
        let value = Value::Obj(BTreeMap::from([
            ("u64".into(), Value::U64(27)),
            ("i64".into(), Value::I64(-28)),
            ("f64".into(), Value::F64(-29.5)),
            ("usize".into(), Value::USize(30)),
            ("isize".into(), Value::ISize(-31)),
            ("bool".into(), Value::Bool(true)),
            ("str".into(), Value::Str("The king will come".into())),
            ("email".into(), Value::Str("plato@gmail.com".into())),
            ("date".into(), Value::Str("2025-04-26".into())),
            ("time".into(), Value::Str("18:27".into())),
            ("datetime".into(), Value::Str("2025-04-26T18:27Z".into())),
            ("enum_usize".into(), Value::USize(2)),
            ("enum_isize".into(), Value::ISize(-1)),
            ("enum_str".into(), Value::Str("MELON".into())),
        ]));
        let mut json_map = serde_json::Map::new();
        json_map.insert("u64".into(), serde_json::Value::Number(serde_json::Number::from_u128(27).unwrap()));
        json_map.insert("i64".into(), serde_json::Value::Number(serde_json::Number::from_i128(-28).unwrap()));
        json_map.insert("f64".into(), serde_json::Value::Number(serde_json::Number::from_f64(-29.5).unwrap()));
        json_map.insert("usize".into(), serde_json::Value::Number(serde_json::Number::from_u128(30).unwrap()));
        json_map.insert("isize".into(), serde_json::Value::Number(serde_json::Number::from_i128(-31).unwrap()));
        json_map.insert("bool".into(), serde_json::Value::Bool(true));
        json_map.insert("str".into(), serde_json::Value::String("The king will come".into()));
        json_map.insert("email".into(), serde_json::Value::String("plato@gmail.com".into()));
        json_map.insert("date".into(), serde_json::Value::String("2025-04-26".into()));
        json_map.insert("time".into(), serde_json::Value::String("18:27".into()));
        json_map.insert("datetime".into(), serde_json::Value::String("2025-04-26T18:27Z".into()));
        json_map.insert("enum_usize".into(), serde_json::Value::Number(serde_json::Number::from_u128(2).unwrap()));
        json_map.insert("enum_isize".into(), serde_json::Value::Number(serde_json::Number::from_i128(-1).unwrap()));
        json_map.insert("enum_str".into(), serde_json::Value::String("MELON".into()));
        let json_value = serde_json::Value::Object(json_map);
        assert_eq!(value_from_json_and_schema(&json_value, &schema), value);
    }

    #[test]
    fn value_from_json_and_schema_obj_other_type() {
        let schema = Schema::U64(U64Schema::default());
        let value = Value::Obj(BTreeMap::from([
            ("u64".into(), Value::U64(27)),
            ("i64".into(), Value::I64(-28)),
            ("f64".into(), Value::F64(-29.5)),
            ("usize".into(), Value::U64(30)),
            ("isize".into(), Value::I64(-31)),
            ("bool".into(), Value::Bool(true)),
            ("datetime".into(), Value::Str("2025-04-26T18:27Z".into())),
        ]));
        let mut json_map = serde_json::Map::new();
        json_map.insert("u64".into(), serde_json::Value::Number(serde_json::Number::from_u128(27).unwrap()));
        json_map.insert("i64".into(), serde_json::Value::Number(serde_json::Number::from_i128(-28).unwrap()));
        json_map.insert("f64".into(), serde_json::Value::Number(serde_json::Number::from_f64(-29.5).unwrap()));
        json_map.insert("usize".into(), serde_json::Value::Number(serde_json::Number::from_u128(30).unwrap()));
        json_map.insert("isize".into(), serde_json::Value::Number(serde_json::Number::from_i128(-31).unwrap()));
        json_map.insert("bool".into(), serde_json::Value::Bool(true));
        json_map.insert("datetime".into(), serde_json::Value::String("2025-04-26T18:27Z".into()));
        let json_value = serde_json::Value::Object(json_map);
        assert_eq!(value_from_json_and_schema(&json_value, &schema), value);
    }

    #[test]
    fn value_from_json_and_schema_arr() {
        let schema = Schema::Obj(ObjSchema::from(BTreeMap::from([
            ("u64".into(), Schema::U64(U64Schema::default())),
            ("f64".into(), Schema::F64(F64Schema::default())),
            ("usize".into(), Schema::USize(USizeSchema::default())),
            ("isize".into(), Schema::ISize(ISizeSchema::default())),
        ])));
        let json_value = serde_json::Value::Array(vec![
            serde_json::Value::Number(serde_json::Number::from_u128(27).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_i128(-28).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_f64(-29.5).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_u128(30).unwrap()),
            serde_json::Value::Number(serde_json::Number::from_i128(-31).unwrap()),
        ]);
        let value = Value::Arr(vec![Value::U64(27), Value::I64(-28), Value::F64(-29.5), Value::U64(30), Value::I64(-31)]);
        assert_eq!(value_from_json_and_schema(&json_value, &schema), value);
    }

    #[test]
    fn value_from_json_nested_obj() {
        let usize_values: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        let isize_values: Vec<isize> = vec![0, -1, -2, -3, -4, -5];
        let string_values: Vec<String> = vec!["APPLE".into(), "MELON".into(), "TOMATO".into(), "ORANGE".into(), "PEACH".into()];
        let v = Schema::Obj(ObjSchema::from(BTreeMap::from([(
            "lvl_1".into(),
            Schema::Obj(ObjSchema::from(BTreeMap::from([(
                "lvl_2".into(),
                Schema::Obj(ObjSchema::from(BTreeMap::from([(
                    "lvl_3".into(),
                    Schema::Obj(ObjSchema::from(BTreeMap::from([
                        ("u64".into(), Schema::U64(U64Schema::default())),
                        ("i64".into(), Schema::I64(I64Schema::default())),
                        ("f64".into(), Schema::F64(F64Schema::default())),
                        ("usize".into(), Schema::USize(USizeSchema::default())),
                        ("isize".into(), Schema::ISize(ISizeSchema::default())),
                        ("bool".into(), Schema::Bool(BoolSchema::default())),
                        ("str".into(), Schema::Str(StrSchema::default())),
                        ("email".into(), Schema::Email(EmailSchema::default())),
                        ("date".into(), Schema::Date(DateSchema::default())),
                        ("time".into(), Schema::Time(TimeSchema::default())),
                        ("datetime".into(), Schema::DateTime(DateTimeSchema::default())),
                        ("enum_usize".into(), Schema::Enum(EnumSchema::from(usize_values))),
                        ("enum_isize".into(), Schema::Enum(EnumSchema::from(isize_values))),
                        ("enum_str".into(), Schema::Enum(EnumSchema::from(string_values))),
                    ]))),
                )]))),
            )]))),
        )])));
        let value = Value::Obj(BTreeMap::from([(
            "lvl_1".into(),
            Value::Obj(BTreeMap::from([(
                "lvl_2".into(),
                Value::Obj(BTreeMap::from([(
                    "lvl_3".into(),
                    Value::Obj(BTreeMap::from([
                        ("u64".into(), Value::U64(27)),
                        ("i64".into(), Value::I64(-28)),
                        ("f64".into(), Value::F64(-29.5)),
                        ("usize".into(), Value::USize(30)),
                        ("isize".into(), Value::ISize(-31)),
                        ("bool".into(), Value::Bool(true)),
                        ("str".into(), Value::Str("The king will come".into())),
                        ("email".into(), Value::Str("plato@gmail.com".into())),
                        ("date".into(), Value::Str("2025-04-26".into())),
                        ("time".into(), Value::Str("18:27".into())),
                        ("datetime".into(), Value::Str("2025-04-26T18:27Z".into())),
                        ("enum_usize".into(), Value::USize(2)),
                        ("enum_isize".into(), Value::ISize(-1)),
                        ("enum_str".into(), Value::Str("MELON".into())),
                    ])),
                )])),
            )])),
        )]));

        let mut json_map = serde_json::Map::new();
        json_map.insert("u64".into(), serde_json::Value::Number(serde_json::Number::from_u128(27).unwrap()));
        json_map.insert("i64".into(), serde_json::Value::Number(serde_json::Number::from_i128(-28).unwrap()));
        json_map.insert("f64".into(), serde_json::Value::Number(serde_json::Number::from_f64(-29.5).unwrap()));
        json_map.insert("usize".into(), serde_json::Value::Number(serde_json::Number::from_u128(30).unwrap()));
        json_map.insert("isize".into(), serde_json::Value::Number(serde_json::Number::from_i128(-31).unwrap()));
        json_map.insert("bool".into(), serde_json::Value::Bool(true));
        json_map.insert("str".into(), serde_json::Value::String("The king will come".into()));
        json_map.insert("email".into(), serde_json::Value::String("plato@gmail.com".into()));
        json_map.insert("date".into(), serde_json::Value::String("2025-04-26".into()));
        json_map.insert("time".into(), serde_json::Value::String("18:27".into()));
        json_map.insert("datetime".into(), serde_json::Value::String("2025-04-26T18:27Z".into()));
        json_map.insert("enum_usize".into(), serde_json::Value::Number(serde_json::Number::from_u128(2).unwrap()));
        json_map.insert("enum_isize".into(), serde_json::Value::Number(serde_json::Number::from_i128(-1).unwrap()));
        json_map.insert("enum_str".into(), serde_json::Value::String("MELON".into()));

        let mut map_level_2 = serde_json::Map::new();
        map_level_2.insert("lvl_3".into(), serde_json::Value::Object(json_map));
        let mut map_level_1 = serde_json::Map::new();
        map_level_1.insert("lvl_2".into(), serde_json::Value::Object(map_level_2));
        let mut map = serde_json::Map::new();
        map.insert("lvl_1".into(), serde_json::Value::Object(map_level_1));
        let json_value = serde_json::Value::Object(map);

        assert_eq!(value_from_json_and_schema(&json_value, &v), value);
    }
}

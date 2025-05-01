use std::collections::BTreeMap;

use serde::{Serialize, Serializer};

#[derive(Debug, PartialEq, Clone)]
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

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::None => "".serialize(serializer),
            Value::U64(value) => value.serialize(serializer),
            Value::I64(value) => value.serialize(serializer),
            Value::F64(value) => value.serialize(serializer),
            Value::USize(value) => value.serialize(serializer),
            Value::ISize(value) => value.serialize(serializer),
            Value::Bool(value) => value.serialize(serializer),
            Value::Str(value) => value.serialize(serializer),
            Value::Arr(value) => value.serialize(serializer),
            Value::Obj(value) => value.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use araucaria::value::stub::{arr_str_stub, obj_stub};

    use super::{Value, to_value};

    #[test]
    fn araucaria_value_to_value() {
        let arr = Value::Arr(vec![
            Value::Str("George Harrison".into()),
            Value::Str("John Lennon".into()),
            Value::Str("Paul McCartney".into()),
            Value::Str("Ringo Starr".into()),
        ]);
        let obj = Value::Obj(BTreeMap::from([
            ("name".into(), Value::Str("The Beatles".into())),
            (
                "members".into(),
                Value::Arr(vec![
                    Value::Str("George Harrison".into()),
                    Value::Str("John Lennon".into()),
                    Value::Str("Paul McCartney".into()),
                    Value::Str("Ringo Starr".into()),
                ]),
            ),
            ("start_year".into(), Value::U64(1960)),
            ("end_year".into(), Value::U64(1960)),
            ("number_of_albums".into(), Value::U64(13)),
            ("greatest_band".into(), Value::Bool(true)),
        ]));
        assert_eq!(to_value(araucaria::value::Value::None), Value::None);
        assert_eq!(to_value(araucaria::value::Value::U64(12)), Value::U64(12));
        assert_eq!(to_value(araucaria::value::Value::I64(-34)), Value::I64(-34));
        assert_eq!(to_value(araucaria::value::Value::F64(-64.5)), Value::F64(-64.5));
        assert_eq!(to_value(araucaria::value::Value::USize(84)), Value::USize(84));
        assert_eq!(to_value(araucaria::value::Value::ISize(-79)), Value::ISize(-79));
        assert_eq!(to_value(araucaria::value::Value::Bool(false)), Value::Bool(false));
        assert_eq!(to_value(araucaria::value::Value::Str("Under blue moon".into())), Value::Str("Under blue moon".into()));
        assert_eq!(to_value(arr_str_stub()), arr);
        assert_eq!(to_value(obj_stub()), obj);
    }

    #[test]
    fn serialize_value() {
        let arr_str = Value::Arr(vec![Value::Str("Naruto".into()), Value::Str("Sasuke".into()), Value::Str("Sakura".into())]);
        let arr_num = Value::Arr(vec![Value::U64(83), Value::I64(-19), Value::F64(1.25), Value::USize(28), Value::ISize(-378)]);
        let obj = Value::Obj(BTreeMap::from([
            ("name".into(), Value::Str("The Beatles".into())),
            ("start".into(), Value::U64(1960)),
            ("end".into(), Value::U64(1969)),
            ("greatest_band".into(), Value::Bool(true)),
        ]));
        assert_eq!(serde_json::to_string(&Value::None).unwrap(), r#""""#.to_string());
        assert_eq!(serde_json::to_string(&Value::U64(12)).unwrap(), "12".to_string());
        assert_eq!(serde_json::to_string(&Value::I64(-34)).unwrap(), "-34".to_string());
        assert_eq!(serde_json::to_string(&Value::F64(-64.5)).unwrap(), "-64.5".to_string());
        assert_eq!(serde_json::to_string(&Value::USize(84)).unwrap(), "84".to_string());
        assert_eq!(serde_json::to_string(&Value::ISize(-79)).unwrap(), "-79".to_string());
        assert_eq!(serde_json::to_string(&Value::Bool(false)).unwrap(), "false".to_string());
        assert_eq!(serde_json::to_string(&Value::Str("Under blue moon".into())).unwrap(), r#""Under blue moon""#.to_string());
        assert_eq!(serde_json::to_string(&arr_str).unwrap(), r#"["Naruto","Sasuke","Sakura"]"#.to_string());
        assert_eq!(serde_json::to_string(&arr_num).unwrap(), r#"[83,-19,1.25,28,-378]"#.to_string());
        assert_eq!(serde_json::to_string(&arr_num).unwrap(), r#"[83,-19,1.25,28,-378]"#.to_string());
        assert_eq!(serde_json::to_string(&obj).unwrap(), r#"{"end":1969,"greatest_band":true,"name":"The Beatles","start":1960}"#.to_string());
    }
}

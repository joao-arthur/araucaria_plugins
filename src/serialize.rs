use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Value {
    None,
    Bool(bool),
    NumU(u64),
    NumI(i64),
    NumF(f64),
    Str(String),
    Arr(Vec<Value>),
    Obj(HashMap<String, Value>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ValidationErr {
    Required,
    Bool,
    Str,
    NumU,
    NumI,
    NumF,
    Eq(Value),
    Ne(Value),
    Gt(Value),
    Lt(Value),
    Ge(Value),
    Le(Value),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaErr {
    Validation(Vec<ValidationErr>),
    Obj(HashMap<String, SchemaErr>),
}

impl Serialize for SchemaErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SchemaErr::Validation(vec) => vec.serialize(serializer),
            SchemaErr::Obj(map) => map.serialize(serializer),
        }
    }
}

pub fn map_value(value: araucaria::value::Value) -> Value {
    match value {
        araucaria::value::Value::None => Value::None,
        araucaria::value::Value::Bool(value) => Value::Bool(value),
        araucaria::value::Value::NumU(value) => Value::NumU(value),
        araucaria::value::Value::NumI(value) => Value::NumI(value),
        araucaria::value::Value::NumF(value) => Value::NumF(value),
        araucaria::value::Value::Str(value) => Value::Str(value),
        araucaria::value::Value::Arr(value) => {
            Value::Arr(value.into_iter().map(map_value).collect())
        }
        araucaria::value::Value::Obj(value) => Value::Obj(
            value.into_iter().map(|(k, v)| (String::from(k.clone()), map_value(v))).collect(),
        ),
    }
}

pub fn map_err(value: araucaria::error::ValidationErr) -> ValidationErr {
    match value {
        araucaria::error::ValidationErr::Required => ValidationErr::Required,
        araucaria::error::ValidationErr::Bool => ValidationErr::Bool,
        araucaria::error::ValidationErr::Str => ValidationErr::Str,
        araucaria::error::ValidationErr::NumU => ValidationErr::NumU,
        araucaria::error::ValidationErr::NumI => ValidationErr::NumI,
        araucaria::error::ValidationErr::NumF => ValidationErr::NumF,
        araucaria::error::ValidationErr::Eq(value) => ValidationErr::Eq(map_value(value)),
        araucaria::error::ValidationErr::Ne(value) => ValidationErr::Ne(map_value(value)),
        araucaria::error::ValidationErr::Gt(value) => ValidationErr::Gt(map_value(value)),
        araucaria::error::ValidationErr::Lt(value) => ValidationErr::Lt(map_value(value)),
        araucaria::error::ValidationErr::Ge(value) => ValidationErr::Ge(map_value(value)),
        araucaria::error::ValidationErr::Le(value) => ValidationErr::Le(map_value(value)),
    }
}

pub fn map_err_wrap(value: araucaria::error::SchemaErr) -> SchemaErr {
    match value {
        araucaria::error::SchemaErr::Validation(value) => {
            SchemaErr::Validation(value.into_iter().map(map_err).collect())
        }
        araucaria::error::SchemaErr::Obj(value) => SchemaErr::Obj(
            value.into_iter().map(|(k, v)| (String::from(k.clone()), map_err_wrap(v))).collect(),
        ),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&SchemaErr::Obj(HashMap::from([(
                String::from("is"),
                SchemaErr::Validation(vec![
                    ValidationErr::Bool,
                    ValidationErr::Required,
                    ValidationErr::Eq(Value::Bool(false))
                ])
            )])))
            .unwrap(),
            String::from(r#"{"is":["Bool","Required",{"Eq":{"Bool":false}}]}"#)
        );
    }
}

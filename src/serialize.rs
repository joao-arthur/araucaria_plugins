use std::collections::HashMap;

use araucaria::validation::{bool::BoolValidation, ObjValidation, Validation};
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
pub enum Err {
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
pub enum ErrWrap {
    Arr(Vec<Err>),
    Obj(HashMap<String, ErrWrap>),
}

impl Serialize for ErrWrap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ErrWrap::Arr(vec) => vec.serialize(serializer),
            ErrWrap::Obj(map) => map.serialize(serializer),
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

pub fn map_err(value: araucaria::error::Err) -> Err {
    match value {
        araucaria::error::Err::Required => Err::Required,
        araucaria::error::Err::Bool => Err::Bool,
        araucaria::error::Err::Str => Err::Str,
        araucaria::error::Err::NumU => Err::NumU,
        araucaria::error::Err::NumI => Err::NumI,
        araucaria::error::Err::NumF => Err::NumF,
        araucaria::error::Err::Eq(value) => Err::Eq(map_value(value)),
        araucaria::error::Err::Ne(value) => Err::Ne(map_value(value)),
        araucaria::error::Err::Gt(value) => Err::Gt(map_value(value)),
        araucaria::error::Err::Lt(value) => Err::Lt(map_value(value)),
        araucaria::error::Err::Ge(value) => Err::Ge(map_value(value)),
        araucaria::error::Err::Le(value) => Err::Le(map_value(value)),
    }
}

pub fn map_err_wrap(value: araucaria::error::ErrWrap) -> ErrWrap {
    match value {
        araucaria::error::ErrWrap::Arr(value) => {
            ErrWrap::Arr(value.into_iter().map(map_err).collect())
        }
        araucaria::error::ErrWrap::Obj(value) => ErrWrap::Obj(
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
            serde_json::to_string(&ErrWrap::Obj(HashMap::from([(
                String::from("is"),
                ErrWrap::Arr(vec![Err::Bool, Err::Required, Err::Eq(Value::Bool(false))])
            )])))
            .unwrap(),
            String::from(r#"{"is":["Bool","Required",{"Eq":{"Bool":false}}]}"#)
        );
    }
}

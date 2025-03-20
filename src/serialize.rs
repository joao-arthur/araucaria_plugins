use std::collections::HashMap;

use araucaria::{
    validate::validate,
    validation::{bool::BoolValidation, ObjValidation, Validation},
};
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
    NumU,
    NumI,
    NumF,
    Obj,
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

fn mapValue(value: araucaria::value::Value) -> Value {
    match value {
        araucaria::value::Value::None => Value::None,
        araucaria::value::Value::Bool(value) => Value::Bool(value),
        araucaria::value::Value::NumU(value) => Value::NumU(value),
        araucaria::value::Value::NumI(value) => Value::NumI(value),
        araucaria::value::Value::NumF(value) => Value::NumF(value),
        araucaria::value::Value::Str(value) => Value::Str(value),
        araucaria::value::Value::Arr(value) => Value::Arr(value.into_iter().map(mapValue).collect()),
        araucaria::value::Value::Obj(value) => Value::Obj(value.into_iter().map(|(k, v)| (String::from(k.clone()), mapValue(v))).collect()),
    }
}

fn mapErr(value: araucaria::error::Err) -> Err {
    match value {
        araucaria::error::Err::Required => Err::Required,
        araucaria::error::Err::Bool => Err::Bool,
        araucaria::error::Err::NumU => Err::NumU,
        araucaria::error::Err::NumI => Err::NumI,
        araucaria::error::Err::NumF => Err::NumF,
        araucaria::error::Err::Obj => Err::Obj,
        araucaria::error::Err::Eq(value) => Err::Eq(mapValue(value)),
        araucaria::error::Err::Ne(value) => Err::Ne(mapValue(value)),
        araucaria::error::Err::Gt(value) => Err::Gt(mapValue(value)),
        araucaria::error::Err::Lt(value) => Err::Lt(mapValue(value)),
        araucaria::error::Err::Ge(value) => Err::Ge(mapValue(value)),
        araucaria::error::Err::Le(value) => Err::Le(mapValue(value)),
    }
}

fn mapErrWrap(value: araucaria::error::ErrWrap) -> ErrWrap {
    match value {
        araucaria::error::ErrWrap::Arr(value) => ErrWrap::Arr(value.into_iter().map(mapErr).collect()),
        araucaria::error::ErrWrap::Obj(value) => ErrWrap::Obj(value.into_iter().map(|(k, v)| (String::from(k.clone()), mapErrWrap(v))).collect()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(
            &ErrWrap::Obj(HashMap::from([(String::from("is"), ErrWrap::Arr(vec![Err::Bool, Err::Required, Err::Eq(Value::Bool(false))]))]))
            ).unwrap(),
            String::from(r#"{"is":["Bool","Required",{"Eq":{"Bool":false}}]}"#)
        );
    }
}
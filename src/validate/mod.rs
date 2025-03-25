use std::collections::HashMap;

use araucaria::{error::ErrWrap, validation::Validation, value::Value};
use num_f::validate_num_f;
use num_i::validate_num_i;
use num_u::validate_num_u;
use str::validate_str;

use crate::validate::bool::validate_bool;

pub mod bool;
pub mod num_f;
pub mod num_i;
pub mod num_u;
pub mod str;

pub fn validate(validation: &Validation, value: &Value) -> Option<ErrWrap> {
    match validation {
        Validation::Bool(v) => validate_bool(v, value),
        Validation::Str(v) => validate_str(v, value),
        Validation::NumU(v) => validate_num_u(v, value),
        Validation::NumI(v) => validate_num_i(v, value),
        Validation::NumF(v) => validate_num_f(v, value),
        Validation::Obj(v) => match value {
            Value::Obj(value) => {
                let result: HashMap<String, ErrWrap> = v
                    .validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            String::from(k.clone()),
                            validate(&v, value.get(k.clone()).unwrap_or(&Value::None)),
                        )
                    })
                    .filter(|(k, v)| v.is_some())
                    .map(|(k, v)| (k, v.unwrap()))
                    .collect();
                if result.is_empty() {
                    None
                } else {
                    Some(ErrWrap::Obj(result))
                }
            }
            Value::None => {
                let result: HashMap<String, ErrWrap> = v
                    .validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (String::from(k.clone()), validate(&v, &Value::None)))
                    .filter(|(k, v)| v.is_some())
                    .map(|(k, v)| (k, v.unwrap()))
                    .collect();
                if result.is_empty() {
                    None
                } else {
                    Some(ErrWrap::Obj(result))
                }
            }
            _ => {
                let result: HashMap<String, ErrWrap> = v
                    .validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (String::from(k.clone()), validate(&v, &Value::None)))
                    .filter(|(k, v)| v.is_some())
                    .map(|(k, v)| (k, v.unwrap()))
                    .collect();
                if result.is_empty() {
                    None
                } else {
                    Some(ErrWrap::Obj(result))
                }
            }
        },
        _ => None,
    }
}

#[cfg(test)]
mod test {

    use araucaria::{
        error::Err,
        validation::{bool::BoolValidation, ObjValidation},
    };

    use super::*;

    #[test]
    fn test_bool() {
        assert_eq!(
            validate(
                &Validation::Bool(BoolValidation::default().required().eq(false)),
                &Value::Bool(false)
            ),
            None
        );
        assert_eq!(
            validate(
                &Validation::Bool(BoolValidation::default().required().eq(false)),
                &Value::None
            ),
            ErrWrap::arr([Err::Bool, Err::Required, Err::Eq(Value::Bool(false))])
        );
    }

    #[test]
    fn test_bool_some() {
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default()), &Value::NumU(1)),
            ErrWrap::arr([Err::Bool])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().required()), &Value::None),
            ErrWrap::arr([Err::Bool, Err::Required])
        );
        assert_eq!(
            validate(&Validation::Bool(BoolValidation::default().eq(false)), &Value::Bool(true)),
            ErrWrap::arr([Err::Eq(Value::Bool(false))])
        );
    }

    #[test]
    fn test_obj_ok() {
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation {
                    validation: HashMap::from([(
                        "is",
                        Validation::Bool(BoolValidation::default().required().eq(false))
                    )]),
                    required: false
                }),
                &Value::Obj(HashMap::from([(String::from("is"), Value::Bool(false))]))
            ),
            None
        );
    }

    #[test]
    fn test_obj_err() {
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation {
                    validation: HashMap::from([(
                        "is",
                        Validation::Bool(BoolValidation::default().required().eq(false))
                    )]),
                    required: false
                }),
                &Value::None
            ),
            ErrWrap::obj([(
                String::from("is"),
                ErrWrap::Arr(vec![Err::Bool, Err::Required, Err::Eq(Value::Bool(false))])
            )])
        );
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation {
                    validation: HashMap::from([(
                        "is",
                        Validation::Bool(BoolValidation::default().required().eq(false))
                    )]),
                    required: true
                }),
                &Value::None
            ),
            ErrWrap::obj([(
                String::from("is"),
                ErrWrap::Arr(vec![Err::Bool, Err::Required, Err::Eq(Value::Bool(false))])
            )])
        );
        assert_eq!(
            validate(
                &Validation::Obj(ObjValidation {
                    validation: HashMap::from([(
                        "is",
                        Validation::Bool(BoolValidation::default().required().eq(false))
                    )]),
                    required: false
                }),
                &Value::Bool(false)
            ),
            ErrWrap::obj([(
                String::from("is"),
                ErrWrap::Arr(vec![Err::Bool, Err::Required, Err::Eq(Value::Bool(false))])
            )])
        );
    }
}

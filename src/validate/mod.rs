use std::collections::HashMap;

use araucaria::{error::SchemaErr, validation::Validation, value::Value};
use bool::validate_bool;
use email::validate_email;
use num_f::validate_num_f;
use num_i::validate_num_i;
use num_u::validate_num_u;
use str::validate_str;

pub mod bool;
pub mod email;
pub mod num_f;
pub mod num_i;
pub mod num_u;
pub mod str;

pub fn validate(validation: &Validation, value: &Value) -> Result<(), SchemaErr> {
    match validation {
        Validation::U64(v) => validate_num_u(v, value),
        Validation::I64(v) => validate_num_i(v, value),
        Validation::F64(v) => validate_num_f(v, value),
        Validation::Bool(v) => validate_bool(v, value),
        Validation::Str(v) => validate_str(v, value),
        Validation::Date(v) => Ok(()),
        Validation::Time(v) => Ok(()),
        Validation::DateTime(V) => Ok(()),
        Validation::Email(v) => validate_email(v, value),
        Validation::Obj(v) => match value {
            Value::Obj(value) => {
                let result: HashMap<String, SchemaErr> = v
                    .validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k.clone(), validate(&v, value.get(&k).unwrap_or(&Value::None))))
                    .filter(|(k, v)| v.is_err())
                    .map(|(k, v)| (k, v.unwrap_err()))
                    .collect();
                if result.is_empty() {
                    Ok(())
                } else {
                    Err(SchemaErr::Obj(result))
                }
            }
            Value::None => {
                let result: HashMap<String, SchemaErr> = v
                    .validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k.clone(), validate(&v, &Value::None)))
                    .filter(|(k, v)| v.is_err())
                    .map(|(k, v)| (k, v.unwrap_err()))
                    .collect();
                if result.is_empty() {
                    Ok(())
                } else {
                    Err(SchemaErr::Obj(result))
                }
            }
            _ => {
                let result: HashMap<String, SchemaErr> = v
                    .validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k.clone(), validate(&v, &Value::None)))
                    .filter(|(k, v)| v.is_err())
                    .map(|(k, v)| (k, v.unwrap_err()))
                    .collect();
                if result.is_empty() {
                    Ok(())
                } else {
                    Err(SchemaErr::Obj(result))
                }
            }
        },
    }
}

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::{
            bool::BoolValidation, date::DateValidation, datetime::DateTimeValidation, email::EmailValidation, num_f::NumFValidation,
            num_i::NumIValidation, num_u::NumUValidation, str::StrValidation, time::TimeValidation, ObjValidation,
        },
        value::Value,
    };

    use super::{validate, Validation};

    #[test]
    fn test_validate_primite_types() {
        assert_eq!(validate(&Validation::U64(NumUValidation::default().eq(1917)), &Value::U64(1917)), Ok(()));
        assert_eq!(validate(&Validation::I64(NumIValidation::default().eq(-800)), &Value::I64(-800)), Ok(()));
        assert_eq!(validate(&Validation::F64(NumFValidation::default().eq(1.5)), &Value::F64(1.5)), Ok(()));
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().eq(false)), &Value::Bool(false)), Ok(()));
        assert_eq!(validate(&Validation::Str(StrValidation::default().eq(String::from("Gladius"))), &Value::Str(String::from("Gladius"))), Ok(()));
        assert_eq!(
            validate(&Validation::Date(DateValidation::default().eq(String::from("2015-12-28"))), &Value::Str(String::from("2015-12-28"))),
            Ok(())
        );
        assert_eq!(validate(&Validation::Time(TimeValidation::default().eq(String::from("20:38"))), &Value::Str(String::from("20:38"))), Ok(()));
        assert_eq!(
            validate(
                &Validation::DateTime(DateTimeValidation::default().eq(String::from("2015-12-28T20:38Z"))),
                &Value::Str(String::from("2015-12-28T20:38Z"))
            ),
            Ok(())
        );
        assert_eq!(validate(&Validation::Email(EmailValidation::default()), &Value::Str(String::from("bruno@gmail.com"))), Ok(()));
    }

    #[test]
    fn test_obj_ok() {
        assert_eq!(
            validate(
                &Validation::Obj(
                    ObjValidation::default().validation(HashMap::from([(String::from("is"), Validation::Bool(BoolValidation::default().eq(false)))]))
                ),
                &Value::Obj(HashMap::from([(String::from("is"), Value::Bool(false))]))
            ),
            Ok(())
        );
    }

    #[test]
    fn test_obj_err() {
        assert_eq!(
            validate(
                &Validation::Obj(
                    ObjValidation::default().validation(HashMap::from([(String::from("is"), Validation::Bool(BoolValidation::default().eq(false)))]))
                ),
                &Value::None
            ),
            Err(SchemaErr::obj([(
                String::from("is"),
                SchemaErr::Validation(vec![
                    ValidationErr::Required,
                    ValidationErr::Bool,
                    ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false))))
                ])
            )]))
        );
        assert_eq!(
            validate(
                &Validation::Obj(
                    ObjValidation::default().validation(HashMap::from([(String::from("is"), Validation::Bool(BoolValidation::default().eq(false)))]))
                ),
                &Value::None
            ),
            Err(SchemaErr::obj([(
                String::from("is"),
                SchemaErr::Validation(vec![
                    ValidationErr::Required,
                    ValidationErr::Bool,
                    ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false))))
                ])
            )]))
        );
        assert_eq!(
            validate(
                &Validation::Obj(
                    ObjValidation::default().validation(HashMap::from([(String::from("is"), Validation::Bool(BoolValidation::default().eq(false)))]))
                ),
                &Value::Bool(false)
            ),
            Err(SchemaErr::obj([(
                String::from("is"),
                SchemaErr::Validation(vec![
                    ValidationErr::Required,
                    ValidationErr::Bool,
                    ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false))))
                ])
            )]))
        );
    }
}

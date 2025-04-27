use std::collections::BTreeMap;

use araucaria::{error::SchemaErr, validation::Validation, value::Value};
use validate_bool::validate_bool;
use validate_date::validate_date;
use validate_date_time::validate_date_time;
use validate_email::validate_email;
use validate_f64::validate_f64;
use validate_i64::validate_i64;
use validate_isize::validate_isize;
use validate_str::validate_str;
use validate_time::validate_time;
use validate_u64::validate_u64;
use validate_usize::validate_usize;

mod validate_bool;
mod validate_date;
mod validate_date_time;
mod validate_email;
mod validate_f64;
mod validate_i64;
mod validate_isize;
mod validate_str;
mod validate_time;
mod validate_u64;
mod validate_usize;

pub fn validate(validation: &Validation, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    match validation {
        Validation::U64(v) => validate_u64(v, value, root),
        Validation::I64(v) => validate_i64(v, value, root),
        Validation::F64(v) => validate_f64(v, value, root),
        Validation::USize(v) => validate_usize(v, value, root),
        Validation::ISize(v) => validate_isize(v, value, root),
        Validation::Bool(v) => validate_bool(v, value, root),
        Validation::Str(v) => validate_str(v, value, root),
        Validation::Date(v) => validate_date(v, value, root),
        Validation::Time(v) => validate_time(v, value, root),
        Validation::DateTime(v) => validate_date_time(v, value, root),
        Validation::Email(v) => validate_email(v, value),
        Validation::Obj(v) => match value {
            Value::Obj(value) => {
                let result: BTreeMap<String, SchemaErr> = v
                    .validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k.clone(), validate(&v, value.get(&k).unwrap_or(&Value::None), root)))
                    .filter(|(_k, v)| v.is_err())
                    .map(|(k, v)| (k, v.unwrap_err()))
                    .collect();
                if result.is_empty() { Ok(()) } else { Err(SchemaErr::Obj(result)) }
            }
            Value::None => {
                let result: BTreeMap<String, SchemaErr> = v
                    .validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k.clone(), validate(&v, &Value::None, root)))
                    .filter(|(_k, v)| v.is_err())
                    .map(|(k, v)| (k, v.unwrap_err()))
                    .collect();
                if result.is_empty() { Ok(()) } else { Err(SchemaErr::Obj(result)) }
            }
            _ => {
                let result: BTreeMap<String, SchemaErr> = v
                    .validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k.clone(), validate(&v, &Value::None, root)))
                    .filter(|(_k, v)| v.is_err())
                    .map(|(k, v)| (k, v.unwrap_err()))
                    .collect();
                if result.is_empty() { Ok(()) } else { Err(SchemaErr::Obj(result)) }
            }
        },
        Validation::Enum(v) => Ok(()),
    }
}

#[cfg(test)]
mod tests {

    use std::collections::BTreeMap;

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::{
            BoolValidation, DateTimeValidation, DateValidation, EmailValidation, F64Validation, I64Validation, ISizeValidation, ObjValidation,
            StrValidation, TimeValidation, U64Validation, USizeValidation, Validation,
        },
        value::Value,
    };

    use super::validate;

    #[test]
    fn validate_primite_types() {
        let root = Value::None;
        assert_eq!(validate(&Validation::U64(U64Validation::default().eq(1917)), &Value::U64(1917), &root), Ok(()));
        assert_eq!(validate(&Validation::I64(I64Validation::default().eq(-800)), &Value::I64(-800), &root), Ok(()));
        assert_eq!(validate(&Validation::F64(F64Validation::default().eq(1.5)), &Value::F64(1.5), &root), Ok(()));
        assert_eq!(validate(&Validation::USize(USizeValidation::default().eq(1917)), &Value::USize(1917), &root), Ok(()));
        assert_eq!(validate(&Validation::ISize(ISizeValidation::default().eq(-284)), &Value::ISize(-284), &root), Ok(()));
        assert_eq!(validate(&Validation::Bool(BoolValidation::default().eq(false)), &Value::Bool(false), &root), Ok(()));
        assert_eq!(validate(&Validation::Str(StrValidation::default().eq("Gladius".into())), &Value::from("Gladius"), &root), Ok(()));
        assert_eq!(validate(&Validation::Date(DateValidation::default().eq("2015-12-28".into())), &Value::from("2015-12-28"), &root), Ok(()));
        assert_eq!(validate(&Validation::Time(TimeValidation::default().eq("20:38".into())), &Value::from("20:38"), &root), Ok(()));
        assert_eq!(
            validate(&Validation::DateTime(DateTimeValidation::default().eq("2015-12-28T20:38Z".into())), &Value::from("2015-12-28T20:38Z"), &root),
            Ok(())
        );
        assert_eq!(validate(&Validation::Email(EmailValidation::default()), &Value::from("bruno@gmail.com"), &root), Ok(()));
    }

    #[test]
    fn obj_ok() {
        let root = Value::None;
        assert_eq!(
            validate(
                &Validation::Obj(
                    ObjValidation::default().validation(BTreeMap::from([("is".into(), Validation::Bool(BoolValidation::default().eq(false)))]))
                ),
                &Value::Obj(BTreeMap::from([("is".into(), Value::Bool(false))])),
                &root
            ),
            Ok(())
        );
    }

    #[test]
    fn obj_err() {
        let root = Value::None;
        assert_eq!(
            validate(
                &Validation::Obj(
                    ObjValidation::default().validation(BTreeMap::from([("is".into(), Validation::Bool(BoolValidation::default().eq(false)))]))
                ),
                &Value::None,
                &root
            ),
            Err(SchemaErr::obj([(
                "is".into(),
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
                    ObjValidation::default().validation(BTreeMap::from([("is".into(), Validation::Bool(BoolValidation::default().eq(false)))]))
                ),
                &Value::None,
                &root
            ),
            Err(SchemaErr::obj([(
                "is".into(),
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
                    ObjValidation::default().validation(BTreeMap::from([("is".into(), Validation::Bool(BoolValidation::default().eq(false)))]))
                ),
                &Value::Bool(false),
                &root
            ),
            Err(SchemaErr::obj([(
                "is".into(),
                SchemaErr::Validation(vec![
                    ValidationErr::Required,
                    ValidationErr::Bool,
                    ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false))))
                ])
            )]))
        );
    }
}

use std::collections::BTreeMap;

use araucaria::{error::SchemaErr, validation::Validation, value::Value};
use validate_bool::validate_bool;
use validate_date::validate_date;
use validate_date_time::validate_date_time;
use validate_email::validate_email;
use validate_enum::validate_enum;
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
mod validate_enum;
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
        Validation::Enum(v) => validate_enum(v, value),
    }
}

#[cfg(test)]
mod tests {

    use std::{collections::BTreeMap, sync::LazyLock};

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::{
            BoolValidation, DateTimeValidation, DateValidation, EmailValidation, EnumValidation, F64Validation, I64Validation, ISizeValidation,
            ObjValidation, StrValidation, TimeValidation, U64Validation, USizeValidation, Validation,
        },
        value::Value,
    };

    use super::validate;

    static ROOT: LazyLock<Value> = LazyLock::new(|| Value::None);
    const REQUIRED: ValidationErr = ValidationErr::Required;
    const BOOL: ValidationErr = ValidationErr::Bool;

    #[test]
    fn validate_required_not_nested() {
        let v_u64 = Validation::U64(U64Validation::default().eq(1917));
        let v_i64 = Validation::I64(I64Validation::default().eq(-800));
        let v_f64 = Validation::F64(F64Validation::default().eq(1.5));
        let v_usize = Validation::USize(USizeValidation::default().eq(1917));
        let v_isize = Validation::ISize(ISizeValidation::default().eq(-284));
        let v_bool = Validation::Bool(BoolValidation::default().eq(false));
        let v_str = Validation::Str(StrValidation::default().eq("Gladius".into()));
        let v_email = Validation::Email(EmailValidation::default());
        let v_date = Validation::Date(DateValidation::default().eq("2015-12-28".into()));
        let v_time = Validation::Time(TimeValidation::default().eq("20:38".into()));
        let v_date_time = Validation::DateTime(DateTimeValidation::default().eq("2015-12-28T20:38Z".into()));
        let v_enum = Validation::Enum(EnumValidation::from(vec!["UNIX".to_string(), "LINUX".to_string(), "FREEBSD".to_string()]));

        assert_eq!(validate(&v_u64, &Value::U64(1917), &ROOT), Ok(()));
        assert_eq!(validate(&v_i64, &Value::I64(-800), &ROOT), Ok(()));
        assert_eq!(validate(&v_f64, &Value::F64(1.5), &ROOT), Ok(()));
        assert_eq!(validate(&v_usize, &Value::USize(1917), &ROOT), Ok(()));
        assert_eq!(validate(&v_isize, &Value::ISize(-284), &ROOT), Ok(()));
        assert_eq!(validate(&v_bool, &Value::Bool(false), &ROOT), Ok(()));
        assert_eq!(validate(&v_str, &Value::from("Gladius"), &ROOT), Ok(()));
        assert_eq!(validate(&v_email, &Value::from("bruno@gmail.com"), &ROOT), Ok(()));
        assert_eq!(validate(&v_date, &Value::from("2015-12-28"), &ROOT), Ok(()));
        assert_eq!(validate(&v_time, &Value::from("20:38"), &ROOT), Ok(()));
        assert_eq!(validate(&v_date_time, &Value::from("2015-12-28T20:38Z"), &ROOT), Ok(()));
        assert_eq!(validate(&v_enum, &Value::from("LINUX"), &ROOT), Ok(()));
    }

    #[test]
    fn validate_obj_nested_ok() {
        let v = Validation::Obj(
            ObjValidation::default().validation(BTreeMap::from([("is".into(), Validation::Bool(BoolValidation::default().eq(false)))])),
        );
        let value = Value::Obj(BTreeMap::from([("is".into(), Value::Bool(false))]));
        assert_eq!(validate(&v, &value, &ROOT), Ok(()));
    }

    #[test]
    fn validate_obj_none() {
        let v = Validation::Obj(
            ObjValidation::default().validation(BTreeMap::from([("is".into(), Validation::Bool(BoolValidation::default().eq(false)))])),
        );
        let value = Value::None;
        assert_eq!(
            validate(&v, &value, &ROOT),
            Err(SchemaErr::obj([(
                "is".into(),
                SchemaErr::Validation(vec![REQUIRED, BOOL, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false))))])
            )]))
        );
    }

    #[test]
    fn validate_obj_wrong() {
        let v = Validation::Obj(
            ObjValidation::default().validation(BTreeMap::from([("is".into(), Validation::Bool(BoolValidation::default().eq(false)))])),
        );
        let value = Value::Bool(false);
        assert_eq!(
            validate(&v, &value, &ROOT),
            Err(SchemaErr::obj([(
                "is".into(),
                SchemaErr::Validation(vec![REQUIRED, BOOL, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false))))])
            )]))
        );
    }
}

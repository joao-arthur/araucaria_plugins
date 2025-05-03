use std::collections::BTreeMap;

use araucaria::{
    error::{SchemaErr, ValidationErr, schema_err_has_required},
    validation::Validation,
    value::Value,
};
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

fn internal_validate(validation: &Validation, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let result = match validation {
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
                    .map(|(k, v)| (k.clone(), internal_validate(&v, value.get(&k).unwrap_or(&Value::None), root)))
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
                    .map(|(k, v)| (k.clone(), internal_validate(&v, &Value::None, root)))
                    .filter(|(_k, v)| v.is_err())
                    .map(|(k, v)| (k, v.unwrap_err()))
                    .collect();
                if result.is_empty() {
                    return Ok(());
                } else {
                    if v.required {
                        return Err(SchemaErr::arr([SchemaErr::validation([ValidationErr::Required]), SchemaErr::Obj(result)]));
                    } else {
                        return Err(SchemaErr::Obj(result));
                    }
                }
            }
            _ => {
                let result: BTreeMap<String, SchemaErr> = v
                    .validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k.clone(), internal_validate(&v, &Value::None, root)))
                    .filter(|(_k, v)| v.is_err())
                    .map(|(k, v)| (k, v.unwrap_err()))
                    .collect();
                if result.is_empty() { Ok(()) } else { Err(SchemaErr::Obj(result)) }
            }
        },
        Validation::Enum(v) => validate_enum(v, value),
    };

    match result {
        Ok(()) => Ok(()),
        Err(err) => {
            if schema_err_has_required(err.clone()) {
                Err(err)
            } else {
                Ok(())
            }
        }
    }
}

pub fn validate(validation: &Validation, value: &Value) -> Result<(), SchemaErr> {
    internal_validate(validation, value, value)
}

#[cfg(test)]
mod tests {

    use std::collections::BTreeMap;

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        validation::{
            BoolValidation, DateTimeValidation, DateValidation, EmailValidation, EnumValidation, EnumValues, F64Validation, I64Validation,
            ISizeValidation, ObjValidation, StrValidation, TimeValidation, U64Validation, USizeValidation, Validation,
        },
        value::{Value, stub::bool_stub},
    };

    use super::validate;

    const REQUIRED: ValidationErr = ValidationErr::Required;
    const U64: ValidationErr = ValidationErr::U64;
    const I64: ValidationErr = ValidationErr::I64;
    const F64: ValidationErr = ValidationErr::F64;
    const USIZE: ValidationErr = ValidationErr::USize;
    const ISIZE: ValidationErr = ValidationErr::ISize;
    const BOOL: ValidationErr = ValidationErr::Bool;
    const STR: ValidationErr = ValidationErr::Str;
    const EMAIL: ValidationErr = ValidationErr::Email;
    const DATE: ValidationErr = ValidationErr::Date;
    const TIME: ValidationErr = ValidationErr::Time;
    const DATE_TIME: ValidationErr = ValidationErr::DateTime;

    #[test]
    fn validate_default() {
        let str_values = vec!["UNIX".to_string(), "LINUX".to_string(), "FREEBSD".to_string()];
        let v_u64 = Validation::U64(U64Validation::default());
        let v_i64 = Validation::I64(I64Validation::default());
        let v_f64 = Validation::F64(F64Validation::default());
        let v_usize = Validation::USize(USizeValidation::default());
        let v_isize = Validation::ISize(ISizeValidation::default());
        let v_bool = Validation::Bool(BoolValidation::default());
        let v_str = Validation::Str(StrValidation::default());
        let v_email = Validation::Email(EmailValidation::default());
        let v_date = Validation::Date(DateValidation::default());
        let v_time = Validation::Time(TimeValidation::default());
        let v_date_time = Validation::DateTime(DateTimeValidation::default());
        let v_enum = Validation::Enum(EnumValidation::from(str_values.clone()));

        assert_eq!(validate(&v_u64, &Value::U64(1917)), Ok(()));
        assert_eq!(validate(&v_i64, &Value::I64(-800)), Ok(()));
        assert_eq!(validate(&v_f64, &Value::F64(1.5)), Ok(()));
        assert_eq!(validate(&v_usize, &Value::USize(1917)), Ok(()));
        assert_eq!(validate(&v_isize, &Value::ISize(-284)), Ok(()));
        assert_eq!(validate(&v_bool, &Value::Bool(false)), Ok(()));
        assert_eq!(validate(&v_str, &Value::from("Gladius")), Ok(()));
        assert_eq!(validate(&v_email, &Value::from("bruno@gmail.com")), Ok(()));
        assert_eq!(validate(&v_date, &Value::from("2015-12-28")), Ok(()));
        assert_eq!(validate(&v_time, &Value::from("20:38")), Ok(()));
        assert_eq!(validate(&v_date_time, &Value::from("2015-12-28T20:38Z")), Ok(()));
        assert_eq!(validate(&v_enum, &Value::from("LINUX")), Ok(()));

        assert_eq!(validate(&v_u64, &Value::None), Err(SchemaErr::validation([REQUIRED, U64,])));
        assert_eq!(validate(&v_i64, &Value::None), Err(SchemaErr::validation([REQUIRED, I64])));
        assert_eq!(validate(&v_f64, &Value::None), Err(SchemaErr::validation([REQUIRED, F64])));
        assert_eq!(validate(&v_usize, &Value::None), Err(SchemaErr::validation([REQUIRED, USIZE])));
        assert_eq!(validate(&v_isize, &Value::None), Err(SchemaErr::validation([REQUIRED, ISIZE])));
        assert_eq!(validate(&v_bool, &Value::None), Err(SchemaErr::validation([REQUIRED, BOOL])));
        assert_eq!(validate(&v_str, &Value::None), Err(SchemaErr::validation([REQUIRED, STR])));
        assert_eq!(validate(&v_email, &Value::None), Err(SchemaErr::validation([REQUIRED, EMAIL])));
        assert_eq!(validate(&v_date, &Value::None), Err(SchemaErr::validation([REQUIRED, DATE])));
        assert_eq!(validate(&v_time, &Value::None), Err(SchemaErr::validation([REQUIRED, TIME])));
        assert_eq!(validate(&v_date_time, &Value::None), Err(SchemaErr::validation([REQUIRED, DATE_TIME])));
        assert_eq!(
            validate(&v_enum, &Value::None),
            Err(SchemaErr::validation([REQUIRED, ValidationErr::Enumerated(EnumValues::from(str_values.clone()))]))
        );
    }

    // #[test]
    fn validate_optional() {
        let str_values = vec!["UNIX".to_string(), "LINUX".to_string(), "FREEBSD".to_string()];
        let v_u64 = Validation::U64(U64Validation::default().optional());
        let v_i64 = Validation::I64(I64Validation::default().optional());
        let v_f64 = Validation::F64(F64Validation::default().optional());
        let v_usize = Validation::USize(USizeValidation::default().optional());
        let v_isize = Validation::ISize(ISizeValidation::default().optional());
        let v_bool = Validation::Bool(BoolValidation::default().optional());
        let v_str = Validation::Str(StrValidation::default().optional());
        let v_email = Validation::Email(EmailValidation::default().optional());
        let v_date = Validation::Date(DateValidation::default().optional());
        let v_time = Validation::Time(TimeValidation::default().optional());
        let v_date_time = Validation::DateTime(DateTimeValidation::default().optional());
        let v_enum = Validation::Enum(EnumValidation::from(str_values.clone()).optional());

        assert_eq!(validate(&v_u64, &Value::U64(1917)), Ok(()));
        assert_eq!(validate(&v_i64, &Value::I64(-800)), Ok(()));
        assert_eq!(validate(&v_f64, &Value::F64(1.5)), Ok(()));
        assert_eq!(validate(&v_usize, &Value::USize(1917)), Ok(()));
        assert_eq!(validate(&v_isize, &Value::ISize(-284)), Ok(()));
        assert_eq!(validate(&v_bool, &Value::Bool(false)), Ok(()));
        assert_eq!(validate(&v_str, &Value::from("Gladius")), Ok(()));
        assert_eq!(validate(&v_email, &Value::from("bruno@gmail.com")), Ok(()));
        assert_eq!(validate(&v_date, &Value::from("2015-12-28")), Ok(()));
        assert_eq!(validate(&v_time, &Value::from("20:38")), Ok(()));
        assert_eq!(validate(&v_date_time, &Value::from("2015-12-28T20:38Z")), Ok(()));
        assert_eq!(validate(&v_enum, &Value::from("LINUX")), Ok(()));

        assert_eq!(validate(&v_u64, &Value::None), Ok(()));
        assert_eq!(validate(&v_i64, &Value::None), Ok(()));
        assert_eq!(validate(&v_f64, &Value::None), Ok(()));
        assert_eq!(validate(&v_usize, &Value::None), Ok(()));
        assert_eq!(validate(&v_isize, &Value::None), Ok(()));
        assert_eq!(validate(&v_bool, &Value::None), Ok(()));
        assert_eq!(validate(&v_str, &Value::None), Ok(()));
        assert_eq!(validate(&v_email, &Value::None), Ok(()));
        assert_eq!(validate(&v_date, &Value::None), Ok(()));
        assert_eq!(validate(&v_time, &Value::None), Ok(()));
        assert_eq!(validate(&v_date_time, &Value::None), Ok(()));
        assert_eq!(validate(&v_enum, &Value::None), Ok(()));
    }

    // #[test]
    fn validate_obj_required() {
        let v = Validation::Obj(ObjValidation::default().validation(BTreeMap::from([("bool".into(), Validation::Bool(BoolValidation::default()))])));
        assert_eq!(
            validate(&v, &Value::None),
            Err(SchemaErr::arr([SchemaErr::validation([REQUIRED]), SchemaErr::obj([("bool".into(), SchemaErr::validation([REQUIRED, BOOL]))])]))
        );
        assert_eq!(validate(&v, &bool_stub()), Err(SchemaErr::obj([("bool".into(), SchemaErr::validation([REQUIRED, BOOL]))])));
    }

    //  #[test]
    fn validate_obj_optional() {
        let v = Validation::Obj(
            ObjValidation::default().optional().validation(BTreeMap::from([("bool".into(), Validation::Bool(BoolValidation::default()))])),
        );
        assert_eq!(validate(&v, &Value::None), Ok(()));
        assert_eq!(validate(&v, &bool_stub()), Err(SchemaErr::obj([("bool".into(), SchemaErr::validation([REQUIRED, BOOL]))])));
    }

    // #[test]
    fn validate_obj_required_nested_required() {
        //        let v = Validation::Obj(ObjValidation::default().validation(BTreeMap::from([("bool".into(), Validation::Bool(BoolValidation::default()))])));
        //
        //        let err = SchemaErr::obj([("bool".into(), SchemaErr::validation([REQUIRED, BOOL]))]);
        //
        //        let value_nested_bool = Value::from([("bool".into(), Value::Bool(false))]);
        //        let value_nested_none = Value::from([("bool".into(), Value::None)]);
        //        let value_nested_other_type = Value::from([("bool".into(), Value::U64(19))]);
        //        let value_nested_missing_field = Value::from([("u64".into(), Value::U64(19))]);
        //
        //        let err_nested_none = SchemaErr::obj([("bool".into(), SchemaErr::validation([REQUIRED, BOOL]))]);
        //        let err_nested_other_type = SchemaErr::obj([("bool".into(), SchemaErr::validation([BOOL]))]);
        //
        //        assert_eq!(validate(&v, &value_nested_bool), Ok(()));
        //        assert_eq!(validate(&v, &value_nested_none), Err(err_nested_none.clone()));
        //        assert_eq!(validate(&v, &value_nested_other_type), Err(err_nested_other_type.clone()));
        //        assert_eq!(validate(&v, &value_nested_missing_field), Err(err.clone()));
        //        assert_eq!(validate(&v, &Value::None), Err(err.clone()));
        //        assert_eq!(validate(&v, &bool_stub()), Err(err.clone()));
    }

    #[test]
    fn validate_obj_required_nested_optional() {}

    #[test]
    fn validate_obj_optional_nested_optional() {}

    #[test]
    fn validate_obj_optional_nested_optional__() {}

    //    #[test]
    //    fn validate_enforce_optional_obj_optional() {
    //        let v = Validation::Obj(
    //            ObjValidation::default().optional().validation(BTreeMap::from([("bool".into(), Validation::Bool(BoolValidation::default()))])),
    //        );
    //        let err = SchemaErr::obj([("bool".into(), SchemaErr::validation([REQUIRED, BOOL]))]);
    //
    //        let value_nested_ok = Value::Obj(BTreeMap::from([("bool".into(), Value::Bool(false))]));
    //        let value_nested_none = Value::Obj(BTreeMap::from([("bool".into(), Value::None)]));
    //        let value_nested_other_type = Value::Obj(BTreeMap::from([("bool".into(), Value::U64(19))]));
    //        let value_nested_missing_field = Value::Obj(BTreeMap::from([("u64".into(), Value::U64(19))]));
    //
    //        let err_nested_none = SchemaErr::obj([("bool".into(), SchemaErr::validation([REQUIRED, BOOL]))]);
    //        let err_nested_other_type = SchemaErr::obj([("bool".into(), SchemaErr::validation([BOOL]))]);
    //
    //        assert_eq!(validate(&v, &value_nested_ok), Ok(()));
    //        assert_eq!(validate(&v, &value_nested_none), Err(err_nested_none.clone()));
    //        assert_eq!(validate(&v, &value_nested_other_type), Err(err_nested_other_type.clone()));
    //        assert_eq!(validate(&v, &value_nested_missing_field), Err(err.clone()));
    //
    //        assert_eq!(validate(&v, &Value::None), Err(err.clone()));
    //        assert_eq!(validate(&v, &bool_stub()), Err(err.clone()));
    //    }
    //
    //    #[test]
    //    fn validate_enforce_optional_obj_default_optional() {
    //        let v = Validation::Obj(
    //            ObjValidation::default().validation(BTreeMap::from([("bool".into(), Validation::Bool(BoolValidation::default().optional()))])),
    //        );
    //        let err = SchemaErr::obj([("bool".into(), SchemaErr::validation([BOOL]))]);
    //
    //        let value_nested_ok = Value::Obj(BTreeMap::from([("bool".into(), Value::Bool(false))]));
    //        let value_nested_none = Value::Obj(BTreeMap::from([("bool".into(), Value::None)]));
    //        let value_nested_other_type = Value::Obj(BTreeMap::from([("bool".into(), Value::U64(19))]));
    //        let value_nested_missing_field = Value::Obj(BTreeMap::from([("u64".into(), Value::U64(19))]));
    //
    //        assert_eq!(validate(&v, &value_nested_ok), Ok(()));
    //        assert_eq!(validate(&v, &value_nested_none), Err(err.clone()));
    //        assert_eq!(validate(&v, &value_nested_other_type), Err(err.clone()));
    //        assert_eq!(validate(&v, &value_nested_missing_field), Err(err.clone()));
    //
    //        assert_eq!(validate(&v, &Value::None), Err(err.clone()));
    //        assert_eq!(validate(&v, &bool_stub()), Err(err.clone()));
    //    }
    //
    //    #[test]
    //    fn validate_enforce_optional_empty_obj() {
    //        let v = Validation::Obj(ObjValidation::default());
    //
    //        let value_nested_ok = Value::Obj(BTreeMap::from([("bool".into(), Value::Bool(false))]));
    //        let value_nested_none = Value::Obj(BTreeMap::from([("bool".into(), Value::None)]));
    //        let value_nested_other_type = Value::Obj(BTreeMap::from([("bool".into(), Value::U64(19))]));
    //        let value_nested_missing_field = Value::Obj(BTreeMap::from([("u64".into(), Value::U64(19))]));
    //
    //        assert_eq!(validate(&v, &value_nested_ok), Ok(()));
    //        assert_eq!(validate(&v, &value_nested_none), Ok(()));
    //        assert_eq!(validate(&v, &value_nested_other_type), Ok(()));
    //        assert_eq!(validate(&v, &value_nested_missing_field), Ok(()));
    //
    //        assert_eq!(validate(&v, &Value::None), Ok(()));
    //        assert_eq!(validate(&v, &bool_stub()), Ok(()));
    //    }
}

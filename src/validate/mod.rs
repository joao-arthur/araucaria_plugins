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

fn validate(validation: &Validation, value: &Value, root: &Value, enforce_optional: bool) -> Result<(), SchemaErr> {
    let result = match validation {
        Validation::U64(v) => validate_u64(v, value, root, enforce_optional),
        Validation::I64(v) => validate_i64(v, value, root, enforce_optional),
        Validation::F64(v) => validate_f64(v, value, root, enforce_optional),
        Validation::USize(v) => validate_usize(v, value, root, enforce_optional),
        Validation::ISize(v) => validate_isize(v, value, root, enforce_optional),
        Validation::Bool(v) => validate_bool(v, value, root, enforce_optional),
        Validation::Str(v) => validate_str(v, value, root, enforce_optional),
        Validation::Date(v) => validate_date(v, value, root, enforce_optional),
        Validation::Time(v) => validate_time(v, value, root, enforce_optional),
        Validation::DateTime(v) => validate_date_time(v, value, root, enforce_optional),
        Validation::Email(v) => validate_email(v, value, enforce_optional),
        Validation::Obj(v) => match value {
            Value::Obj(value) => {
                let result: BTreeMap<String, SchemaErr> = v
                    .validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k.clone(), validate(&v, value.get(&k).unwrap_or(&Value::None), root, enforce_optional)))
                    .filter(|(_k, v)| v.is_err())
                    .map(|(k, v)| (k, v.unwrap_err()))
                    .collect();
                if result.is_empty() { Ok(()) } else { Err(SchemaErr::Obj(result)) }
            }
            Value::None => {
                if enforce_optional {
                    let result: BTreeMap<String, SchemaErr> = v
                        .validation
                        .clone()
                        .into_iter()
                        .map(|(k, v)| (k.clone(), validate(&v, &Value::None, root, enforce_optional)))
                        .filter(|(_k, v)| v.is_err())
                        .map(|(k, v)| (k, v.unwrap_err()))
                        .collect();
                    if result.is_empty() {
                        return Ok(());
                    } else {
                        return Err(SchemaErr::Obj(result));
                    }
                } else {
                    if v.required {
                        let result: BTreeMap<String, SchemaErr> = v
                            .validation
                            .clone()
                            .into_iter()
                            .map(|(k, v)| (k.clone(), validate(&v, &Value::None, root, enforce_optional)))
                            .filter(|(_k, v)| v.is_err())
                            .map(|(k, v)| (k, v.unwrap_err()))
                            .collect();
                        if result.is_empty() {
                            return Ok(());
                        } else {
                            return Err(SchemaErr::Obj(result));
                        }
                    }
                }

                Ok(())
            }
            _ => {
                let result: BTreeMap<String, SchemaErr> = v
                    .validation
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k.clone(), validate(&v, &Value::None, root, enforce_optional)))
                    .filter(|(_k, v)| v.is_err())
                    .map(|(k, v)| (k, v.unwrap_err()))
                    .collect();
                if result.is_empty() { Ok(()) } else { Err(SchemaErr::Obj(result)) }
            }
        },
        Validation::Enum(v) => validate_enum(v, value, enforce_optional),
    };

    result
}

pub fn validate_enforce_required(validation: &Validation, value: &Value) -> Result<(), SchemaErr> {
    validate(validation, value, value, false)
}

pub fn validate_enforce_optional(validation: &Validation, value: &Value) -> Result<(), SchemaErr> {
    validate(validation, value, value, true)
}

#[cfg(test)]
mod tests {

    use std::collections::BTreeMap;

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        validation::{
            BoolValidation, DateTimeValidation, DateValidation, EmailValidation, EnumValidation, EnumValues, F64Validation, I64Validation, ISizeValidation, ObjValidation, StrValidation, TimeValidation, U64Validation, USizeValidation, Validation
        },
        value::{stub::bool_stub, Value},
    };

    use super::{validate_enforce_optional, validate_enforce_required};

    const REQUIRED: ValidationErr = ValidationErr::Required;
    const BOOL: ValidationErr = ValidationErr::Bool;
    const U64: ValidationErr = ValidationErr::U64;

    #[test]
    fn validate_enforce_required_not_nested_required_ok() {
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

        assert_eq!(validate_enforce_required(&v_u64, &Value::U64(1917)), Ok(()));
        assert_eq!(validate_enforce_required(&v_i64, &Value::I64(-800)), Ok(()));
        assert_eq!(validate_enforce_required(&v_f64, &Value::F64(1.5)), Ok(()));
        assert_eq!(validate_enforce_required(&v_usize, &Value::USize(1917)), Ok(()));
        assert_eq!(validate_enforce_required(&v_isize, &Value::ISize(-284)), Ok(()));
        assert_eq!(validate_enforce_required(&v_bool, &Value::Bool(false)), Ok(()));
        assert_eq!(validate_enforce_required(&v_str, &Value::from("Gladius")), Ok(()));
        assert_eq!(validate_enforce_required(&v_email, &Value::from("bruno@gmail.com")), Ok(()));
        assert_eq!(validate_enforce_required(&v_date, &Value::from("2015-12-28")), Ok(()));
        assert_eq!(validate_enforce_required(&v_time, &Value::from("20:38")), Ok(()));
        assert_eq!(validate_enforce_required(&v_date_time, &Value::from("2015-12-28T20:38Z")), Ok(()));
        assert_eq!(validate_enforce_required(&v_enum, &Value::from("LINUX")), Ok(()));
    }

    #[test]
    fn validate_enforce_required_not_nested_optional_ok() {
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

        assert_eq!(validate_enforce_required(&v_u64, &Value::None), Ok(()));
        assert_eq!(validate_enforce_required(&v_i64, &Value::None), Ok(()));
        assert_eq!(validate_enforce_required(&v_f64, &Value::None), Ok(()));
        assert_eq!(validate_enforce_required(&v_usize, &Value::None), Ok(()));
        assert_eq!(validate_enforce_required(&v_isize, &Value::None), Ok(()));
        assert_eq!(validate_enforce_required(&v_bool, &Value::None), Ok(()));
        assert_eq!(validate_enforce_required(&v_str, &Value::None), Ok(()));
        assert_eq!(validate_enforce_required(&v_email, &Value::None), Ok(()));
        assert_eq!(validate_enforce_required(&v_date, &Value::None), Ok(()));
        assert_eq!(validate_enforce_required(&v_time, &Value::None), Ok(()));
        assert_eq!(validate_enforce_required(&v_date_time, &Value::None), Ok(()));
        assert_eq!(validate_enforce_required(&v_enum, &Value::None), Ok(()));
    }

    #[test]
    fn validate_enforce_required_obj_default() {
        let v = Validation::Obj(ObjValidation::default().validation(BTreeMap::from([("bool".into(), Validation::Bool(BoolValidation::default()))])));
        let err = SchemaErr::obj([("bool".into(), SchemaErr::arr([REQUIRED, BOOL]))]);

        let value_nested_ok = Value::Obj(BTreeMap::from([("bool".into(), Value::Bool(false))]));
        let value_nested_none = Value::Obj(BTreeMap::from([("bool".into(), Value::None)]));
        let value_nested_other_type = Value::Obj(BTreeMap::from([("bool".into(), Value::U64(19))]));
        let value_nested_missing_field = Value::Obj(BTreeMap::from([("u64".into(), Value::U64(19))]));

        let err_nested_none = SchemaErr::obj([("bool".into(), SchemaErr::arr([REQUIRED, BOOL]))]);
        let err_nested_other_type = SchemaErr::obj([("bool".into(), SchemaErr::arr([BOOL]))]);

        assert_eq!(validate_enforce_required(&v, &value_nested_ok), Ok(()));
        assert_eq!(validate_enforce_required(&v, &value_nested_none), Err(err_nested_none.clone()));
        assert_eq!(validate_enforce_required(&v, &value_nested_other_type), Err(err_nested_other_type.clone()));
        assert_eq!(validate_enforce_required(&v, &value_nested_missing_field), Err(err.clone()));
        //
        assert_eq!(validate_enforce_required(&v, &Value::None), Err(err.clone()));
        assert_eq!(validate_enforce_required(&v, &Value::None), Err(err.clone()));
        assert_eq!(validate_enforce_required(&v, &bool_stub()), Err(err.clone()));
    }

    #[test]
    fn validate_enforce_required_obj_optional() {
        let v = Validation::Obj(
            ObjValidation::default().optional().validation(BTreeMap::from([("bool".into(), Validation::Bool(BoolValidation::default()))])),
        );
        let err = SchemaErr::obj([("bool".into(), SchemaErr::arr([REQUIRED, BOOL]))]);

        let value_nested_ok = Value::Obj(BTreeMap::from([("bool".into(), Value::Bool(false))]));
        let value_nested_none = Value::Obj(BTreeMap::from([("bool".into(), Value::None)]));
        let value_nested_other_type = Value::Obj(BTreeMap::from([("bool".into(), Value::U64(19))]));
        let value_nested_missing_field = Value::Obj(BTreeMap::from([("u64".into(), Value::U64(19))]));

        let err_nested_none = SchemaErr::obj([("bool".into(), SchemaErr::arr([REQUIRED, BOOL]))]);
        let err_nested_other_type = SchemaErr::obj([("bool".into(), SchemaErr::arr([BOOL]))]);

        assert_eq!(validate_enforce_required(&v, &value_nested_ok), Ok(()));
        assert_eq!(validate_enforce_required(&v, &value_nested_none), Err(err_nested_none.clone()));
        assert_eq!(validate_enforce_required(&v, &value_nested_other_type), Err(err_nested_other_type.clone()));
        assert_eq!(validate_enforce_required(&v, &value_nested_missing_field), Err(err.clone()));
        //
        assert_eq!(validate_enforce_required(&v, &Value::None), Ok(()));
        assert_eq!(validate_enforce_required(&v, &bool_stub()), Err(err.clone()));
    }

    #[test]
    fn validate_enforce_required_obj_default_optional() {
        let v = Validation::Obj(
            ObjValidation::default().validation(BTreeMap::from([("bool".into(), Validation::Bool(BoolValidation::default().optional()))])),
        );
        let err = SchemaErr::obj([("bool".into(), SchemaErr::arr([BOOL]))]);

        let value_nested_ok = Value::Obj(BTreeMap::from([("bool".into(), Value::Bool(false))]));
        let value_nested_none = Value::Obj(BTreeMap::from([("bool".into(), Value::None)]));
        let value_nested_other_type = Value::Obj(BTreeMap::from([("bool".into(), Value::U64(19))]));
        let value_nested_missing_field = Value::Obj(BTreeMap::from([("u64".into(), Value::U64(19))]));

        assert_eq!(validate_enforce_required(&v, &value_nested_ok), Ok(()));
        assert_eq!(validate_enforce_required(&v, &value_nested_none), Ok(()));
        assert_eq!(validate_enforce_required(&v, &value_nested_other_type), Err(err.clone()));
        assert_eq!(validate_enforce_required(&v, &value_nested_missing_field), Ok(()));
    }

    #[test]
    fn validate_enforce_optional_not_nested_required_ok() {
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

        assert_eq!(validate_enforce_optional(&v_u64, &Value::U64(1917)), Ok(()));
        assert_eq!(validate_enforce_optional(&v_i64, &Value::I64(-800)), Ok(()));
        assert_eq!(validate_enforce_optional(&v_f64, &Value::F64(1.5)), Ok(()));
        assert_eq!(validate_enforce_optional(&v_usize, &Value::USize(1917)), Ok(()));
        assert_eq!(validate_enforce_optional(&v_isize, &Value::ISize(-284)), Ok(()));
        assert_eq!(validate_enforce_optional(&v_bool, &Value::Bool(false)), Ok(()));
        assert_eq!(validate_enforce_optional(&v_str, &Value::from("Gladius")), Ok(()));
        assert_eq!(validate_enforce_optional(&v_email, &Value::from("bruno@gmail.com")), Ok(()));
        assert_eq!(validate_enforce_optional(&v_date, &Value::from("2015-12-28")), Ok(()));
        assert_eq!(validate_enforce_optional(&v_time, &Value::from("20:38")), Ok(()));
        assert_eq!(validate_enforce_optional(&v_date_time, &Value::from("2015-12-28T20:38Z")), Ok(()));
        assert_eq!(validate_enforce_optional(&v_enum, &Value::from("LINUX")), Ok(()));
    }

    #[test]
    fn validate_enforce_optional_not_nested_optional_ok() {
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

        assert_eq!(validate_enforce_optional(&v_u64, &Value::None), Err(SchemaErr::arr([U64])));
        assert_eq!(validate_enforce_optional(&v_i64, &Value::None), Err(SchemaErr::arr([ValidationErr::I64])));
        assert_eq!(validate_enforce_optional(&v_f64, &Value::None), Err(SchemaErr::arr([ValidationErr::F64])));
        assert_eq!(validate_enforce_optional(&v_usize, &Value::None), Err(SchemaErr::arr([ValidationErr::USize])));
        assert_eq!(validate_enforce_optional(&v_isize, &Value::None), Err(SchemaErr::arr([ValidationErr::ISize])));
        assert_eq!(validate_enforce_optional(&v_bool, &Value::None), Err(SchemaErr::arr([ValidationErr::Bool])));
        assert_eq!(validate_enforce_optional(&v_str, &Value::None), Err(SchemaErr::arr([ValidationErr::Str])));
        assert_eq!(validate_enforce_optional(&v_email, &Value::None), Err(SchemaErr::arr([ValidationErr::Email])));
        assert_eq!(validate_enforce_optional(&v_date, &Value::None), Err(SchemaErr::arr([ValidationErr::Date])));
        assert_eq!(validate_enforce_optional(&v_time, &Value::None), Err(SchemaErr::arr([ValidationErr::Time])));
        assert_eq!(validate_enforce_optional(&v_date_time, &Value::None), Err(SchemaErr::arr([ValidationErr::DateTime])));
        assert_eq!(validate_enforce_optional(&v_enum, &Value::None), Err(SchemaErr::arr([ValidationErr::Enumerated(EnumValues::from(str_values.clone()))])));
    }

    #[test]
    fn validate_enforce_optional_obj_default() {
        let v = Validation::Obj(ObjValidation::default().validation(BTreeMap::from([("bool".into(), Validation::Bool(BoolValidation::default()))])));
        let err = SchemaErr::obj([("bool".into(), SchemaErr::arr([REQUIRED, BOOL]))]);

        let value_nested_ok = Value::Obj(BTreeMap::from([("bool".into(), Value::Bool(false))]));
        let value_nested_none = Value::Obj(BTreeMap::from([("bool".into(), Value::None)]));
        let value_nested_other_type = Value::Obj(BTreeMap::from([("bool".into(), Value::U64(19))]));
        let value_nested_missing_field = Value::Obj(BTreeMap::from([("u64".into(), Value::U64(19))]));

        let err_nested_none = SchemaErr::obj([("bool".into(), SchemaErr::arr([REQUIRED, BOOL]))]);
        let err_nested_other_type = SchemaErr::obj([("bool".into(), SchemaErr::arr([BOOL]))]);

        assert_eq!(validate_enforce_optional(&v, &value_nested_ok), Ok(()));
        assert_eq!(validate_enforce_optional(&v, &value_nested_none), Err(err_nested_none.clone()));
        assert_eq!(validate_enforce_optional(&v, &value_nested_other_type), Err(err_nested_other_type.clone()));
        assert_eq!(validate_enforce_optional(&v, &value_nested_missing_field), Err(err.clone()));

        assert_eq!(validate_enforce_optional(&v, &Value::None), Err(err.clone()));
        assert_eq!(validate_enforce_optional(&v, &bool_stub()), Err(err.clone()));
    }

    #[test]
    fn validate_enforce_optional_obj_optional() {
        let v = Validation::Obj(
            ObjValidation::default().optional().validation(BTreeMap::from([("bool".into(), Validation::Bool(BoolValidation::default()))])),
        );
        let err = SchemaErr::obj([("bool".into(), SchemaErr::arr([REQUIRED, BOOL]))]);

        let value_nested_ok = Value::Obj(BTreeMap::from([("bool".into(), Value::Bool(false))]));
        let value_nested_none = Value::Obj(BTreeMap::from([("bool".into(), Value::None)]));
        let value_nested_other_type = Value::Obj(BTreeMap::from([("bool".into(), Value::U64(19))]));
        let value_nested_missing_field = Value::Obj(BTreeMap::from([("u64".into(), Value::U64(19))]));

        let err_nested_none = SchemaErr::obj([("bool".into(), SchemaErr::arr([REQUIRED, BOOL]))]);
        let err_nested_other_type = SchemaErr::obj([("bool".into(), SchemaErr::arr([BOOL]))]);

        assert_eq!(validate_enforce_optional(&v, &value_nested_ok), Ok(()));
        assert_eq!(validate_enforce_optional(&v, &value_nested_none), Err(err_nested_none.clone()));
        assert_eq!(validate_enforce_optional(&v, &value_nested_other_type), Err(err_nested_other_type.clone()));
        assert_eq!(validate_enforce_optional(&v, &value_nested_missing_field), Err(err.clone()));
        //
        assert_eq!(validate_enforce_optional(&v, &Value::None), Err(err.clone()));
        assert_eq!(validate_enforce_optional(&v, &bool_stub()), Err(err.clone()));
    }

    #[test]
    fn validate_enforce_optional_obj_default_optional() {
        let v = Validation::Obj(
            ObjValidation::default().validation(BTreeMap::from([("bool".into(), Validation::Bool(BoolValidation::default().optional()))])),
        );
        let err = SchemaErr::obj([("bool".into(), SchemaErr::arr([BOOL]))]);

        let value_nested_none = Value::Obj(BTreeMap::from([("bool".into(), Value::None)]));
        let value_nested_other_type = Value::Obj(BTreeMap::from([("bool".into(), Value::U64(19))]));
        let value_nested_missing_field = Value::Obj(BTreeMap::from([("u64".into(), Value::U64(19))]));

        assert_eq!(validate_enforce_optional(&v, &value_nested_none), Err(err.clone()));
        assert_eq!(validate_enforce_optional(&v, &value_nested_other_type), Err(err.clone()));
        assert_eq!(validate_enforce_optional(&v, &value_nested_other_type), Err(err.clone()));
        assert_eq!(validate_enforce_optional(&v, &value_nested_missing_field), Err(err.clone()));
    }
}

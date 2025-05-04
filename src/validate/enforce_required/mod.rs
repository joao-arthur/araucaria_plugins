use std::collections::BTreeMap;

use araucaria::{error::SchemaErr, schema::Schema, value::Value};
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

fn internal_validate(schema: &Schema, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let result = match schema {
        Schema::U64(v) => validate_u64(v, value, root),
        Schema::I64(v) => validate_i64(v, value, root),
        Schema::F64(v) => validate_f64(v, value, root),
        Schema::USize(v) => validate_usize(v, value, root),
        Schema::ISize(v) => validate_isize(v, value, root),
        Schema::Bool(v) => validate_bool(v, value, root),
        Schema::Str(v) => validate_str(v, value, root),
        Schema::Date(v) => validate_date(v, value, root),
        Schema::Time(v) => validate_time(v, value, root),
        Schema::DateTime(v) => validate_date_time(v, value, root),
        Schema::Email(v) => validate_email(v, value),
        Schema::Obj(v) => match value {
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
                if v.required {
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
                        return Err(SchemaErr::Obj(result));
                    }
                } else {
                    return Ok(());
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
        Schema::Enum(v) => validate_enum(v, value),
    };

    result
}

pub fn validate(schema: &Schema, value: &Value) -> Result<(), SchemaErr> {
    internal_validate(schema, value, value)
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, sync::LazyLock};

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        schema::{
            BoolSchema, DateSchema, DateTimeSchema, EmailSchema, EnumSchema, EnumValues, F64Schema, I64Schema, ISizeSchema, ObjSchema, Schema,
            StrSchema, TimeSchema, U64Schema, USizeSchema,
        },
        value::{Value, stub::bool_stub},
    };

    use super::validate;

    const ENUM_STR: [&str; 3] = ["UNIX", "LINUX", "FREEBSD"];
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

    static V_U64: LazyLock<Schema> = LazyLock::new(|| Schema::U64(U64Schema::default()));
    static V_I64: LazyLock<Schema> = LazyLock::new(|| Schema::I64(I64Schema::default()));
    static V_F64: LazyLock<Schema> = LazyLock::new(|| Schema::F64(F64Schema::default()));
    static V_USIZE: LazyLock<Schema> = LazyLock::new(|| Schema::USize(USizeSchema::default()));
    static V_ISIZE: LazyLock<Schema> = LazyLock::new(|| Schema::ISize(ISizeSchema::default()));
    static V_BOOL: LazyLock<Schema> = LazyLock::new(|| Schema::Bool(BoolSchema::default()));
    static V_STR: LazyLock<Schema> = LazyLock::new(|| Schema::Str(StrSchema::default()));
    static V_EMAIL: LazyLock<Schema> = LazyLock::new(|| Schema::Email(EmailSchema::default()));
    static V_DATE: LazyLock<Schema> = LazyLock::new(|| Schema::Date(DateSchema::default()));
    static V_TIME: LazyLock<Schema> = LazyLock::new(|| Schema::Time(TimeSchema::default()));
    static V_DATE_TIME: LazyLock<Schema> = LazyLock::new(|| Schema::DateTime(DateTimeSchema::default()));
    static V_ENUM: LazyLock<Schema> = LazyLock::new(|| Schema::Enum(EnumSchema::from(ENUM_STR)));

    static V_U64_OPTIONAL: LazyLock<Schema> = LazyLock::new(|| Schema::U64(U64Schema::default().optional()));
    static V_I64_OPTIONAL: LazyLock<Schema> = LazyLock::new(|| Schema::I64(I64Schema::default().optional()));
    static V_F64_OPTIONAL: LazyLock<Schema> = LazyLock::new(|| Schema::F64(F64Schema::default().optional()));
    static V_USIZE_OPTIONAL: LazyLock<Schema> = LazyLock::new(|| Schema::USize(USizeSchema::default().optional()));
    static V_ISIZE_OPTIONAL: LazyLock<Schema> = LazyLock::new(|| Schema::ISize(ISizeSchema::default().optional()));
    static V_BOOL_OPTIONAL: LazyLock<Schema> = LazyLock::new(|| Schema::Bool(BoolSchema::default().optional()));
    static V_STR_OPTIONAL: LazyLock<Schema> = LazyLock::new(|| Schema::Str(StrSchema::default().optional()));
    static V_EMAIL_OPTIONAL: LazyLock<Schema> = LazyLock::new(|| Schema::Email(EmailSchema::default().optional()));
    static V_DATE_OPTIONAL: LazyLock<Schema> = LazyLock::new(|| Schema::Date(DateSchema::default().optional()));
    static V_TIME_OPTIONAL: LazyLock<Schema> = LazyLock::new(|| Schema::Time(TimeSchema::default().optional()));
    static V_DATE_TIME_OPTIONAL: LazyLock<Schema> = LazyLock::new(|| Schema::DateTime(DateTimeSchema::default().optional()));
    static V_ENUM_OPTIONAL: LazyLock<Schema> = LazyLock::new(|| Schema::Enum(EnumSchema::from(ENUM_STR).optional()));

    #[test]
    fn validate_default_correct_value() {
        assert_eq!(validate(&V_U64, &Value::U64(1917)), Ok(()));
        assert_eq!(validate(&V_I64, &Value::I64(-800)), Ok(()));
        assert_eq!(validate(&V_F64, &Value::F64(1.5)), Ok(()));
        assert_eq!(validate(&V_USIZE, &Value::USize(1917)), Ok(()));
        assert_eq!(validate(&V_ISIZE, &Value::ISize(-284)), Ok(()));
        assert_eq!(validate(&V_BOOL, &Value::Bool(false)), Ok(()));
        assert_eq!(validate(&V_STR, &Value::from("Gladius")), Ok(()));
        assert_eq!(validate(&V_EMAIL, &Value::from("bruno@gmail.com")), Ok(()));
        assert_eq!(validate(&V_DATE, &Value::from("2015-12-28")), Ok(()));
        assert_eq!(validate(&V_TIME, &Value::from("20:38")), Ok(()));
        assert_eq!(validate(&V_DATE_TIME, &Value::from("2015-12-28T20:38Z")), Ok(()));
        assert_eq!(validate(&V_ENUM, &Value::from("LINUX")), Ok(()));
    }

    #[test]
    fn validate_default_none_value() {
        assert_eq!(validate(&V_U64, &Value::None), Err(SchemaErr::from([REQUIRED, U64])));
        assert_eq!(validate(&V_I64, &Value::None), Err(SchemaErr::from([REQUIRED, I64])));
        assert_eq!(validate(&V_F64, &Value::None), Err(SchemaErr::from([REQUIRED, F64])));
        assert_eq!(validate(&V_USIZE, &Value::None), Err(SchemaErr::from([REQUIRED, USIZE])));
        assert_eq!(validate(&V_ISIZE, &Value::None), Err(SchemaErr::from([REQUIRED, ISIZE])));
        assert_eq!(validate(&V_BOOL, &Value::None), Err(SchemaErr::from([REQUIRED, BOOL])));
        assert_eq!(validate(&V_STR, &Value::None), Err(SchemaErr::from([REQUIRED, STR])));
        assert_eq!(validate(&V_EMAIL, &Value::None), Err(SchemaErr::from([REQUIRED, EMAIL])));
        assert_eq!(validate(&V_DATE, &Value::None), Err(SchemaErr::from([REQUIRED, DATE])));
        assert_eq!(validate(&V_TIME, &Value::None), Err(SchemaErr::from([REQUIRED, TIME])));
        assert_eq!(validate(&V_DATE_TIME, &Value::None), Err(SchemaErr::from([REQUIRED, DATE_TIME])));
        assert_eq!(validate(&V_ENUM, &Value::None), Err(SchemaErr::from([REQUIRED, ValidationErr::Enumerated(EnumValues::from(ENUM_STR))])));
    }

    #[test]
    fn validate_optional_correct_value() {
        assert_eq!(validate(&V_U64_OPTIONAL, &Value::U64(1917)), Ok(()));
        assert_eq!(validate(&V_I64_OPTIONAL, &Value::I64(-800)), Ok(()));
        assert_eq!(validate(&V_F64_OPTIONAL, &Value::F64(1.5)), Ok(()));
        assert_eq!(validate(&V_USIZE_OPTIONAL, &Value::USize(1917)), Ok(()));
        assert_eq!(validate(&V_ISIZE_OPTIONAL, &Value::ISize(-284)), Ok(()));
        assert_eq!(validate(&V_BOOL_OPTIONAL, &Value::Bool(false)), Ok(()));
        assert_eq!(validate(&V_STR_OPTIONAL, &Value::from("Gladius")), Ok(()));
        assert_eq!(validate(&V_EMAIL_OPTIONAL, &Value::from("bruno@gmail.com")), Ok(()));
        assert_eq!(validate(&V_DATE_OPTIONAL, &Value::from("2015-12-28")), Ok(()));
        assert_eq!(validate(&V_TIME_OPTIONAL, &Value::from("20:38")), Ok(()));
        assert_eq!(validate(&V_DATE_TIME_OPTIONAL, &Value::from("2015-12-28T20:38Z")), Ok(()));
        assert_eq!(validate(&V_ENUM_OPTIONAL, &Value::from("LINUX")), Ok(()));
    }

    #[test]
    fn validate_optional_none_value() {
        assert_eq!(validate(&V_U64_OPTIONAL, &Value::None), Ok(()));
        assert_eq!(validate(&V_I64_OPTIONAL, &Value::None), Ok(()));
        assert_eq!(validate(&V_F64_OPTIONAL, &Value::None), Ok(()));
        assert_eq!(validate(&V_USIZE_OPTIONAL, &Value::None), Ok(()));
        assert_eq!(validate(&V_ISIZE_OPTIONAL, &Value::None), Ok(()));
        assert_eq!(validate(&V_BOOL_OPTIONAL, &Value::None), Ok(()));
        assert_eq!(validate(&V_STR_OPTIONAL, &Value::None), Ok(()));
        assert_eq!(validate(&V_EMAIL_OPTIONAL, &Value::None), Ok(()));
        assert_eq!(validate(&V_DATE_OPTIONAL, &Value::None), Ok(()));
        assert_eq!(validate(&V_TIME_OPTIONAL, &Value::None), Ok(()));
        assert_eq!(validate(&V_DATE_TIME_OPTIONAL, &Value::None), Ok(()));
        assert_eq!(validate(&V_ENUM_OPTIONAL, &Value::None), Ok(()));
    }

    #[test]
    fn validate_obj_required() {
        let v = Schema::Obj(ObjSchema::from(BTreeMap::from([("bool".into(), Schema::Bool(BoolSchema::default()))])));
        assert_eq!(validate(&v, &Value::None), Err(SchemaErr::from([("bool".into(), SchemaErr::from([REQUIRED, BOOL]))])));
        assert_eq!(validate(&v, &bool_stub()), Err(SchemaErr::from([("bool".into(), SchemaErr::from([REQUIRED, BOOL]))])));
    }

    #[test]
    fn validate_obj_optional() {
        let v = Schema::Obj(ObjSchema::from(BTreeMap::from([("bool".into(), Schema::Bool(BoolSchema::default()))])).optional());
        assert_eq!(validate(&v, &Value::None), Ok(()));
        assert_eq!(validate(&v, &bool_stub()), Err(SchemaErr::from([("bool".into(), SchemaErr::from([REQUIRED, BOOL]))])));
    }

    #[test]
    fn validate_obj_required_nested_required() {
        let v = Schema::Obj(ObjSchema::from(BTreeMap::from([("bool".into(), Schema::Bool(BoolSchema::default()))])));

        let value_bool = Value::from([("bool".into(), Value::Bool(false))]);
        let value_other_type = Value::from([("bool".into(), Value::U64(19))]);
        let value_none = Value::from([("bool".into(), Value::None)]);
        let value_missing_field = Value::from([("u64".into(), Value::U64(19))]);
        assert_eq!(validate(&v, &value_bool), Ok(()));
        assert_eq!(validate(&v, &value_other_type), Err(SchemaErr::from([("bool".into(), SchemaErr::from([BOOL]))])));
        assert_eq!(validate(&v, &value_none), Err(SchemaErr::from([("bool".into(), SchemaErr::from([REQUIRED, BOOL]))])));
        assert_eq!(validate(&v, &value_missing_field), Err(SchemaErr::from([("bool".into(), SchemaErr::from([REQUIRED, BOOL]))])));
    }

    #[test]
    fn validate_obj_optional_nested_required() {
        let v = Schema::Obj(ObjSchema::from(BTreeMap::from([("bool".into(), Schema::Bool(BoolSchema::default()))])).optional());
        let value_bool = Value::from([("bool".into(), Value::Bool(false))]);
        let value_other_type = Value::from([("bool".into(), Value::U64(19))]);
        let value_none = Value::from([("bool".into(), Value::None)]);
        let value_missing_field = Value::from([("u64".into(), Value::U64(19))]);
        assert_eq!(validate(&v, &value_bool), Ok(()));
        assert_eq!(validate(&v, &value_other_type), Err(SchemaErr::from([("bool".into(), SchemaErr::from([BOOL]))])));
        assert_eq!(validate(&v, &value_none), Err(SchemaErr::from([("bool".into(), SchemaErr::from([REQUIRED, BOOL]))])));
        assert_eq!(validate(&v, &value_missing_field), Err(SchemaErr::from([("bool".into(), SchemaErr::from([REQUIRED, BOOL]))])));
    }

    #[test]
    fn validate_obj_required_nested_optional() {
        let v = Schema::Obj(ObjSchema::from(BTreeMap::from([("bool".into(), Schema::Bool(BoolSchema::default().optional()))])));
        let value_bool = Value::from([("bool".into(), Value::Bool(false))]);
        let value_other_type = Value::from([("bool".into(), Value::U64(19))]);
        let value_none = Value::from([("bool".into(), Value::None)]);
        let value_missing_field = Value::from([("u64".into(), Value::U64(19))]);
        assert_eq!(validate(&v, &value_bool), Ok(()));
        assert_eq!(validate(&v, &value_other_type), Err(SchemaErr::from([("bool".into(), SchemaErr::from([BOOL]))])));
        assert_eq!(validate(&v, &value_none), Ok(()));
        assert_eq!(validate(&v, &value_missing_field), Ok(()));
    }

    #[test]
    fn validate_obj_optional_nested_optional() {
        let v = Schema::Obj(ObjSchema::from(BTreeMap::from([("bool".into(), Schema::Bool(BoolSchema::default().optional()))])).optional());
        let value_bool = Value::from([("bool".into(), Value::Bool(false))]);
        let value_other_type = Value::from([("bool".into(), Value::U64(19))]);
        let value_none = Value::from([("bool".into(), Value::None)]);
        let value_missing_field = Value::from([("u64".into(), Value::U64(19))]);
        assert_eq!(validate(&v, &value_bool), Ok(()));
        assert_eq!(validate(&v, &value_other_type), Err(SchemaErr::from([("bool".into(), SchemaErr::from([BOOL]))])));
        assert_eq!(validate(&v, &value_none), Ok(()));
        assert_eq!(validate(&v, &value_missing_field), Ok(()));
    }

    #[test]
    fn validate_obj_empty_required() {
        let v = Schema::Obj(ObjSchema::from(BTreeMap::new()));
        assert_eq!(validate(&v, &Value::Obj(BTreeMap::new())), Ok(()));
        assert_eq!(validate(&v, &Value::None), Ok(()));
        assert_eq!(validate(&v, &bool_stub()), Ok(()));
    }

    #[test]
    fn validate_obj_empty_optional() {
        let v = Schema::Obj(ObjSchema::from(BTreeMap::new()).optional());
        assert_eq!(validate(&v, &Value::Obj(BTreeMap::new())), Ok(()));
        assert_eq!(validate(&v, &Value::None), Ok(()));
        assert_eq!(validate(&v, &bool_stub()), Ok(()));
    }
}

use std::sync::LazyLock;

use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::DateTimeValidation,
    value::Value,
};
use regex::Regex;

#[derive(Debug, PartialEq)]
struct InternalDtTm(pub u32, pub u16, pub u16, pub u8, pub u8);

static DT_TM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2})Z$").unwrap());

fn parse_date_time(s: &str) -> Result<InternalDtTm, ()> {
    if let Some(caps) = DT_TM_REGEX.captures(s) {
        let c: (&str, [&str; 5]) = caps.extract();
        let yyyy = c.1[0].parse::<u32>().map_err(|_| ())?;
        let mm = c.1[1].parse::<u16>().map_err(|_| ())?;
        let dd = c.1[2].parse::<u16>().map_err(|_| ())?;
        let h = c.1[3].parse::<u8>().map_err(|_| ())?;
        let m = c.1[4].parse::<u8>().map_err(|_| ())?;
        Ok(InternalDtTm(yyyy, mm, dd, h, m))
    } else {
        Err(())
    }
}

pub fn validate_date_time(validation: &DateTimeValidation, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            if parse_date_time(str_value).is_ok() {
                if let Some(operation) = &validation.operation {
                    if let Some(Err(())) = compare(operation, &OperandValue::Str(str_value.clone()), root) {
                        base.push(ValidationErr::Operation(operation.clone()));
                    }
                }
            } else {
                base.push(ValidationErr::DateTime);
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::DateTime);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
        _ => {
            base.push(ValidationErr::DateTime);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
    }
    if !base.is_empty() { Err(SchemaErr::Validation(base)) } else { Ok(()) }
}

#[cfg(test)]
mod test {
    use std::{collections::BTreeMap, sync::LazyLock};

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::DateTimeValidation,
        value::{Value, stub::u64_stub},
    };

    use super::{InternalDtTm, parse_date_time, validate_date_time};

    static ROOT: LazyLock<Value> = LazyLock::new(|| {
        Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2025-05-05T22:03Z"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2026-07-10T04:31Z"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2027-09-15T09:48Z"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2028-11-20T11:27Z"))])),
            ]),
        )]))
    });

    #[test]
    fn test_validate_date_default() {
        let v = DateTimeValidation::default();
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:27Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("not a date and time"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
    }

    #[test]
    fn test_validate_date_optional() {
        let v = DateTimeValidation::default().optional();
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:27Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:27Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("not a date and time"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
    }

    #[test]
    fn test_validate_date_eq_value() {
        let v = DateTimeValidation::default().eq("2026-10-28T11:27Z".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("2026-10-28T11:27Z"))));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:27Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2025-04-18T23:18Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_ne_value() {
        let v = DateTimeValidation::default().ne("2026-10-28T11:27Z".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::from("2026-10-28T11:27Z"))));
        assert_eq!(validate_date_time(&v, &Value::from("2025-04-18T23:18Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:27Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_gt_value() {
        let v = DateTimeValidation::default().gt("2026-10-28T11:27Z".into());
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::from("2026-10-28T11:27Z"))));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-27T01:29Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:26Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:27Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:28Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-29T03:47Z"), &ROOT), Ok(()));
        assert_eq!(
            validate_date_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_ge_value() {
        let v = DateTimeValidation::default().ge("2026-10-28T11:27Z".into());
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::from("2026-10-28T11:27Z"))));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-27T01:29Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:26Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:27Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:28Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-29T03:47Z"), &ROOT), Ok(()));
        assert_eq!(
            validate_date_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_lt_value() {
        let v = DateTimeValidation::default().lt("2026-10-28T11:27Z".into());
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::from("2026-10-28T11:27Z"))));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-27T01:29Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:26Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:27Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:28Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-29T03:47Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_le_value() {
        let v = DateTimeValidation::default().le("2026-10-28T11:27Z".into());
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::from("2026-10-28T11:27Z"))));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-27T01:29Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:26Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:27Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:28Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-29T03:47Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_btwn_value() {
        let v = DateTimeValidation::default().btwn("2026-10-01T00:00Z".into(), "2026-10-31T23:59Z".into());
        let op_err = ValidationErr::Operation(Operation::Btwn(
            Operand::Value(OperandValue::from("2026-10-01T00:00Z")),
            Operand::Value(OperandValue::from("2026-10-31T23:59Z")),
        ));
        assert_eq!(validate_date_time(&v, &Value::from("2026-09-30T23:59Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-01T00:00Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-01T00:01Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-02T04:21Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-30T16:51Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-31T23:59Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2026-11-01T01:01Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_time_eq_field() {
        let v = DateTimeValidation::default().eq_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-19T11:26Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-20T11:27Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-21T11:28Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_time_ne_field() {
        let v = DateTimeValidation::default().ne_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-19T11:26Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-20T11:27Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-21T11:28Z"), &ROOT), Ok(()));
        assert_eq!(
            validate_date_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_time_gt_field() {
        let v = DateTimeValidation::default().gt_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-19T11:26Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-20T11:27Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-21T11:28Z"), &ROOT), Ok(()));
        assert_eq!(
            validate_date_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_time_ge_field() {
        let v = DateTimeValidation::default().ge_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-19T11:26Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-20T11:27Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-21T11:28Z"), &ROOT), Ok(()));
        assert_eq!(
            validate_date_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_time_lt_field() {
        let v = DateTimeValidation::default().lt_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-19T11:26Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-20T11:27Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-21T11:28Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_time_le_field() {
        let v = DateTimeValidation::default().le_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Le(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-19T11:26Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-20T11:27Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-21T11:28Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_time_btwn_field() {
        let v = DateTimeValidation::default().btwn_field("values.2.value".into(), "values.3.value".into());
        let op_err =
            ValidationErr::Operation(Operation::Btwn(Operand::FieldPath("values.2.value".into()), Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_date_time(&v, &Value::from("2027-09-14T09:47Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::from("2027-09-15T09:48Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2027-09-16T09:49Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-19T11:26Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-20T11:27Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-21T11:28Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_invalid_format() {
        let v = DateTimeValidation::default();
        let root = Value::None;
        assert_eq!(validate_date_time(&v, &Value::from("10-10-2026"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("10-2026-10"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("2026/10/28"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("28/10/2026"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("20261028"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("28102026"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));

        assert_eq!(validate_date_time(&v, &Value::from("10:27:23.235"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("10:27:24"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("1061"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("106"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("10"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("1"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));

        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T10:27:29Z"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T10:27:29.973Z"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("10-2026-28T10:27:29.973Z"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("28-10-2026T10:27:29.973Z"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
    }

    #[test]
    fn test_validate_date_invalid_date() {
        // TODO
    }

    #[test]
    fn test_parse_iso() {
        assert_eq!(parse_date_time("2029-12-31T06:11Z".into()), Ok(InternalDtTm(2029, 12, 31, 6, 11)));
    }
}

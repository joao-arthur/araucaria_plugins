use std::sync::LazyLock;

use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::date::DateValidation,
    value::Value,
};
use regex::Regex;

#[derive(Debug, PartialEq)]
struct InternalDt(pub u32, pub u8, pub u8);

static DT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$").unwrap());

fn parse_date(s: &str) -> Result<InternalDt, ()> {
    if let Some(caps) = DT_REGEX.captures(s) {
        let c: (&str, [&str; 3]) = caps.extract();
        let yyyy = c.1[0].parse::<u32>().unwrap();
        let mm = c.1[1].parse::<u8>().unwrap();
        let dd = c.1[2].parse::<u8>().unwrap();
        Ok(InternalDt(yyyy, mm, dd))
    } else {
        Err(())
    }
}

pub fn validate_date(validation: &DateValidation, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            if parse_date(str_value).is_ok() {
                if let Some(operation) = &validation.operation {
                    if let Some(Err(())) = compare(operation, &OperandValue::Str(str_value.clone()), root) {
                        base.push(ValidationErr::Operation(operation.clone()));
                    }
                }
            } else {
                base.push(ValidationErr::Date);
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::Date);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
        _ => {
            base.push(ValidationErr::Date);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
    }
    if !base.is_empty() { Err(SchemaErr::Validation(base)) } else { Ok(()) }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::date::DateValidation,
        value::{Value, stub::num_u_stub},
    };

    use super::{InternalDt, parse_date, validate_date};

    #[test]
    fn test_validate_date_default() {
        let v = DateValidation::default();
        let root = Value::None;
        assert_eq!(validate_date(&v, &Value::from("2026-10-28"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("not a date"), &root), Err(SchemaErr::validation([ValidationErr::Date])));
        assert_eq!(validate_date(&v, &Value::None, &root), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Date])));
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date])));
    }

    #[test]
    fn test_validate_date_optional() {
        let v = DateValidation::default().optional();
        let root = Value::None;
        assert_eq!(validate_date(&v, &Value::from("2026-10-28"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2026-10-28"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("not a date"), &root), Err(SchemaErr::validation([ValidationErr::Date])));
        assert_eq!(validate_date(&v, &Value::None, &root), Err(SchemaErr::validation([ValidationErr::Date])));
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date])));
    }

    #[test]
    fn test_validate_date_eq_value() {
        let v = DateValidation::default().eq("2026-10-28".into());
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("2026-10-28"))));
        assert_eq!(validate_date(&v, &Value::from("2026-10-28"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2025-04-18"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Date, op_err.clone()]))
        );
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_ne_value() {
        let v = DateValidation::default().ne("2026-10-28".into());
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::from("2026-10-28"))));
        assert_eq!(validate_date(&v, &Value::from("2025-04-18"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2026-10-28"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Date, op_err.clone()]))
        );
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_gt_value() {
        let v = DateValidation::default().gt("2026-10-28".into());
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::from("2026-10-28"))));
        assert_eq!(validate_date(&v, &Value::from("2026-10-27"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date(&v, &Value::from("2026-10-28"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date(&v, &Value::from("2026-10-29"), &root), Ok(()));
        assert_eq!(
            validate_date(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Date, op_err.clone()]))
        );
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_ge_value() {
        let v = DateValidation::default().ge("2026-10-28".into());
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::from("2026-10-28"))));
        assert_eq!(validate_date(&v, &Value::from("2026-10-27"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date(&v, &Value::from("2026-10-28"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2026-10-29"), &root), Ok(()));
        assert_eq!(
            validate_date(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Date, op_err.clone()]))
        );
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_lt_value() {
        let v = DateValidation::default().lt("2026-10-28".into());
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::from("2026-10-28"))));
        assert_eq!(validate_date(&v, &Value::from("2026-10-27"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2026-10-28"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date(&v, &Value::from("2026-10-29"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Date, op_err.clone()]))
        );
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_le_value() {
        let v = DateValidation::default().le("2026-10-28".into());
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::from("2026-10-28"))));
        assert_eq!(validate_date(&v, &Value::from("2026-10-27"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2026-10-28"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2026-10-29"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Date, op_err.clone()]))
        );
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_btwn_value() {
        let v = DateValidation::default().btwn("2026-10-01".into(), "2026-10-31".into());
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Btwn(
            Operand::Value(OperandValue::from("2026-10-01")),
            Operand::Value(OperandValue::from("2026-10-31")),
        ));
        assert_eq!(validate_date(&v, &Value::from("2026-09-30"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date(&v, &Value::from("2026-10-01"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2026-10-02"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2026-10-30"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2026-10-31"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2026-11-01"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Date, op_err.clone()]))
        );
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_eq_field() {
        let v = DateValidation::default().eq_field("values.3.value".into());
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2025-05-05"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2026-07-10"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2027-09-15"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2028-11-20"))])),
            ]),
        )]));
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_date(&v, &Value::from("2028-11-19"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date(&v, &Value::from("2028-11-20"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2028-11-21"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Date, op_err.clone()]))
        );
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_ne_field() {
        let v = DateValidation::default().ne_field("values.3.value".into());
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2025-05-05"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2026-07-10"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2027-09-15"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2028-11-20"))])),
            ]),
        )]));
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_date(&v, &Value::from("2028-11-19"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2028-11-20"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date(&v, &Value::from("2028-11-21"), &root), Ok(()));
        assert_eq!(
            validate_date(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Date, op_err.clone()]))
        );
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_gt_field() {
        let v = DateValidation::default().gt_field("values.3.value".into());
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2025-05-05"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2026-07-10"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2027-09-15"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2028-11-20"))])),
            ]),
        )]));
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_date(&v, &Value::from("2028-11-19"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date(&v, &Value::from("2028-11-20"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date(&v, &Value::from("2028-11-21"), &root), Ok(()));
        assert_eq!(
            validate_date(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Date, op_err.clone()]))
        );
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_ge_field() {
        let v = DateValidation::default().ge_field("values.3.value".into());
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2025-05-05"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2026-07-10"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2027-09-15"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2028-11-20"))])),
            ]),
        )]));
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_date(&v, &Value::from("2028-11-19"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date(&v, &Value::from("2028-11-20"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2028-11-21"), &root), Ok(()));
        assert_eq!(
            validate_date(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Date, op_err.clone()]))
        );
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_lt_field() {
        let v = DateValidation::default().lt_field("values.3.value".into());
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2025-05-05"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2026-07-10"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2027-09-15"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2028-11-20"))])),
            ]),
        )]));
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_date(&v, &Value::from("2028-11-19"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2028-11-20"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date(&v, &Value::from("2028-11-21"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Date, op_err.clone()]))
        );
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_le_field() {
        let v = DateValidation::default().le_field("values.3.value".into());
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2025-05-05"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2026-07-10"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2027-09-15"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2028-11-20"))])),
            ]),
        )]));
        let op_err = ValidationErr::Operation(Operation::Le(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_date(&v, &Value::from("2028-11-19"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2028-11-20"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2028-11-21"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Date, op_err.clone()]))
        );
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_btwn_field() {
        let v = DateValidation::default().btwn_field("values.2.value".into(), "values.3.value".into());
        let root = Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2025-05-05"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2026-07-10"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2027-09-15"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2028-11-20"))])),
            ]),
        )]));
        let op_err = ValidationErr::Operation(Operation::Btwn(
            Operand::FieldPath("values.2.value".into()),
            Operand::FieldPath("values.3.value".into()),
        ));
        assert_eq!(validate_date(&v, &Value::from("2027-09-14"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date(&v, &Value::from("2027-09-15"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2027-09-16"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2028-11-19"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2028-11-20"), &root), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2028-11-21"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Date, op_err.clone()]))
        );
        assert_eq!(validate_date(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Date, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_invalid_format() {
        let v = DateValidation::default();
        let root = Value::None;
        assert_eq!(validate_date(&v, &Value::from("10-10-2026"), &root), Err(SchemaErr::validation([ValidationErr::Date])));
        assert_eq!(validate_date(&v, &Value::from("10-2026-10"), &root), Err(SchemaErr::validation([ValidationErr::Date])));
        assert_eq!(validate_date(&v, &Value::from("2026/10/28"), &root), Err(SchemaErr::validation([ValidationErr::Date])));
        assert_eq!(validate_date(&v, &Value::from("28/10/2026"), &root), Err(SchemaErr::validation([ValidationErr::Date])));
        assert_eq!(validate_date(&v, &Value::from("20261028"), &root), Err(SchemaErr::validation([ValidationErr::Date])));
        assert_eq!(validate_date(&v, &Value::from("28102026"), &root), Err(SchemaErr::validation([ValidationErr::Date])));
    }

    #[test]
    fn test_validate_date_invalid_date() {
        // TODO
    }

    #[test]
    fn test_parse_date() {
        assert_eq!(parse_date("2029-12-31".into()), Ok(InternalDt(2029, 12, 31)));
    }
}

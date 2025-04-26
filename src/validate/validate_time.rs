use std::sync::LazyLock;

use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::TimeValidation,
    value::Value,
};
use regex::Regex;

#[derive(Debug, PartialEq)]
struct InternalTm(pub u8, pub u8);

static TM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([0-9]{2}):([0-9]{2})$").unwrap());

fn parse_time(s: &str) -> Result<InternalTm, ()> {
    let caps = TM_REGEX.captures(s).ok_or(())?;
    let (_, [h, m]) = caps.extract();
    let h = h.parse::<u8>().map_err(|_| ())?;
    let m = m.parse::<u8>().map_err(|_| ())?;
    if h <= 23 && m <= 59 { Ok(InternalTm(h, m)) } else { Err(()) }
}

pub fn validate_time(validation: &TimeValidation, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            if parse_time(str_value).is_ok() {
                if let Some(operation) = &validation.operation {
                    if let Some(Err(())) = compare(operation, &OperandValue::Str(str_value.clone()), root) {
                        base.push(ValidationErr::Operation(operation.clone()));
                    }
                }
            } else {
                base.push(ValidationErr::Time);
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::Time);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
        _ => {
            base.push(ValidationErr::Time);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
    }
    if !base.is_empty() { Err(SchemaErr::Validation(base)) } else { Ok(()) }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, sync::LazyLock};

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::TimeValidation,
        value::{Value, stub::u64_stub},
    };

    use super::{InternalTm, parse_time, validate_time};

    static ROOT: LazyLock<Value> = LazyLock::new(|| {
        Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::from("22:03"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("04:31"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("09:48"))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::from("11:27"))])),
            ]),
        )]))
    });

    #[test]
    fn parse_time_ok() {
        assert_eq!(parse_time("06:11"), Ok(InternalTm(6, 11)));
    }

    #[test]
    fn parse_time_invalid_format() {
        assert_eq!(parse_time("10:27:23.235"), Err(()));
        assert_eq!(parse_time("10:27:24"), Err(()));
        assert_eq!(parse_time("1061"), Err(()));
        assert_eq!(parse_time("106"), Err(()));
        assert_eq!(parse_time("10"), Err(()));
        assert_eq!(parse_time("1"), Err(()));
    }

    #[test]
    fn parse_time_invalid_value() {
        assert_eq!(parse_time("24:00"), Err(()));
        assert_eq!(parse_time("00:60"), Err(()));
        assert_eq!(parse_time("24:20"), Err(()));
        assert_eq!(parse_time("04:99"), Err(()));
        assert_eq!(parse_time("72:93"), Err(()));
    }

    #[test]
    fn validate_date_default() {
        let v = TimeValidation::default();
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("not a time"), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time])));
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
    }

    #[test]
    fn validate_date_optional() {
        let v = TimeValidation::default().optional();
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("not a time"), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
    }

    #[test]
    fn validate_date_eq_value() {
        let v = TimeValidation::default().eq("11:27".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("11:27"))));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("23:18"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_date_ne_value() {
        let v = TimeValidation::default().ne("11:27".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::from("11:27"))));
        assert_eq!(validate_time(&v, &Value::from("23:18"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_date_gt_value() {
        let v = TimeValidation::default().gt("11:27".into());
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::from("11:27"))));
        assert_eq!(validate_time(&v, &Value::from("11:26"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("11:28"), &ROOT), Ok(()));
        assert_eq!(
            validate_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_date_ge_value() {
        let v = TimeValidation::default().ge("11:27".into());
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::from("11:27"))));
        assert_eq!(validate_time(&v, &Value::from("11:26"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:28"), &ROOT), Ok(()));
        assert_eq!(
            validate_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_date_lt_value() {
        let v = TimeValidation::default().lt("11:27".into());
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::from("11:27"))));
        assert_eq!(validate_time(&v, &Value::from("11:26"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("11:28"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_date_le_value() {
        let v = TimeValidation::default().le("11:27".into());
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::from("11:27"))));
        assert_eq!(validate_time(&v, &Value::from("11:26"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:28"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_date_btwn_value() {
        let v = TimeValidation::default().btwn("09:00".into(), "09:59".into());
        let op_err =
            ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::from("09:00")), Operand::Value(OperandValue::from("09:59"))));
        assert_eq!(validate_time(&v, &Value::from("08:59"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("09:00"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("09:01"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("09:58"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("09:59"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("10:00"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_time_eq_field() {
        let v = TimeValidation::default().eq_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_time(&v, &Value::from("11:26"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:28"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_time_ne_field() {
        let v = TimeValidation::default().ne_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_time(&v, &Value::from("11:26"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("11:28"), &ROOT), Ok(()));
        assert_eq!(
            validate_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_time_gt_field() {
        let v = TimeValidation::default().gt_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_time(&v, &Value::from("11:26"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("11:28"), &ROOT), Ok(()));
        assert_eq!(
            validate_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_time_ge_field() {
        let v = TimeValidation::default().ge_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_time(&v, &Value::from("11:26"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:28"), &ROOT), Ok(()));
        assert_eq!(
            validate_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_time_lt_field() {
        let v = TimeValidation::default().lt_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_time(&v, &Value::from("11:26"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("11:28"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_time_le_field() {
        let v = TimeValidation::default().le_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Le(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_time(&v, &Value::from("11:26"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:28"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_time_btwn_field() {
        let v = TimeValidation::default().btwn_field("values.2.value".into(), "values.3.value".into());
        let op_err =
            ValidationErr::Operation(Operation::Btwn(Operand::FieldPath("values.2.value".into()), Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_time(&v, &Value::from("09:47"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("09:48"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("09:49"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:26"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:28"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_time(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_time_invalid_format() {
        let v = TimeValidation::default();
        assert_eq!(validate_time(&v, &Value::from("10:27:23.235"), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("10:27:24"), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("1061"), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("106"), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("10"), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("1"), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
    }

    #[test]
    fn validate_time_invalid_value() {
        let v = TimeValidation::default();
        assert_eq!(validate_time(&v, &Value::from("24:20"), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("04:99"), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("72:93"), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
    }
}

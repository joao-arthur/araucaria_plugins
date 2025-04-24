use std::sync::LazyLock;

use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::time::TimeValidation,
    value::Value,
};
use regex::Regex;

#[derive(Debug, PartialEq)]
struct InternalTm(pub u8, pub u8);

static TM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([0-9]{2}):([0-9]{2})$").unwrap());

fn parse_time(s: &str) -> Result<InternalTm, ()> {
    if let Some(caps) = TM_REGEX.captures(s) {
        let c: (&str, [&str; 2]) = caps.extract();
        let h = c.1[0].parse::<u8>().map_err(|_| ())?;
        let m = c.1[1].parse::<u8>().map_err(|_| ())?;
        Ok(InternalTm(h, m))
    } else {
        Err(())
    }
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
mod test {
    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::time::TimeValidation,
        value::{Value, stub::num_u_stub},
    };

    use super::{InternalTm, parse_time, validate_time};

    #[test]
    fn test_validate_date_default() {
        let v = TimeValidation::default();
        let root = Value::None;
        assert_eq!(validate_time(&v, &Value::from("11:27"), &root), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("not a time"), &root), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::None, &root), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time])));
        assert_eq!(validate_time(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Time])));
    }

    #[test]
    fn test_validate_date_optional() {
        let v = TimeValidation::default().optional();
        let root = Value::None;
        assert_eq!(validate_time(&v, &Value::from("11:27"), &root), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("not a time"), &root), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::None, &root), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Time])));
    }

    #[test]
    fn test_validate_date_eq() {
        let v = TimeValidation::default().eq("11:27".into());
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("11:27"))));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &root), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("23:18"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_time(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_ne() {
        let v = TimeValidation::default().ne("11:27".into());
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::from("11:27"))));
        assert_eq!(validate_time(&v, &Value::from("23:18"), &root), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_time(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_gt() {
        let v = TimeValidation::default().gt("11:27".into());
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::from("11:27"))));
        assert_eq!(validate_time(&v, &Value::from("11:26"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("11:28"), &root), Ok(()));
        assert_eq!(
            validate_time(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_ge() {
        let v = TimeValidation::default().ge("11:27".into());
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::from("11:27"))));
        assert_eq!(validate_time(&v, &Value::from("11:26"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &root), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:28"), &root), Ok(()));
        assert_eq!(
            validate_time(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_lt() {
        let v = TimeValidation::default().lt("11:27".into());
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::from("11:27"))));
        assert_eq!(validate_time(&v, &Value::from("11:26"), &root), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("11:28"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_time(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_le() {
        let v = TimeValidation::default().le("11:27".into());
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::from("11:27"))));
        assert_eq!(validate_time(&v, &Value::from("11:26"), &root), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &root), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:28"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_time(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_btwn() {
        let v = TimeValidation::default().btwn("09:00".into(), "09:59".into());
        let root = Value::None;
        let op_err =
            ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::from("09:00")), Operand::Value(OperandValue::from("09:59"))));
        assert_eq!(validate_time(&v, &Value::from("08:59"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::from("09:00"), &root), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("09:01"), &root), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("09:58"), &root), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("09:59"), &root), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("10:00"), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_time(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()]))
        );
        assert_eq!(validate_time(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn test_validate_time_invalid_format() {
        let v = TimeValidation::default();
        let root = Value::None;
        assert_eq!(validate_time(&v, &Value::from("10:27:23.235"), &root), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("10:27:24"), &root), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("1061"), &root), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("106"), &root), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("10"), &root), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("1"), &root), Err(SchemaErr::validation([ValidationErr::Time])));
    }

    #[test]
    fn test_validate_time_invalid_date() {
        // TODO
    }

    #[test]
    fn test_parse_time() {
        assert_eq!(parse_time("06:11".into()), Ok(InternalTm(6, 11)));
    }
}

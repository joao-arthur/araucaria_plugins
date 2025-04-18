use std::sync::LazyLock;

use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{compare, OperandValue},
    validation::time::TimeValidation,
    value::Value,
};
use regex::Regex;

#[derive(Debug, PartialEq)]
struct InternalTM(pub u8, pub u8);

static TM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([0-9]{2}):([0-9]{2})$").unwrap());

pub fn parse_time(s: &str) -> Result<InternalTM, ()> {
    if let Some(caps) = TM_REGEX.captures(s) {
        let c: (&str, [&str; 2]) = caps.extract();
        let h = c.1[0].parse::<u8>().map_err(|_| ())?;
        let m = c.1[1].parse::<u8>().map_err(|_| ())?;
        return Ok(InternalTM(h, m));
    } else {
        return Err(());
    }
}

pub fn validate_time(validation: &TimeValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            if let Ok(_) = parse_time(str_value) {
                if let Some(operation) = &validation.operation {
                    if let Some(Err(())) = compare(operation, &OperandValue::Str(str_value.clone())) {
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
    if !base.is_empty() {
        Err(SchemaErr::Validation(base))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        value::{stub::num_u_stub, Value},
    };

    use super::{parse_time, validate_time, InternalTM, TimeValidation};

    #[test]
    fn test_validate_date_default() {
        let v = TimeValidation::default();
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:27"))), Ok(()));
        assert_eq!(validate_time(&v, &Value::Str(String::from("not a time"))), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time])));
        assert_eq!(validate_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Time])));
    }

    #[test]
    fn test_validate_date_optional() {
        let v = TimeValidation::default().optional();
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:27"))), Ok(()));
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:27"))), Ok(()));
        assert_eq!(validate_time(&v, &Value::Str(String::from("not a time"))), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Time])));
    }

    #[test]
    fn test_validate_date_eq() {
        let v = TimeValidation::default().eq(String::from("11:27"));
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Str(String::from("11:27")))));
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:27"))), Ok(()));
        assert_eq!(validate_time(&v, &Value::Str(String::from("23:18"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()])));
        assert_eq!(validate_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_ne() {
        let v = TimeValidation::default().ne(String::from("11:27"));
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Str(String::from("11:27")))));
        assert_eq!(validate_time(&v, &Value::Str(String::from("23:18"))), Ok(()));
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:27"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()])));
        assert_eq!(validate_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_gt() {
        let v = TimeValidation::default().gt(String::from("11:27"));
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::Str(String::from("11:27")))));
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:26"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:27"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:28"))), Ok(()));
        assert_eq!(validate_time(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()])));
        assert_eq!(validate_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_ge() {
        let v = TimeValidation::default().ge(String::from("11:27"));
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::Str(String::from("11:27")))));
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:26"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:27"))), Ok(()));
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:28"))), Ok(()));
        assert_eq!(validate_time(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()])));
        assert_eq!(validate_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_lt() {
        let v = TimeValidation::default().lt(String::from("11:27"));
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::Str(String::from("11:27")))));
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:26"))), Ok(()));
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:27"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:28"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()])));
        assert_eq!(validate_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_le() {
        let v = TimeValidation::default().le(String::from("11:27"));
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::Str(String::from("11:27")))));
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:26"))), Ok(()));
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:27"))), Ok(()));
        assert_eq!(validate_time(&v, &Value::Str(String::from("11:28"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()])));
        assert_eq!(validate_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_btwn() {
        let v = TimeValidation::default().btwn(String::from("09:00"), String::from("09:59"));
        let op_err = ValidationErr::Operation(Operation::Btwn(
            Operand::Value(OperandValue::Str(String::from("09:00"))),
            Operand::Value(OperandValue::Str(String::from("09:59"))),
        ));
        assert_eq!(validate_time(&v, &Value::Str(String::from("08:59"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::Str(String::from("09:00"))), Ok(()));
        assert_eq!(validate_time(&v, &Value::Str(String::from("09:01"))), Ok(()));
        assert_eq!(validate_time(&v, &Value::Str(String::from("09:58"))), Ok(()));
        assert_eq!(validate_time(&v, &Value::Str(String::from("09:59"))), Ok(()));
        assert_eq!(validate_time(&v, &Value::Str(String::from("10:00"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()])));
        assert_eq!(validate_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn test_validate_time_invalid_format() {
        let v = TimeValidation::default();
        assert_eq!(validate_time(&v, &Value::from("10:27:23.235")), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("10:27:24")), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("1061")), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("106")), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("10")), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::from("1")), Err(SchemaErr::validation([ValidationErr::Time])));
    }

    #[test]
    fn test_validate_time_invalid_date() {
        // TODO
    }

    #[test]
    fn test_parse_time() {
        assert_eq!(parse_time(&String::from("06:11")), Ok(InternalTM(06, 11)));
    }
}

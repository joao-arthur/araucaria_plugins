use std::sync::LazyLock;

use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{compare, OperandValue},
    validation::date_time::DateTimeValidation,
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

pub fn validate_date_time(validation: &DateTimeValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            if parse_date_time(str_value).is_ok() {
                if let Some(operation) = &validation.operation {
                    if let Some(Err(())) = compare(operation, &OperandValue::Str(str_value.clone())) {
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
        validation::date_time::DateTimeValidation,
        value::{stub::num_u_stub, Value},
    };

    use super::{parse_date_time, validate_date_time, InternalDtTm};

    #[test]
    fn test_validate_date_default() {
        let v = DateTimeValidation::default();
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:27Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("not a date and time"))), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::DateTime])));
    }

    #[test]
    fn test_validate_date_optional() {
        let v = DateTimeValidation::default().optional();
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:27Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:27Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("not a date and time"))), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::DateTime])));
    }

    #[test]
    fn test_validate_date_eq() {
        let v = DateTimeValidation::default().eq(String::from("2026-10-28T11:27Z"));
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Str(String::from("2026-10-28T11:27Z")))));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:27Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2025-04-18T23:18Z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date_time(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_ne() {
        let v = DateTimeValidation::default().ne(String::from("2026-10-28T11:27Z"));
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Str(String::from("2026-10-28T11:27Z")))));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2025-04-18T23:18Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:27Z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date_time(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_gt() {
        let v = DateTimeValidation::default().gt(String::from("2026-10-28T11:27Z"));
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::Str(String::from("2026-10-28T11:27Z")))));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-27T01:29Z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:26Z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:27Z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:28Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-29T03:47Z"))), Ok(()));
        assert_eq!(
            validate_date_time(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_ge() {
        let v = DateTimeValidation::default().ge(String::from("2026-10-28T11:27Z"));
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::Str(String::from("2026-10-28T11:27Z")))));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-27T01:29Z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:26Z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:27Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:28Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-29T03:47Z"))), Ok(()));
        assert_eq!(
            validate_date_time(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_lt() {
        let v = DateTimeValidation::default().lt(String::from("2026-10-28T11:27Z"));
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::Str(String::from("2026-10-28T11:27Z")))));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-27T01:29Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:26Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:27Z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:28Z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-29T03:47Z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date_time(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_le() {
        let v = DateTimeValidation::default().le(String::from("2026-10-28T11:27Z"));
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::Str(String::from("2026-10-28T11:27Z")))));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-27T01:29Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:26Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:27Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-28T11:28Z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-29T03:47Z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date_time(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_btwn() {
        let v = DateTimeValidation::default().btwn(String::from("2026-10-01T00:00Z"), String::from("2026-10-31T23:59Z"));
        let op_err = ValidationErr::Operation(Operation::Btwn(
            Operand::Value(OperandValue::Str(String::from("2026-10-01T00:00Z"))),
            Operand::Value(OperandValue::Str(String::from("2026-10-31T23:59Z"))),
        ));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-09-30T23:59Z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-01T00:00Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-01T00:01Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-02T04:21Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-30T16:51Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-10-31T23:59Z"))), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::Str(String::from("2026-11-01T01:01Z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_date_time(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()]))
        );
        assert_eq!(validate_date_time(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn test_validate_date_invalid_format() {
        let v = DateTimeValidation::default();
        assert_eq!(validate_date_time(&v, &Value::from("10-10-2026")), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("10-2026-10")), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("2026/10/28")), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("28/10/2026")), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("20261028")), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("28102026")), Err(SchemaErr::validation([ValidationErr::DateTime])));

        assert_eq!(validate_date_time(&v, &Value::from("10:27:23.235")), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("10:27:24")), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("1061")), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("106")), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("10")), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("1")), Err(SchemaErr::validation([ValidationErr::DateTime])));

        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T10:27:29Z")), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T10:27:29.973Z")), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("10-2026-28T10:27:29.973Z")), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &Value::from("28-10-2026T10:27:29.973Z")), Err(SchemaErr::validation([ValidationErr::DateTime])));
    }

    #[test]
    fn test_validate_date_invalid_date() {
        // TODO
    }

    #[test]
    fn test_parse_iso() {
        assert_eq!(parse_date_time(&String::from("2029-12-31T06:11Z")), Ok(InternalDtTm(2029, 12, 31, 6, 11)));
    }
}

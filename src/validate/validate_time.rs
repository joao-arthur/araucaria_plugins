use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::TimeValidation,
    value::Value,
};

use crate::utils::time::parse_time;

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

    use super::validate_time;

    static ROOT: LazyLock<Value> = LazyLock::new(|| Value::Obj(BTreeMap::from([("time_value".into(), Value::from("11:27"))])));

    #[test]
    fn validate_time_default() {
        let v = TimeValidation::default();
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("not a time"), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time])));
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
    }

    #[test]
    fn validate_time_optional() {
        let v = TimeValidation::default().optional();
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("not a time"), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
    }

    #[test]
    fn validate_time_value() {
        let v = TimeValidation::default().eq("11:27".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("11:27"))));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("23:18"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()])));
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_time_field() {
        let v = TimeValidation::default().ne_field("time_value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("time_value".into())));
        assert_eq!(validate_time(&v, &Value::from("02:18"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Time, op_err.clone()])));
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Time, op_err.clone()])));
    }

    #[test]
    fn validate_time_invalid_format() {
        let v = TimeValidation::default();
        assert_eq!(validate_time(&v, &Value::from("10:27:23.235"), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
    }

    #[test]
    fn validate_time_invalid_value() {
        let v = TimeValidation::default();
        assert_eq!(validate_time(&v, &Value::from("72:93"), &ROOT), Err(SchemaErr::validation([ValidationErr::Time])));
    }
}

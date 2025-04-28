use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::TimeValidation,
    value::Value,
};

use crate::utils::time::parse_time;

pub fn validate_time(validation: &TimeValidation, value: &Value, root: &Value, enforce_optional: bool) -> Result<(), SchemaErr> {
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
            if enforce_optional {
                if validation.required {
                    base.push(ValidationErr::Required);
                }
                base.push(ValidationErr::Time);
                if let Some(operation) = &validation.operation {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
            } else {
                if validation.required {
                    base.push(ValidationErr::Required);
                    base.push(ValidationErr::Time);
                    if let Some(operation) = &validation.operation {
                        base.push(ValidationErr::Operation(operation.clone()));
                    }
                }
            }
        }
        _ => {
            base.push(ValidationErr::Time);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
    }
    if !base.is_empty() { Err(SchemaErr::Arr(base)) } else { Ok(()) }
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
    const REQUIRED: ValidationErr = ValidationErr::Required;
    const TIME: ValidationErr = ValidationErr::Time;

    #[test]
    fn validate_time_default() {
        let v = TimeValidation::default();
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT, false), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("not a time"), &ROOT, false), Err(SchemaErr::arr([TIME])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, TIME])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, TIME])));
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([TIME])));
    }

    #[test]
    fn validate_time_optional() {
        let v = TimeValidation::default().optional();
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT, false), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("not a time"), &ROOT, false), Err(SchemaErr::arr([TIME])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([TIME])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT, false), Ok(()));
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([TIME])));
    }

    #[test]
    fn validate_time_operation_value() {
        let v = TimeValidation::default().eq("11:27".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("11:27"))));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT, false), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("23:18"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, TIME, op_err.clone()])));
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([TIME, op_err.clone()])));
    }

    #[test]
    fn validate_time_operation_field() {
        let v = TimeValidation::default().ne_field("time_value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("time_value".into())));
        assert_eq!(validate_time(&v, &Value::from("02:18"), &ROOT, false), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, TIME, op_err.clone()])));
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([TIME, op_err.clone()])));
    }

    #[test]
    fn validate_time_invalid_format() {
        let v = TimeValidation::default();
        assert_eq!(validate_time(&v, &Value::from("10:27:23.235"), &ROOT, false), Err(SchemaErr::arr([TIME])));
    }

    #[test]
    fn validate_time_invalid_value() {
        let v = TimeValidation::default();
        assert_eq!(validate_time(&v, &Value::from("72:93"), &ROOT, false), Err(SchemaErr::arr([TIME])));
    }
}

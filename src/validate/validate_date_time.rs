use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::DateTimeValidation,
    value::Value,
};

use crate::utils::date_time::parse_date_time;

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
mod tests {
    use std::{collections::BTreeMap, sync::LazyLock};

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::DateTimeValidation,
        value::{Value, stub::u64_stub},
    };

    use super::validate_date_time;

    static ROOT: LazyLock<Value> = LazyLock::new(|| {
        Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::from("2028-11-20T11:27Z"))])),
            ]),
        )]))
    });

    #[test]
    fn validate_date_time_default() {
        let v = DateTimeValidation::default();
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:27Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
    }

    #[test]
    fn validate_date_time_optional() {
        let v = DateTimeValidation::default().optional();
        assert_eq!(validate_date_time(&v, &Value::from("2026-10-28T11:27Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
    }

    #[test]
    fn validate_date_time_value() {
        let v = DateTimeValidation::default().eq("2028-11-20T11:27Z".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("2028-11-20T11:27Z"))));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-20T11:27Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2025-04-18T23:18Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()])));
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn validate_date_time_field() {
        let v = DateTimeValidation::default().ne_field("values.0.value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("values.0.value".into())));
        assert_eq!(validate_date_time(&v, &Value::from("2025-04-27T11:26Z"), &ROOT), Ok(()));
        assert_eq!(validate_date_time(&v, &Value::from("2028-11-20T11:27Z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_date_time(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::DateTime, op_err.clone()])));
        assert_eq!(validate_date_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime, op_err.clone()])));
    }

    #[test]
    fn validate_date_time_invalid_format() {
        let v = DateTimeValidation::default();
        assert_eq!(validate_date_time(&v, &Value::from("28-10-2026T10:27:29.973Z"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
    }

    #[test]
    fn validate_date_time_invalid_value() {
        let v = DateTimeValidation::default();
        assert_eq!(validate_date_time(&v, &Value::from("2029-17-73T82:93Z"), &ROOT), Err(SchemaErr::validation([ValidationErr::DateTime])));
    }
}

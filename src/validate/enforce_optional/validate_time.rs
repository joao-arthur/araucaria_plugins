use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    schema::TimeSchema,
    value::Value,
};

use crate::utils::time::parse_time;

pub fn validate_time(schema: &TimeSchema, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            if parse_time(str_value).is_ok() {
                if let Some(operation) = &schema.operation {
                    if let Some(Err(())) = compare(operation, &OperandValue::Str(str_value.clone()), root) {
                        base.push(ValidationErr::Operation(operation.clone()));
                    }
                }
            } else {
                base.push(ValidationErr::Time);
            }
        }
        Value::None => {
            if schema.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::Time);
            if let Some(operation) = &schema.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
        _ => {
            base.push(ValidationErr::Time);
            if let Some(operation) = &schema.operation {
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
        schema::TimeSchema,
        value::{Value, stub::u64_stub},
    };

    use super::validate_time;

    static ROOT: LazyLock<Value> = LazyLock::new(|| Value::Obj(BTreeMap::from([("time_value".into(), Value::from("11:27"))])));
    const REQUIRED: ValidationErr = ValidationErr::Required;
    const TIME: ValidationErr = ValidationErr::Time;

    #[test]
    fn validate_time_default() {
        let v = TimeSchema::default();
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("not a time"), &ROOT), Err(SchemaErr::from([TIME])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, TIME])));
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([TIME])));
    }

    #[test]
    fn validate_time_optional() {
        let v = TimeSchema::default().optional();
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("not a time"), &ROOT), Err(SchemaErr::from([TIME])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT), Err(SchemaErr::from([TIME])));
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([TIME])));
    }

    #[test]
    fn validate_time_operation_value() {
        let v = TimeSchema::default().eq("11:27".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("11:27"))));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("23:18"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, TIME, op_err.clone()])));
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([TIME, op_err.clone()])));
    }

    #[test]
    fn validate_time_operation_field() {
        let v = TimeSchema::default().ne_field("time_value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("time_value".into())));
        assert_eq!(validate_time(&v, &Value::from("02:18"), &ROOT), Ok(()));
        assert_eq!(validate_time(&v, &Value::from("11:27"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_time(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, TIME, op_err.clone()])));
        assert_eq!(validate_time(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([TIME, op_err.clone()])));
    }

    #[test]
    fn validate_time_invalid_format() {
        let v = TimeSchema::default();
        assert_eq!(validate_time(&v, &Value::from("10:27:23.235"), &ROOT), Err(SchemaErr::from([TIME])));
    }

    #[test]
    fn validate_time_invalid_value() {
        let v = TimeSchema::default();
        assert_eq!(validate_time(&v, &Value::from("72:93"), &ROOT), Err(SchemaErr::from([TIME])));
    }
}

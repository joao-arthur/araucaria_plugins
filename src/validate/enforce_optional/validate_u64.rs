use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    schema::U64Schema,
    value::Value,
};

pub fn validate_u64(validation: &U64Schema, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::U64(u64_value) => {
            if let Some(operation) = &validation.operation {
                if let Some(Err(())) = compare(operation, &OperandValue::U64(*u64_value), root) {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::U64);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
        _ => {
            base.push(ValidationErr::U64);
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
        schema::U64Schema,
        value::{Value, stub::bool_stub},
    };

    use super::validate_u64;

    static ROOT: LazyLock<Value> = LazyLock::new(|| Value::Obj(BTreeMap::from([("u64_value".into(), Value::U64(42))])));
    const REQUIRED: ValidationErr = ValidationErr::Required;
    const U64: ValidationErr = ValidationErr::U64;

    #[test]
    fn validate_u64_default() {
        let v = U64Schema::default();
        assert_eq!(validate_u64(&v, &Value::U64(42), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, U64])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([U64])));
    }

    #[test]
    fn validate_u64_optional() {
        let v = U64Schema::default().optional();
        assert_eq!(validate_u64(&v, &Value::U64(42), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::from([U64])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([U64])));
    }

    #[test]
    fn validate_u64_operation_value() {
        let v = U64Schema::default().eq(42);
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(42))));
        assert_eq!(validate_u64(&v, &Value::U64(42), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(418), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([U64, op_err.clone()])));
    }

    #[test]
    fn validate_u64_operation_field() {
        let v = U64Schema::default().ne_field("u64_value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("u64_value".into())));
        assert_eq!(validate_u64(&v, &Value::U64(418), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(42), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([U64, op_err.clone()])));
    }
}

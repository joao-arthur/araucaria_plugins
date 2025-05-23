use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    schema::I64Schema,
    value::Value,
};

pub fn validate_i64(schema: &I64Schema, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::I64(i64_value) => {
            if let Some(operation) = &schema.operation {
                if let Some(Err(())) = compare(operation, &OperandValue::I64(*i64_value), root) {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
            }
        }
        Value::None => {
            if schema.required {
                base.push(ValidationErr::Required);
                base.push(ValidationErr::I64);
                if let Some(operation) = &schema.operation {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
            }
        }
        _ => {
            base.push(ValidationErr::I64);
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
        schema::I64Schema,
        value::{Value, stub::bool_stub},
    };

    use super::validate_i64;

    static ROOT: LazyLock<Value> = LazyLock::new(|| Value::Obj(BTreeMap::from([("i64_value".into(), Value::I64(-42))])));
    const REQUIRED: ValidationErr = ValidationErr::Required;
    const I64: ValidationErr = ValidationErr::I64;

    #[test]
    fn validate_i64_default() {
        let v = I64Schema::default();
        assert_eq!(validate_i64(&v, &Value::I64(-42), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, I64])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([I64])));
    }

    #[test]
    fn validate_i64_optional() {
        let v = I64Schema::default().optional();
        assert_eq!(validate_i64(&v, &Value::I64(-42), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([I64])));
    }

    #[test]
    fn validate_i64_operation_value() {
        let v = I64Schema::default().eq(-42);
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::I64(-42))));
        assert_eq!(validate_i64(&v, &Value::I64(-42), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(-418), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([I64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_operation_field() {
        let v = I64Schema::default().ne_field("i64_value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("i64_value".into())));
        assert_eq!(validate_i64(&v, &Value::I64(-418), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(-42), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([I64, op_err.clone()])));
    }
}

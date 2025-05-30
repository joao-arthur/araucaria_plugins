use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    schema::BoolSchema,
    value::Value,
};

pub fn validate_bool(schema: &BoolSchema, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Bool(bool_value) => {
            if let Some(operation) = &schema.operation {
                if let Some(Err(())) = compare(operation, &OperandValue::Bool(*bool_value), root) {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
            }
        }
        Value::None => {
            if schema.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::Bool);
            if let Some(operation) = &schema.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
        _ => {
            base.push(ValidationErr::Bool);
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
        schema::BoolSchema,
        value::{Value, stub::u64_stub},
    };

    use super::validate_bool;

    static ROOT: LazyLock<Value> = LazyLock::new(|| Value::Obj(BTreeMap::from([("bool_value".into(), Value::Bool(false))])));
    const REQUIRED: ValidationErr = ValidationErr::Required;
    const BOOL: ValidationErr = ValidationErr::Bool;

    #[test]
    fn validate_bool_default() {
        let v = BoolSchema::default();
        assert_eq!(validate_bool(&v, &Value::Bool(false), &ROOT), Ok(()));
        assert_eq!(validate_bool(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, BOOL])));
        assert_eq!(validate_bool(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([BOOL])));
    }

    #[test]
    fn validate_bool_optional() {
        let v = BoolSchema::default().optional();
        assert_eq!(validate_bool(&v, &Value::Bool(true), &ROOT), Ok(()));
        assert_eq!(validate_bool(&v, &Value::None, &ROOT), Err(SchemaErr::from([BOOL])));
        assert_eq!(validate_bool(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([BOOL])));
    }

    #[test]
    fn validate_bool_operation_value() {
        let v = BoolSchema::default().eq(false);
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false))));
        assert_eq!(validate_bool(&v, &Value::Bool(false), &ROOT), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(true), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_bool(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, BOOL, op_err.clone()])));
        assert_eq!(validate_bool(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([BOOL, op_err.clone()])));
    }

    #[test]
    fn validate_bool_operation_field() {
        let v = BoolSchema::default().ne_field("bool_value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("bool_value".into())));
        assert_eq!(validate_bool(&v, &Value::Bool(true), &ROOT), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(false), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_bool(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, BOOL, op_err.clone()])));
        assert_eq!(validate_bool(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([BOOL, op_err.clone()])));
    }
}

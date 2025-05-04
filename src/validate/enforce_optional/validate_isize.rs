use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    schema::ISizeSchema,
    value::Value,
};

pub fn validate_isize(schema: &ISizeSchema, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::ISize(isize_value) => {
            if let Some(operation) = &schema.operation {
                if let Some(Err(())) = compare(operation, &OperandValue::ISize(*isize_value), root) {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
            }
        }
        Value::None => {
            if schema.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::ISize);
            if let Some(operation) = &schema.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
        _ => {
            base.push(ValidationErr::ISize);
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
        schema::ISizeSchema,
        value::{Value, stub::bool_stub},
    };

    use super::validate_isize;

    static ROOT: LazyLock<Value> = LazyLock::new(|| Value::Obj(BTreeMap::from([("isize_value".into(), Value::ISize(42))])));
    const REQUIRED: ValidationErr = ValidationErr::Required;
    const ISIZE: ValidationErr = ValidationErr::ISize;

    #[test]
    fn validate_isize_default() {
        let v = ISizeSchema::default();
        assert_eq!(validate_isize(&v, &Value::ISize(-42), &ROOT), Ok(()));
        assert_eq!(validate_isize(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, ISIZE])));
        assert_eq!(validate_isize(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([ISIZE])));
    }

    #[test]
    fn validate_isize_optional() {
        let v = ISizeSchema::default().optional();
        assert_eq!(validate_isize(&v, &Value::ISize(-42), &ROOT), Ok(()));
        assert_eq!(validate_isize(&v, &Value::None, &ROOT), Err(SchemaErr::from([ISIZE])));
        assert_eq!(validate_isize(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([ISIZE])));
    }

    #[test]
    fn validate_isize_operation_value() {
        let v = ISizeSchema::default().eq(-42);
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::ISize(-42))));
        assert_eq!(validate_isize(&v, &Value::ISize(-42), &ROOT), Ok(()));
        assert_eq!(validate_isize(&v, &Value::ISize(-418), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_isize(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, ISIZE, op_err.clone()])));
        assert_eq!(validate_isize(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([ISIZE, op_err.clone()])));
    }

    #[test]
    fn validate_isize_operation_field() {
        let v = ISizeSchema::default().ne_field("isize_value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("isize_value".into())));
        assert_eq!(validate_isize(&v, &Value::ISize(418), &ROOT), Ok(()));
        assert_eq!(validate_isize(&v, &Value::ISize(42), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_isize(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, ISIZE, op_err.clone()])));
        assert_eq!(validate_isize(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([ISIZE, op_err.clone()])));
    }
}

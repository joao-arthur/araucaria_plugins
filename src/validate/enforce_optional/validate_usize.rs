use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    schema::USizeSchema,
    value::Value,
};

pub fn validate_usize(validation: &USizeSchema, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::USize(usize_value) => {
            if let Some(operation) = &validation.operation {
                if let Some(Err(())) = compare(operation, &OperandValue::USize(*usize_value), root) {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::USize);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
        _ => {
            base.push(ValidationErr::USize);
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
        schema::USizeSchema,
        value::{Value, stub::bool_stub},
    };

    use super::validate_usize;

    static ROOT: LazyLock<Value> = LazyLock::new(|| Value::Obj(BTreeMap::from([("usize_value".into(), Value::USize(42))])));
    const REQUIRED: ValidationErr = ValidationErr::Required;
    const USIZE: ValidationErr = ValidationErr::USize;

    #[test]
    fn validate_usize_default() {
        let v = USizeSchema::default();
        assert_eq!(validate_usize(&v, &Value::USize(42), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, USIZE])));
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([USIZE])));
    }

    #[test]
    fn validate_usize_optional() {
        let v = USizeSchema::default().optional();
        assert_eq!(validate_usize(&v, &Value::USize(42), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::None, &ROOT), Err(SchemaErr::from([USIZE])));
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([USIZE])));
    }

    #[test]
    fn validate_usize_operation_value() {
        let v = USizeSchema::default().eq(42);
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::USize(42))));
        assert_eq!(validate_usize(&v, &Value::USize(42), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(418), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_usize(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, USIZE, op_err.clone()])));
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([USIZE, op_err.clone()])));
    }

    #[test]
    fn validate_usize_operation_field() {
        let v = USizeSchema::default().ne_field("usize_value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("usize_value".into())));
        assert_eq!(validate_usize(&v, &Value::USize(418), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(42), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_usize(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, USIZE, op_err.clone()])));
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::from([USIZE, op_err.clone()])));
    }
}

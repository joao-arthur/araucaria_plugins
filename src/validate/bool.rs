use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::bool::BoolValidation,
    value::Value,
};

pub fn validate_bool(validation: &BoolValidation, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Bool(bool_value) => {
            if let Some(operation) = &validation.operation {
                if let Some(Err(())) = compare(operation, &OperandValue::Bool(*bool_value), root) {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::Bool);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
        _ => {
            base.push(ValidationErr::Bool);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
    }
    if !base.is_empty() { Err(SchemaErr::Validation(base)) } else { Ok(()) }
}

#[cfg(test)]
mod test {
    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::bool::BoolValidation,
        value::{Value, stub::num_u_stub},
    };

    use super::validate_bool;

    #[test]
    fn test_validate_bool_default() {
        let v = BoolValidation::default();
        let root = Value::None;
        assert_eq!(validate_bool(&v, &Value::Bool(false), &root), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(true), &root), Ok(()));
        assert_eq!(validate_bool(&v, &Value::None, &root), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Bool])));
        assert_eq!(validate_bool(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Bool])));
    }

    #[test]
    fn test_validate_bool_optional() {
        let v = BoolValidation::default().optional();
        let root = Value::None;
        assert_eq!(validate_bool(&v, &Value::Bool(false), &root), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(true), &root), Ok(()));
        assert_eq!(validate_bool(&v, &Value::None, &root), Err(SchemaErr::validation([ValidationErr::Bool])));
        assert_eq!(validate_bool(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Bool])));
    }

    #[test]
    fn test_validate_bool_eq() {
        let v = BoolValidation::default().eq(false);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false))));
        assert_eq!(validate_bool(&v, &Value::Bool(false), &root), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(true), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_bool(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Bool, op_err.clone()]))
        );
        assert_eq!(validate_bool(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Bool, op_err.clone()])));
    }

    #[test]
    fn test_validate_bool_ne() {
        let v = BoolValidation::default().ne(false);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Bool(false))));
        assert_eq!(validate_bool(&v, &Value::Bool(true), &root), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(false), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_bool(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Bool, op_err.clone()]))
        );
        assert_eq!(validate_bool(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Bool, op_err.clone()])));
    }

    #[test]
    fn test_validate_bool_gt() {
        let v = BoolValidation::default().gt(false);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::Bool(false))));
        assert_eq!(validate_bool(&v, &Value::Bool(true), &root), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(false), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_bool(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Bool, op_err.clone()]))
        );
        assert_eq!(validate_bool(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Bool, op_err.clone()])));
    }

    #[test]
    fn test_validate_bool_ge() {
        let v = BoolValidation::default().ge(true);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::Bool(true))));
        assert_eq!(validate_bool(&v, &Value::Bool(true), &root), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(false), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_bool(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Bool, op_err.clone()]))
        );
        assert_eq!(validate_bool(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Bool, op_err.clone()])));
    }

    #[test]
    fn test_validate_bool_lt() {
        let v = BoolValidation::default().lt(true);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::Bool(true))));
        assert_eq!(validate_bool(&v, &Value::Bool(false), &root), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(true), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_bool(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Bool, op_err.clone()]))
        );
        assert_eq!(validate_bool(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Bool, op_err.clone()])));
    }

    #[test]
    fn test_validate_bool_le() {
        let v = BoolValidation::default().le(false);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::Bool(false))));
        assert_eq!(validate_bool(&v, &Value::Bool(false), &root), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(true), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_bool(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Bool, op_err.clone()]))
        );
        assert_eq!(validate_bool(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Bool, op_err.clone()])));
    }

    #[test]
    fn test_validate_bool_btwn() {
        let v = BoolValidation::default().btwn(false, true);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::Bool(false)), Operand::Value(OperandValue::Bool(true))));
        assert_eq!(validate_bool(&v, &Value::Bool(true), &root), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(false), &root), Ok(()));
        assert_eq!(
            validate_bool(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Bool, op_err.clone()]))
        );
        assert_eq!(validate_bool(&v, &num_u_stub(), &root), Err(SchemaErr::validation([ValidationErr::Bool, op_err.clone()])));
    }
}

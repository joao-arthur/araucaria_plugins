use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::num_i::NumIValidation,
    value::Value,
};

pub fn validate_num_i(validation: &NumIValidation, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::I64(i64_value) => {
            if let Some(operation) = &validation.operation {
                if let Some(Err(())) = compare(operation, &OperandValue::I64(*i64_value), root) {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::I64);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
        _ => {
            base.push(ValidationErr::I64);
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
        validation::num_i::NumIValidation,
        value::{Value, stub::bool_stub},
    };

    use super::validate_num_i;

    #[test]
    fn test_validate_num_i_default() {
        let v = NumIValidation::default();
        let root = Value::None;
        assert_eq!(validate_num_i(&v, &Value::I64(-42), &root), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::None, &root), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64])));
        assert_eq!(validate_num_i(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::I64])));
    }

    #[test]
    fn test_validate_num_i_optional() {
        let v = NumIValidation::default().optional();
        let root = Value::None;
        assert_eq!(validate_num_i(&v, &Value::I64(-42), &root), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::None, &root), Err(SchemaErr::validation([ValidationErr::I64])));
        assert_eq!(validate_num_i(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::I64])));
    }

    #[test]
    fn test_validate_num_i_eq_value() {
        let v = NumIValidation::default().eq(-42);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::I64(-42))));
        assert_eq!(validate_num_i(&v, &Value::I64(-42), &root), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(-7), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_i(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()]))
        );
        assert_eq!(validate_num_i(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_i_ne_value() {
        let v = NumIValidation::default().ne(-22);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::I64(-22))));
        assert_eq!(validate_num_i(&v, &Value::I64(-42), &root), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(-22), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_i(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()]))
        );
        assert_eq!(validate_num_i(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_i_gt_value() {
        let v = NumIValidation::default().gt(-2);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::I64(-2))));
        assert_eq!(validate_num_i(&v, &Value::I64(-1), &root), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(-2), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_i(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()]))
        );
        assert_eq!(validate_num_i(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_i_ge_value() {
        let v = NumIValidation::default().ge(-2);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::I64(-2))));
        assert_eq!(validate_num_i(&v, &Value::I64(-2), &root), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(-3), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_i(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()]))
        );
        assert_eq!(validate_num_i(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_i_lt_value() {
        let v = NumIValidation::default().lt(-5);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::I64(-5))));
        assert_eq!(validate_num_i(&v, &Value::I64(-6), &root), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(-5), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_i(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()]))
        );
        assert_eq!(validate_num_i(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_i_le_value() {
        let v = NumIValidation::default().le(-5);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::I64(-5))));
        assert_eq!(validate_num_i(&v, &Value::I64(-5), &root), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(-4), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_i(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()]))
        );
        assert_eq!(validate_num_i(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_u_btwn_value() {
        let v = NumIValidation::default().btwn(5, 6);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::I64(5)), Operand::Value(OperandValue::I64(6))));
        assert_eq!(validate_num_i(&v, &Value::I64(4), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_num_i(&v, &Value::I64(5), &root), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(6), &root), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(7), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_i(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()]))
        );
        assert_eq!(validate_num_i(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }
}

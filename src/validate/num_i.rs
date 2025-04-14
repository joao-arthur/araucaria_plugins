use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{compare, OperandValue},
    validation::num_i::NumIValidation,
    value::Value,
};

pub fn validate_num_i(validation: &NumIValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::I64(i64_value) => {
            if let Some(operation) = &validation.operation {
                if let Some(Err(())) = compare(operation, &OperandValue::I64(*i64_value)) {
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
    if !base.is_empty() {
        Err(SchemaErr::Validation(base))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        value::{stub::bool_stub, Value},
    };

    use super::{validate_num_i, NumIValidation};

    #[test]
    fn test_validate_num_i_default() {
        let v = NumIValidation::default();
        assert_eq!(validate_num_i(&v, &Value::I64(-42)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64])));
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::I64])));
    }

    #[test]
    fn test_validate_num_i_optional() {
        let v = NumIValidation::default().optional();
        assert_eq!(validate_num_i(&v, &Value::I64(-42)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::I64])));
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::I64])));
    }

    #[test]
    fn test_validate_num_i_eq() {
        let v = NumIValidation::default().eq(-42);
        assert_eq!(validate_num_i(&v, &Value::I64(-42)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(-7)), Err(SchemaErr::validation([ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::I64(-42))))])));
        assert_eq!(validate_num_i(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::I64(-42))))])));
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::I64, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::I64(-42))))])));
    }

    #[test]
    fn test_validate_num_i_ne() {
        let v = NumIValidation::default().ne(-22);
        assert_eq!(validate_num_i(&v, &Value::I64(-42)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(-22)), Err(SchemaErr::validation([ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::I64(-22))))])));
        assert_eq!(validate_num_i(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::I64(-22))))])));
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::I64, ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::I64(-22))))])));
    }

    #[test]
    fn test_validate_num_i_gt() {
        let v = NumIValidation::default().gt(-2);
        assert_eq!(validate_num_i(&v, &Value::I64(-1)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(-2)), Err(SchemaErr::validation([ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::I64(-2))))])));
        assert_eq!(validate_num_i(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::I64(-2))))])));
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::I64, ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::I64(-2))))])));
    }

    #[test]
    fn test_validate_num_i_lt() {
        let v = NumIValidation::default().lt(-5);
        assert_eq!(validate_num_i(&v, &Value::I64(-6)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(-5)), Err(SchemaErr::validation([ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::I64(-5))))])));
        assert_eq!(validate_num_i(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::I64(-5))))])));
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::I64, ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::I64(-5))))])));
    }

    #[test]
    fn test_validate_num_i_ge() {
        let v = NumIValidation::default().ge(-2);
        assert_eq!(validate_num_i(&v, &Value::I64(-2)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(-3)), Err(SchemaErr::validation([ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::I64(-2))))])));
        assert_eq!(validate_num_i(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::I64(-2))))])));
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::I64, ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::I64(-2))))])));
    }

    #[test]
    fn test_validate_num_i_le() {
        let v = NumIValidation::default().le(-5);
        assert_eq!(validate_num_i(&v, &Value::I64(-5)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(-4)), Err(SchemaErr::validation([ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::I64(-5))))])));
        assert_eq!(validate_num_i(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::I64(-5))))])));
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::I64, ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::I64(-5))))])));
    }

    #[test]
    fn test_validate_num_u_btwn() {
        let v = NumIValidation::default().btwn(5, 6);
        assert_eq!(validate_num_i(&v, &Value::I64(4)), Err(SchemaErr::validation([ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::I64(5)), Operand::Value(OperandValue::I64(6))))])));
        assert_eq!(validate_num_i(&v, &Value::I64(5)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(6)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::I64(7)), Err(SchemaErr::validation([ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::I64(5)), Operand::Value(OperandValue::I64(6))))])));
        assert_eq!(validate_num_i(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::I64(5)), Operand::Value(OperandValue::I64(6))))])));
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::I64, ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::I64(5)), Operand::Value(OperandValue::I64(6))))])));
    }
}

use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{compare, OperandValue},
    validation::num_f::NumFValidation,
    value::Value,
};

pub fn validate_num_f(validation: &NumFValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::F64(f64_value) => {
            if let Some(operation) = &validation.operation {
                if let Some(Err(())) = compare(operation, &OperandValue::F64(*f64_value)) {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::F64);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
        }
        _ => {
            base.push(ValidationErr::F64);
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
        validation::num_f::NumFValidation,
        value::{stub::bool_stub, Value},
    };

    use super::validate_num_f;

    #[test]
    fn test_validate_num_f_default() {
        let v = NumFValidation::default();
        assert_eq!(validate_num_f(&v, &Value::F64(-42.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64])));
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::F64])));
    }

    #[test]
    fn test_validate_num_f_optional() {
        let v = NumFValidation::default().optional();
        assert_eq!(validate_num_f(&v, &Value::F64(-42.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::F64])));
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::F64])));
    }

    #[test]
    fn test_validate_num_f_eq() {
        let v = NumFValidation::default().eq(-42.5);
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::F64(-42.5))));
        assert_eq!(validate_num_f(&v, &Value::F64(-42.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(-7.5)), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_num_f(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_ne() {
        let v = NumFValidation::default().ne(-22.5);
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::F64(-22.5))));
        assert_eq!(validate_num_f(&v, &Value::F64(-42.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(-22.5)), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_num_f(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_gt() {
        let v = NumFValidation::default().gt(-2.5);
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::F64(-2.5))));
        assert_eq!(validate_num_f(&v, &Value::F64(-1.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(-2.5)), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_num_f(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_lt() {
        let v = NumFValidation::default().lt(-5.5);
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::F64(-5.5))));
        assert_eq!(validate_num_f(&v, &Value::F64(-6.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(-5.5)), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_num_f(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_ge() {
        let v = NumFValidation::default().ge(-2.5);
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::F64(-2.5))));
        assert_eq!(validate_num_f(&v, &Value::F64(-2.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(-3.5)), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_num_f(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_le() {
        let v = NumFValidation::default().le(-5.5);
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::F64(-5.5))));
        assert_eq!(validate_num_f(&v, &Value::F64(-5.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(-4.5)), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_num_f(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_btwn() {
        let v = NumFValidation::default().btwn(5.0, 6.0);
        let op_err = ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::F64(5.0)), Operand::Value(OperandValue::F64(6.0))));
        assert_eq!(validate_num_f(&v, &Value::F64(4.0)), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_num_f(&v, &Value::F64(5.0)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(6.0)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(7.0)), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_num_f(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }
}

use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::num_f::NumFValidation,
    value::Value,
};

pub fn validate_num_f(validation: &NumFValidation, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::F64(f64_value) => {
            if let Some(operation) = &validation.operation {
                if let Some(Err(())) = compare(operation, &OperandValue::F64(*f64_value), root) {
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
    if !base.is_empty() { Err(SchemaErr::Validation(base)) } else { Ok(()) }
}

#[cfg(test)]
mod test {
    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::num_f::NumFValidation,
        value::{Value, stub::bool_stub},
    };

    use super::validate_num_f;

    #[test]
    fn test_validate_num_f_default() {
        let v = NumFValidation::default();
        let root = Value::None;
        assert_eq!(validate_num_f(&v, &Value::F64(-42.5), &root), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::None, &root), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64])));
        assert_eq!(validate_num_f(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::F64])));
    }

    #[test]
    fn test_validate_num_f_optional() {
        let v = NumFValidation::default().optional();
        let root = Value::None;
        assert_eq!(validate_num_f(&v, &Value::F64(-42.5), &root), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::None, &root), Err(SchemaErr::validation([ValidationErr::F64])));
        assert_eq!(validate_num_f(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::F64])));
    }

    #[test]
    fn test_validate_num_f_eq_value() {
        let v = NumFValidation::default().eq(-42.5);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::F64(-42.5))));
        assert_eq!(validate_num_f(&v, &Value::F64(-42.5), &root), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(-7.5), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_f(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()]))
        );
        assert_eq!(validate_num_f(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_ne_value() {
        let v = NumFValidation::default().ne(-22.5);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::F64(-22.5))));
        assert_eq!(validate_num_f(&v, &Value::F64(-42.5), &root), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(-22.5), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_f(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()]))
        );
        assert_eq!(validate_num_f(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_gt_value() {
        let v = NumFValidation::default().gt(-2.5);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::F64(-2.5))));
        assert_eq!(validate_num_f(&v, &Value::F64(-1.5), &root), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(-2.5), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_f(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()]))
        );
        assert_eq!(validate_num_f(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_ge_value() {
        let v = NumFValidation::default().ge(-2.5);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::F64(-2.5))));
        assert_eq!(validate_num_f(&v, &Value::F64(-2.5), &root), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(-3.5), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_f(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()]))
        );
        assert_eq!(validate_num_f(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_lt_value() {
        let v = NumFValidation::default().lt(-5.5);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::F64(-5.5))));
        assert_eq!(validate_num_f(&v, &Value::F64(-6.5), &root), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(-5.5), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_f(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()]))
        );
        assert_eq!(validate_num_f(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_le_value() {
        let v = NumFValidation::default().le(-5.5);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::F64(-5.5))));
        assert_eq!(validate_num_f(&v, &Value::F64(-5.5), &root), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(-4.5), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_f(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()]))
        );
        assert_eq!(validate_num_f(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_btwn_value() {
        let v = NumFValidation::default().btwn(5.0, 6.0);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::F64(5.0)), Operand::Value(OperandValue::F64(6.0))));
        assert_eq!(validate_num_f(&v, &Value::F64(4.0), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_num_f(&v, &Value::F64(5.0), &root), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(6.0), &root), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::F64(7.0), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_f(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()]))
        );
        assert_eq!(validate_num_f(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }
}

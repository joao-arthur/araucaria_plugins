use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::num_u::NumUValidation,
    value::Value,
};

pub fn validate_num_u(validation: &NumUValidation, value: &Value, root: &Value) -> Result<(), SchemaErr> {
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
mod test {
    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::num_u::NumUValidation,
        value::{Value, stub::bool_stub},
    };

    use super::validate_num_u;

    #[test]
    fn test_validate_num_u_default() {
        let v = NumUValidation::default();
        let root = Value::None;
        assert_eq!(validate_num_u(&v, &Value::U64(42), &root), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::None, &root), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64])));
        assert_eq!(validate_num_u(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::U64])));
    }

    #[test]
    fn test_validate_num_u_optional() {
        let v = NumUValidation::default().optional();
        let root = Value::None;
        assert_eq!(validate_num_u(&v, &Value::U64(42), &root), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::None, &root), Err(SchemaErr::validation([ValidationErr::U64])));
        assert_eq!(validate_num_u(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::U64])));
    }

    #[test]
    fn test_validate_num_u_eq() {
        let v = NumUValidation::default().eq(42);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(42))));
        assert_eq!(validate_num_u(&v, &Value::U64(42), &root), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::U64(0), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_u(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()]))
        );
        assert_eq!(validate_num_u(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_u_ne() {
        let v = NumUValidation::default().ne(22);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::U64(22))));
        assert_eq!(validate_num_u(&v, &Value::U64(42), &root), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::U64(22), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_u(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()]))
        );
        assert_eq!(validate_num_u(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_u_gt() {
        let v = NumUValidation::default().gt(1);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::U64(1))));
        assert_eq!(validate_num_u(&v, &Value::U64(2), &root), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::U64(1), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_u(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()]))
        );
        assert_eq!(validate_num_u(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_u_lt() {
        let v = NumUValidation::default().lt(5);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::U64(5))));
        assert_eq!(validate_num_u(&v, &Value::U64(4), &root), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::U64(5), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_u(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()]))
        );
        assert_eq!(validate_num_u(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_u_ge() {
        let v = NumUValidation::default().ge(1);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::U64(1))));
        assert_eq!(validate_num_u(&v, &Value::U64(1), &root), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::U64(0), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_u(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()]))
        );
        assert_eq!(validate_num_u(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_u_le() {
        let v = NumUValidation::default().le(5);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::U64(5))));
        assert_eq!(validate_num_u(&v, &Value::U64(5), &root), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::U64(6), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_u(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()]))
        );
        assert_eq!(validate_num_u(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_u_btwn() {
        let v = NumUValidation::default().btwn(5, 6);
        let root = Value::None;
        let op_err = ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::U64(5)), Operand::Value(OperandValue::U64(6))));
        assert_eq!(validate_num_u(&v, &Value::U64(4), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_num_u(&v, &Value::U64(5), &root), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::U64(6), &root), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::U64(7), &root), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_num_u(&v, &Value::None, &root),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()]))
        );
        assert_eq!(validate_num_u(&v, &bool_stub(), &root), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }
}

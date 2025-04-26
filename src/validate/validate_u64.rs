use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::U64Validation,
    value::Value,
};

pub fn validate_u64(validation: &U64Validation, value: &Value, root: &Value) -> Result<(), SchemaErr> {
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
    use std::{collections::BTreeMap, sync::LazyLock};

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::U64Validation,
        value::{Value, stub::bool_stub},
    };

    use super::validate_u64;

    static ROOT: LazyLock<Value> = LazyLock::new(|| {
        Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::U64(42))])),
            ]),
        )]))
    });

    #[test]
    fn test_validate_u64_default() {
        let v = U64Validation::default();
        assert_eq!(validate_u64(&v, &Value::U64(42), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64])));
    }

    #[test]
    fn test_validate_u64_optional() {
        let v = U64Validation::default().optional();
        assert_eq!(validate_u64(&v, &Value::U64(42), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::U64])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64])));
    }

    #[test]
    fn test_validate_u64_eq_value() {
        let v = U64Validation::default().eq(42);
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(42))));
        assert_eq!(validate_u64(&v, &Value::U64(42), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(0), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_u64_ne_value() {
        let v = U64Validation::default().ne(22);
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::U64(22))));
        assert_eq!(validate_u64(&v, &Value::U64(42), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(22), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_u64_gt_value() {
        let v = U64Validation::default().gt(1);
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::U64(1))));
        assert_eq!(validate_u64(&v, &Value::U64(2), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(1), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_u64_ge_value() {
        let v = U64Validation::default().ge(1);
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::U64(1))));
        assert_eq!(validate_u64(&v, &Value::U64(1), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(0), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_u64_lt_value() {
        let v = U64Validation::default().lt(5);
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::U64(5))));
        assert_eq!(validate_u64(&v, &Value::U64(4), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_u64_le_value() {
        let v = U64Validation::default().le(5);
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::U64(5))));
        assert_eq!(validate_u64(&v, &Value::U64(5), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(6), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_u64_btwn_value() {
        let v = U64Validation::default().btwn(5, 6);
        let op_err = ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::U64(5)), Operand::Value(OperandValue::U64(6))));
        assert_eq!(validate_u64(&v, &Value::U64(4), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::U64(5), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(6), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(7), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_u64_eq_field() {
        let v = U64Validation::default().eq_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_u64(&v, &Value::U64(41), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::U64(42), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(43), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_u64_ne_field() {
        let v = U64Validation::default().ne_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_u64(&v, &Value::U64(41), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(42), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::U64(43), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_u64_gt_field() {
        let v = U64Validation::default().gt_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_u64(&v, &Value::U64(41), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::U64(42), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::U64(43), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_u64_ge_field() {
        let v = U64Validation::default().ge_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_u64(&v, &Value::U64(41), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::U64(42), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(43), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_u64_lt_field() {
        let v = U64Validation::default().lt_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_u64(&v, &Value::U64(41), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(42), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::U64(43), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_u64_le_field() {
        let v = U64Validation::default().le_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Le(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_u64(&v, &Value::U64(41), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(42), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(43), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }

    #[test]
    fn test_validate_u64_btwn_field() {
        let v = U64Validation::default().btwn_field("values.2.value".into(), "values.3.value".into());
        let op_err =
            ValidationErr::Operation(Operation::Btwn(Operand::FieldPath("values.2.value".into()), Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_u64(&v, &Value::U64(31), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::U64(32), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(33), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(41), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(42), &ROOT), Ok(()));
        assert_eq!(validate_u64(&v, &Value::U64(43), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_u64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::U64, op_err.clone()])));
        assert_eq!(validate_u64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::U64, op_err.clone()])));
    }
}

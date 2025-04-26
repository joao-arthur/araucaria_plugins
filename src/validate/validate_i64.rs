use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::I64Validation,
    value::Value,
};

pub fn validate_i64(validation: &I64Validation, value: &Value, root: &Value) -> Result<(), SchemaErr> {
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
mod tests {
    use std::{collections::BTreeMap, sync::LazyLock};

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::I64Validation,
        value::{Value, stub::bool_stub},
    };

    use super::validate_i64;

    static ROOT: LazyLock<Value> = LazyLock::new(|| {
        Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::I64(42))])),
            ]),
        )]))
    });

    #[test]
    fn validate_i64_default() {
        let v = I64Validation::default();
        assert_eq!(validate_i64(&v, &Value::I64(-42), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64])));
    }

    #[test]
    fn validate_i64_optional() {
        let v = I64Validation::default().optional();
        assert_eq!(validate_i64(&v, &Value::I64(-42), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::I64])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64])));
    }

    #[test]
    fn validate_i64_eq_value() {
        let v = I64Validation::default().eq(-42);
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::I64(-42))));
        assert_eq!(validate_i64(&v, &Value::I64(-42), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(-7), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_ne_value() {
        let v = I64Validation::default().ne(-22);
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::I64(-22))));
        assert_eq!(validate_i64(&v, &Value::I64(-42), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(-22), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_gt_value() {
        let v = I64Validation::default().gt(-2);
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::I64(-2))));
        assert_eq!(validate_i64(&v, &Value::I64(-1), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(-2), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_ge_value() {
        let v = I64Validation::default().ge(-2);
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::I64(-2))));
        assert_eq!(validate_i64(&v, &Value::I64(-2), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(-3), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_lt_value() {
        let v = I64Validation::default().lt(-5);
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::I64(-5))));
        assert_eq!(validate_i64(&v, &Value::I64(-6), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(-5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_le_value() {
        let v = I64Validation::default().le(-5);
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::I64(-5))));
        assert_eq!(validate_i64(&v, &Value::I64(-5), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(-4), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_btwn_value() {
        let v = I64Validation::default().btwn(5, 6);
        let op_err = ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::I64(5)), Operand::Value(OperandValue::I64(6))));
        assert_eq!(validate_i64(&v, &Value::I64(4), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::I64(5), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(6), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(7), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_eq_field() {
        let v = I64Validation::default().eq_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_i64(&v, &Value::I64(41), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::I64(42), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(43), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_ne_field() {
        let v = I64Validation::default().ne_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_i64(&v, &Value::I64(41), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(42), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::I64(43), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_gt_field() {
        let v = I64Validation::default().gt_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_i64(&v, &Value::I64(41), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::I64(42), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::I64(43), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_ge_field() {
        let v = I64Validation::default().ge_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_i64(&v, &Value::I64(41), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::I64(42), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(43), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_lt_field() {
        let v = I64Validation::default().lt_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_i64(&v, &Value::I64(41), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(42), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::I64(43), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_le_field() {
        let v = I64Validation::default().le_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Le(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_i64(&v, &Value::I64(41), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(42), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(43), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_btwn_field() {
        let v = I64Validation::default().btwn_field("values.2.value".into(), "values.3.value".into());
        let op_err =
            ValidationErr::Operation(Operation::Btwn(Operand::FieldPath("values.2.value".into()), Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_i64(&v, &Value::I64(31), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::I64(32), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(33), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(41), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(42), &ROOT), Ok(()));
        assert_eq!(validate_i64(&v, &Value::I64(43), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_i64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::I64, op_err.clone()])));
        assert_eq!(validate_i64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::I64, op_err.clone()])));
    }
}

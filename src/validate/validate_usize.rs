use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::USizeValidation,
    value::Value,
};

pub fn validate_usize(validation: &USizeValidation, value: &Value, root: &Value) -> Result<(), SchemaErr> {
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
mod test {
    use std::{collections::BTreeMap, sync::LazyLock};

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::USizeValidation,
        value::{Value, stub::bool_stub},
    };

    use super::validate_usize;

    static ROOT: LazyLock<Value> = LazyLock::new(|| {
        Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::USize(12))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::USize(22))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::USize(32))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::USize(42))])),
            ]),
        )]))
    });

    #[test]
    fn test_validate_usize_default() {
        let v = USizeValidation::default();
        assert_eq!(validate_usize(&v, &Value::USize(42), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::USize])));
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize])));
    }

    #[test]
    fn test_validate_usize_optional() {
        let v = USizeValidation::default().optional();
        assert_eq!(validate_usize(&v, &Value::USize(42), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::USize])));
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize])));
    }

    #[test]
    fn test_validate_usize_eq_value() {
        let v = USizeValidation::default().eq(42);
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::USize(42))));
        assert_eq!(validate_usize(&v, &Value::USize(42), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(0), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_usize(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::USize, op_err.clone()]))
        );
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize, op_err.clone()])));
    }

    #[test]
    fn test_validate_usize_ne_value() {
        let v = USizeValidation::default().ne(22);
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::USize(22))));
        assert_eq!(validate_usize(&v, &Value::USize(42), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(22), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_usize(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::USize, op_err.clone()]))
        );
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize, op_err.clone()])));
    }

    #[test]
    fn test_validate_usize_gt_value() {
        let v = USizeValidation::default().gt(1);
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::USize(1))));
        assert_eq!(validate_usize(&v, &Value::USize(2), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(1), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_usize(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::USize, op_err.clone()]))
        );
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize, op_err.clone()])));
    }

    #[test]
    fn test_validate_usize_ge_value() {
        let v = USizeValidation::default().ge(1);
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::USize(1))));
        assert_eq!(validate_usize(&v, &Value::USize(1), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(0), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_usize(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::USize, op_err.clone()]))
        );
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize, op_err.clone()])));
    }

    #[test]
    fn test_validate_usize_lt_value() {
        let v = USizeValidation::default().lt(5);
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::USize(5))));
        assert_eq!(validate_usize(&v, &Value::USize(4), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_usize(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::USize, op_err.clone()]))
        );
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize, op_err.clone()])));
    }

    #[test]
    fn test_validate_usize_le_value() {
        let v = USizeValidation::default().le(5);
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::USize(5))));
        assert_eq!(validate_usize(&v, &Value::USize(5), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(6), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_usize(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::USize, op_err.clone()]))
        );
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize, op_err.clone()])));
    }

    #[test]
    fn test_validate_usize_btwn_value() {
        let v = USizeValidation::default().btwn(5, 6);
        let op_err = ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::USize(5)), Operand::Value(OperandValue::USize(6))));
        assert_eq!(validate_usize(&v, &Value::USize(4), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_usize(&v, &Value::USize(5), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(6), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(7), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_usize(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::USize, op_err.clone()]))
        );
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize, op_err.clone()])));
    }

    #[test]
    fn test_validate_usize_eq_field() {
        let v = USizeValidation::default().eq_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_usize(&v, &Value::USize(41), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_usize(&v, &Value::USize(42), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(43), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_usize(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::USize, op_err.clone()]))
        );
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize, op_err.clone()])));
    }

    #[test]
    fn test_validate_usize_ne_field() {
        let v = USizeValidation::default().ne_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_usize(&v, &Value::USize(41), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(42), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_usize(&v, &Value::USize(43), &ROOT), Ok(()));
        assert_eq!(
            validate_usize(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::USize, op_err.clone()]))
        );
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize, op_err.clone()])));
    }

    #[test]
    fn test_validate_usize_gt_field() {
        let v = USizeValidation::default().gt_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_usize(&v, &Value::USize(41), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_usize(&v, &Value::USize(42), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_usize(&v, &Value::USize(43), &ROOT), Ok(()));
        assert_eq!(
            validate_usize(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::USize, op_err.clone()]))
        );
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize, op_err.clone()])));
    }

    #[test]
    fn test_validate_usize_ge_field() {
        let v = USizeValidation::default().ge_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_usize(&v, &Value::USize(41), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_usize(&v, &Value::USize(42), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(43), &ROOT), Ok(()));
        assert_eq!(
            validate_usize(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::USize, op_err.clone()]))
        );
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize, op_err.clone()])));
    }

    #[test]
    fn test_validate_usize_lt_field() {
        let v = USizeValidation::default().lt_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_usize(&v, &Value::USize(41), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(42), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_usize(&v, &Value::USize(43), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_usize(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::USize, op_err.clone()]))
        );
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize, op_err.clone()])));
    }

    #[test]
    fn test_validate_usize_le_field() {
        let v = USizeValidation::default().le_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Le(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_usize(&v, &Value::USize(41), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(42), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(43), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_usize(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::USize, op_err.clone()]))
        );
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize, op_err.clone()])));
    }

    #[test]
    fn test_validate_usize_btwn_field() {
        let v = USizeValidation::default().btwn_field("values.2.value".into(), "values.3.value".into());
        let op_err =
            ValidationErr::Operation(Operation::Btwn(Operand::FieldPath("values.2.value".into()), Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_usize(&v, &Value::USize(31), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_usize(&v, &Value::USize(32), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(33), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(41), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(42), &ROOT), Ok(()));
        assert_eq!(validate_usize(&v, &Value::USize(43), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(
            validate_usize(&v, &Value::None, &ROOT),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::USize, op_err.clone()]))
        );
        assert_eq!(validate_usize(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::USize, op_err.clone()])));
    }
}

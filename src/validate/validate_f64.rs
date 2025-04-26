use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::F64Validation,
    value::Value,
};

pub fn validate_f64(validation: &F64Validation, value: &Value, root: &Value) -> Result<(), SchemaErr> {
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
    use std::{collections::BTreeMap, sync::LazyLock};

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::F64Validation,
        value::{Value, stub::bool_stub},
    };

    use super::validate_f64;

    static ROOT: LazyLock<Value> = LazyLock::new(|| {
        Value::Obj(BTreeMap::from([(
            "values".into(),
            Value::Arr(vec![
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(12.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(22.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(32.5))])),
                Value::Obj(BTreeMap::from([("value".into(), Value::F64(42.5))])),
            ]),
        )]))
    });

    #[test]
    fn test_validate_num_f_default() {
        let v = F64Validation::default();
        assert_eq!(validate_f64(&v, &Value::F64(-42.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64])));
    }

    #[test]
    fn test_validate_num_f_optional() {
        let v = F64Validation::default().optional();
        assert_eq!(validate_f64(&v, &Value::F64(-42.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::F64])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64])));
    }

    #[test]
    fn test_validate_num_f_eq_value() {
        let v = F64Validation::default().eq(-42.5);
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::F64(-42.5))));
        assert_eq!(validate_f64(&v, &Value::F64(-42.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(-7.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_ne_value() {
        let v = F64Validation::default().ne(-22.5);
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::F64(-22.5))));
        assert_eq!(validate_f64(&v, &Value::F64(-42.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(-22.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_gt_value() {
        let v = F64Validation::default().gt(-2.5);
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::F64(-2.5))));
        assert_eq!(validate_f64(&v, &Value::F64(-1.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(-2.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_ge_value() {
        let v = F64Validation::default().ge(-2.5);
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::F64(-2.5))));
        assert_eq!(validate_f64(&v, &Value::F64(-2.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(-3.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_lt_value() {
        let v = F64Validation::default().lt(-5.5);
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::F64(-5.5))));
        assert_eq!(validate_f64(&v, &Value::F64(-6.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(-5.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_le_value() {
        let v = F64Validation::default().le(-5.5);
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::F64(-5.5))));
        assert_eq!(validate_f64(&v, &Value::F64(-5.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(-4.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_f_btwn_value() {
        let v = F64Validation::default().btwn(5.0, 6.0);
        let op_err = ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::F64(5.0)), Operand::Value(OperandValue::F64(6.0))));
        assert_eq!(validate_f64(&v, &Value::F64(4.0), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::F64(5.0), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(6.0), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(7.0), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_i_eq_field() {
        let v = F64Validation::default().eq_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_f64(&v, &Value::F64(41.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::F64(42.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(43.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_i_ne_field() {
        let v = F64Validation::default().ne_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_f64(&v, &Value::F64(41.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(42.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::F64(43.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_i_gt_field() {
        let v = F64Validation::default().gt_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_f64(&v, &Value::F64(41.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::F64(42.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::F64(43.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_i_ge_field() {
        let v = F64Validation::default().ge_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_f64(&v, &Value::F64(41.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::F64(42.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(43.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_i_lt_field() {
        let v = F64Validation::default().lt_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_f64(&v, &Value::F64(41.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(42.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::F64(43.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_i_le_field() {
        let v = F64Validation::default().le_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Le(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_f64(&v, &Value::F64(41.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(42.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(43.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn test_validate_num_i_btwn_field() {
        let v = F64Validation::default().btwn_field("values.2.value".into(), "values.3.value".into());
        let op_err =
            ValidationErr::Operation(Operation::Btwn(Operand::FieldPath("values.2.value".into()), Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_f64(&v, &Value::F64(31.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::F64(32.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(33.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(41.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(42.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(43.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }
}

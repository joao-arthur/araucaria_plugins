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
mod tests {
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
    fn validate_f64_default() {
        let v = F64Validation::default();
        assert_eq!(validate_f64(&v, &Value::F64(-42.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64])));
    }

    #[test]
    fn validate_f64_optional() {
        let v = F64Validation::default().optional();
        assert_eq!(validate_f64(&v, &Value::F64(-42.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::F64])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64])));
    }

    #[test]
    fn validate_f64_value() {
        let v = F64Validation::default().eq(-42.5);
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::F64(-42.5))));
        assert_eq!(validate_f64(&v, &Value::F64(-42.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(-7.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_field() {
        let v = F64Validation::default().ne_field("values.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("values.3.value".into())));
        assert_eq!(validate_f64(&v, &Value::F64(41.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(42.5), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::F64(43.5), &ROOT), Ok(()));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::F64, op_err.clone()])));
    }
}

use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::F64Validation,
    value::Value,
};

pub fn validate_f64(validation: &F64Validation, value: &Value, root: &Value, enforce_optional: bool) -> Result<(), SchemaErr> {
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
            if enforce_optional {
                if validation.required {
                    base.push(ValidationErr::Required);
                }
                base.push(ValidationErr::F64);
                if let Some(operation) = &validation.operation {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
            } else {
                if validation.required {
                    base.push(ValidationErr::Required);
                    base.push(ValidationErr::F64);
                    if let Some(operation) = &validation.operation {
                        base.push(ValidationErr::Operation(operation.clone()));
                    }
                }
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

    static ROOT: LazyLock<Value> = LazyLock::new(|| Value::Obj(BTreeMap::from([("f64_value".into(), Value::F64(-42.5))])));
    const REQUIRED: ValidationErr = ValidationErr::Required;
    const F64: ValidationErr = ValidationErr::F64;

    #[test]
    fn validate_f64_default() {
        let v = F64Validation::default();
        assert_eq!(validate_f64(&v, &Value::F64(-42.5), &ROOT, false), Ok(()));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT, true), Err(SchemaErr::validation([REQUIRED, F64])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT, false), Err(SchemaErr::validation([REQUIRED, F64])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT, false), Err(SchemaErr::validation([F64])));
    }

    #[test]
    fn validate_f64_optional() {
        let v = F64Validation::default().optional();
        assert_eq!(validate_f64(&v, &Value::F64(-42.5), &ROOT, false), Ok(()));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT, true), Err(SchemaErr::validation([F64])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT, false), Ok(()));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT, false), Err(SchemaErr::validation([F64])));
    }

    #[test]
    fn validate_f64_operation_value() {
        let v = F64Validation::default().eq(-42.5);
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::F64(-42.5))));
        assert_eq!(validate_f64(&v, &Value::F64(-42.5), &ROOT, false), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(-418.0), &ROOT, false), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT, false), Err(SchemaErr::validation([REQUIRED, F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT, false), Err(SchemaErr::validation([F64, op_err.clone()])));
    }

    #[test]
    fn validate_i64_operation_field() {
        let v = F64Validation::default().ne_field("f64_value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("f64_value".into())));
        assert_eq!(validate_f64(&v, &Value::F64(-418.0), &ROOT, false), Ok(()));
        assert_eq!(validate_f64(&v, &Value::F64(-42.5), &ROOT, false), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_f64(&v, &Value::None, &ROOT, false), Err(SchemaErr::validation([REQUIRED, F64, op_err.clone()])));
        assert_eq!(validate_f64(&v, &bool_stub(), &ROOT, false), Err(SchemaErr::validation([F64, op_err.clone()])));
    }
}

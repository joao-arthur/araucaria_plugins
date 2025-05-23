use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    schema::DateSchema,
    value::Value,
};

use crate::utils::date::parse_date;

pub fn validate_date(schema: &DateSchema, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            if parse_date(str_value).is_ok() {
                if let Some(operation) = &schema.operation {
                    if let Some(Err(())) = compare(operation, &OperandValue::Str(str_value.clone()), root) {
                        base.push(ValidationErr::Operation(operation.clone()));
                    }
                }
            } else {
                base.push(ValidationErr::Date);
            }
        }
        Value::None => {
            if schema.required {
                base.push(ValidationErr::Required);
                base.push(ValidationErr::Date);
                if let Some(operation) = &schema.operation {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
            }
        }
        _ => {
            base.push(ValidationErr::Date);
            if let Some(operation) = &schema.operation {
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
        schema::DateSchema,
        value::{Value, stub::u64_stub},
    };

    use super::validate_date;

    static ROOT: LazyLock<Value> = LazyLock::new(|| Value::Obj(BTreeMap::from([("date_value".into(), Value::from("2026-10-28"))])));
    const REQUIRED: ValidationErr = ValidationErr::Required;
    const DATE: ValidationErr = ValidationErr::Date;

    #[test]
    fn validate_date_default() {
        let v = DateSchema::default();
        assert_eq!(validate_date(&v, &Value::from("2026-10-28"), &ROOT), Ok(()));
        assert_eq!(validate_date(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, DATE])));
        assert_eq!(validate_date(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([DATE])));
    }

    #[test]
    fn validate_date_optional() {
        let v = DateSchema::default().optional();
        assert_eq!(validate_date(&v, &Value::from("2026-10-28"), &ROOT), Ok(()));
        assert_eq!(validate_date(&v, &Value::None, &ROOT), Ok(()));
        assert_eq!(validate_date(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([DATE])));
    }

    #[test]
    fn validate_date_operation_value() {
        let v = DateSchema::default().eq("2026-10-28".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("2026-10-28"))));
        assert_eq!(validate_date(&v, &Value::from("2026-10-28"), &ROOT), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2025-04-18"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_date(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, DATE, op_err.clone()])));
        assert_eq!(validate_date(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([DATE, op_err.clone()])));
    }

    #[test]
    fn validate_date_operation_field() {
        let v = DateSchema::default().ne_field("date_value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("date_value".into())));
        assert_eq!(validate_date(&v, &Value::from("2028-11-19"), &ROOT), Ok(()));
        assert_eq!(validate_date(&v, &Value::from("2026-10-28"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_date(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, DATE, op_err.clone()])));
        assert_eq!(validate_date(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([DATE, op_err.clone()])));
    }

    #[test]
    fn validate_date_invalid_format() {
        let v = DateSchema::default();
        assert_eq!(validate_date(&v, &Value::from("10-10-2026"), &ROOT), Err(SchemaErr::from([DATE])));
    }

    #[test]
    fn validate_date_invalid_value() {
        let v = DateSchema::default();
        assert_eq!(validate_date(&v, &Value::from("2029-12-00"), &ROOT), Err(SchemaErr::from([DATE])));
    }
}

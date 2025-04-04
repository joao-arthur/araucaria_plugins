use araucaria::{
    error::{SchemaErr, ValidationErr},
    validation::bool::BoolValidation,
    value::Value,
};

pub fn validate_bool(validation: &BoolValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Bool(bool_value) => {
            if let Some(v) = validation.eq {
                if bool_value != &v {
                    base.push(ValidationErr::Eq(Value::Bool(v)));
                }
            }
            if let Some(v) = validation.ne {
                if bool_value == &v {
                    base.push(ValidationErr::Ne(Value::Bool(v)));
                }
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::Bool);
            if let Some(v) = validation.eq {
                base.push(ValidationErr::Eq(Value::Bool(v)));
            }
            if let Some(v) = validation.ne {
                base.push(ValidationErr::Ne(Value::Bool(v)));
            }
        }
        _ => {
            base.push(ValidationErr::Bool);
            if let Some(v) = validation.eq {
                base.push(ValidationErr::Eq(Value::Bool(v)));
            }
            if let Some(v) = validation.ne {
                base.push(ValidationErr::Ne(Value::Bool(v)));
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
    use araucaria::value::stub::num_u_stub;

    use super::*;

    #[test]
    fn test_validate_bool_default() {
        let v = BoolValidation::default();
        assert_eq!(validate_bool(&v, &Value::Bool(false)), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(true)), Ok(()));
        assert_eq!(validate_bool(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Bool])));
        assert_eq!(validate_bool(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Bool])));
    }

    #[test]
    fn test_validate_bool_optional() {
        let v = BoolValidation::default().optional();
        assert_eq!(validate_bool(&v, &Value::Bool(false)), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(true)), Ok(()));
        assert_eq!(validate_bool(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Bool])));
        assert_eq!(validate_bool(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Bool])));
    }

    #[test]
    fn test_validate_bool_eq() {
        let v = BoolValidation::default().eq(false);
        assert_eq!(validate_bool(&v, &Value::Bool(false)), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(true)), Err(SchemaErr::validation([ValidationErr::Eq(Value::Bool(false))])));
        assert_eq!(
            validate_bool(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Bool, ValidationErr::Eq(Value::Bool(false))]))
        );
        assert_eq!(validate_bool(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Bool, ValidationErr::Eq(Value::Bool(false))])));
    }

    #[test]
    fn test_validate_bool_ne() {
        let v = BoolValidation::default().ne(false);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(false)), Err(SchemaErr::validation([ValidationErr::Ne(Value::Bool(false))])));
        assert_eq!(
            validate_bool(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Bool, ValidationErr::Ne(Value::Bool(false))]))
        );
        assert_eq!(validate_bool(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Bool, ValidationErr::Ne(Value::Bool(false))])));
    }
}

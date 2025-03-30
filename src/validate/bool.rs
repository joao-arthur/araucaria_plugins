use araucaria::{
    error::{SchemaErr, ValidationErr},
    validation::bool::BoolValidation,
    value::Value,
};

pub fn validate_bool(validation: &BoolValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Bool(bool_value) => {
            if let Some(eq_v) = validation.eq {
                if bool_value != &eq_v {
                    base.push(ValidationErr::Eq(Value::Bool(eq_v)));
                }
            }
            if let Some(ne_v) = validation.ne {
                if bool_value == &ne_v {
                    base.push(ValidationErr::Ne(Value::Bool(ne_v)));
                }
            }
        }
        Value::None => {
            base.push(ValidationErr::Bool);
            if validation.required {
                base.push(ValidationErr::Required);
            }
            if let Some(eq_v) = validation.eq {
                base.push(ValidationErr::Eq(Value::Bool(eq_v)));
            }
            if let Some(ne_v) = validation.ne {
                base.push(ValidationErr::Ne(Value::Bool(ne_v)));
            }
        }
        _ => {
            base.push(ValidationErr::Bool);
            if let Some(eq_v) = validation.eq {
                base.push(ValidationErr::Eq(Value::Bool(eq_v)));
            }
            if let Some(ne_v) = validation.ne {
                base.push(ValidationErr::Ne(Value::Bool(ne_v)));
            }
        }
    }
    if !base.is_empty() {
        Err(SchemaErr::Arr(base))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use araucaria::value::stub::{
        arr_bool_stub, num_f_stub, num_i_stub, num_u_stub, obj_stub, str_stub,
    };

    use super::*;

    #[test]
    fn test_validate_bool_default() {
        let v = BoolValidation::default();
        assert_eq!(validate_bool(&v, &Value::Bool(false)), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(true)), Ok(()));
        assert_eq!(
            validate_bool(&v, &Value::None),
            Err(SchemaErr::arr([ValidationErr::Bool, ValidationErr::Required]))
        );
        assert_eq!(validate_bool(&v, &num_u_stub()), Err(SchemaErr::arr([ValidationErr::Bool])));
    }

    #[test]
    fn test_validate_bool_optional() {
        let v = BoolValidation::default().optional();
        assert_eq!(validate_bool(&v, &Value::Bool(false)), Ok(()));
        assert_eq!(validate_bool(&v, &Value::Bool(true)), Ok(()));
        assert_eq!(validate_bool(&v, &Value::None), Err(SchemaErr::arr([ValidationErr::Bool])));
        assert_eq!(validate_bool(&v, &num_u_stub()), Err(SchemaErr::arr([ValidationErr::Bool])));
    }

    #[test]
    fn test_validate_bool_eq() {
        let v = BoolValidation::default().eq(false);
        assert_eq!(validate_bool(&v, &Value::Bool(false)), Ok(()));
        assert_eq!(
            validate_bool(&v, &Value::Bool(true)),
            Err(SchemaErr::arr([ValidationErr::Eq(Value::Bool(false))]))
        );
        assert_eq!(
            validate_bool(&v, &Value::None),
            Err(SchemaErr::arr([
                ValidationErr::Bool,
                ValidationErr::Required,
                ValidationErr::Eq(Value::Bool(false),)
            ]))
        );
        assert_eq!(
            validate_bool(&v, &num_u_stub()),
            Err(SchemaErr::arr([ValidationErr::Bool, ValidationErr::Eq(Value::Bool(false))]))
        );
    }

    #[test]
    fn test_validate_bool_ne() {
        let v = BoolValidation::default().ne(false);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), Ok(()));
        assert_eq!(
            validate_bool(&v, &Value::Bool(false)),
            Err(SchemaErr::arr([ValidationErr::Ne(Value::Bool(false))]))
        );
        assert_eq!(
            validate_bool(&v, &Value::None),
            Err(SchemaErr::arr([
                ValidationErr::Bool,
                ValidationErr::Required,
                ValidationErr::Ne(Value::Bool(false))
            ]))
        );
        assert_eq!(
            validate_bool(&v, &num_u_stub()),
            Err(SchemaErr::arr([ValidationErr::Bool, ValidationErr::Ne(Value::Bool(false))]))
        );
    }
}

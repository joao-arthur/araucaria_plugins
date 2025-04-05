use araucaria::{
    error::{SchemaErr, ValidationErr},
    validation::num_i::NumIValidation,
    value::Value,
};

pub fn validate_num_i(validation: &NumIValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::NumI(num_i_value) => {
            if let Some(v) = validation.eq {
                if num_i_value != &v {
                    base.push(ValidationErr::Eq(Value::NumI(v)));
                }
            }
            if let Some(v) = validation.ne {
                if num_i_value == &v {
                    base.push(ValidationErr::Ne(Value::NumI(v)));
                }
            }
            if let Some(v) = validation.gt {
                if num_i_value <= &v {
                    base.push(ValidationErr::Gt(Value::NumI(v)));
                }
            }
            if let Some(v) = validation.lt {
                if num_i_value >= &v {
                    base.push(ValidationErr::Lt(Value::NumI(v)));
                }
            }
            if let Some(v) = validation.ge {
                if num_i_value < &v {
                    base.push(ValidationErr::Ge(Value::NumI(v)));
                }
            }
            if let Some(v) = validation.le {
                if num_i_value > &v {
                    base.push(ValidationErr::Le(Value::NumI(v)));
                }
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::NumI);
            if let Some(v) = validation.eq {
                base.push(ValidationErr::Eq(Value::NumI(v)));
            }
            if let Some(v) = validation.ne {
                base.push(ValidationErr::Ne(Value::NumI(v)));
            }
            if let Some(v) = validation.gt {
                base.push(ValidationErr::Gt(Value::NumI(v)));
            }
            if let Some(v) = validation.lt {
                base.push(ValidationErr::Lt(Value::NumI(v)));
            }
            if let Some(v) = validation.ge {
                base.push(ValidationErr::Ge(Value::NumI(v)));
            }
            if let Some(v) = validation.le {
                base.push(ValidationErr::Le(Value::NumI(v)));
            }
        }
        _ => {
            base.push(ValidationErr::NumI);
            if let Some(v) = validation.eq {
                base.push(ValidationErr::Eq(Value::NumI(v)));
            }
            if let Some(v) = validation.ne {
                base.push(ValidationErr::Ne(Value::NumI(v)));
            }
            if let Some(v) = validation.gt {
                base.push(ValidationErr::Gt(Value::NumI(v)));
            }
            if let Some(v) = validation.lt {
                base.push(ValidationErr::Lt(Value::NumI(v)));
            }
            if let Some(v) = validation.ge {
                base.push(ValidationErr::Ge(Value::NumI(v)));
            }
            if let Some(v) = validation.le {
                base.push(ValidationErr::Le(Value::NumI(v)));
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
    use araucaria::value::stub::bool_stub;

    use super::*;

    #[test]
    fn test_validate_num_i_default() {
        let v = NumIValidation::default();
        assert_eq!(validate_num_i(&v, &Value::NumI(-42)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumI])));
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumI])));
    }

    #[test]
    fn test_validate_num_i_optional() {
        let v = NumIValidation::default().optional();
        assert_eq!(validate_num_i(&v, &Value::NumI(-42)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::NumI])));
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumI])));
    }

    #[test]
    fn test_validate_num_i_eq() {
        let v = NumIValidation::default().eq(-42);
        assert_eq!(validate_num_i(&v, &Value::NumI(-42)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::NumI(-7)), Err(SchemaErr::validation([ValidationErr::Eq(Value::NumI(-42))])));
        assert_eq!(
            validate_num_i(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumI, ValidationErr::Eq(Value::NumI(-42))]))
        );
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumI, ValidationErr::Eq(Value::NumI(-42))])));
    }

    #[test]
    fn test_validate_num_i_ne() {
        let v = NumIValidation::default().ne(-22);
        assert_eq!(validate_num_i(&v, &Value::NumI(-42)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::NumI(-22)), Err(SchemaErr::validation([ValidationErr::Ne(Value::NumI(-22))])));
        assert_eq!(
            validate_num_i(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumI, ValidationErr::Ne(Value::NumI(-22))]))
        );
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumI, ValidationErr::Ne(Value::NumI(-22))])));
    }

    #[test]
    fn test_validate_num_i_gt() {
        let v = NumIValidation::default().gt(-2);
        assert_eq!(validate_num_i(&v, &Value::NumI(-1)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::NumI(-2)), Err(SchemaErr::validation([ValidationErr::Gt(Value::NumI(-2))])));
        assert_eq!(
            validate_num_i(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumI, ValidationErr::Gt(Value::NumI(-2))]))
        );
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumI, ValidationErr::Gt(Value::NumI(-2))])));
    }

    #[test]
    fn test_validate_num_i_lt() {
        let v = NumIValidation::default().lt(-5);
        assert_eq!(validate_num_i(&v, &Value::NumI(-6)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::NumI(-5)), Err(SchemaErr::validation([ValidationErr::Lt(Value::NumI(-5))])));
        assert_eq!(
            validate_num_i(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumI, ValidationErr::Lt(Value::NumI(-5))]))
        );
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumI, ValidationErr::Lt(Value::NumI(-5))])));
    }

    #[test]
    fn test_validate_num_i_ge() {
        let v = NumIValidation::default().ge(-2);
        assert_eq!(validate_num_i(&v, &Value::NumI(-2)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::NumI(-3)), Err(SchemaErr::validation([ValidationErr::Ge(Value::NumI(-2))])));
        assert_eq!(
            validate_num_i(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumI, ValidationErr::Ge(Value::NumI(-2))]))
        );
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumI, ValidationErr::Ge(Value::NumI(-2))])));
    }

    #[test]
    fn test_validate_num_i_le() {
        let v = NumIValidation::default().le(-5);
        assert_eq!(validate_num_i(&v, &Value::NumI(-5)), Ok(()));
        assert_eq!(validate_num_i(&v, &Value::NumI(-4)), Err(SchemaErr::validation([ValidationErr::Le(Value::NumI(-5))])));
        assert_eq!(
            validate_num_i(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumI, ValidationErr::Le(Value::NumI(-5))]))
        );
        assert_eq!(validate_num_i(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumI, ValidationErr::Le(Value::NumI(-5))])));
    }
}

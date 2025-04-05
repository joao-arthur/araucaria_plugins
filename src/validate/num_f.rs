use araucaria::{
    error::{SchemaErr, ValidationErr},
    validation::num_f::NumFValidation,
    value::Value,
};

pub fn validate_num_f(validation: &NumFValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::NumF(num_f_value) => {
            if let Some(v) = validation.eq {
                if num_f_value != &v {
                    base.push(ValidationErr::Eq(Value::NumF(v)));
                }
            }
            if let Some(v) = validation.ne {
                if num_f_value == &v {
                    base.push(ValidationErr::Ne(Value::NumF(v)));
                }
            }
            if let Some(v) = validation.gt {
                if num_f_value <= &v {
                    base.push(ValidationErr::Gt(Value::NumF(v)));
                }
            }
            if let Some(v) = validation.lt {
                if num_f_value >= &v {
                    base.push(ValidationErr::Lt(Value::NumF(v)));
                }
            }
            if let Some(v) = validation.ge {
                if num_f_value < &v {
                    base.push(ValidationErr::Ge(Value::NumF(v)));
                }
            }
            if let Some(v) = validation.le {
                if num_f_value > &v {
                    base.push(ValidationErr::Le(Value::NumF(v)));
                }
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::NumF);
            if let Some(v) = validation.eq {
                base.push(ValidationErr::Eq(Value::NumF(v)));
            }
            if let Some(v) = validation.ne {
                base.push(ValidationErr::Ne(Value::NumF(v)));
            }
            if let Some(v) = validation.gt {
                base.push(ValidationErr::Gt(Value::NumF(v)));
            }
            if let Some(v) = validation.lt {
                base.push(ValidationErr::Lt(Value::NumF(v)));
            }
            if let Some(v) = validation.ge {
                base.push(ValidationErr::Ge(Value::NumF(v)));
            }
            if let Some(v) = validation.le {
                base.push(ValidationErr::Le(Value::NumF(v)));
            }
        }
        _ => {
            base.push(ValidationErr::NumF);
            if let Some(v) = validation.eq {
                base.push(ValidationErr::Eq(Value::NumF(v)));
            }
            if let Some(v) = validation.ne {
                base.push(ValidationErr::Ne(Value::NumF(v)));
            }
            if let Some(v) = validation.gt {
                base.push(ValidationErr::Gt(Value::NumF(v)));
            }
            if let Some(v) = validation.lt {
                base.push(ValidationErr::Lt(Value::NumF(v)));
            }
            if let Some(v) = validation.ge {
                base.push(ValidationErr::Ge(Value::NumF(v)));
            }
            if let Some(v) = validation.le {
                base.push(ValidationErr::Le(Value::NumF(v)));
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
    fn test_validate_num_f_default() {
        let v = NumFValidation::default();
        assert_eq!(validate_num_f(&v, &Value::NumF(-42.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumF])));
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumF])));
    }

    #[test]
    fn test_validate_num_f_optional() {
        let v = NumFValidation::default().optional();
        assert_eq!(validate_num_f(&v, &Value::NumF(-42.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::NumF])));
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumF])));
    }

    #[test]
    fn test_validate_num_f_eq() {
        let v = NumFValidation::default().eq(-42.5);
        assert_eq!(validate_num_f(&v, &Value::NumF(-42.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::NumF(-7.5)), Err(SchemaErr::validation([ValidationErr::Eq(Value::NumF(-42.5))])));
        assert_eq!(
            validate_num_f(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumF, ValidationErr::Eq(Value::NumF(-42.5))]))
        );
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumF, ValidationErr::Eq(Value::NumF(-42.5))])));
    }

    #[test]
    fn test_validate_num_f_ne() {
        let v = NumFValidation::default().ne(-22.5);
        assert_eq!(validate_num_f(&v, &Value::NumF(-42.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::NumF(-22.5)), Err(SchemaErr::validation([ValidationErr::Ne(Value::NumF(-22.5))])));
        assert_eq!(
            validate_num_f(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumF, ValidationErr::Ne(Value::NumF(-22.5))]))
        );
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumF, ValidationErr::Ne(Value::NumF(-22.5))])));
    }

    #[test]
    fn test_validate_num_f_gt() {
        let v = NumFValidation::default().gt(-2.5);
        assert_eq!(validate_num_f(&v, &Value::NumF(-1.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::NumF(-2.5)), Err(SchemaErr::validation([ValidationErr::Gt(Value::NumF(-2.5))])));
        assert_eq!(
            validate_num_f(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumF, ValidationErr::Gt(Value::NumF(-2.5))]))
        );
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumF, ValidationErr::Gt(Value::NumF(-2.5))])));
    }

    #[test]
    fn test_validate_num_f_lt() {
        let v = NumFValidation::default().lt(-5.5);
        assert_eq!(validate_num_f(&v, &Value::NumF(-6.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::NumF(-5.5)), Err(SchemaErr::validation([ValidationErr::Lt(Value::NumF(-5.5))])));
        assert_eq!(
            validate_num_f(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumF, ValidationErr::Lt(Value::NumF(-5.5))]))
        );
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumF, ValidationErr::Lt(Value::NumF(-5.5))])));
    }

    #[test]
    fn test_validate_num_f_ge() {
        let v = NumFValidation::default().ge(-2.5);
        assert_eq!(validate_num_f(&v, &Value::NumF(-2.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::NumF(-3.5)), Err(SchemaErr::validation([ValidationErr::Ge(Value::NumF(-2.5))])));
        assert_eq!(
            validate_num_f(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumF, ValidationErr::Ge(Value::NumF(-2.5))]))
        );
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumF, ValidationErr::Ge(Value::NumF(-2.5))])));
    }

    #[test]
    fn test_validate_num_f_le() {
        let v = NumFValidation::default().le(-5.5);
        assert_eq!(validate_num_f(&v, &Value::NumF(-5.5)), Ok(()));
        assert_eq!(validate_num_f(&v, &Value::NumF(-4.5)), Err(SchemaErr::validation([ValidationErr::Le(Value::NumF(-5.5))])));
        assert_eq!(
            validate_num_f(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumF, ValidationErr::Le(Value::NumF(-5.5))]))
        );
        assert_eq!(validate_num_f(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumF, ValidationErr::Le(Value::NumF(-5.5))])));
    }
}

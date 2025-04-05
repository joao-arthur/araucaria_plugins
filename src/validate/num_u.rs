use araucaria::{
    error::{SchemaErr, ValidationErr},
    validation::num_u::NumUValidation,
    value::Value,
};

pub fn validate_num_u(validation: &NumUValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::NumU(num_u_value) => {
            if let Some(v) = validation.eq {
                if num_u_value != &v {
                    base.push(ValidationErr::Eq(Value::NumU(v)));
                }
            }
            if let Some(v) = validation.ne {
                if num_u_value == &v {
                    base.push(ValidationErr::Ne(Value::NumU(v)));
                }
            }
            if let Some(v) = validation.gt {
                if num_u_value <= &v {
                    base.push(ValidationErr::Gt(Value::NumU(v)));
                }
            }
            if let Some(v) = validation.lt {
                if num_u_value >= &v {
                    base.push(ValidationErr::Lt(Value::NumU(v)));
                }
            }
            if let Some(v) = validation.ge {
                if num_u_value < &v {
                    base.push(ValidationErr::Ge(Value::NumU(v)));
                }
            }
            if let Some(v) = validation.le {
                if num_u_value > &v {
                    base.push(ValidationErr::Le(Value::NumU(v)));
                }
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::NumU);
            if let Some(v) = validation.eq {
                base.push(ValidationErr::Eq(Value::NumU(v)));
            }
            if let Some(v) = validation.ne {
                base.push(ValidationErr::Ne(Value::NumU(v)));
            }
            if let Some(v) = validation.gt {
                base.push(ValidationErr::Gt(Value::NumU(v)));
            }
            if let Some(v) = validation.lt {
                base.push(ValidationErr::Lt(Value::NumU(v)));
            }
            if let Some(v) = validation.ge {
                base.push(ValidationErr::Ge(Value::NumU(v)));
            }
            if let Some(v) = validation.le {
                base.push(ValidationErr::Le(Value::NumU(v)));
            }
        }
        _ => {
            base.push(ValidationErr::NumU);
            if let Some(v) = validation.eq {
                base.push(ValidationErr::Eq(Value::NumU(v)));
            }
            if let Some(v) = validation.ne {
                base.push(ValidationErr::Ne(Value::NumU(v)));
            }
            if let Some(v) = validation.gt {
                base.push(ValidationErr::Gt(Value::NumU(v)));
            }
            if let Some(v) = validation.lt {
                base.push(ValidationErr::Lt(Value::NumU(v)));
            }
            if let Some(v) = validation.ge {
                base.push(ValidationErr::Ge(Value::NumU(v)));
            }
            if let Some(v) = validation.le {
                base.push(ValidationErr::Le(Value::NumU(v)));
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
    fn test_validate_num_u_default() {
        let v = NumUValidation::default();
        assert_eq!(validate_num_u(&v, &Value::NumU(42)), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumU])));
        assert_eq!(validate_num_u(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumU])));
    }

    #[test]
    fn test_validate_num_u_optional() {
        let v = NumUValidation::default().optional();
        assert_eq!(validate_num_u(&v, &Value::NumU(42)), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::NumU])));
        assert_eq!(validate_num_u(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumU])));
    }

    #[test]
    fn test_validate_num_u_eq() {
        let v = NumUValidation::default().eq(42);
        assert_eq!(validate_num_u(&v, &Value::NumU(42)), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::NumU(0)), Err(SchemaErr::validation([ValidationErr::Eq(Value::NumU(42))])));
        assert_eq!(
            validate_num_u(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumU, ValidationErr::Eq(Value::NumU(42))]))
        );
        assert_eq!(validate_num_u(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumU, ValidationErr::Eq(Value::NumU(42))])));
    }

    #[test]
    fn test_validate_num_u_ne() {
        let v = NumUValidation::default().ne(22);
        assert_eq!(validate_num_u(&v, &Value::NumU(42)), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::NumU(22)), Err(SchemaErr::validation([ValidationErr::Ne(Value::NumU(22))])));
        assert_eq!(
            validate_num_u(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumU, ValidationErr::Ne(Value::NumU(22))]))
        );
        assert_eq!(validate_num_u(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumU, ValidationErr::Ne(Value::NumU(22))])));
    }

    #[test]
    fn test_validate_num_u_gt() {
        let v = NumUValidation::default().gt(1);
        assert_eq!(validate_num_u(&v, &Value::NumU(2)), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::NumU(1)), Err(SchemaErr::validation([ValidationErr::Gt(Value::NumU(1))])));
        assert_eq!(
            validate_num_u(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumU, ValidationErr::Gt(Value::NumU(1))]))
        );
        assert_eq!(validate_num_u(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumU, ValidationErr::Gt(Value::NumU(1))])));
    }

    #[test]
    fn test_validate_num_u_lt() {
        let v = NumUValidation::default().lt(5);
        assert_eq!(validate_num_u(&v, &Value::NumU(4)), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::NumU(5)), Err(SchemaErr::validation([ValidationErr::Lt(Value::NumU(5))])));
        assert_eq!(
            validate_num_u(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumU, ValidationErr::Lt(Value::NumU(5))]))
        );
        assert_eq!(validate_num_u(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumU, ValidationErr::Lt(Value::NumU(5))])));
    }

    #[test]
    fn test_validate_num_u_ge() {
        let v = NumUValidation::default().ge(1);
        assert_eq!(validate_num_u(&v, &Value::NumU(1)), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::NumU(0)), Err(SchemaErr::validation([ValidationErr::Ge(Value::NumU(1))])));
        assert_eq!(
            validate_num_u(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumU, ValidationErr::Ge(Value::NumU(1))]))
        );
        assert_eq!(validate_num_u(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumU, ValidationErr::Ge(Value::NumU(1))])));
    }

    #[test]
    fn test_validate_num_u_le() {
        let v = NumUValidation::default().le(5);
        assert_eq!(validate_num_u(&v, &Value::NumU(5)), Ok(()));
        assert_eq!(validate_num_u(&v, &Value::NumU(6)), Err(SchemaErr::validation([ValidationErr::Le(Value::NumU(5))])));
        assert_eq!(
            validate_num_u(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::NumU, ValidationErr::Le(Value::NumU(5))]))
        );
        assert_eq!(validate_num_u(&v, &bool_stub()), Err(SchemaErr::validation([ValidationErr::NumU, ValidationErr::Le(Value::NumU(5))])));
    }
}

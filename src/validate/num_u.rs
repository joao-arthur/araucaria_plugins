use araucaria::{
    error::{SchemaErr, ValidationErr},
    validation::num_u::NumUValidation,
    value::Value,
};

pub fn validate_num_u(validation: &NumUValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::NumU(num_u_value) => {
            if let Some(eq_v) = validation.eq {
                if num_u_value != &eq_v {
                    base.push(ValidationErr::Eq(Value::NumU(eq_v)));
                }
            }
            if let Some(ne_v) = validation.ne {
                if num_u_value == &ne_v {
                    base.push(ValidationErr::Ne(Value::NumU(ne_v)));
                }
            }
            if let Some(gt_v) = validation.gt {
                if num_u_value <= &gt_v {
                    base.push(ValidationErr::Gt(Value::NumU(gt_v)));
                }
            }
            if let Some(lt_v) = validation.lt {
                if num_u_value >= &lt_v {
                    base.push(ValidationErr::Lt(Value::NumU(lt_v)));
                }
            }
            if let Some(ge_v) = validation.ge {
                if num_u_value < &ge_v {
                    base.push(ValidationErr::Ge(Value::NumU(ge_v)));
                }
            }
            if let Some(le_v) = validation.le {
                if num_u_value > &le_v {
                    base.push(ValidationErr::Le(Value::NumU(le_v)));
                }
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::NumU);
            if let Some(eq_v) = validation.eq {
                base.push(ValidationErr::Eq(Value::NumU(eq_v)));
            }
            if let Some(ne_v) = validation.ne {
                base.push(ValidationErr::Ne(Value::NumU(ne_v)));
            }
            if let Some(gt_v) = validation.gt {
                base.push(ValidationErr::Gt(Value::NumU(gt_v)));
            }
            if let Some(lt_v) = validation.lt {
                base.push(ValidationErr::Lt(Value::NumU(lt_v)));
            }
            if let Some(ge_v) = validation.ge {
                base.push(ValidationErr::Ge(Value::NumU(ge_v)));
            }
            if let Some(le_v) = validation.le {
                base.push(ValidationErr::Le(Value::NumU(le_v)));
            }
        }
        _ => {
            base.push(ValidationErr::NumU);
            if let Some(eq_v) = validation.eq {
                base.push(ValidationErr::Eq(Value::NumU(eq_v)));
            }
            if let Some(ne_v) = validation.ne {
                base.push(ValidationErr::Ne(Value::NumU(ne_v)));
            }
            if let Some(gt_v) = validation.gt {
                base.push(ValidationErr::Gt(Value::NumU(gt_v)));
            }
            if let Some(lt_v) = validation.lt {
                base.push(ValidationErr::Lt(Value::NumU(lt_v)));
            }
            if let Some(ge_v) = validation.ge {
                base.push(ValidationErr::Ge(Value::NumU(ge_v)));
            }
            if let Some(le_v) = validation.le {
                base.push(ValidationErr::Le(Value::NumU(le_v)));
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

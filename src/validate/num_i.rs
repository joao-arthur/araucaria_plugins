use araucaria::{
    error::{SchemaErr, ValidationErr},
    validation::num_i::NumIValidation,
    value::Value,
};

pub fn validate_num_i(validation: &NumIValidation, value: &Value) -> Option<SchemaErr> {
    let mut base = vec![];
    match value {
        Value::NumI(num_i_value) => {
            if let Some(eq_v) = validation.eq {
                if num_i_value != &eq_v {
                    base.push(ValidationErr::Eq(Value::NumI(eq_v)));
                }
            }
            if let Some(ne_v) = validation.ne {
                if num_i_value == &ne_v {
                    base.push(ValidationErr::Ne(Value::NumI(ne_v)));
                }
            }
            if let Some(gt_v) = validation.gt {
                if num_i_value <= &gt_v {
                    base.push(ValidationErr::Gt(Value::NumI(gt_v)));
                }
            }
            if let Some(lt_v) = validation.lt {
                if num_i_value >= &lt_v {
                    base.push(ValidationErr::Lt(Value::NumI(lt_v)));
                }
            }
            if let Some(ge_v) = validation.ge {
                if num_i_value < &ge_v {
                    base.push(ValidationErr::Ge(Value::NumI(ge_v)));
                }
            }
            if let Some(le_v) = validation.le {
                if num_i_value > &le_v {
                    base.push(ValidationErr::Le(Value::NumI(le_v)));
                }
            }
        }
        Value::None => {
            base.push(ValidationErr::NumI);
            if validation.required {
                base.push(ValidationErr::Required);
            }
            if let Some(eq_v) = validation.eq {
                base.push(ValidationErr::Eq(Value::NumI(eq_v)));
            }
            if let Some(ne_v) = validation.ne {
                base.push(ValidationErr::Ne(Value::NumI(ne_v)));
            }
            if let Some(gt_v) = validation.gt {
                base.push(ValidationErr::Gt(Value::NumI(gt_v)));
            }
            if let Some(lt_v) = validation.lt {
                base.push(ValidationErr::Lt(Value::NumI(lt_v)));
            }
            if let Some(ge_v) = validation.ge {
                base.push(ValidationErr::Ge(Value::NumI(ge_v)));
            }
            if let Some(le_v) = validation.le {
                base.push(ValidationErr::Le(Value::NumI(le_v)));
            }
        }
        _ => {
            base.push(ValidationErr::NumI);
            if let Some(eq_v) = validation.eq {
                base.push(ValidationErr::Eq(Value::NumI(eq_v)));
            }
            if let Some(ne_v) = validation.ne {
                base.push(ValidationErr::Ne(Value::NumI(ne_v)));
            }
            if let Some(gt_v) = validation.gt {
                base.push(ValidationErr::Gt(Value::NumI(gt_v)));
            }
            if let Some(lt_v) = validation.lt {
                base.push(ValidationErr::Lt(Value::NumI(lt_v)));
            }
            if let Some(ge_v) = validation.ge {
                base.push(ValidationErr::Ge(Value::NumI(ge_v)));
            }
            if let Some(le_v) = validation.le {
                base.push(ValidationErr::Le(Value::NumI(le_v)));
            }
        }
    }
    if !base.is_empty() {
        Some(SchemaErr::Arr(base))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::stub::{
        arr_bool_stub, bool_stub, num_f_stub, num_i_stub, num_u_stub, obj_stub, str_stub,
    };

    use super::*;

    #[test]
    fn test_validate_num_i_default() {
        let v = NumIValidation::default();
        assert_eq!(validate_num_i(&v, &Value::NumI(-42)), None);
        assert_eq!(
            validate_num_i(&v, &Value::None),
            SchemaErr::arr([ValidationErr::NumI, ValidationErr::Required])
        );
        assert_eq!(validate_num_i(&v, &bool_stub()), SchemaErr::arr([ValidationErr::NumI]));
    }

    #[test]
    fn test_validate_num_i_optional() {
        let v = NumIValidation::default().optional();
        assert_eq!(validate_num_i(&v, &Value::NumI(-42)), None);
        assert_eq!(validate_num_i(&v, &Value::None), SchemaErr::arr([ValidationErr::NumI]));
        assert_eq!(validate_num_i(&v, &bool_stub()), SchemaErr::arr([ValidationErr::NumI]));
    }

    #[test]
    fn test_validate_num_i_eq() {
        let v = NumIValidation::default().eq(-42);
        assert_eq!(validate_num_i(&v, &Value::NumI(-42)), None);
        assert_eq!(
            validate_num_i(&v, &Value::NumI(-7)),
            SchemaErr::arr([ValidationErr::Eq(Value::NumI(-42))])
        );
        assert_eq!(
            validate_num_i(&v, &Value::None),
            SchemaErr::arr([
                ValidationErr::NumI,
                ValidationErr::Required,
                ValidationErr::Eq(Value::NumI(-42))
            ])
        );
        assert_eq!(
            validate_num_i(&v, &bool_stub()),
            SchemaErr::arr([ValidationErr::NumI, ValidationErr::Eq(Value::NumI(-42))])
        );
    }

    #[test]
    fn test_validate_num_i_ne() {
        let v = NumIValidation::default().ne(-22);
        assert_eq!(validate_num_i(&v, &Value::NumI(-42)), None);
        assert_eq!(
            validate_num_i(&v, &Value::NumI(-22)),
            SchemaErr::arr([ValidationErr::Ne(Value::NumI(-22))])
        );
        assert_eq!(
            validate_num_i(&v, &Value::None),
            SchemaErr::arr([
                ValidationErr::NumI,
                ValidationErr::Required,
                ValidationErr::Ne(Value::NumI(-22))
            ])
        );
        assert_eq!(
            validate_num_i(&v, &bool_stub()),
            SchemaErr::arr([ValidationErr::NumI, ValidationErr::Ne(Value::NumI(-22))])
        );
    }

    #[test]
    fn test_validate_num_i_gt() {
        let v = NumIValidation::default().gt(-2);
        assert_eq!(validate_num_i(&v, &Value::NumI(-1)), None);
        assert_eq!(
            validate_num_i(&v, &Value::NumI(-2)),
            SchemaErr::arr([ValidationErr::Gt(Value::NumI(-2))])
        );
        assert_eq!(
            validate_num_i(&v, &Value::None),
            SchemaErr::arr([
                ValidationErr::NumI,
                ValidationErr::Required,
                ValidationErr::Gt(Value::NumI(-2))
            ])
        );
        assert_eq!(
            validate_num_i(&v, &bool_stub()),
            SchemaErr::arr([ValidationErr::NumI, ValidationErr::Gt(Value::NumI(-2))])
        );
    }

    #[test]
    fn test_validate_num_i_lt() {
        let v = NumIValidation::default().lt(-5);
        assert_eq!(validate_num_i(&v, &Value::NumI(-6)), None);
        assert_eq!(
            validate_num_i(&v, &Value::NumI(-5)),
            SchemaErr::arr([ValidationErr::Lt(Value::NumI(-5))])
        );
        assert_eq!(
            validate_num_i(&v, &Value::None),
            SchemaErr::arr([
                ValidationErr::NumI,
                ValidationErr::Required,
                ValidationErr::Lt(Value::NumI(-5))
            ])
        );
        assert_eq!(
            validate_num_i(&v, &bool_stub()),
            SchemaErr::arr([ValidationErr::NumI, ValidationErr::Lt(Value::NumI(-5))])
        );
    }

    #[test]
    fn test_validate_num_i_ge() {
        let v = NumIValidation::default().ge(-2);
        assert_eq!(validate_num_i(&v, &Value::NumI(-2)), None);
        assert_eq!(
            validate_num_i(&v, &Value::NumI(-3)),
            SchemaErr::arr([ValidationErr::Ge(Value::NumI(-2))])
        );
        assert_eq!(
            validate_num_i(&v, &Value::None),
            SchemaErr::arr([
                ValidationErr::NumI,
                ValidationErr::Required,
                ValidationErr::Ge(Value::NumI(-2))
            ])
        );
        assert_eq!(
            validate_num_i(&v, &bool_stub()),
            SchemaErr::arr([ValidationErr::NumI, ValidationErr::Ge(Value::NumI(-2))])
        );
    }

    #[test]
    fn test_validate_num_i_le() {
        let v = NumIValidation::default().le(-5);
        assert_eq!(validate_num_i(&v, &Value::NumI(-5)), None);
        assert_eq!(
            validate_num_i(&v, &Value::NumI(-4)),
            SchemaErr::arr([ValidationErr::Le(Value::NumI(-5))])
        );
        assert_eq!(
            validate_num_i(&v, &Value::None),
            SchemaErr::arr([
                ValidationErr::NumI,
                ValidationErr::Required,
                ValidationErr::Le(Value::NumI(-5))
            ])
        );
        assert_eq!(
            validate_num_i(&v, &bool_stub()),
            SchemaErr::arr([ValidationErr::NumI, ValidationErr::Le(Value::NumI(-5))])
        );
    }
}

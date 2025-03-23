use crate::{
    error::{Err, ErrWrap},
    validation::num_f::NumFValidation,
    value::Value,
};

pub fn validate_num_f(validation: &NumFValidation, value: &Value) -> Option<ErrWrap> {
    let mut base = vec![];
    match value {
        Value::NumF(num_f_value) => {
            if let Some(eq_v) = validation.eq {
                if num_f_value != &eq_v {
                    base.push(Err::Eq(Value::NumF(eq_v)));
                }
            }
            if let Some(ne_v) = validation.ne {
                if num_f_value == &ne_v {
                    base.push(Err::Ne(Value::NumF(ne_v)));
                }
            }
            if let Some(gt_v) = validation.gt {
                if num_f_value <= &gt_v {
                    base.push(Err::Gt(Value::NumF(gt_v)));
                }
            }
            if let Some(lt_v) = validation.lt {
                if num_f_value >= &lt_v {
                    base.push(Err::Lt(Value::NumF(lt_v)));
                }
            }
            if let Some(ge_v) = validation.ge {
                if num_f_value < &ge_v {
                    base.push(Err::Ge(Value::NumF(ge_v)));
                }
            }
            if let Some(le_v) = validation.le {
                if num_f_value > &le_v {
                    base.push(Err::Le(Value::NumF(le_v)));
                }
            }
        }
        Value::None => {
            base.push(Err::NumF);
            if validation.required {
                base.push(Err::Required);
            }
            if let Some(eq_v) = validation.eq {
                base.push(Err::Eq(Value::NumF(eq_v)));
            }
            if let Some(ne_v) = validation.ne {
                base.push(Err::Ne(Value::NumF(ne_v)));
            }
            if let Some(gt_v) = validation.gt {
                base.push(Err::Gt(Value::NumF(gt_v)));
            }
            if let Some(lt_v) = validation.lt {
                base.push(Err::Lt(Value::NumF(lt_v)));
            }
            if let Some(ge_v) = validation.ge {
                base.push(Err::Ge(Value::NumF(ge_v)));
            }
            if let Some(le_v) = validation.le {
                base.push(Err::Le(Value::NumF(le_v)));
            }
        }
        _ => {
            base.push(Err::NumF);
            if let Some(eq_v) = validation.eq {
                base.push(Err::Eq(Value::NumF(eq_v)));
            }
            if let Some(ne_v) = validation.ne {
                base.push(Err::Ne(Value::NumF(ne_v)));
            }
            if let Some(gt_v) = validation.gt {
                base.push(Err::Gt(Value::NumF(gt_v)));
            }
            if let Some(lt_v) = validation.lt {
                base.push(Err::Lt(Value::NumF(lt_v)));
            }
            if let Some(ge_v) = validation.ge {
                base.push(Err::Ge(Value::NumF(ge_v)));
            }
            if let Some(le_v) = validation.le {
                base.push(Err::Le(Value::NumF(le_v)));
            }
        }
    }
    if !base.is_empty() {
        Some(ErrWrap::Arr(base))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::value::stub::{
        arr_bool_stub, bool_stub, num_f_stub, num_i_stub, num_u_stub, obj_stub, str_stub,
    };

    use super::*;

    #[test]
    fn test_validate_num_f_default() {
        let v = NumFValidation::default();
        assert_eq!(validate_num_f(&v, &Value::NumF(-42.5)), None);
        assert_eq!(validate_num_f(&v, &Value::None), ErrWrap::arr([Err::NumF]));
        assert_eq!(validate_num_f(&v, &bool_stub()), ErrWrap::arr([Err::NumF]));
    }

    #[test]
    fn test_validate_num_f_required() {
        let v = NumFValidation::default().required();
        assert_eq!(validate_num_f(&v, &Value::NumF(-42.5)), None);
        assert_eq!(validate_num_f(&v, &Value::None), ErrWrap::arr([Err::NumF, Err::Required]));
        assert_eq!(validate_num_f(&v, &bool_stub()), ErrWrap::arr([Err::NumF]));
    }

    #[test]
    fn test_validate_num_f_eq() {
        let v = NumFValidation::default().eq(-42.5);
        assert_eq!(validate_num_f(&v, &Value::NumF(-42.5)), None);
        assert_eq!(validate_num_f(&v, &Value::NumF(-7.5)), ErrWrap::arr([Err::Eq(Value::NumF(-42.5))]));
        assert_eq!(validate_num_f(&v, &Value::None), ErrWrap::arr([Err::NumF, Err::Eq(Value::NumF(-42.5))]));
        assert_eq!(validate_num_f(&v, &bool_stub()), ErrWrap::arr([Err::NumF, Err::Eq(Value::NumF(-42.5))]));
    }

    #[test]
    fn test_validate_num_f_ne() {
        let v = NumFValidation::default().ne(-22.5);
        assert_eq!(validate_num_f(&v, &Value::NumF(-42.5)), None);
        assert_eq!(validate_num_f(&v, &Value::NumF(-22.5)), ErrWrap::arr([Err::Ne(Value::NumF(-22.5))]));
        assert_eq!(validate_num_f(&v, &Value::None), ErrWrap::arr([Err::NumF, Err::Ne(Value::NumF(-22.5))]));
        assert_eq!(validate_num_f(&v, &bool_stub()), ErrWrap::arr([Err::NumF, Err::Ne(Value::NumF(-22.5))]));
    }

    #[test]
    fn test_validate_num_f_gt() {
        let v = NumFValidation::default().gt(-2.5);
        assert_eq!(validate_num_f(&v, &Value::NumF(-1.5)), None);
        assert_eq!(validate_num_f(&v, &Value::NumF(-2.5)), ErrWrap::arr([Err::Gt(Value::NumF(-2.5))]));
        assert_eq!(validate_num_f(&v, &Value::None), ErrWrap::arr([Err::NumF, Err::Gt(Value::NumF(-2.5))]));
        assert_eq!(validate_num_f(&v, &bool_stub()), ErrWrap::arr([Err::NumF, Err::Gt(Value::NumF(-2.5))]));
    }

    #[test]
    fn test_validate_num_f_lt() {
        let v = NumFValidation::default().lt(-5.5);
        assert_eq!(validate_num_f(&v, &Value::NumF(-6.5)), None);
        assert_eq!(validate_num_f(&v, &Value::NumF(-5.5)), ErrWrap::arr([Err::Lt(Value::NumF(-5.5))]));
        assert_eq!(validate_num_f(&v, &Value::None), ErrWrap::arr([Err::NumF, Err::Lt(Value::NumF(-5.5))]));
        assert_eq!(validate_num_f(&v, &bool_stub()), ErrWrap::arr([Err::NumF, Err::Lt(Value::NumF(-5.5))]));
    }

    #[test]
    fn test_validate_num_f_ge() {
        let v = NumFValidation::default().ge(-2.5);
        assert_eq!(validate_num_f(&v, &Value::NumF(-2.5)), None);
        assert_eq!(validate_num_f(&v, &Value::NumF(-3.5)), ErrWrap::arr([Err::Ge(Value::NumF(-2.5))]));
        assert_eq!(validate_num_f(&v, &Value::None), ErrWrap::arr([Err::NumF, Err::Ge(Value::NumF(-2.5))]));
        assert_eq!(validate_num_f(&v, &bool_stub()), ErrWrap::arr([Err::NumF, Err::Ge(Value::NumF(-2.5))]));
    }

    #[test]
    fn test_validate_num_f_le() {
        let v = NumFValidation::default().le(-5.5);
        assert_eq!(validate_num_f(&v, &Value::NumF(-5.5)), None);
        assert_eq!(validate_num_f(&v, &Value::NumF(-4.5)), ErrWrap::arr([Err::Le(Value::NumF(-5.5))]));
        assert_eq!(validate_num_f(&v, &Value::None), ErrWrap::arr([Err::NumF, Err::Le(Value::NumF(-5.5))]));
        assert_eq!(validate_num_f(&v, &bool_stub()), ErrWrap::arr([Err::NumF, Err::Le(Value::NumF(-5.5))]));
    }
}

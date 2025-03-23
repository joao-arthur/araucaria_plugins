use crate::{
    error::{Err, ErrWrap},
    validation::bool::BoolValidation,
    value::Value,
};

pub fn validate_bool(validation: &BoolValidation, value: &Value) -> Option<ErrWrap> {
    let mut base = vec![];
    match value {
        Value::Bool(bool_value) => {
            if let Some(eq_v) = validation.eq {
                if bool_value != &eq_v {
                    base.push(Err::Eq(Value::Bool(eq_v)));
                }
            }
            if let Some(ne_v) = validation.ne {
                if bool_value == &ne_v {
                    base.push(Err::Ne(Value::Bool(ne_v)));
                }
            }
        }
        Value::None => {
            base.push(Err::Bool);
            if validation.required {
                base.push(Err::Required);
            }
            if let Some(eq_v) = validation.eq {
                base.push(Err::Eq(Value::Bool(eq_v)));
            }
            if let Some(ne_v) = validation.ne {
                base.push(Err::Ne(Value::Bool(ne_v)));
            }
        }
        _ => {
            base.push(Err::Bool);
            if let Some(eq_v) = validation.eq {
                base.push(Err::Eq(Value::Bool(eq_v)));
            }
            if let Some(ne_v) = validation.ne {
                base.push(Err::Ne(Value::Bool(ne_v)));
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
        arr_bool_stub, num_f_stub, num_i_stub, num_u_stub, obj_stub, str_stub,
    };

    use super::*;

    #[test]
    fn test_validate_bool_default() {
        let v = BoolValidation::default();
        assert_eq!(validate_bool(&v, &Value::Bool(false)), None);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), None);
        assert_eq!(validate_bool(&v, &Value::None), ErrWrap::arr([Err::Bool]));
        assert_eq!(validate_bool(&v, &num_u_stub()), ErrWrap::arr([Err::Bool]));
    }

    #[test]
    fn test_validate_bool_required() {
        let v = BoolValidation::default().required();
        assert_eq!(validate_bool(&v, &Value::Bool(false)), None);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), None);
        assert_eq!(validate_bool(&v, &Value::None), ErrWrap::arr([Err::Bool, Err::Required]));
        assert_eq!(validate_bool(&v, &num_u_stub()), ErrWrap::arr([Err::Bool]));
    }

    #[test]
    fn test_validate_bool_eq() {
        let v = BoolValidation::default().eq(false);
        assert_eq!(validate_bool(&v, &Value::Bool(false)), None);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), ErrWrap::arr([Err::Eq(Value::Bool(false))]));
        assert_eq!(validate_bool(&v, &Value::None), ErrWrap::arr([Err::Bool,  Err::Eq(Value::Bool(false))]));
        assert_eq!(validate_bool(&v, &num_u_stub()), ErrWrap::arr([Err::Bool, Err::Eq(Value::Bool(false))]));
    }

    #[test]
    fn test_validate_bool_ne() {
        let v = BoolValidation::default().ne(false);
        assert_eq!(validate_bool(&v, &Value::Bool(true)), None);
        assert_eq!(validate_bool(&v, &Value::Bool(false)), ErrWrap::arr([Err::Ne(Value::Bool(false))]));
        assert_eq!(validate_bool(&v, &Value::None), ErrWrap::arr([Err::Bool, Err::Ne(Value::Bool(false))]));
        assert_eq!(validate_bool(&v, &num_u_stub()), ErrWrap::arr([Err::Bool, Err::Ne(Value::Bool(false))]));
    }
}

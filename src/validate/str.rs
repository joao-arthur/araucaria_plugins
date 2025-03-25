use araucaria::{
    error::{Err, ErrWrap},
    validation::str::StrValidation,
    value::Value,
};

pub fn validate_str(validation: &StrValidation, value: &Value) -> Option<ErrWrap> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            if let Some(eq_v) = &validation.eq {
                if str_value != eq_v {
                    base.push(Err::Eq(Value::Str(eq_v.clone())));
                }
            }
            if let Some(ne_v) = &validation.ne {
                if str_value == ne_v {
                    base.push(Err::Ne(Value::Str(ne_v.clone())));
                }
            }
        }
        Value::None => {
            base.push(Err::Str);
            if validation.required {
                base.push(Err::Required);
            }
            if let Some(eq_v) = &validation.eq {
                base.push(Err::Eq(Value::Str(eq_v.clone())));
            }
            if let Some(ne_v) = &validation.ne {
                base.push(Err::Ne(Value::Str(ne_v.clone())));
            }
        }
        _ => {
            base.push(Err::Str);
            if let Some(eq_v) = &validation.eq {
                base.push(Err::Eq(Value::Str(eq_v.clone())));
            }
            if let Some(ne_v) = &validation.ne {
                base.push(Err::Ne(Value::Str(ne_v.clone())));
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
    use crate::stub::{num_f_stub, num_i_stub, num_u_stub, obj_stub, str_stub};

    use super::*;

    #[test]
    fn test_validate_str_default() {
        let v = StrValidation::default();
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum")), None);
        assert_eq!(validate_str(&v, &Value::None), ErrWrap::arr([Err::Str]));
        assert_eq!(validate_str(&v, &num_u_stub()), ErrWrap::arr([Err::Str]));
    }

    #[test]
    fn test_validate_str_required() {
        let v = StrValidation::default().required();
        assert_eq!(validate_str(&v, &Value::Str(String::from("Cogito ergo sum"))), None);
        assert_eq!(validate_str(&v, &Value::None), ErrWrap::arr([Err::Str, Err::Required]));
        assert_eq!(validate_str(&v, &num_u_stub()), ErrWrap::arr([Err::Str]));
    }

    #[test]
    fn test_validate_str_eq() {
        let v = StrValidation::default().eq(String::from("Cogito ergo sum"));
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum")), None);
        assert_eq!(
            validate_str(&v, &Value::from("Memento mori")),
            ErrWrap::arr([Err::Eq(Value::Str(String::from("Cogito ergo sum")))])
        );
        assert_eq!(
            validate_str(&v, &Value::None),
            ErrWrap::arr([Err::Str, Err::Eq(Value::Str(String::from("Cogito ergo sum")))])
        );
        assert_eq!(
            validate_str(&v, &num_u_stub()),
            ErrWrap::arr([Err::Str, Err::Eq(Value::Str(String::from("Cogito ergo sum")))])
        );
    }

    #[test]
    fn test_validate_str_ne() {
        let v = StrValidation::default().ne(String::from("Cogito ergo sum"));
        assert_eq!(validate_str(&v, &Value::Str(String::from("Memento mori"))), None);
        assert_eq!(
            validate_str(&v, &Value::Str(String::from("Cogito ergo sum"))),
            ErrWrap::arr([Err::Ne(Value::Str(String::from("Cogito ergo sum")))])
        );
        assert_eq!(
            validate_str(&v, &Value::None),
            ErrWrap::arr([Err::Str, Err::Ne(Value::Str(String::from("Cogito ergo sum")))])
        );
        assert_eq!(
            validate_str(&v, &num_u_stub()),
            ErrWrap::arr([Err::Str, Err::Ne(Value::Str(String::from("Cogito ergo sum")))])
        );
    }
}

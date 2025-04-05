use araucaria::{
    error::{SchemaErr, ValidationErr},
    validation::str::StrValidation,
    value::Value,
};
use unicode_segmentation::UnicodeSegmentation;

pub fn validate_str(validation: &StrValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            if let Some(v) = &validation.eq {
                if str_value != v {
                    base.push(ValidationErr::Eq(Value::Str(v.clone())));
                }
            }
            if let Some(v) = &validation.ne {
                if str_value == v {
                    base.push(ValidationErr::Ne(Value::Str(v.clone())));
                }
            }
            if let Some(v) = validation.min_bytes_len {
                if str_value.as_bytes().len() < v {
                    base.push(ValidationErr::MinBytesLen);
                }
            }
            if let Some(v) = validation.max_bytes_len {
                if str_value.as_bytes().len() > v {
                    base.push(ValidationErr::MaxBytesLen);
                }
            }
            if let Some(v) = validation.min_graphemes_len {
                if str_value.graphemes(true).collect::<Vec<&str>>().len() < v {
                    base.push(ValidationErr::MinBytesLen);
                }
            }
            if let Some(v) = validation.max_graphemes_len {
                if str_value.graphemes(true).collect::<Vec<&str>>().len() > v {
                    base.push(ValidationErr::MaxBytesLen);
                }
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::Str);
            if let Some(v) = &validation.eq {
                base.push(ValidationErr::Eq(Value::Str(v.clone())));
            }
            if let Some(v) = &validation.ne {
                base.push(ValidationErr::Ne(Value::Str(v.clone())));
            }
        }
        _ => {
            base.push(ValidationErr::Str);
            if let Some(v) = &validation.eq {
                base.push(ValidationErr::Eq(Value::Str(v.clone())));
            }
            if let Some(v) = &validation.ne {
                base.push(ValidationErr::Ne(Value::Str(v.clone())));
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
    fn test_validate_str_default() {
        let v = StrValidation::default();
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum")), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str])));
    }

    #[test]
    fn test_validate_str_optional() {
        let v = StrValidation::default().optional();
        assert_eq!(validate_str(&v, &Value::Str(String::from("Cogito ergo sum"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Str])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str])));
    }

    #[test]
    fn test_validate_str_eq() {
        let v = StrValidation::default().eq(String::from("Cogito ergo sum"));
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum")), Ok(()));
        assert_eq!(
            validate_str(&v, &Value::from("Memento mori")),
            Err(SchemaErr::validation([ValidationErr::Eq(Value::Str(String::from("Cogito ergo sum")))]))
        );
        assert_eq!(
            validate_str(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, ValidationErr::Eq(Value::Str(String::from("Cogito ergo sum")))]))
        );
        assert_eq!(
            validate_str(&v, &num_u_stub()),
            Err(SchemaErr::validation([ValidationErr::Str, ValidationErr::Eq(Value::Str(String::from("Cogito ergo sum")))]))
        );
    }

    #[test]
    fn test_validate_str_ne() {
        let v = StrValidation::default().ne(String::from("Cogito ergo sum"));
        assert_eq!(validate_str(&v, &Value::Str(String::from("Memento mori"))), Ok(()));
        assert_eq!(
            validate_str(&v, &Value::Str(String::from("Cogito ergo sum"))),
            Err(SchemaErr::validation([ValidationErr::Ne(Value::Str(String::from("Cogito ergo sum")))]))
        );
        assert_eq!(
            validate_str(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, ValidationErr::Ne(Value::Str(String::from("Cogito ergo sum")))]))
        );
        assert_eq!(
            validate_str(&v, &num_u_stub()),
            Err(SchemaErr::validation([ValidationErr::Str, ValidationErr::Ne(Value::Str(String::from("Cogito ergo sum")))]))
        );
    }

    #[test]
    fn test_validate_min_bytes_len() {
        let v = StrValidation::default().min_bytes_len(23);
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::MinBytesLen])));
    }

    #[test]
    fn test_validate_max_bytes_len() {
        let v = StrValidation::default().max_bytes_len(23);
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::MaxBytesLen])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
    }

}

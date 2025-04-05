use araucaria::{
    error::{SchemaErr, ValidationErr}, operation::Operation, validation::str::StrValidation, value::Value
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
            if let Some(op) = &validation.bytes_len {
                let len = str_value.as_bytes().len();
                match op {
                    Operation::Eq(v) => {
                        if len != *v {
                            base.push(ValidationErr::MinBytesLen);
                        }
                    }
                    Operation::Ne(v) => {
                        if len == *v {
                            base.push(ValidationErr::MinBytesLen);
                        }
                    }
                    Operation::Gt(v) => {
                        if len <= *v {
                            base.push(ValidationErr::MinBytesLen);
                        }
                    }
                    Operation::Lt(v) => {
                        if len >= *v {
                            base.push(ValidationErr::MinBytesLen);
                        } 
                    }
                    Operation::Ge(v) => {
                        if len < *v {
                            base.push(ValidationErr::MinBytesLen);
                        }
                    }
                    Operation::Le(v) => {
                        if len > *v {
                            base.push(ValidationErr::MinBytesLen);
                        }
                    }
                    Operation::Btwn(a, b) => {
                        if len < *a || len >  *b {
                            base.push(ValidationErr::MinBytesLen);
                        }
                    }
                }
            }
            if let Some(op) = &validation.graphemes_len {
                let len = str_value.graphemes(true).collect::<Vec<&str>>().len() ;
                match op {
                    Operation::Eq(v) => {
                        if len != *v {
                            base.push(ValidationErr::MinGraphemesLen);
                        }
                    }
                    Operation::Ne(v) => {
                        if len == *v {
                            base.push(ValidationErr::MinGraphemesLen);
                        }
                    }
                    Operation::Gt(v) => {
                        if len <= *v {
                            base.push(ValidationErr::MinGraphemesLen);
                        }
                    }
                    Operation::Lt(v) => {
                        if len >= *v {
                            base.push(ValidationErr::MinGraphemesLen);
                        } 
                    }
                    Operation::Ge(v) => {
                        if len < *v {
                            base.push(ValidationErr::MinGraphemesLen);
                        }
                    }
                    Operation::Le(v) => {
                        if len > *v {
                            base.push(ValidationErr::MinGraphemesLen);
                        }
                    }
                    Operation::Btwn(a, b) => {
                        if len < *a || len >  *b {
                            base.push(ValidationErr::MinGraphemesLen);
                        }
                    }
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
        let v = StrValidation::default().bytes_len(Operation::Ge(23));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::MinBytesLen])));
    }

    #[test]
    fn test_validate_max_bytes_len() {
        let v = StrValidation::default().bytes_len(Operation::Le(23));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::MinBytesLen])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
    }
}

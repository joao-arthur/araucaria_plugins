use araucaria::{
    error::{SchemaErr, ValidationErr},
    validation::str::StrValidation,
    value::Value,
};

pub fn validate_str(validation: &StrValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            if let Some(eq_v) = &validation.eq {
                if str_value != eq_v {
                    base.push(ValidationErr::Eq(Value::Str(eq_v.clone())));
                }
            }
            if let Some(ne_v) = &validation.ne {
                if str_value == ne_v {
                    base.push(ValidationErr::Ne(Value::Str(ne_v.clone())));
                }
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::Str);
            if let Some(eq_v) = &validation.eq {
                base.push(ValidationErr::Eq(Value::Str(eq_v.clone())));
            }
            if let Some(ne_v) = &validation.ne {
                base.push(ValidationErr::Ne(Value::Str(ne_v.clone())));
            }
        }
        _ => {
            base.push(ValidationErr::Str);
            if let Some(eq_v) = &validation.eq {
                base.push(ValidationErr::Eq(Value::Str(eq_v.clone())));
            }
            if let Some(ne_v) = &validation.ne {
                base.push(ValidationErr::Ne(Value::Str(ne_v.clone())));
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
    use araucaria::value::stub::{num_f_stub, num_i_stub, num_u_stub, obj_stub, str_stub};

    use super::*;

    #[test]
    fn test_validate_str_default() {
        let v = StrValidation::default();
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum")), Ok(()));
        assert_eq!(
            validate_str(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str]))
        );
        assert_eq!(
            validate_str(&v, &num_u_stub()),
            Err(SchemaErr::validation([ValidationErr::Str]))
        );
    }

    #[test]
    fn test_validate_str_optional() {
        let v = StrValidation::default().optional();
        assert_eq!(validate_str(&v, &Value::Str(String::from("Cogito ergo sum"))), Ok(()));
        assert_eq!(
            validate_str(&v, &Value::None),
            Err(SchemaErr::validation([ValidationErr::Str]))
        );
        assert_eq!(
            validate_str(&v, &num_u_stub()),
            Err(SchemaErr::validation([ValidationErr::Str]))
        );
    }

    #[test]
    fn test_validate_str_eq() {
        let v = StrValidation::default().eq(String::from("Cogito ergo sum"));
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum")), Ok(()));
        assert_eq!(
            validate_str(&v, &Value::from("Memento mori")),
            Err(SchemaErr::validation([ValidationErr::Eq(Value::Str(String::from(
                "Cogito ergo sum"
            )))]))
        );
        assert_eq!(
            validate_str(&v, &Value::None),
            Err(SchemaErr::validation([
                ValidationErr::Required,
                ValidationErr::Str,
                ValidationErr::Eq(Value::Str(String::from("Cogito ergo sum")))
            ]))
        );
        assert_eq!(
            validate_str(&v, &num_u_stub()),
            Err(SchemaErr::validation([
                ValidationErr::Str,
                ValidationErr::Eq(Value::Str(String::from("Cogito ergo sum")))
            ]))
        );
    }

    #[test]
    fn test_validate_str_ne() {
        let v = StrValidation::default().ne(String::from("Cogito ergo sum"));
        assert_eq!(validate_str(&v, &Value::Str(String::from("Memento mori"))), Ok(()));
        assert_eq!(
            validate_str(&v, &Value::Str(String::from("Cogito ergo sum"))),
            Err(SchemaErr::validation([ValidationErr::Ne(Value::Str(String::from(
                "Cogito ergo sum"
            )))]))
        );
        assert_eq!(
            validate_str(&v, &Value::None),
            Err(SchemaErr::validation([
                ValidationErr::Required,
                ValidationErr::Str,
                ValidationErr::Ne(Value::Str(String::from("Cogito ergo sum")))
            ]))
        );
        assert_eq!(
            validate_str(&v, &num_u_stub()),
            Err(SchemaErr::validation([
                ValidationErr::Str,
                ValidationErr::Ne(Value::Str(String::from("Cogito ergo sum")))
            ]))
        );
    }
}

use araucaria::{
    error::{SchemaErr, ValidationErr},
    validation::email::EmailValidation,
    value::Value,
};
use email_address::EmailAddress;

pub fn validate_email(validation: &EmailValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            if !EmailAddress::is_valid(str_value) {
                base.push(ValidationErr::Email);
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::Email);
        }
        _ => {
            base.push(ValidationErr::Email);
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
    fn test_validate_email_default() {
        let v = EmailValidation::default();
        assert_eq!(validate_email(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Email])));
        assert_eq!(validate_email(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Email])));
    }

    #[test]
    fn test_validate_email_optional() {
        let v = EmailValidation::default().optional();
        assert_eq!(validate_email(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Email])));
        assert_eq!(validate_email(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Email])));
    }

    #[test]
    fn test_validate_email_valid() {
        let v = EmailValidation::default();
        assert_eq!(validate_email(&v, &Value::from("john.lennon@gmail.com")), Ok(()));
        assert_eq!(validate_email(&v, &Value::from("paul_macca@hotmail.com")), Ok(()));
        assert_eq!(validate_email(&v, &Value::from("ringo-starr@outlook.com")), Ok(()));
        assert_eq!(validate_email(&v, &Value::from("GeorgeHarrison@live.com")), Ok(()));
    }

    #[test]
    fn test_validate_email_invalid() {
        let v = EmailValidation::default();
        assert_eq!(validate_email(&v, &Value::from("paullivecom")), Err(SchemaErr::validation([ValidationErr::Email])));
        assert_eq!(validate_email(&v, &Value::from("paullive.com")), Err(SchemaErr::validation([ValidationErr::Email])));
        assert_eq!(validate_email(&v, &Value::from("paul@liv@e.com")), Err(SchemaErr::validation([ValidationErr::Email])));
        assert_eq!(validate_email(&v, &Value::from("live.com")), Err(SchemaErr::validation([ValidationErr::Email])));
        assert_eq!(validate_email(&v, &Value::from("@live.com")), Err(SchemaErr::validation([ValidationErr::Email])));
    }
}

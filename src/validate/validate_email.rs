use araucaria::{
    error::{SchemaErr, ValidationErr},
    validation::EmailValidation,
    value::Value,
};

use crate::utils::email::email_is_valid;

pub fn validate_email(validation: &EmailValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            if !email_is_valid(str_value) {
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
    if !base.is_empty() { Err(SchemaErr::Validation(base)) } else { Ok(()) }
}

#[cfg(test)]
mod tests {
    use araucaria::{
        error::{SchemaErr, ValidationErr},
        validation::EmailValidation,
        value::{Value, stub::u64_stub},
    };

    use super::validate_email;

    #[test]
    fn validate_email_default() {
        let v = EmailValidation::default();
        assert_eq!(validate_email(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Email])));
        assert_eq!(validate_email(&v, &u64_stub()), Err(SchemaErr::validation([ValidationErr::Email])));
    }

    #[test]
    fn validate_email_optional() {
        let v = EmailValidation::default().optional();
        assert_eq!(validate_email(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Email])));
        assert_eq!(validate_email(&v, &u64_stub()), Err(SchemaErr::validation([ValidationErr::Email])));
    }

    #[test]
    fn validate_email_valid() {
        let v = EmailValidation::default();
        assert_eq!(validate_email(&v, &Value::from("john.lennon@gmail.com")), Ok(()));
    }

    #[test]
    fn validate_email_invalid() {
        let v = EmailValidation::default();
        assert_eq!(validate_email(&v, &Value::from("paullivecom")), Err(SchemaErr::validation([ValidationErr::Email])));
    }
}

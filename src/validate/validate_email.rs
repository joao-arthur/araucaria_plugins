use araucaria::{
    error::{SchemaErr, ValidationErr},
    validation::EmailValidation,
    value::Value,
};

use crate::utils::email::email_is_valid;

pub fn validate_email(validation: &EmailValidation, value: &Value, enforce_optional: bool) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            if !email_is_valid(str_value) {
                base.push(ValidationErr::Email);
            }
        }
        Value::None => {
            if enforce_optional {
                if validation.required {
                    base.push(ValidationErr::Required);
                }
                base.push(ValidationErr::Email);
            } else {
                if validation.required {
                    base.push(ValidationErr::Required);
                    base.push(ValidationErr::Email);
                }
            }
        }
        _ => {
            base.push(ValidationErr::Email);
        }
    }
    if !base.is_empty() { Err(SchemaErr::Arr(base)) } else { Ok(()) }
}

#[cfg(test)]
mod tests {
    use araucaria::{
        error::{SchemaErr, ValidationErr},
        validation::EmailValidation,
        value::{Value, stub::u64_stub},
    };

    use super::validate_email;

    const REQUIRED: ValidationErr = ValidationErr::Required;
    const EMAIL: ValidationErr = ValidationErr::Email;

    #[test]
    fn validate_email_default() {
        let v = EmailValidation::default();
        assert_eq!(validate_email(&v, &Value::None, true), Err(SchemaErr::arr([REQUIRED, EMAIL])));
        assert_eq!(validate_email(&v, &Value::None, false), Err(SchemaErr::arr([REQUIRED, EMAIL])));
        assert_eq!(validate_email(&v, &u64_stub(), false), Err(SchemaErr::arr([EMAIL])));
    }

    #[test]
    fn validate_email_optional() {
        let v = EmailValidation::default().optional();
        assert_eq!(validate_email(&v, &Value::None, true), Err(SchemaErr::arr([EMAIL])));
        assert_eq!(validate_email(&v, &Value::None, false), Ok(()));
        assert_eq!(validate_email(&v, &u64_stub(), false), Err(SchemaErr::arr([EMAIL])));
    }

    #[test]
    fn validate_email_valid() {
        let v = EmailValidation::default();
        assert_eq!(validate_email(&v, &Value::from("john.lennon@gmail.com"), false), Ok(()));
    }

    #[test]
    fn validate_email_invalid() {
        let v = EmailValidation::default();
        assert_eq!(validate_email(&v, &Value::from("paullivecom"), false), Err(SchemaErr::arr([EMAIL])));
    }
}

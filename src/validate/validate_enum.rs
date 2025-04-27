use araucaria::{
    error::{SchemaErr, ValidationErr},
    validation::{EnumValidation, EnumValues},
    value::Value,
};

pub fn validate_enum(validation: &EnumValidation, value: &Value) -> Result<(), SchemaErr> {
    match &validation.values {
        EnumValues::USize(usize_enum) => {
            let mut base = vec![];
            match value {
                Value::USize(usize_value) => {
                    if !usize_enum.contains(usize_value) {
                        base.push(ValidationErr::USizeEnum(usize_enum.clone()));
                    }
                }
                Value::None => {
                    if validation.required {
                        base.push(ValidationErr::Required);
                    }
                    base.push(ValidationErr::USizeEnum(usize_enum.clone()));
                }
                _ => {
                    base.push(ValidationErr::USizeEnum(usize_enum.clone()));
                }
            }
            if !base.is_empty() { Err(SchemaErr::Validation(base)) } else { Ok(()) }
        }
        EnumValues::ISize(isize_enum) => {
            let mut base = vec![];
            match value {
                Value::ISize(isize_value) => {
                    if !isize_enum.contains(isize_value) {
                        base.push(ValidationErr::ISizeEnum(isize_enum.clone()));
                    }
                }
                Value::None => {
                    if validation.required {
                        base.push(ValidationErr::Required);
                    }
                    base.push(ValidationErr::ISizeEnum(isize_enum.clone()));
                }
                _ => {
                    base.push(ValidationErr::ISizeEnum(isize_enum.clone()));
                }
            }
            if !base.is_empty() { Err(SchemaErr::Validation(base)) } else { Ok(()) }
        }
        EnumValues::Str(str_enum) => {
            let mut base = vec![];
            match value {
                Value::Str(str_value) => {
                    if !str_enum.contains(str_value) {
                        base.push(ValidationErr::StrEnum(str_enum.clone()));
                    }
                }
                Value::None => {
                    if validation.required {
                        base.push(ValidationErr::Required);
                    }
                    base.push(ValidationErr::StrEnum(str_enum.clone()));
                }
                _ => {
                    base.push(ValidationErr::StrEnum(str_enum.clone()));
                }
            }
            if !base.is_empty() { Err(SchemaErr::Validation(base)) } else { Ok(()) }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        validation::EnumValidation,
        value::{Value, stub::bool_stub},
    };

    use super::validate_enum;

    static USIZE_VALUES: LazyLock<Vec<usize>> = LazyLock::new(|| vec![10, 20, 30, 40, 50]);
    static ISIZE_VALUES: LazyLock<Vec<isize>> = LazyLock::new(|| vec![0, -1, -2, -3, -4, -5]);
    static STRING_VALUES: LazyLock<Vec<String>> =
        LazyLock::new(|| vec!["APPLE".into(), "BANANA".into(), "GRAPE".into(), "ORANGE".into(), "PEACH".into()]);
    const REQUIRED: ValidationErr = ValidationErr::Required;

    #[test]
    fn validate_enum_usize_default() {
        let v = EnumValidation::from(USIZE_VALUES.clone());
        let enum_err = ValidationErr::USizeEnum(USIZE_VALUES.clone());
        assert_eq!(validate_enum(&v, &Value::USize(10)), Ok(()));
        assert_eq!(validate_enum(&v, &Value::USize(3)), Err(SchemaErr::validation([enum_err.clone()])));
        assert_eq!(validate_enum(&v, &Value::None), Err(SchemaErr::validation([REQUIRED, enum_err.clone()])));
        assert_eq!(validate_enum(&v, &bool_stub()), Err(SchemaErr::validation([enum_err.clone()])));
    }

    #[test]
    fn validate_enum_usize_optional() {
        let v = EnumValidation::from(USIZE_VALUES.clone()).optional();
        let enum_err = ValidationErr::USizeEnum(USIZE_VALUES.clone());
        assert_eq!(validate_enum(&v, &Value::USize(10)), Ok(()));
        assert_eq!(validate_enum(&v, &Value::USize(3)), Err(SchemaErr::validation([enum_err.clone()])));
        assert_eq!(validate_enum(&v, &Value::None), Err(SchemaErr::validation([enum_err.clone()])));
        assert_eq!(validate_enum(&v, &bool_stub()), Err(SchemaErr::validation([enum_err.clone()])));
    }
}

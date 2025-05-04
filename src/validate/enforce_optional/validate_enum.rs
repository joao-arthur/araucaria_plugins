use araucaria::{
    error::{SchemaErr, ValidationErr},
    schema::{EnumSchema, EnumValues},
    value::Value,
};

pub fn validate_enum(validation: &EnumSchema, value: &Value) -> Result<(), SchemaErr> {
    match &validation.values {
        EnumValues::USize(usize_enum) => {
            let mut base = vec![];
            match value {
                Value::USize(usize_value) => {
                    if !usize_enum.contains(usize_value) {
                        base.push(ValidationErr::Enumerated(validation.values.clone()));
                    }
                }
                Value::None => {
                    if validation.required {
                        base.push(ValidationErr::Required);
                    }
                    base.push(ValidationErr::Enumerated(validation.values.clone()));
                }
                _ => {
                    base.push(ValidationErr::Enumerated(validation.values.clone()));
                }
            }
            if !base.is_empty() { Err(SchemaErr::Validation(base)) } else { Ok(()) }
        }
        EnumValues::ISize(isize_enum) => {
            let mut base = vec![];
            match value {
                Value::ISize(isize_value) => {
                    if !isize_enum.contains(isize_value) {
                        base.push(ValidationErr::Enumerated(validation.values.clone()));
                    }
                }
                Value::None => {
                    if validation.required {
                        base.push(ValidationErr::Required);
                    }
                    base.push(ValidationErr::Enumerated(validation.values.clone()));
                }
                _ => {
                    base.push(ValidationErr::Enumerated(validation.values.clone()));
                }
            }
            if !base.is_empty() { Err(SchemaErr::Validation(base)) } else { Ok(()) }
        }
        EnumValues::Str(str_enum) => {
            let mut base = vec![];
            match value {
                Value::Str(str_value) => {
                    if !str_enum.contains(str_value) {
                        base.push(ValidationErr::Enumerated(validation.values.clone()));
                    }
                }
                Value::None => {
                    if validation.required {
                        base.push(ValidationErr::Required);
                    }
                    base.push(ValidationErr::Enumerated(validation.values.clone()));
                }
                _ => {
                    base.push(ValidationErr::Enumerated(validation.values.clone()));
                }
            }
            if !base.is_empty() { Err(SchemaErr::Validation(base)) } else { Ok(()) }
        }
    }
}

#[cfg(test)]
mod tests {
    use araucaria::{
        error::{SchemaErr, ValidationErr},
        schema::{EnumSchema, EnumValues},
        value::{Value, stub::bool_stub},
    };

    use super::validate_enum;

    const REQUIRED: ValidationErr = ValidationErr::Required;

    const USIZE_VALUES: [usize; 5] = [10, 20, 30, 40, 50];
    const ISIZE_VALUES: [isize; 6] = [0, -1, -2, -3, -4, -5];
    const STR_VALUES: [&str; 5] = ["APPLE", "BANANA", "GRAPE", "ORANGE", "PEACH"];

    #[test]
    fn validate_enum_usize_default() {
        let v = EnumSchema::from(USIZE_VALUES);
        let enum_err = ValidationErr::Enumerated(EnumValues::from(USIZE_VALUES));
        assert_eq!(validate_enum(&v, &Value::USize(10)), Ok(()));
        assert_eq!(validate_enum(&v, &Value::USize(3)), Err(SchemaErr::from([enum_err.clone()])));
        assert_eq!(validate_enum(&v, &Value::None), Err(SchemaErr::from([REQUIRED, enum_err.clone()])));
        assert_eq!(validate_enum(&v, &bool_stub()), Err(SchemaErr::from([enum_err.clone()])));
    }

    #[test]
    fn validate_enum_usize_optional() {
        let v = EnumSchema::from(USIZE_VALUES).optional();
        let enum_err = ValidationErr::Enumerated(EnumValues::from(USIZE_VALUES));
        assert_eq!(validate_enum(&v, &Value::USize(10)), Ok(()));
        assert_eq!(validate_enum(&v, &Value::USize(3)), Err(SchemaErr::from([enum_err.clone()])));
        assert_eq!(validate_enum(&v, &Value::None), Err(SchemaErr::from([enum_err.clone()])));
        assert_eq!(validate_enum(&v, &bool_stub()), Err(SchemaErr::from([enum_err.clone()])));
    }

    #[test]
    fn validate_enum_isize_default() {
        let v = EnumSchema::from(ISIZE_VALUES.clone());
        let enum_err = ValidationErr::Enumerated(EnumValues::from(ISIZE_VALUES));
        assert_eq!(validate_enum(&v, &Value::ISize(-3)), Ok(()));
        assert_eq!(validate_enum(&v, &Value::ISize(1)), Err(SchemaErr::from([enum_err.clone()])));
        assert_eq!(validate_enum(&v, &Value::None), Err(SchemaErr::from([REQUIRED, enum_err.clone()])));
        assert_eq!(validate_enum(&v, &bool_stub()), Err(SchemaErr::from([enum_err.clone()])));
    }

    #[test]
    fn validate_enum_isize_optional() {
        let v = EnumSchema::from(ISIZE_VALUES.clone()).optional();
        let enum_err = ValidationErr::Enumerated(EnumValues::from(ISIZE_VALUES));
        assert_eq!(validate_enum(&v, &Value::ISize(-3)), Ok(()));
        assert_eq!(validate_enum(&v, &Value::ISize(1)), Err(SchemaErr::from([enum_err.clone()])));
        assert_eq!(validate_enum(&v, &Value::None), Err(SchemaErr::from([enum_err.clone()])));
        assert_eq!(validate_enum(&v, &bool_stub()), Err(SchemaErr::from([enum_err.clone()])));
    }

    #[test]
    fn validate_enum_str_default() {
        let v = EnumSchema::from(STR_VALUES.clone());
        let enum_err = ValidationErr::Enumerated(EnumValues::from(STR_VALUES));
        assert_eq!(validate_enum(&v, &Value::from("GRAPE")), Ok(()));
        assert_eq!(validate_enum(&v, &Value::from("TOMATO")), Err(SchemaErr::from([enum_err.clone()])));
        assert_eq!(validate_enum(&v, &Value::None), Err(SchemaErr::from([REQUIRED, enum_err.clone()])));
        assert_eq!(validate_enum(&v, &bool_stub()), Err(SchemaErr::from([enum_err.clone()])));
    }

    #[test]
    fn validate_enum_str_optional() {
        let v = EnumSchema::from(STR_VALUES.clone()).optional();
        let enum_err = ValidationErr::Enumerated(EnumValues::from(STR_VALUES));
        assert_eq!(validate_enum(&v, &Value::from("GRAPE")), Ok(()));
        assert_eq!(validate_enum(&v, &Value::from("TOMATO")), Err(SchemaErr::from([enum_err.clone()])));
        assert_eq!(validate_enum(&v, &Value::None), Err(SchemaErr::from([enum_err.clone()])));
        assert_eq!(validate_enum(&v, &bool_stub()), Err(SchemaErr::from([enum_err.clone()])));
    }
}

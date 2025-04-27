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


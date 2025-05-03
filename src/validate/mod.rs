use araucaria::{error::SchemaErr, validation::Validation, value::Value};

mod enforce_optional;
mod enforce_required;

pub fn validate(validation: &Validation, value: &Value) -> Result<(), SchemaErr> {
    let result = enforce_required::validate(validation, value);

    match result {
        Ok(_) => Ok(()),
        Err(_) => enforce_optional::validate(validation, value),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        validation::{BoolValidation, F64Validation, ObjValidation, StrValidation, U64Validation, Validation},
        value::Value,
    };

    use super::validate;

    const REQUIRED: ValidationErr = ValidationErr::Required;
    const U64: ValidationErr = ValidationErr::U64;
    const F64: ValidationErr = ValidationErr::F64;
    const BOOL: ValidationErr = ValidationErr::Bool;
    const STR: ValidationErr = ValidationErr::Str;

    #[test]
    fn validate_ok() {
        assert_eq!(validate(&Validation::Bool(BoolValidation::default()), &Value::Bool(false)), Ok(()));
    }

    #[test]
    fn validate_err() {
        assert_eq!(validate(&Validation::Bool(BoolValidation::default()), &Value::U64(217)), Err(SchemaErr::validation([BOOL])));
    }

    #[test]
    fn validate_missing_required_field() {
        let v = Validation::Obj(ObjValidation::default().validation(BTreeMap::from([
            ("name".into(), Validation::Str(StrValidation::default())),
            ("age".into(), Validation::U64(U64Validation::default().optional())),
            ("height".into(), Validation::F64(F64Validation::default().optional())),
        ])));
        let value = Value::Obj(BTreeMap::from([("height".into(), Value::from(1.75))]));
        let err = SchemaErr::obj([("name".into(), SchemaErr::validation([REQUIRED, STR])), ("age".into(), SchemaErr::validation([U64]))]);
        assert_eq!(validate(&v, &value), Err(err));
    }

    #[test]
    fn validate_missing_optional_field() {
        let v = Validation::Obj(ObjValidation::default().validation(BTreeMap::from([
            ("name".into(), Validation::Str(StrValidation::default())),
            ("age".into(), Validation::U64(U64Validation::default().optional())),
            ("height".into(), Validation::F64(F64Validation::default().optional())),
        ])));
        let value = Value::Obj(BTreeMap::from([("name".into(), Value::from("John"))]));
        assert_eq!(validate(&v, &value), Ok(()));
    }

    #[test]
    fn validate_missing_required_obj() {
        let v = Validation::Obj(ObjValidation::default().validation(BTreeMap::from([
            (
                "user".into(),
                Validation::Obj(ObjValidation::default().validation(BTreeMap::from([
                    ("name".into(), Validation::Str(StrValidation::default())),
                    ("age".into(), Validation::U64(U64Validation::default().optional())),
                    ("height".into(), Validation::F64(F64Validation::default().optional())),
                ]))),
            ),
            ("version".into(), Validation::U64(U64Validation::default().optional())),
        ])));
        let value = Value::Obj(BTreeMap::from([("version".into(), Value::U64(2))]));
        let err = SchemaErr::obj([(
            "user".into(),
            SchemaErr::obj([
                ("name".into(), SchemaErr::validation([REQUIRED, STR])),
                ("age".into(), SchemaErr::validation([U64])),
                ("height".into(), SchemaErr::validation([F64])),
            ]),
        )]);
        assert_eq!(validate(&v, &value), Err(err));
    }

    #[test]
    fn validate_missing_optional_obj() {
        let v = Validation::Obj(ObjValidation::default().validation(BTreeMap::from([
            (
                "user".into(),
                Validation::Obj(ObjValidation::default().validation(BTreeMap::from([
                    ("name".into(), Validation::Str(StrValidation::default())),
                    ("age".into(), Validation::U64(U64Validation::default().optional())),
                    ("height".into(), Validation::F64(F64Validation::default().optional())),
                ]))),
            ),
            ("version".into(), Validation::U64(U64Validation::default().optional())),
        ])));
        let value = Value::Obj(BTreeMap::from([("user".into(), Value::Obj(BTreeMap::from([("name".into(), Value::from("John"))])))]));
        assert_eq!(validate(&v, &value), Ok(()));
    }
}

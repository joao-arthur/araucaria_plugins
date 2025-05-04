use araucaria::{error::SchemaErr, schema::Schema, value::Value};

mod enforce_optional;
mod enforce_required;

pub fn validate(validation: &Schema, value: &Value) -> Result<(), SchemaErr> {
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
        schema::{BoolSchema, F64Schema, ObjSchema, StrSchema, U64Schema, Schema},
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
        assert_eq!(validate(&Schema::Bool(BoolSchema::default()), &Value::Bool(false)), Ok(()));
    }

    #[test]
    fn validate_err() {
        assert_eq!(validate(&Schema::Bool(BoolSchema::default()), &Value::U64(217)), Err(SchemaErr::from([BOOL])));
    }

    #[test]
    fn validate_missing_required_field() {
        let v = Schema::Obj(ObjSchema::from(BTreeMap::from([
            ("name".into(), Schema::Str(StrSchema::default())),
            ("age".into(), Schema::U64(U64Schema::default().optional())),
            ("height".into(), Schema::F64(F64Schema::default().optional())),
        ])));
        let value = Value::Obj(BTreeMap::from([("height".into(), Value::from(1.75))]));
        let err = SchemaErr::from([("name".into(), SchemaErr::from([REQUIRED, STR])), ("age".into(), SchemaErr::from([U64]))]);
        assert_eq!(validate(&v, &value), Err(err));
    }

    #[test]
    fn validate_missing_optional_field() {
        let v = Schema::Obj(ObjSchema::from(BTreeMap::from([
            ("name".into(), Schema::Str(StrSchema::default())),
            ("age".into(), Schema::U64(U64Schema::default().optional())),
            ("height".into(), Schema::F64(F64Schema::default().optional())),
        ])));
        let value = Value::Obj(BTreeMap::from([("name".into(), Value::from("John"))]));
        assert_eq!(validate(&v, &value), Ok(()));
    }

    #[test]
    fn validate_missing_required_obj() {
        let v = Schema::Obj(ObjSchema::from(BTreeMap::from([
            (
                "user".into(),
                Schema::Obj(ObjSchema::from(BTreeMap::from([
                    ("name".into(), Schema::Str(StrSchema::default())),
                    ("age".into(), Schema::U64(U64Schema::default().optional())),
                    ("height".into(), Schema::F64(F64Schema::default().optional())),
                ]))),
            ),
            ("version".into(), Schema::U64(U64Schema::default().optional())),
        ])));
        let value = Value::Obj(BTreeMap::from([("version".into(), Value::U64(2))]));
        let err = SchemaErr::from([(
            "user".into(),
            SchemaErr::from([
                ("name".into(), SchemaErr::from([REQUIRED, STR])),
                ("age".into(), SchemaErr::from([U64])),
                ("height".into(), SchemaErr::from([F64])),
            ]),
        )]);
        assert_eq!(validate(&v, &value), Err(err));
    }

    #[test]
    fn validate_missing_optional_obj() {
        let v = Schema::Obj(ObjSchema::from(BTreeMap::from([
            (
                "user".into(),
                Schema::Obj(ObjSchema::from(BTreeMap::from([
                    ("name".into(), Schema::Str(StrSchema::default())),
                    ("age".into(), Schema::U64(U64Schema::default().optional())),
                    ("height".into(), Schema::F64(F64Schema::default().optional())),
                ]))),
            ),
            ("version".into(), Schema::U64(U64Schema::default().optional())),
        ])));
        let value = Value::Obj(BTreeMap::from([("user".into(), Value::Obj(BTreeMap::from([("name".into(), Value::from("John"))])))]));
        assert_eq!(validate(&v, &value), Ok(()));
    }
}

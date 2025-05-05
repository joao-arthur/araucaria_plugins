use araucaria::{
    locale::{Locale, localize_schema_err},
    schema::Schema,
};
use serde::de::DeserializeOwned;

use crate::{
    serialize::{SchemaErrLocale, to_schema_err_locale},
    validate::validate,
};

use super::value_from_json_and_schema;

pub fn deserialize_from_json<T>(json: serde_json::Value, schema: &Schema, locale: &Locale) -> Result<T, SchemaErrLocale>
where
    T: DeserializeOwned,
{
    let internal_value = value_from_json_and_schema(&json, schema);
    match validate(schema, &internal_value) {
        Ok(()) => Ok(serde_json::from_value(json).unwrap()),
        Err(schema_err) => {
            let schema_err_locale = localize_schema_err(&schema_err, &locale);
            let schema_err_locale_serializable = to_schema_err_locale(schema_err_locale);
            Err(schema_err_locale_serializable)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, sync::LazyLock};

    use araucaria::schema::{BoolSchema, ObjSchema, Schema, StrSchema, U64Schema};
    use serde::Deserialize;
    use serde_json::json;

    use crate::{locale::locale_pt_long, serialize::SchemaErrLocale};

    use super::deserialize_from_json;

    #[derive(Debug, PartialEq, Deserialize)]
    struct User {
        name: String,
        score: u64,
        is_active: Option<bool>,
    }

    pub static USER_SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
        Schema::from(ObjSchema::from([
            ("name".into(), Schema::from(StrSchema::default())),
            ("score".into(), Schema::from(U64Schema::default())),
            ("is_active".into(), Schema::from(BoolSchema::default().optional())),
        ]))
    });

    #[test]
    fn deserialize_struct_missing_optional() {
        let locale = locale_pt_long();
        let json = json!({
            "name": "John Lennon",
            "score": 92,
        });
        let instance = User { name: "John Lennon".into(), score: 92, is_active: None };
        assert_eq!(deserialize_from_json(json, &USER_SCHEMA, &locale), Ok(instance));
    }

    #[test]
    fn deserialize_struct_missing_required() {
        let locale = locale_pt_long();
        let json = json!({
            "name": "John Lennon",
            "is_active": true
        });
        assert_eq!(
            deserialize_from_json::<User>(json, &USER_SCHEMA, &locale),
            Err(SchemaErrLocale::Obj(BTreeMap::from([(
                "score".into(),
                SchemaErrLocale::Validation(vec!["É obrigatório".into(), "Deve ser um número inteiro sem sinal de 64 bits".into()])
            )])))
        );
    }
}

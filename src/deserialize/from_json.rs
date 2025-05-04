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
    use std::sync::LazyLock;

    use araucaria::schema::{BoolSchema, F64Schema, I64Schema, ISizeSchema, ObjSchema, Schema, StrSchema, U64Schema, USizeSchema};
    use serde::Deserialize;
    use serde_json::json;

    use crate::locale::locale_pt_long;

    use super::deserialize_from_json;

    #[derive(Debug, PartialEq, Deserialize)]
    struct NumberValues {
        u64_field: u64,
        i64_field: i64,
        f64_field: f64,
        usize_field: usize,
        isize_field: isize,
    }

    pub static FOO_BAR_SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
        Schema::from(ObjSchema::from([
            ("u64_field".into(), Schema::from(U64Schema::default())),
            ("i64_field".into(), Schema::from(I64Schema::default())),
            ("f64_field".into(), Schema::from(F64Schema::default())),
            ("usize_field".into(), Schema::from(USizeSchema::default())),
            ("isize_field".into(), Schema::from(ISizeSchema::default())),
        ]))
    });

    #[test]
    fn deserialize_struct() {
        let locale = locale_pt_long();
        let json = json!({
            "u64_field": 83,
            "i64_field": -12,
            "f64_field": -3.75,
            "usize_field": 27,
            "isize_field": -34,
        });
        let instance = NumberValues { u64_field: 83, i64_field: -12, f64_field: -3.75, usize_field: 27, isize_field: -34 };
        assert_eq!(deserialize_from_json(json, &FOO_BAR_SCHEMA, &locale), Ok(instance));
    }
}

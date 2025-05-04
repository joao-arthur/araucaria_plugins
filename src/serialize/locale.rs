use serde::{Serialize, Serializer};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaErrLocale {
    Validation(Vec<String>),
    Arr(Vec<SchemaErrLocale>),
    Obj(BTreeMap<String, SchemaErrLocale>),
}

impl Serialize for SchemaErrLocale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SchemaErrLocale::Validation(vec) => vec.serialize(serializer),
            SchemaErrLocale::Arr(vec) => vec.serialize(serializer),
            SchemaErrLocale::Obj(map) => map.serialize(serializer),
        }
    }
}

pub fn to_schema_localized_err(value: araucaria::locale::SchemaErrLocale) -> SchemaErrLocale {
    match value {
        araucaria::locale::SchemaErrLocale::Validation(value) => SchemaErrLocale::Validation(value.into_iter().collect()),
        araucaria::locale::SchemaErrLocale::Arr(value) => SchemaErrLocale::Arr(value.into_iter().map(to_schema_localized_err).collect()),
        araucaria::locale::SchemaErrLocale::Obj(value) => {
            SchemaErrLocale::Obj(value.into_iter().map(|(k, v)| (k.clone(), to_schema_localized_err(v))).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{SchemaErrLocale, to_schema_localized_err};

    #[test]
    fn araucaria_schema_localized_arr_to_schema_localized_err() {
        let araucaria_err = araucaria::locale::SchemaErrLocale::Obj(BTreeMap::from([
            ("name".into(), araucaria::locale::SchemaErrLocale::from(["Deve ser uma string".to_string()])),
            ("birthdate".into(), araucaria::locale::SchemaErrLocale::from(["Deve ser uma string".to_string()])),
            ("bands".into(), araucaria::locale::SchemaErrLocale::from(["Deve ser uma string".to_string()])),
        ]));
        let err = SchemaErrLocale::Obj(BTreeMap::from([
            ("name".into(), SchemaErrLocale::Validation(vec!["Deve ser uma string".into()])),
            ("birthdate".into(), SchemaErrLocale::Validation(vec!["Deve ser uma string".into()])),
            ("bands".into(), SchemaErrLocale::Validation(vec!["Deve ser uma string".into()])),
        ]));
        assert_eq!(to_schema_localized_err(araucaria_err), err);
    }

    #[test]
    fn serialize_schema_localized_err_obj() {
        let err_name = SchemaErrLocale::Obj(BTreeMap::from([(
            "name".into(),
            SchemaErrLocale::Validation(vec!["É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "Paul McCartney""#.into()]),
        )]));
        let err_birthdate = SchemaErrLocale::Obj(BTreeMap::from([(
            "birthdate".into(),
            SchemaErrLocale::Validation(vec!["É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "1942-06-18""#.into()]),
        )]));
        let err_alive = SchemaErrLocale::Obj(BTreeMap::from([(
            "alive".into(),
            SchemaErrLocale::Validation(vec!["É obrigatório".into(), "Deve ser um booleano".into(), "Deve ser igual a true".into()]),
        )]));
        let err_bands = SchemaErrLocale::Obj(BTreeMap::from([(
            "bands".into(),
            SchemaErrLocale::Validation(vec!["É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "The Beatles""#.into()]),
        )]));
        assert_eq!(
            serde_json::to_string(&err_name).unwrap(),
            r#"{"name":["É obrigatório","Deve ser uma string","Deve ser igual a \"Paul McCartney\""]}"#.to_string()
        );
        assert_eq!(
            serde_json::to_string(&err_birthdate).unwrap(),
            r#"{"birthdate":["É obrigatório","Deve ser uma string","Deve ser igual a \"1942-06-18\""]}"#.to_string()
        );
        assert_eq!(
            serde_json::to_string(&err_alive).unwrap(),
            r#"{"alive":["É obrigatório","Deve ser um booleano","Deve ser igual a true"]}"#.to_string()
        );
        assert_eq!(
            serde_json::to_string(&err_bands).unwrap(),
            r#"{"bands":["É obrigatório","Deve ser uma string","Deve ser igual a \"The Beatles\""]}"#.to_string()
        );
    }

    #[test]
    fn serialize_schema_localized_err_obj_order() {
        let err = SchemaErrLocale::Obj(BTreeMap::from([
            ("name".into(), SchemaErrLocale::Validation(vec!["Deve ser uma string".into()])),
            ("birthdate".into(), SchemaErrLocale::Validation(vec!["Deve ser uma string".into()])),
            ("bands".into(), SchemaErrLocale::Validation(vec!["Deve ser uma string".into()])),
        ]));
        assert_eq!(
            serde_json::to_string(&err).unwrap(),
            r#"{"bands":["Deve ser uma string"],"birthdate":["Deve ser uma string"],"name":["Deve ser uma string"]}"#.to_string()
        );
    }
}

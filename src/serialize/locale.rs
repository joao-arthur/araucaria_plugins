use serde::{Serialize, Serializer};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaLocalizedErr {
    Validation(Vec<String>),
    Arr(Vec<SchemaLocalizedErr>),
    Obj(BTreeMap<String, SchemaLocalizedErr>),
}

impl Serialize for SchemaLocalizedErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SchemaLocalizedErr::Validation(vec) => vec.serialize(serializer),
            SchemaLocalizedErr::Arr(vec) => vec.serialize(serializer),
            SchemaLocalizedErr::Obj(map) => map.serialize(serializer),
        }
    }
}

pub fn to_schema_localized_err(value: araucaria::locale::SchemaLocalizedErr) -> SchemaLocalizedErr {
    match value {
        araucaria::locale::SchemaLocalizedErr::Validation(value) => SchemaLocalizedErr::Validation(value.into_iter().collect()),
        araucaria::locale::SchemaLocalizedErr::Arr(value) => SchemaLocalizedErr::Arr(value.into_iter().map(to_schema_localized_err).collect()),
        araucaria::locale::SchemaLocalizedErr::Obj(value) => {
            SchemaLocalizedErr::Obj(value.into_iter().map(|(k, v)| (k.clone(), to_schema_localized_err(v))).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{SchemaLocalizedErr, to_schema_localized_err};

    #[test]
    fn araucaria_schema_localized_arr_to_schema_localized_err() {
        let araucaria_err = araucaria::locale::SchemaLocalizedErr::Obj(BTreeMap::from([
            ("name".into(), araucaria::locale::SchemaLocalizedErr::Validation(vec!["Deve ser uma string".into()])),
            ("birthdate".into(), araucaria::locale::SchemaLocalizedErr::Validation(vec!["Deve ser uma string".into()])),
            ("bands".into(), araucaria::locale::SchemaLocalizedErr::Validation(vec!["Deve ser uma string".into()])),
        ]));
        let err = SchemaLocalizedErr::Obj(BTreeMap::from([
            ("name".into(), SchemaLocalizedErr::Validation(vec!["Deve ser uma string".into()])),
            ("birthdate".into(), SchemaLocalizedErr::Validation(vec!["Deve ser uma string".into()])),
            ("bands".into(), SchemaLocalizedErr::Validation(vec!["Deve ser uma string".into()])),
        ]));
        assert_eq!(to_schema_localized_err(araucaria_err), err);
    }

    #[test]
    fn serialize_schema_localized_err_obj() {
        let err_name = SchemaLocalizedErr::Obj(BTreeMap::from([(
            "name".into(),
            SchemaLocalizedErr::Validation(vec!["É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "Paul McCartney""#.into()]),
        )]));
        let err_birthdate = SchemaLocalizedErr::Obj(BTreeMap::from([(
            "birthdate".into(),
            SchemaLocalizedErr::Validation(vec!["É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "1942-06-18""#.into()]),
        )]));
        let err_alive = SchemaLocalizedErr::Obj(BTreeMap::from([(
            "alive".into(),
            SchemaLocalizedErr::Validation(vec!["É obrigatório".into(), "Deve ser um booleano".into(), "Deve ser igual a true".into()]),
        )]));
        let err_bands = SchemaLocalizedErr::Obj(BTreeMap::from([(
            "bands".into(),
            SchemaLocalizedErr::Validation(vec!["É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "The Beatles""#.into()]),
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
        let err = SchemaLocalizedErr::Obj(BTreeMap::from([
            ("name".into(), SchemaLocalizedErr::Validation(vec!["Deve ser uma string".into()])),
            ("birthdate".into(), SchemaLocalizedErr::Validation(vec!["Deve ser uma string".into()])),
            ("bands".into(), SchemaLocalizedErr::Validation(vec!["Deve ser uma string".into()])),
        ]));
        assert_eq!(
            serde_json::to_string(&err).unwrap(),
            r#"{"bands":["Deve ser uma string"],"birthdate":["Deve ser uma string"],"name":["Deve ser uma string"]}"#.to_string()
        );
    }
}

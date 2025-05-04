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
    fn araucaria_schema_localized_arr_to_schema_localized_err_validation() {
        let araucaria_err = araucaria::locale::SchemaErrLocale::from(["required".to_string(), "str".to_string()]);
        let err = SchemaErrLocale::Validation(vec!["required".to_string(), "str".into()]);
        assert_eq!(to_schema_localized_err(araucaria_err), err);
    }

    #[test]
    fn araucaria_schema_localized_arr_to_schema_localized_err_arr() {
        let araucaria_err = araucaria::locale::SchemaErrLocale::from([
            araucaria::locale::SchemaErrLocale::from([
                araucaria::locale::SchemaErrLocale::from(["required".to_string(), "str".to_string()]),
                araucaria::locale::SchemaErrLocale::from(["required".to_string(), "str".to_string()]),
            ]),
            araucaria::locale::SchemaErrLocale::from([
                araucaria::locale::SchemaErrLocale::from(["str".to_string()]),
                araucaria::locale::SchemaErrLocale::from(["str".to_string()]),
            ]),
        ]);
        let err = SchemaErrLocale::Arr(vec![
            SchemaErrLocale::Arr(vec![
                SchemaErrLocale::Validation(vec!["required".to_string(), "str".to_string()]),
                SchemaErrLocale::Validation(vec!["required".to_string(), "str".to_string()]),
            ]),
            SchemaErrLocale::Arr(vec![SchemaErrLocale::Validation(vec!["str".to_string()]), SchemaErrLocale::Validation(vec!["str".to_string()])]),
        ]);
        assert_eq!(to_schema_localized_err(araucaria_err), err);
    }

    #[test]
    fn araucaria_schema_localized_arr_to_schema_localized_err_obj() {
        let araucaria_err = araucaria::locale::SchemaErrLocale::Obj(BTreeMap::from([
            ("name".into(), araucaria::locale::SchemaErrLocale::from(["str".to_string()])),
            ("birthdate".into(), araucaria::locale::SchemaErrLocale::from(["str".to_string()])),
            ("bands".into(), araucaria::locale::SchemaErrLocale::from(["str".to_string()])),
        ]));
        let err = SchemaErrLocale::Obj(BTreeMap::from([
            ("name".into(), SchemaErrLocale::Validation(vec!["str".into()])),
            ("birthdate".into(), SchemaErrLocale::Validation(vec!["str".into()])),
            ("bands".into(), SchemaErrLocale::Validation(vec!["str".into()])),
        ]));
        assert_eq!(to_schema_localized_err(araucaria_err), err);
    }

    #[test]
    fn serialize_schema_localized_err_validation() {
        let validation_name = SchemaErrLocale::Validation(vec!["str".into(), r#"== "Paul McCartney""#.into()]);
        let validation_birthdate = SchemaErrLocale::Validation(vec!["str".into(), r#"== "1942-06-18""#.into()]);
        let validation_alive = SchemaErrLocale::Validation(vec!["bool".into(), "== true".into()]);
        let validation_band = SchemaErrLocale::Validation(vec!["str".into(), r#"== "The Beatles""#.into()]);

        assert_eq!(serde_json::to_string(&validation_name).unwrap(), r#"["str","== \"Paul McCartney\""]"#.to_string());
        assert_eq!(serde_json::to_string(&validation_birthdate).unwrap(), r#"["str","== \"1942-06-18\""]"#.to_string());
        assert_eq!(serde_json::to_string(&validation_alive).unwrap(), r#"["bool","== true"]"#.to_string());
        assert_eq!(serde_json::to_string(&validation_band).unwrap(), r#"["str","== \"The Beatles\""]"#.to_string());
    }

    #[test]
    fn serialize_schema_localized_err_arr() {
        let err = SchemaErrLocale::Arr(vec![
            SchemaErrLocale::Obj(BTreeMap::from([
                ("name".into(), SchemaErrLocale::Validation(vec!["str".into()])),
                ("birthdate".into(), SchemaErrLocale::Validation(vec!["str".into()])),
            ])),
            SchemaErrLocale::Arr(vec![SchemaErrLocale::Validation(vec!["bool".into()]), SchemaErrLocale::Validation(vec!["u64".into()])]),
        ]);
        assert_eq!(serde_json::to_string(&err).unwrap(), r#"[{"birthdate":["str"],"name":["str"]},[["bool"],["u64"]]]"#.to_string());
    }

    #[test]
    fn serialize_schema_localized_err_obj() {
        let validation_name = SchemaErrLocale::Validation(vec!["str".into(), r#"== "Paul McCartney""#.into()]);
        let validation_birthdate = SchemaErrLocale::Validation(vec!["str".into(), r#"== "1942-06-18""#.into()]);
        let validation_alive = SchemaErrLocale::Validation(vec!["bool".into(), "== true".into()]);
        let validation_band = SchemaErrLocale::Validation(vec!["str".into(), r#"== "The Beatles""#.into()]);

        let err_name = SchemaErrLocale::Obj(BTreeMap::from([("name".into(), validation_name)]));
        let err_birthdate = SchemaErrLocale::Obj(BTreeMap::from([("birthdate".into(), validation_birthdate)]));
        let err_alive = SchemaErrLocale::Obj(BTreeMap::from([("alive".into(), validation_alive)]));
        let err_band = SchemaErrLocale::Obj(BTreeMap::from([("band".into(), validation_band)]));

        assert_eq!(serde_json::to_string(&err_name).unwrap(), r#"{"name":["str","== \"Paul McCartney\""]}"#.to_string());
        assert_eq!(serde_json::to_string(&err_birthdate).unwrap(), r#"{"birthdate":["str","== \"1942-06-18\""]}"#.to_string());
        assert_eq!(serde_json::to_string(&err_alive).unwrap(), r#"{"alive":["bool","== true"]}"#.to_string());
        assert_eq!(serde_json::to_string(&err_band).unwrap(), r#"{"band":["str","== \"The Beatles\""]}"#.to_string());
    }

    #[test]
    fn serialize_schema_localized_err_obj_order() {
        let err = SchemaErrLocale::Obj(BTreeMap::from([
            ("name".into(), SchemaErrLocale::Validation(vec!["str".into()])),
            ("birthdate".into(), SchemaErrLocale::Validation(vec!["str".into()])),
            ("bands".into(), SchemaErrLocale::Validation(vec!["str".into()])),
        ]));
        assert_eq!(serde_json::to_string(&err).unwrap(), r#"{"bands":["str"],"birthdate":["str"],"name":["str"]}"#.to_string());
    }
}

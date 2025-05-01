use std::collections::BTreeMap;
use serde::{Serialize, Serializer};

pub struct Locale {
    required: String,
    u64: String,
    i64: String,
    f64: String,
    usize: String,
    isize: String,
    bool: String,
    str: String,
    email: String,
    date: String,
    time: String,
    date_time: String,
    eq: String,
    ne: String,
    gt: String,
    lt: String,
    ge: String,
    le: String,
    btwn: String,
    eq_field: String,
    ne_field: String,
    gt_field: String,
    lt_field: String,
    ge_field: String,
    le_field: String,
    bytes_len_eq: String,
    bytes_len_ne: String,
    bytes_len_gt: String,
    bytes_len_ge: String,
    bytes_len_lt: String,
    bytes_len_le: String,
    bytes_len_btwn: String,
    chars_len_eq: String,
    chars_len_ne: String,
    chars_len_gt: String,
    chars_len_ge: String,
    chars_len_lt: String,
    chars_len_le: String,
    chars_len_btwn: String,
    graphemes_len_eq: String,
    graphemes_len_ne: String,
    graphemes_len_gt: String,
    graphemes_len_ge: String,
    graphemes_len_lt: String,
    graphemes_len_le: String,
    graphemes_len_btwn: String,
    lowercase_len_eq: String,
    lowercase_len_ne: String,
    lowercase_len_gt: String,
    lowercase_len_ge: String,
    lowercase_len_lt: String,
    lowercase_len_le: String,
    lowercase_len_btwn: String,
    uppercase_len_eq: String,
    uppercase_len_ne: String,
    uppercase_len_gt: String,
    uppercase_len_ge: String,
    uppercase_len_lt: String,
    uppercase_len_le: String,
    uppercase_len_btwn: String,
    number_len_eq: String,
    number_len_ne: String,
    number_len_gt: String,
    number_len_ge: String,
    number_len_lt: String,
    number_len_le: String,
    number_len_btwn: String,
    symbols_eq: String,
    symbols_ne: String,
    symbols_gt: String,
    symbols_ge: String,
    symbols_lt: String,
    symbols_le: String,
    symbols_btwn: String,
    enumerated: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaLocalizedErr {
    Arr(Vec<String>),
    Obj(BTreeMap<String, SchemaLocalizedErr>),
}

impl Serialize for SchemaLocalizedErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SchemaLocalizedErr::Arr(vec) => vec.serialize(serializer),
            SchemaLocalizedErr::Obj(map) => map.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{SchemaLocalizedErr, Locale};

    #[test]
    fn serialize_schema_localized_err_obj() {
        let err = SchemaLocalizedErr::Obj(BTreeMap::from([
            (
                "name".into(),
                SchemaLocalizedErr::Arr(vec!["É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "Paul McCartney""#.into()]),
            ),
            (
                "birthdate".into(),
                SchemaLocalizedErr::Arr(vec!["É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "1942-06-18""#.into()]),
            ),
            ("alive".into(), SchemaLocalizedErr::Arr(vec!["É obrigatório".into(), "Deve ser um booleano".into(), "Deve ser igual a true".into()])),
            (
                "bands".into(),
                SchemaLocalizedErr::Arr(vec!["É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "The Beatles""#.into()]),
            ),
        ]));
        assert_eq!(
            serde_json::to_string(&err).unwrap(),
            r#"{"alive":["É obrigatório","Deve ser um booleano","Deve ser igual a true"],"bands":["É obrigatório","Deve ser uma string","Deve ser igual a \"The Beatles\""],"birthdate":["É obrigatório","Deve ser uma string","Deve ser igual a \"1942-06-18\""],"name":["É obrigatório","Deve ser uma string","Deve ser igual a \"Paul McCartney\""]}"#.to_string()
        );
    }
}


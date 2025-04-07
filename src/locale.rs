use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

use araucaria::{
    error::{SchemaErr, ValidationErr}, operation::Operation, value::value_to_string
};

pub struct Locale {
    required: String,
    num_u: String,
    num_i: String,
    num_f: String,
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
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum SchemaLocalizedErr {
    Arr(Vec<String>),
    Obj(HashMap<String, SchemaLocalizedErr>),
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

pub fn schema_err_to_locale(err: &SchemaErr, locale: &Locale) -> SchemaLocalizedErr {
    match err {
        SchemaErr::Validation(arr) => SchemaLocalizedErr::Arr(arr.iter().map(|item| validation_err_to_locale(item, locale)).collect()),
        SchemaErr::Obj(obj) => {
            let mut result: HashMap<String, SchemaLocalizedErr> = HashMap::new();
            for (key, item) in obj {
                result.insert(key.clone(), schema_err_to_locale(item, locale));
            }
            SchemaLocalizedErr::Obj(result)
        }
    }
}

pub fn validation_err_to_locale(error: &ValidationErr, locale: &Locale) -> String {
    match error {
        ValidationErr::Required => locale.required.clone(),
        ValidationErr::NumU => locale.num_u.clone(),
        ValidationErr::NumI => locale.num_i.clone(),
        ValidationErr::NumF => locale.num_f.clone(),
        ValidationErr::Bool => locale.bool.clone(),
        ValidationErr::Str => locale.str.clone(),
        ValidationErr::Email => locale.email.clone(),
        ValidationErr::Date => locale.date.clone(),
        ValidationErr::Time => locale.time.clone(),
        ValidationErr::DateTime => locale.date_time.clone(),
        ValidationErr::Eq(value) => locale.eq.replace("%value%", &value_to_string(&value)),
        ValidationErr::Ne(value) => locale.ne.replace("%value%", &value_to_string(&value)),
        ValidationErr::Gt(value) => locale.gt.replace("%value%", &value_to_string(&value)),
        ValidationErr::Lt(value) => locale.lt.replace("%value%", &value_to_string(&value)),
        ValidationErr::Ge(value) => locale.ge.replace("%value%", &value_to_string(&value)),
        ValidationErr::Le(value) => locale.le.replace("%value%", &value_to_string(&value)),
        ValidationErr::BytesLen(operation) => {
            match operation {
                Operation::Eq(v) => locale.bytes_len_eq.replace("%value%", &v.to_string()),
                Operation::Ne(v) => locale.bytes_len_ne.replace("%value%", &v.to_string()),
                Operation::Gt(v) => locale.bytes_len_gt.replace("%value%", &v.to_string()),
                Operation::Ge(v) => locale.bytes_len_ge.replace("%value%", &v.to_string()),
                Operation::Lt(v) => locale.bytes_len_lt.replace("%value%", &v.to_string()),
                Operation::Le(v) => locale.bytes_len_le.replace("%value%", &v.to_string()),
                Operation::Btwn(a, b) => locale.bytes_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
            }
        },
        ValidationErr::CharsLen(operation) => {
            match operation {
                Operation::Eq(v) => locale.chars_len_eq.replace("%value%", &v.to_string()),
                Operation::Ne(v) => locale.chars_len_ne.replace("%value%", &v.to_string()),
                Operation::Gt(v) => locale.chars_len_gt.replace("%value%", &v.to_string()),
                Operation::Ge(v) => locale.chars_len_ge.replace("%value%", &v.to_string()),
                Operation::Lt(v) => locale.chars_len_lt.replace("%value%", &v.to_string()),
                Operation::Le(v) => locale.chars_len_le.replace("%value%", &v.to_string()),
                Operation::Btwn(a, b) => locale.chars_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
            }
        },
        ValidationErr::GraphemesLen(operation) => {
            match operation {
                Operation::Eq(v) => locale.graphemes_len_eq.replace("%value%", &v.to_string()),
                Operation::Ne(v) => locale.graphemes_len_ne.replace("%value%", &v.to_string()),
                Operation::Gt(v) => locale.graphemes_len_gt.replace("%value%", &v.to_string()),
                Operation::Ge(v) => locale.graphemes_len_ge.replace("%value%", &v.to_string()),
                Operation::Lt(v) => locale.graphemes_len_lt.replace("%value%", &v.to_string()),
                Operation::Le(v) => locale.graphemes_len_le.replace("%value%", &v.to_string()),
                Operation::Btwn(a, b) => locale.graphemes_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()), 
            }
        },
        ValidationErr::LowercaseLen(operation) => {
            match operation {
                Operation::Eq(v) => locale.lowercase_len_eq.replace("%value%", &v.to_string()),
                Operation::Ne(v) => locale.lowercase_len_ne.replace("%value%", &v.to_string()),
                Operation::Gt(v) => locale.lowercase_len_gt.replace("%value%", &v.to_string()),
                Operation::Ge(v) => locale.lowercase_len_ge.replace("%value%", &v.to_string()),
                Operation::Lt(v) => locale.lowercase_len_lt.replace("%value%", &v.to_string()),
                Operation::Le(v) => locale.lowercase_len_le.replace("%value%", &v.to_string()),
                Operation::Btwn(a, b) => locale.lowercase_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
            }
        },
        ValidationErr::UppercaseLen(operation) => {
            match operation {
                Operation::Eq(v) => locale.uppercase_len_eq.replace("%value%", &v.to_string()),
                Operation::Ne(v) => locale.uppercase_len_ne.replace("%value%", &v.to_string()),
                Operation::Gt(v) => locale.uppercase_len_gt.replace("%value%", &v.to_string()),
                Operation::Ge(v) => locale.uppercase_len_ge.replace("%value%", &v.to_string()),
                Operation::Lt(v) => locale.uppercase_len_lt.replace("%value%", &v.to_string()),
                Operation::Le(v) => locale.uppercase_len_le.replace("%value%", &v.to_string()),
                Operation::Btwn(a, b) => locale.uppercase_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()) ,
            }
        },
        ValidationErr::NumbersLen(operation) => {
            match operation {
                Operation::Eq(v) => locale.number_len_eq.replace("%value%", &v.to_string()),
                Operation::Ne(v) => locale.number_len_ne.replace("%value%", &v.to_string()),
                Operation::Gt(v) => locale.number_len_gt.replace("%value%", &v.to_string()),
                Operation::Ge(v) => locale.number_len_ge.replace("%value%", &v.to_string()),
                Operation::Lt(v) => locale.number_len_lt.replace("%value%", &v.to_string()),
                Operation::Le(v) => locale.number_len_le.replace("%value%", &v.to_string()),
                Operation::Btwn(a, b) => locale.number_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()) ,
            }
        },
        ValidationErr::SymbolsLen(operation) => {
            match operation {
                Operation::Eq(v) => locale.symbols_eq.replace("%value%", &v.to_string()),
                Operation::Ne(v) => locale.symbols_ne.replace("%value%", &v.to_string()),
                Operation::Gt(v) => locale.symbols_gt.replace("%value%", &v.to_string()),
                Operation::Ge(v) => locale.symbols_ge.replace("%value%", &v.to_string()),
                Operation::Lt(v) => locale.symbols_lt.replace("%value%", &v.to_string()),
                Operation::Le(v) => locale.symbols_le.replace("%value%", &v.to_string()),
                Operation::Btwn(a, b) => locale.symbols_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()) ,
            }
        },
    }
}

pub fn locale_pt_long() -> Locale {
    Locale {
        required: String::from("É obrigatório"),
        num_u: String::from("Deve ser um número inteiro sem sinal"),
        num_i: String::from("Deve ser um número inteiro"),
        num_f: String::from("Deve ser um número com ponto flutuante"),
        bool: String::from("Deve ser um booleano"),
        str: String::from("Deve ser uma string"),
        email: String::from("Deve ser uma string"),
        date: String::from("Deve ser uma string"),
        time: String::from("Deve ser uma string"),
        date_time: String::from("Deve ser uma string"),

        eq: String::from("Deve ser igual a %value%"),
        ne: String::from("Deve ser diferente de %value%"),
        gt: String::from("Deve ser maior que %value%"),
        ge: String::from("Deve ser maior ou igual a %value%"),
        lt: String::from("Deve ser menor que %value%"),
        le: String::from("Deve ser menor ou igual a %value%"),

        bytes_len_eq: String::from("A quantidade de bytes deve ser igual a %value%"),
        bytes_len_ne: String::from("A quantidade de bytes deve ser diferente de %value%"),
        bytes_len_gt: String::from("A quantidade de bytes deve ser maior que %value%"),
        bytes_len_ge: String::from("A quantidade de bytes deve ser maior ou igual a %value%"),
        bytes_len_lt: String::from("A quantidade de bytes deve ser menor que %value%"),
        bytes_len_le: String::from("A quantidade de bytes deve ser menor ou igual a %value%"),
        bytes_len_btwn: String::from("A quantidade de bytes deve estar entre %value_a% e %value_b%"),

        chars_len_eq: String::from("A quantidade de caracteres deve ser igual a %value%"),
        chars_len_ne: String::from("A quantidade de caracteres deve ser diferente de %value%"),
        chars_len_gt: String::from("A quantidade de caracteres deve ser maior que %value%"),
        chars_len_ge: String::from("A quantidade de caracteres deve ser maior ou igual a %value%"),
        chars_len_lt: String::from("A quantidade de caracteres deve ser menor que %value%"),
        chars_len_le: String::from("A quantidade de caracteres deve ser menor ou igual a %value%"),
        chars_len_btwn: String::from("A quantidade de caracteres deve estar entre %value_a% e %value_b%"),

        graphemes_len_eq: String::from("A quantidade de grafemas deve ser igual a %value%"), 
        graphemes_len_ne: String::from("A quantidade de grafemas deve ser diferente de %value%"),
        graphemes_len_gt: String::from("A quantidade de grafemas deve ser maior que %value%"),
        graphemes_len_ge: String::from("A quantidade de grafemas deve ser maior ou igual a %value%"),
        graphemes_len_lt: String::from("A quantidade de grafemas deve ser menor que %value%"),
        graphemes_len_le: String::from("A quantidade de grafemas deve ser menor ou igual a %value%"),
        graphemes_len_btwn: String::from("A quantidade de grafemas deve estar entre %value_a% e %value_b%"),

        lowercase_len_eq: String::from("A quantidade de caracteres minúsculos deve ser igual a %value%"),
        lowercase_len_ne: String::from("A quantidade de caracteres minúsculos deve ser diferente de %value%"),
        lowercase_len_gt: String::from("A quantidade de caracteres minúsculos deve ser maior que %value%"),
        lowercase_len_ge: String::from("A quantidade de caracteres minúsculos deve ser maior ou igual a %value%"),
        lowercase_len_lt: String::from("A quantidade de caracteres minúsculos deve ser menor que %value%"),
        lowercase_len_le: String::from("A quantidade de caracteres minúsculos deve ser menor ou igual a %value%"),
        lowercase_len_btwn: String::from("A quantidade de caracteres minúsculos deve estar entre %value_a% e %value_b%"),

        uppercase_len_eq: String::from("A quantidade de caracteres maiúsculos deve ser igual a %value%"),
        uppercase_len_ne: String::from("A quantidade de caracteres maiúsculos deve ser diferente de %value%"),
        uppercase_len_gt: String::from("A quantidade de caracteres maiúsculos deve ser maior que %value%"),
        uppercase_len_ge: String::from("A quantidade de caracteres maiúsculos deve ser maior ou igual a %value%"),
        uppercase_len_lt: String::from("A quantidade de caracteres maiúsculos deve ser menor que %value%"),
        uppercase_len_le: String::from("A quantidade de caracteres maiúsculos deve ser menor ou igual a %value%"),
        uppercase_len_btwn: String::from("A quantidade de caracteres maiúsculos deve estar entre %value_a% e %value_b%"),

        number_len_eq: String::from("A quantidade de números deve ser igual a %value%"),
        number_len_ne: String::from("A quantidade de números deve ser diferente de %value%"),
        number_len_gt: String::from("A quantidade de números deve ser maior que %value%"),
        number_len_ge: String::from("A quantidade de números deve ser maior ou igual a %value%"),
        number_len_lt: String::from("A quantidade de números deve ser menor que %value%"),
        number_len_le: String::from("A quantidade de números deve ser menor ou igual a %value%"),
        number_len_btwn: String::from("A quantidade de números deve estar entre %value_a% e %value_b%"),

        symbols_eq: String::from("A quantidade de símbolos deve ser igual a %value%"),
        symbols_ne: String::from("A quantidade de símbolos deve ser diferente de %value%"),
        symbols_gt: String::from("A quantidade de símbolos deve ser maior que %value%"),
        symbols_ge: String::from("A quantidade de símbolos deve ser maior ou igual a %value%"),
        symbols_lt: String::from("A quantidade de símbolos deve ser menor que %value%"),
        symbols_le: String::from("A quantidade de símbolos deve ser menor ou igual a %value%"),
        symbols_btwn: String::from("A quantidade de símbolos deve estar entre %value_a% e %value_b%"),
    }
}

//pub fn locale_es_long() -> Locale {
//    Locale {
//        required: String::from("Se requiere"),
//        bool: String::from("Debe ser un booleano"),
//        str: String::from("Debe ser una cadena"),
//        num_u: String::from("Debe ser un número entero sin signo"),
//        num_i: String::from("Debe ser un número entero"),
//        num_f: String::from("Debe ser un número de punto flotante"),
//        eq: String::from("Debe ser igual a _"),
//        ne: String::from("Debe ser diferente de _"),
//        gt: String::from("Debe ser mayor que _"),
//        lt: String::from("Debe ser menor que _"),
//        ge: String::from("Debe ser mayor o igual a _"),
//        le: String::from("Debe ser menor o igual a _"),
//    }
//}
//
//pub fn locale_en_long() -> Locale {
//    Locale {
//        required: String::from("Is required"),
//        bool: String::from("Must be a boolean"),
//        str: String::from("Must be a string"),
//        num_u: String::from("Must be an unsigned integer"),
//        num_i: String::from("Must be an integer"),
//        num_f: String::from("Must be a float"),
//        eq: String::from("Must be equals to _"),
//        ne: String::from("Must be different to _"),
//        gt: String::from("Must be greater than _"),
//        lt: String::from("Must be smaller than _"),
//        ge: String::from("Must be greater or equals to _"),
//        le: String::from("Must be smaller or equals to _"),
//    }
//}

#[cfg(test)]
mod test {
    use araucaria::value::Value;

    use super::*;

    #[test]
    fn test_schema_err_to_locale() {
        let locale = locale_pt_long();
        assert_eq!(
            schema_err_to_locale(
                &SchemaErr::Validation(vec![ValidationErr::Required, ValidationErr::Bool, ValidationErr::Eq(Value::Bool(true))]),
                &locale
            ),
            SchemaLocalizedErr::Arr(vec![String::from("É obrigatório"), String::from("Deve ser um booleano"), String::from("Deve ser igual a true")])
        );
        assert_eq!(
            schema_err_to_locale(
                &SchemaErr::Obj(HashMap::from([
                    (
                        String::from("name"),
                        SchemaErr::Validation(vec![
                            ValidationErr::Required,
                            ValidationErr::Str,
                            ValidationErr::Eq(Value::Str(String::from("Paul McCartney")))
                        ])
                    ),
                    (
                        String::from("birthdate"),
                        SchemaErr::Validation(vec![
                            ValidationErr::Required,
                            ValidationErr::Str,
                            ValidationErr::Eq(Value::Str(String::from("1942-06-18")))
                        ])
                    ),
                    (
                        String::from("alive"),
                        SchemaErr::Validation(vec![ValidationErr::Required, ValidationErr::Bool, ValidationErr::Eq(Value::Bool(true))])
                    ),
                    (
                        String::from("bands"),
                        SchemaErr::Validation(vec![
                            ValidationErr::Required,
                            ValidationErr::Str,
                            ValidationErr::Eq(Value::Str(String::from("The Beatles")))
                        ])
                    ),
                ])),
                &locale
            ),
            SchemaLocalizedErr::Obj(HashMap::from([
                (
                    String::from("name"),
                    SchemaLocalizedErr::Arr(vec![
                        String::from("É obrigatório"),
                        String::from("Deve ser uma string"),
                        String::from(r#"Deve ser igual a "Paul McCartney""#),
                    ])
                ),
                (
                    String::from("birthdate"),
                    SchemaLocalizedErr::Arr(vec![
                        String::from("É obrigatório"),
                        String::from("Deve ser uma string"),
                        String::from(r#"Deve ser igual a "1942-06-18""#),
                    ])
                ),
                (
                    String::from("alive"),
                    SchemaLocalizedErr::Arr(vec![
                        String::from("É obrigatório"),
                        String::from("Deve ser um booleano"),
                        String::from("Deve ser igual a true"),
                    ])
                ),
                (
                    String::from("bands"),
                    SchemaLocalizedErr::Arr(vec![
                        String::from("É obrigatório"),
                        String::from("Deve ser uma string"),
                        String::from(r#"Deve ser igual a "The Beatles""#),
                    ])
                ),
            ]))
        );
        assert_eq!(
            serde_json::to_string(&SchemaLocalizedErr::Obj(HashMap::from([
                (
                    String::from("name"),
                    SchemaLocalizedErr::Arr(vec![
                        String::from("É obrigatório"),
                        String::from("Deve ser uma string"),
                        String::from(r#"Deve ser igual a "Paul McCartney""#),
                    ])
                ),
                (
                    String::from("birthdate"),
                    SchemaLocalizedErr::Arr(vec![
                        String::from("É obrigatório"),
                        String::from("Deve ser uma string"),
                        String::from(r#"Deve ser igual a "1942-06-18""#),
                    ])
                ),
                (
                    String::from("alive"),
                    SchemaLocalizedErr::Arr(vec![
                        String::from("É obrigatório"),
                        String::from("Deve ser um booleano"),
                        String::from("Deve ser igual a true"),
                    ])
                ),
                (
                    String::from("bands"),
                    SchemaLocalizedErr::Arr(vec![
                        String::from("É obrigatório"),
                        String::from("Deve ser uma string"),
                        String::from(r#"Deve ser igual a "The Beatles""#),
                    ])
                ),
            ])))
            .unwrap(),
            String::from(
                r#"{"alive":["É obrigatório","Deve ser um booleano","Deve ser igual a true"],"bands":["É obrigatório","Deve ser uma string","Deve ser igual a \"The Beatles\""],"name":["É obrigatório","Deve ser uma string","Deve ser igual a \"Paul McCartney\""],"birthdate":["É obrigatório","Deve ser uma string","Deve ser igual a \"1942-06-18\""]}"#
            )
        );
    }

    #[test]
    fn test_locale_pt_long() {
        let locale = locale_pt_long();
        assert_eq!(validation_err_to_locale(&ValidationErr::Required, &locale), String::from("É obrigatório"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumU, &locale), String::from("Deve ser um número inteiro sem sinal"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumI, &locale), String::from("Deve ser um número inteiro"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumF, &locale), String::from("Deve ser um número com ponto flutuante"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Bool, &locale), String::from("Deve ser um booleano"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Str, &locale), String::from("Deve ser uma string"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Email, &locale), String::from("Deve ser uma string"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Date, &locale), String::from("Deve ser uma string"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Time, &locale), String::from("Deve ser uma string"));
        assert_eq!(validation_err_to_locale(&ValidationErr::DateTime, &locale), String::from("Deve ser uma string"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::Bool(false)), &locale), String::from("Deve ser igual a false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumU(34)), &locale), String::from("Deve ser igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumI(-4)), &locale), String::from("Deve ser igual a -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumF(-4.6)), &locale), String::from("Deve ser igual a -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::Str(String::from("aurorae"))), &locale), String::from("Deve ser igual a \"aurorae\""));

        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::Bool(false)), &locale), String::from("Deve ser diferente de false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumU(34)), &locale), String::from("Deve ser diferente de 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumI(-4)), &locale), String::from("Deve ser diferente de -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumF(-4.6)), &locale), String::from("Deve ser diferente de -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::Str(String::from("aurorae"))), &locale), String::from("Deve ser diferente de \"aurorae\""));

        assert_eq!(validation_err_to_locale(&ValidationErr::Gt(Value::NumU(34)), &locale), String::from("Deve ser maior que 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Gt(Value::NumI(-4)), &locale), String::from("Deve ser maior que -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Gt(Value::NumF(-4.6)), &locale), String::from("Deve ser maior que -4.6"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Lt(Value::NumU(34)), &locale), String::from("Deve ser menor que 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Lt(Value::NumI(-4)), &locale), String::from("Deve ser menor que -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Lt(Value::NumF(-4.6)), &locale), String::from("Deve ser menor que -4.6"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Ge(Value::NumU(34)), &locale), String::from("Deve ser maior ou igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ge(Value::NumI(-4)), &locale), String::from("Deve ser maior ou igual a -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ge(Value::NumF(-4.6)), &locale), String::from("Deve ser maior ou igual a -4.6"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Le(Value::NumU(34)), &locale), String::from("Deve ser menor ou igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Le(Value::NumI(-4)), &locale), String::from("Deve ser menor ou igual a -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Le(Value::NumF(-4.6)), &locale), String::from("Deve ser menor ou igual a -4.6"));

        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Eq(10)), &locale), String::from("A quantidade de bytes deve ser igual a 10"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Ne(11)), &locale), String::from("A quantidade de bytes deve ser diferente de 11"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Gt(12)), &locale), String::from("A quantidade de bytes deve ser maior que 12"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Ge(13)), &locale), String::from("A quantidade de bytes deve ser maior ou igual a 13"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Lt(14)), &locale), String::from("A quantidade de bytes deve ser menor que 14"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Le(15)), &locale), String::from("A quantidade de bytes deve ser menor ou igual a 15"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Btwn(16, 17)), &locale), String::from("A quantidade de bytes deve estar entre 16 e 17"));

        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Eq(18)), &locale), String::from("A quantidade de caracteres deve ser igual a 18"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Ne(19)), &locale), String::from("A quantidade de caracteres deve ser diferente de 19"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Gt(20)), &locale), String::from("A quantidade de caracteres deve ser maior que 20"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Ge(21)), &locale), String::from("A quantidade de caracteres deve ser maior ou igual a 21"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Lt(22)), &locale), String::from("A quantidade de caracteres deve ser menor que 22"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Le(23)), &locale), String::from("A quantidade de caracteres deve ser menor ou igual a 23"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Btwn(24, 25)), &locale), String::from("A quantidade de caracteres deve estar entre 24 e 25"));

        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Eq(26)), &locale), String::from("A quantidade de grafemas deve ser igual a 26"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Ne(27)), &locale), String::from("A quantidade de grafemas deve ser diferente de 27"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Gt(28)), &locale), String::from("A quantidade de grafemas deve ser maior que 28"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Ge(29)), &locale), String::from("A quantidade de grafemas deve ser maior ou igual a 29"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Lt(30)), &locale), String::from("A quantidade de grafemas deve ser menor que 30"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Le(31)), &locale), String::from("A quantidade de grafemas deve ser menor ou igual a 31"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Btwn(32, 33)), &locale), String::from("A quantidade de grafemas deve estar entre 32 e 33"));

        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Eq(42)), &locale), String::from("A quantidade de caracteres minúsculos deve ser igual a 42"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Ne(43)), &locale), String::from("A quantidade de caracteres minúsculos deve ser diferente de 43"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Gt(44)), &locale), String::from("A quantidade de caracteres minúsculos deve ser maior que 44"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Ge(45)), &locale), String::from("A quantidade de caracteres minúsculos deve ser maior ou igual a 45"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Lt(46)), &locale), String::from("A quantidade de caracteres minúsculos deve ser menor que 46"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Le(47)), &locale), String::from("A quantidade de caracteres minúsculos deve ser menor ou igual a 47"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Btwn(48, 49)), &locale), String::from("A quantidade de caracteres minúsculos deve estar entre 48 e 49"));

        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Eq(34)), &locale), String::from("A quantidade de caracteres maiúsculos deve ser igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Ne(35)), &locale), String::from("A quantidade de caracteres maiúsculos deve ser diferente de 35"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Gt(36)), &locale), String::from("A quantidade de caracteres maiúsculos deve ser maior que 36"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Ge(37)), &locale), String::from("A quantidade de caracteres maiúsculos deve ser maior ou igual a 37"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Lt(38)), &locale), String::from("A quantidade de caracteres maiúsculos deve ser menor que 38"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Le(39)), &locale), String::from("A quantidade de caracteres maiúsculos deve ser menor ou igual a 39"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Btwn(40, 41)), &locale), String::from("A quantidade de caracteres maiúsculos deve estar entre 40 e 41"));

        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Eq(50)), &locale), String::from("A quantidade de números deve ser igual a 50"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Ne(51)), &locale), String::from("A quantidade de números deve ser diferente de 51"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Gt(52)), &locale), String::from("A quantidade de números deve ser maior que 52"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Ge(53)), &locale), String::from("A quantidade de números deve ser maior ou igual a 53"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Lt(54)), &locale), String::from("A quantidade de números deve ser menor que 54"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Le(55)), &locale), String::from("A quantidade de números deve ser menor ou igual a 55"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Btwn(56, 57)), &locale), String::from("A quantidade de números deve estar entre 56 e 57"));

        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Eq(58)), &locale), String::from("A quantidade de símbolos deve ser igual a 58"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Ne(59)), &locale), String::from("A quantidade de símbolos deve ser diferente de 59"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Gt(60)), &locale), String::from("A quantidade de símbolos deve ser maior que 60"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Ge(61)), &locale), String::from("A quantidade de símbolos deve ser maior ou igual a 61"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Lt(62)), &locale), String::from("A quantidade de símbolos deve ser menor que 62"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Le(63)), &locale), String::from("A quantidade de símbolos deve ser menor ou igual a 63"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Btwn(64, 65)), &locale), String::from("A quantidade de símbolos deve estar entre 64 e 65"));




    }

//    #[test]
//    fn test_locale_es_long() {
//        let locale = locale_es_long();
//        assert_eq!(validation_err_to_locale(&ValidationErr::Required, &locale), String::from("Se requiere"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Bool, &locale), String::from("Debe ser un booleano"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Str, &locale), String::from("Debe ser una cadena"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::NumU, &locale), String::from("Debe ser un número entero sin signo"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::NumI, &locale), String::from("Debe ser un número entero"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::NumF, &locale), String::from("Debe ser un número de punto flotante"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::Bool(false)), &locale), String::from("Debe ser igual a false"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumU(34)), &locale), String::from("Debe ser igual a 34"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumI(-4)), &locale), String::from("Debe ser igual a -4"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumF(-4.6)), &locale), String::from("Debe ser igual a -4.6"));
//        assert_eq!(
//            validation_err_to_locale(&ValidationErr::Eq(Value::Str(String::from("aurorae"))), &locale),
//            String::from("Debe ser igual a \"aurorae\"")
//        );
//        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::Bool(false)), &locale), String::from("Debe ser diferente de false"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumU(34)), &locale), String::from("Debe ser diferente de 34"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumI(-4)), &locale), String::from("Debe ser diferente de -4"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumF(-4.6)), &locale), String::from("Debe ser diferente de -4.6"));
//        assert_eq!(
//            validation_err_to_locale(&ValidationErr::Ne(Value::Str(String::from("aurorae"))), &locale),
//            String::from("Debe ser diferente de \"aurorae\"")
//        );
//        assert_eq!(validation_err_to_locale(&ValidationErr::Gt(Value::NumU(34)), &locale), String::from("Debe ser mayor que 34"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Gt(Value::NumI(-4)), &locale), String::from("Debe ser mayor que -4"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Gt(Value::NumF(-4.6)), &locale), String::from("Debe ser mayor que -4.6"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Lt(Value::NumU(34)), &locale), String::from("Debe ser menor que 34"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Lt(Value::NumI(-4)), &locale), String::from("Debe ser menor que -4"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Lt(Value::NumF(-4.6)), &locale), String::from("Debe ser menor que -4.6"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Ge(Value::NumU(34)), &locale), String::from("Debe ser mayor o igual a 34"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Ge(Value::NumI(-4)), &locale), String::from("Debe ser mayor o igual a -4"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Ge(Value::NumF(-4.6)), &locale), String::from("Debe ser mayor o igual a -4.6"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Le(Value::NumU(34)), &locale), String::from("Debe ser menor o igual a 34"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Le(Value::NumI(-4)), &locale), String::from("Debe ser menor o igual a -4"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Le(Value::NumF(-4.6)), &locale), String::from("Debe ser menor o igual a -4.6"));
//    }
//
//    #[test]
//    fn test_locale_en_long() {
//        let locale = locale_en_long();
//        assert_eq!(validation_err_to_locale(&ValidationErr::Required, &locale), String::from("Is required"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Bool, &locale), String::from("Must be a boolean"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Str, &locale), String::from("Must be a string"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::NumU, &locale), String::from("Must be an unsigned integer"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::NumI, &locale), String::from("Must be an integer"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::NumF, &locale), String::from("Must be a float"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::Bool(false)), &locale), String::from("Must be equals to false"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumU(34)), &locale), String::from("Must be equals to 34"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumI(-4)), &locale), String::from("Must be equals to -4"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumF(-4.6)), &locale), String::from("Must be equals to -4.6"));
//        assert_eq!(
//            validation_err_to_locale(&ValidationErr::Eq(Value::Str(String::from("aurorae"))), &locale),
//            String::from("Must be equals to \"aurorae\"")
//        );
//        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::Bool(false)), &locale), String::from("Must be different to false"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumU(34)), &locale), String::from("Must be different to 34"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumI(-4)), &locale), String::from("Must be different to -4"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumF(-4.6)), &locale), String::from("Must be different to -4.6"));
//        assert_eq!(
//            validation_err_to_locale(&ValidationErr::Ne(Value::Str(String::from("aurorae"))), &locale),
//            String::from("Must be different to \"aurorae\"")
//        );
//        assert_eq!(validation_err_to_locale(&ValidationErr::Gt(Value::NumU(34)), &locale), String::from("Must be greater than 34"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Gt(Value::NumI(-4)), &locale), String::from("Must be greater than -4"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Gt(Value::NumF(-4.6)), &locale), String::from("Must be greater than -4.6"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Lt(Value::NumU(34)), &locale), String::from("Must be smaller than 34"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Lt(Value::NumI(-4)), &locale), String::from("Must be smaller than -4"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Lt(Value::NumF(-4.6)), &locale), String::from("Must be smaller than -4.6"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Ge(Value::NumU(34)), &locale), String::from("Must be greater or equals to 34"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Ge(Value::NumI(-4)), &locale), String::from("Must be greater or equals to -4"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Ge(Value::NumF(-4.6)), &locale), String::from("Must be greater or equals to -4.6"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Le(Value::NumU(34)), &locale), String::from("Must be smaller or equals to 34"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Le(Value::NumI(-4)), &locale), String::from("Must be smaller or equals to -4"));
//        assert_eq!(validation_err_to_locale(&ValidationErr::Le(Value::NumF(-4.6)), &locale), String::from("Must be smaller or equals to -4.6"));
//    }
}

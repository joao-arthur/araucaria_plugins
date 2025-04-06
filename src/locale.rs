use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

use araucaria::{
    error::{SchemaErr, ValidationErr},
    value::value_to_string,
};

pub struct Locale {
    required: String,
    bool: String,
    str: String,
    num_u: String,
    num_i: String,
    num_f: String,
    eq: String,
    ne: String,
    gt: String,
    lt: String,
    ge: String,
    le: String,
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
        ValidationErr::Bool => locale.bool.clone(),
        ValidationErr::Str => locale.str.clone(),
        ValidationErr::NumU => locale.num_u.clone(),
        ValidationErr::NumI => locale.num_i.clone(),
        ValidationErr::NumF => locale.num_f.clone(),
        ValidationErr::Eq(value) => locale.eq.replace("_", &value_to_string(&value)),
        ValidationErr::Ne(value) => locale.ne.replace("_", &value_to_string(&value)),
        ValidationErr::Gt(value) => locale.gt.replace("_", &value_to_string(&value)),
        ValidationErr::Lt(value) => locale.lt.replace("_", &value_to_string(&value)),
        ValidationErr::Ge(value) => locale.ge.replace("_", &value_to_string(&value)),
        ValidationErr::Le(value) => locale.le.replace("_", &value_to_string(&value)),
        _ => String::from(""),
    }
}

pub fn locale_pt_long() -> Locale {
    Locale {
        required: String::from("É obrigatório"),
        bool: String::from("Deve ser um booleano"),
        str: String::from("Deve ser uma string"),
        num_u: String::from("Deve ser um número inteiro sem sinal"),
        num_i: String::from("Deve ser um número inteiro"),
        num_f: String::from("Deve ser um número com ponto flutuante"),
        eq: String::from("Deve ser igual a _"),
        ne: String::from("Deve ser diferente de _"),
        gt: String::from("Deve ser maior que _"),
        lt: String::from("Deve ser menor que _"),
        ge: String::from("Deve ser maior ou igual a _"),
        le: String::from("Deve ser menor ou igual a _"),
    }
}

pub fn locale_es_long() -> Locale {
    Locale {
        required: String::from("Se requiere"),
        bool: String::from("Debe ser un booleano"),
        str: String::from("Debe ser una cadena"),
        num_u: String::from("Debe ser un número entero sin signo"),
        num_i: String::from("Debe ser un número entero"),
        num_f: String::from("Debe ser un número de punto flotante"),
        eq: String::from("Debe ser igual a _"),
        ne: String::from("Debe ser diferente de _"),
        gt: String::from("Debe ser mayor que _"),
        lt: String::from("Debe ser menor que _"),
        ge: String::from("Debe ser mayor o igual a _"),
        le: String::from("Debe ser menor o igual a _"),
    }
}

pub fn locale_en_long() -> Locale {
    Locale {
        required: String::from("Is required"),
        bool: String::from("Must be a boolean"),
        str: String::from("Must be a string"),
        num_u: String::from("Must be an unsigned integer"),
        num_i: String::from("Must be an integer"),
        num_f: String::from("Must be a float"),
        eq: String::from("Must be equals to _"),
        ne: String::from("Must be different to _"),
        gt: String::from("Must be greater than _"),
        lt: String::from("Must be smaller than _"),
        ge: String::from("Must be greater or equals to _"),
        le: String::from("Must be smaller or equals to _"),
    }
}

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
        assert_eq!(validation_err_to_locale(&ValidationErr::Bool, &locale), String::from("Deve ser um booleano"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Str, &locale), String::from("Deve ser uma string"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumU, &locale), String::from("Deve ser um número inteiro sem sinal"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumI, &locale), String::from("Deve ser um número inteiro"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumF, &locale), String::from("Deve ser um número com ponto flutuante"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::Bool(false)), &locale), String::from("Deve ser igual a false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumU(34)), &locale), String::from("Deve ser igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumI(-4)), &locale), String::from("Deve ser igual a -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumF(-4.6)), &locale), String::from("Deve ser igual a -4.6"));
        assert_eq!(
            validation_err_to_locale(&ValidationErr::Eq(Value::Str(String::from("aurorae"))), &locale),
            String::from("Deve ser igual a \"aurorae\"")
        );
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::Bool(false)), &locale), String::from("Deve ser diferente de false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumU(34)), &locale), String::from("Deve ser diferente de 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumI(-4)), &locale), String::from("Deve ser diferente de -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumF(-4.6)), &locale), String::from("Deve ser diferente de -4.6"));
        assert_eq!(
            validation_err_to_locale(&ValidationErr::Ne(Value::Str(String::from("aurorae"))), &locale),
            String::from("Deve ser diferente de \"aurorae\"")
        );
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
    }

    #[test]
    fn test_locale_es_long() {
        let locale = locale_es_long();
        assert_eq!(validation_err_to_locale(&ValidationErr::Required, &locale), String::from("Se requiere"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Bool, &locale), String::from("Debe ser un booleano"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Str, &locale), String::from("Debe ser una cadena"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumU, &locale), String::from("Debe ser un número entero sin signo"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumI, &locale), String::from("Debe ser un número entero"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumF, &locale), String::from("Debe ser un número de punto flotante"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::Bool(false)), &locale), String::from("Debe ser igual a false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumU(34)), &locale), String::from("Debe ser igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumI(-4)), &locale), String::from("Debe ser igual a -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumF(-4.6)), &locale), String::from("Debe ser igual a -4.6"));
        assert_eq!(
            validation_err_to_locale(&ValidationErr::Eq(Value::Str(String::from("aurorae"))), &locale),
            String::from("Debe ser igual a \"aurorae\"")
        );
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::Bool(false)), &locale), String::from("Debe ser diferente de false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumU(34)), &locale), String::from("Debe ser diferente de 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumI(-4)), &locale), String::from("Debe ser diferente de -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumF(-4.6)), &locale), String::from("Debe ser diferente de -4.6"));
        assert_eq!(
            validation_err_to_locale(&ValidationErr::Ne(Value::Str(String::from("aurorae"))), &locale),
            String::from("Debe ser diferente de \"aurorae\"")
        );
        assert_eq!(validation_err_to_locale(&ValidationErr::Gt(Value::NumU(34)), &locale), String::from("Debe ser mayor que 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Gt(Value::NumI(-4)), &locale), String::from("Debe ser mayor que -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Gt(Value::NumF(-4.6)), &locale), String::from("Debe ser mayor que -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Lt(Value::NumU(34)), &locale), String::from("Debe ser menor que 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Lt(Value::NumI(-4)), &locale), String::from("Debe ser menor que -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Lt(Value::NumF(-4.6)), &locale), String::from("Debe ser menor que -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ge(Value::NumU(34)), &locale), String::from("Debe ser mayor o igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ge(Value::NumI(-4)), &locale), String::from("Debe ser mayor o igual a -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ge(Value::NumF(-4.6)), &locale), String::from("Debe ser mayor o igual a -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Le(Value::NumU(34)), &locale), String::from("Debe ser menor o igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Le(Value::NumI(-4)), &locale), String::from("Debe ser menor o igual a -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Le(Value::NumF(-4.6)), &locale), String::from("Debe ser menor o igual a -4.6"));
    }

    #[test]
    fn test_locale_en_long() {
        let locale = locale_en_long();
        assert_eq!(validation_err_to_locale(&ValidationErr::Required, &locale), String::from("Is required"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Bool, &locale), String::from("Must be a boolean"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Str, &locale), String::from("Must be a string"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumU, &locale), String::from("Must be an unsigned integer"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumI, &locale), String::from("Must be an integer"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumF, &locale), String::from("Must be a float"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::Bool(false)), &locale), String::from("Must be equals to false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumU(34)), &locale), String::from("Must be equals to 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumI(-4)), &locale), String::from("Must be equals to -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Eq(Value::NumF(-4.6)), &locale), String::from("Must be equals to -4.6"));
        assert_eq!(
            validation_err_to_locale(&ValidationErr::Eq(Value::Str(String::from("aurorae"))), &locale),
            String::from("Must be equals to \"aurorae\"")
        );
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::Bool(false)), &locale), String::from("Must be different to false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumU(34)), &locale), String::from("Must be different to 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumI(-4)), &locale), String::from("Must be different to -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ne(Value::NumF(-4.6)), &locale), String::from("Must be different to -4.6"));
        assert_eq!(
            validation_err_to_locale(&ValidationErr::Ne(Value::Str(String::from("aurorae"))), &locale),
            String::from("Must be different to \"aurorae\"")
        );
        assert_eq!(validation_err_to_locale(&ValidationErr::Gt(Value::NumU(34)), &locale), String::from("Must be greater than 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Gt(Value::NumI(-4)), &locale), String::from("Must be greater than -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Gt(Value::NumF(-4.6)), &locale), String::from("Must be greater than -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Lt(Value::NumU(34)), &locale), String::from("Must be smaller than 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Lt(Value::NumI(-4)), &locale), String::from("Must be smaller than -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Lt(Value::NumF(-4.6)), &locale), String::from("Must be smaller than -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ge(Value::NumU(34)), &locale), String::from("Must be greater or equals to 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ge(Value::NumI(-4)), &locale), String::from("Must be greater or equals to -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Ge(Value::NumF(-4.6)), &locale), String::from("Must be greater or equals to -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Le(Value::NumU(34)), &locale), String::from("Must be smaller or equals to 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Le(Value::NumI(-4)), &locale), String::from("Must be smaller or equals to -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Le(Value::NumF(-4.6)), &locale), String::from("Must be smaller or equals to -4.6"));
    }
}

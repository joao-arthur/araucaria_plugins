use araucaria::{
    error::ValidationErr,
    value::{value_to_string, Value},
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

pub fn locale_pt_long() -> Locale {
    Locale {
        required: String::from("É obrigatório"),
        bool: String::from("Deve ser um booleano"),
        str: String::from("Deve ser uma string"),
        num_u: String::from("Deve ser um número inteiro sem sinal"),
        num_i: String::from("Deve ser um número inteiro"),
        num_f: String::from("Deve ser um número com ponto flutuante"),
        eq: String::from("Deve ser igual a "),
        ne: String::from("Deve ser diferente de "),
        gt: String::from("Deve ser maior que "),
        lt: String::from("Deve ser menor que "),
        ge: String::from("Deve ser maior ou igual a "),
        le: String::from("Deve ser menor ou igual a "),
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
        eq: String::from("Debe ser igual a "),
        ne: String::from("Debe ser diferente de "),
        gt: String::from("Debe ser mayor que "),
        lt: String::from("Debe ser menor que "),
        ge: String::from("Debe ser mayor o igual a "),
        le: String::from("Debe ser menor o igual a "),
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
        eq: String::from("Must be equals to "),
        ne: String::from("Must be different to "),
        gt: String::from("Must be greater than "),
        lt: String::from("Must be smaller than "),
        ge: String::from("Must be greater or equals to "),
        le: String::from("Must be smaller or equals to "),
    }
}

pub fn err_to_locale(error: &ValidationErr, locale: &Locale) -> String {
    match error {
        ValidationErr::Required => locale.required.clone(),
        ValidationErr::Bool => locale.bool.clone(),
        ValidationErr::Str => locale.str.clone(),
        ValidationErr::NumU => locale.num_u.clone(),
        ValidationErr::NumI => locale.num_i.clone(),
        ValidationErr::NumF => locale.num_f.clone(),
        ValidationErr::Eq(value) => locale.eq.clone() + &value_to_string(&value),
        ValidationErr::Ne(value) => locale.ne.clone() + &value_to_string(&value),
        ValidationErr::Gt(value) => locale.gt.clone() + &value_to_string(&value),
        ValidationErr::Lt(value) => locale.lt.clone() + &value_to_string(&value),
        ValidationErr::Ge(value) => locale.ge.clone() + &value_to_string(&value),
        ValidationErr::Le(value) => locale.le.clone() + &value_to_string(&value),
    }
}

#[cfg(test)]
mod test {
    use araucaria::value::Value;

    use super::*;

    #[test]
    fn test_locale_pt_long() {
        let locale = locale_pt_long();
        assert_eq!(err_to_locale(&ValidationErr::Required, &locale), String::from("É obrigatório"));
        assert_eq!(
            err_to_locale(&ValidationErr::Bool, &locale),
            String::from("Deve ser um booleano")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Str, &locale),
            String::from("Deve ser uma string")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::NumU, &locale),
            String::from("Deve ser um número inteiro sem sinal")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::NumI, &locale),
            String::from("Deve ser um número inteiro")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::NumF, &locale),
            String::from("Deve ser um número com ponto flutuante")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Eq(Value::Bool(false)), &locale),
            String::from("Deve ser igual a false")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Eq(Value::NumU(34)), &locale),
            String::from("Deve ser igual a 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Eq(Value::NumI(-4)), &locale),
            String::from("Deve ser igual a -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Eq(Value::NumF(-4.6)), &locale),
            String::from("Deve ser igual a -4.6")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Eq(Value::Str(String::from("aurorae"))), &locale),
            String::from("Deve ser igual a \"aurorae\"")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ne(Value::Bool(false)), &locale),
            String::from("Deve ser diferente de false")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ne(Value::NumU(34)), &locale),
            String::from("Deve ser diferente de 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ne(Value::NumI(-4)), &locale),
            String::from("Deve ser diferente de -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ne(Value::NumF(-4.6)), &locale),
            String::from("Deve ser diferente de -4.6")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ne(Value::Str(String::from("aurorae"))), &locale),
            String::from("Deve ser diferente de \"aurorae\"")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Gt(Value::NumU(34)), &locale),
            String::from("Deve ser maior que 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Gt(Value::NumI(-4)), &locale),
            String::from("Deve ser maior que -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Gt(Value::NumF(-4.6)), &locale),
            String::from("Deve ser maior que -4.6")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Lt(Value::NumU(34)), &locale),
            String::from("Deve ser menor que 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Lt(Value::NumI(-4)), &locale),
            String::from("Deve ser menor que -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Lt(Value::NumF(-4.6)), &locale),
            String::from("Deve ser menor que -4.6")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ge(Value::NumU(34)), &locale),
            String::from("Deve ser maior ou igual a 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ge(Value::NumI(-4)), &locale),
            String::from("Deve ser maior ou igual a -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ge(Value::NumF(-4.6)), &locale),
            String::from("Deve ser maior ou igual a -4.6")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Le(Value::NumU(34)), &locale),
            String::from("Deve ser menor ou igual a 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Le(Value::NumI(-4)), &locale),
            String::from("Deve ser menor ou igual a -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Le(Value::NumF(-4.6)), &locale),
            String::from("Deve ser menor ou igual a -4.6")
        );
    }

    #[test]
    fn test_locale_es_long() {
        let locale = locale_es_long();
        assert_eq!(err_to_locale(&ValidationErr::Required, &locale), String::from("Se requiere"));
        assert_eq!(
            err_to_locale(&ValidationErr::Bool, &locale),
            String::from("Debe ser un booleano")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Str, &locale),
            String::from("Debe ser una cadena")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::NumU, &locale),
            String::from("Debe ser un número entero sin signo")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::NumI, &locale),
            String::from("Debe ser un número entero")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::NumF, &locale),
            String::from("Debe ser un número de punto flotante")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Eq(Value::Bool(false)), &locale),
            String::from("Debe ser igual a false")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Eq(Value::NumU(34)), &locale),
            String::from("Debe ser igual a 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Eq(Value::NumI(-4)), &locale),
            String::from("Debe ser igual a -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Eq(Value::NumF(-4.6)), &locale),
            String::from("Debe ser igual a -4.6")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Eq(Value::Str(String::from("aurorae"))), &locale),
            String::from("Debe ser igual a \"aurorae\"")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ne(Value::Bool(false)), &locale),
            String::from("Debe ser diferente de false")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ne(Value::NumU(34)), &locale),
            String::from("Debe ser diferente de 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ne(Value::NumI(-4)), &locale),
            String::from("Debe ser diferente de -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ne(Value::NumF(-4.6)), &locale),
            String::from("Debe ser diferente de -4.6")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ne(Value::Str(String::from("aurorae"))), &locale),
            String::from("Debe ser diferente de \"aurorae\"")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Gt(Value::NumU(34)), &locale),
            String::from("Debe ser mayor que 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Gt(Value::NumI(-4)), &locale),
            String::from("Debe ser mayor que -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Gt(Value::NumF(-4.6)), &locale),
            String::from("Debe ser mayor que -4.6")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Lt(Value::NumU(34)), &locale),
            String::from("Debe ser menor que 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Lt(Value::NumI(-4)), &locale),
            String::from("Debe ser menor que -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Lt(Value::NumF(-4.6)), &locale),
            String::from("Debe ser menor que -4.6")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ge(Value::NumU(34)), &locale),
            String::from("Debe ser mayor o igual a 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ge(Value::NumI(-4)), &locale),
            String::from("Debe ser mayor o igual a -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ge(Value::NumF(-4.6)), &locale),
            String::from("Debe ser mayor o igual a -4.6")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Le(Value::NumU(34)), &locale),
            String::from("Debe ser menor o igual a 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Le(Value::NumI(-4)), &locale),
            String::from("Debe ser menor o igual a -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Le(Value::NumF(-4.6)), &locale),
            String::from("Debe ser menor o igual a -4.6")
        );
    }

    #[test]
    fn test_locale_en_long() {
        let locale = locale_en_long();
        assert_eq!(err_to_locale(&ValidationErr::Required, &locale), String::from("Is required"));
        assert_eq!(err_to_locale(&ValidationErr::Bool, &locale), String::from("Must be a boolean"));
        assert_eq!(err_to_locale(&ValidationErr::Str, &locale), String::from("Must be a string"));
        assert_eq!(
            err_to_locale(&ValidationErr::NumU, &locale),
            String::from("Must be an unsigned integer")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::NumI, &locale),
            String::from("Must be an integer")
        );
        assert_eq!(err_to_locale(&ValidationErr::NumF, &locale), String::from("Must be a float"));
        assert_eq!(
            err_to_locale(&ValidationErr::Eq(Value::Bool(false)), &locale),
            String::from("Must be equals to false")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Eq(Value::NumU(34)), &locale),
            String::from("Must be equals to 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Eq(Value::NumI(-4)), &locale),
            String::from("Must be equals to -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Eq(Value::NumF(-4.6)), &locale),
            String::from("Must be equals to -4.6")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Eq(Value::Str(String::from("aurorae"))), &locale),
            String::from("Must be equals to \"aurorae\"")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ne(Value::Bool(false)), &locale),
            String::from("Must be different to false")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ne(Value::NumU(34)), &locale),
            String::from("Must be different to 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ne(Value::NumI(-4)), &locale),
            String::from("Must be different to -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ne(Value::NumF(-4.6)), &locale),
            String::from("Must be different to -4.6")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ne(Value::Str(String::from("aurorae"))), &locale),
            String::from("Must be different to \"aurorae\"")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Gt(Value::NumU(34)), &locale),
            String::from("Must be greater than 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Gt(Value::NumI(-4)), &locale),
            String::from("Must be greater than -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Gt(Value::NumF(-4.6)), &locale),
            String::from("Must be greater than -4.6")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Lt(Value::NumU(34)), &locale),
            String::from("Must be smaller than 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Lt(Value::NumI(-4)), &locale),
            String::from("Must be smaller than -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Lt(Value::NumF(-4.6)), &locale),
            String::from("Must be smaller than -4.6")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ge(Value::NumU(34)), &locale),
            String::from("Must be greater or equals to 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ge(Value::NumI(-4)), &locale),
            String::from("Must be greater or equals to -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Ge(Value::NumF(-4.6)), &locale),
            String::from("Must be greater or equals to -4.6")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Le(Value::NumU(34)), &locale),
            String::from("Must be smaller or equals to 34")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Le(Value::NumI(-4)), &locale),
            String::from("Must be smaller or equals to -4")
        );
        assert_eq!(
            err_to_locale(&ValidationErr::Le(Value::NumF(-4.6)), &locale),
            String::from("Must be smaller or equals to -4.6")
        );
    }
}

use serde::{Deserialize, Serialize, Serializer};
use std::collections::BTreeMap;

use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{Operand, OperandValue, Operation},
};

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

pub fn localize_schema_err(err: &SchemaErr, locale: &Locale) -> SchemaLocalizedErr {
    match err {
        SchemaErr::Arr(arr) => SchemaLocalizedErr::Arr(arr.iter().map(|item| validation_err_to_locale(item, locale)).collect()),
        SchemaErr::Obj(obj) => {
            let mut result: BTreeMap<String, SchemaLocalizedErr> = BTreeMap::new();
            for (key, item) in obj {
                result.insert(key.clone(), localize_schema_err(item, locale));
            }
            SchemaLocalizedErr::Obj(result)
        }
    }
}

pub fn validation_err_to_locale(error: &ValidationErr, locale: &Locale) -> String {
    match error {
        ValidationErr::Required => locale.required.clone(),
        ValidationErr::U64 => locale.u64.clone(),
        ValidationErr::I64 => locale.i64.clone(),
        ValidationErr::F64 => locale.f64.clone(),
        ValidationErr::USize => locale.usize.clone(),
        ValidationErr::ISize => locale.isize.clone(),
        ValidationErr::Bool => locale.bool.clone(),
        ValidationErr::Str => locale.str.clone(),
        ValidationErr::Email => locale.email.clone(),
        ValidationErr::Date => locale.date.clone(),
        ValidationErr::Time => locale.time.clone(),
        ValidationErr::DateTime => locale.date_time.clone(),
        ValidationErr::Operation(operation) => match operation {
            Operation::Eq(v) => locale.eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => locale.btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
        },
        ValidationErr::BytesLen(operation) => match operation {
            Operation::Eq(v) => locale.bytes_len_eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.bytes_len_ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.bytes_len_gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.bytes_len_ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.bytes_len_lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.bytes_len_le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => locale.bytes_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
        },
        ValidationErr::CharsLen(operation) => match operation {
            Operation::Eq(v) => locale.chars_len_eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.chars_len_ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.chars_len_gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.chars_len_ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.chars_len_lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.chars_len_le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => locale.chars_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
        },
        ValidationErr::GraphemesLen(operation) => match operation {
            Operation::Eq(v) => locale.graphemes_len_eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.graphemes_len_ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.graphemes_len_gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.graphemes_len_ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.graphemes_len_lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.graphemes_len_le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => {
                locale.graphemes_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string())
            }
        },
        ValidationErr::LowercaseLen(operation) => match operation {
            Operation::Eq(v) => locale.lowercase_len_eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.lowercase_len_ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.lowercase_len_gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.lowercase_len_ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.lowercase_len_lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.lowercase_len_le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => {
                locale.lowercase_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string())
            }
        },
        ValidationErr::UppercaseLen(operation) => match operation {
            Operation::Eq(v) => locale.uppercase_len_eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.uppercase_len_ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.uppercase_len_gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.uppercase_len_ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.uppercase_len_lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.uppercase_len_le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => {
                locale.uppercase_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string())
            }
        },
        ValidationErr::NumbersLen(operation) => match operation {
            Operation::Eq(v) => locale.number_len_eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.number_len_ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.number_len_gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.number_len_ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.number_len_lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.number_len_le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => locale.number_len_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
        },
        ValidationErr::SymbolsLen(operation) => match operation {
            Operation::Eq(v) => locale.symbols_eq.replace("%value%", &v.to_string()),
            Operation::Ne(v) => locale.symbols_ne.replace("%value%", &v.to_string()),
            Operation::Gt(v) => locale.symbols_gt.replace("%value%", &v.to_string()),
            Operation::Ge(v) => locale.symbols_ge.replace("%value%", &v.to_string()),
            Operation::Lt(v) => locale.symbols_lt.replace("%value%", &v.to_string()),
            Operation::Le(v) => locale.symbols_le.replace("%value%", &v.to_string()),
            Operation::Btwn(a, b) => locale.symbols_btwn.replace("%value_a%", &a.to_string()).replace("%value_b%", &b.to_string()),
        },
        _ => "".into(),
    }
}

pub fn locale_pt_long() -> Locale {
    Locale {
        required: "É obrigatório".into(),
        u64: "Deve ser um número inteiro sem sinal".into(),
        i64: "Deve ser um número inteiro".into(),
        f64: "Deve ser um número com ponto flutuante".into(),
        usize: "".into(),
        isize: "".into(),
        bool: "Deve ser um booleano".into(),
        str: "Deve ser uma string".into(),
        email: "Deve ser um e-mail".into(),
        date: "Deve ser uma data".into(),
        time: "Deve ser uma hora".into(),
        date_time: "Deve ser uma data e hora".into(),
        eq: "Deve ser igual a %value%".into(),
        ne: "Deve ser diferente de %value%".into(),
        gt: "Deve ser maior que %value%".into(),
        ge: "Deve ser maior ou igual a %value%".into(),
        lt: "Deve ser menor que %value%".into(),
        le: "Deve ser menor ou igual a %value%".into(),
        btwn: "Deve estar entre %value_a% e %value_b%".into(),
        bytes_len_eq: "A quantidade de bytes deve ser igual a %value%".into(),
        bytes_len_ne: "A quantidade de bytes deve ser diferente de %value%".into(),
        bytes_len_gt: "A quantidade de bytes deve ser maior que %value%".into(),
        bytes_len_ge: "A quantidade de bytes deve ser maior ou igual a %value%".into(),
        bytes_len_lt: "A quantidade de bytes deve ser menor que %value%".into(),
        bytes_len_le: "A quantidade de bytes deve ser menor ou igual a %value%".into(),
        bytes_len_btwn: "A quantidade de bytes deve estar entre %value_a% e %value_b%".into(),
        chars_len_eq: "A quantidade de caracteres deve ser igual a %value%".into(),
        chars_len_ne: "A quantidade de caracteres deve ser diferente de %value%".into(),
        chars_len_gt: "A quantidade de caracteres deve ser maior que %value%".into(),
        chars_len_ge: "A quantidade de caracteres deve ser maior ou igual a %value%".into(),
        chars_len_lt: "A quantidade de caracteres deve ser menor que %value%".into(),
        chars_len_le: "A quantidade de caracteres deve ser menor ou igual a %value%".into(),
        chars_len_btwn: "A quantidade de caracteres deve estar entre %value_a% e %value_b%".into(),
        graphemes_len_eq: "A quantidade de grafemas deve ser igual a %value%".into(),
        graphemes_len_ne: "A quantidade de grafemas deve ser diferente de %value%".into(),
        graphemes_len_gt: "A quantidade de grafemas deve ser maior que %value%".into(),
        graphemes_len_ge: "A quantidade de grafemas deve ser maior ou igual a %value%".into(),
        graphemes_len_lt: "A quantidade de grafemas deve ser menor que %value%".into(),
        graphemes_len_le: "A quantidade de grafemas deve ser menor ou igual a %value%".into(),
        graphemes_len_btwn: "A quantidade de grafemas deve estar entre %value_a% e %value_b%".into(),
        lowercase_len_eq: "A quantidade de caracteres minúsculos deve ser igual a %value%".into(),
        lowercase_len_ne: "A quantidade de caracteres minúsculos deve ser diferente de %value%".into(),
        lowercase_len_gt: "A quantidade de caracteres minúsculos deve ser maior que %value%".into(),
        lowercase_len_ge: "A quantidade de caracteres minúsculos deve ser maior ou igual a %value%".into(),
        lowercase_len_lt: "A quantidade de caracteres minúsculos deve ser menor que %value%".into(),
        lowercase_len_le: "A quantidade de caracteres minúsculos deve ser menor ou igual a %value%".into(),
        lowercase_len_btwn: "A quantidade de caracteres minúsculos deve estar entre %value_a% e %value_b%".into(),
        uppercase_len_eq: "A quantidade de caracteres maiúsculos deve ser igual a %value%".into(),
        uppercase_len_ne: "A quantidade de caracteres maiúsculos deve ser diferente de %value%".into(),
        uppercase_len_gt: "A quantidade de caracteres maiúsculos deve ser maior que %value%".into(),
        uppercase_len_ge: "A quantidade de caracteres maiúsculos deve ser maior ou igual a %value%".into(),
        uppercase_len_lt: "A quantidade de caracteres maiúsculos deve ser menor que %value%".into(),
        uppercase_len_le: "A quantidade de caracteres maiúsculos deve ser menor ou igual a %value%".into(),
        uppercase_len_btwn: "A quantidade de caracteres maiúsculos deve estar entre %value_a% e %value_b%".into(),
        number_len_eq: "A quantidade de números deve ser igual a %value%".into(),
        number_len_ne: "A quantidade de números deve ser diferente de %value%".into(),
        number_len_gt: "A quantidade de números deve ser maior que %value%".into(),
        number_len_ge: "A quantidade de números deve ser maior ou igual a %value%".into(),
        number_len_lt: "A quantidade de números deve ser menor que %value%".into(),
        number_len_le: "A quantidade de números deve ser menor ou igual a %value%".into(),
        number_len_btwn: "A quantidade de números deve estar entre %value_a% e %value_b%".into(),
        symbols_eq: "A quantidade de símbolos deve ser igual a %value%".into(),
        symbols_ne: "A quantidade de símbolos deve ser diferente de %value%".into(),
        symbols_gt: "A quantidade de símbolos deve ser maior que %value%".into(),
        symbols_ge: "A quantidade de símbolos deve ser maior ou igual a %value%".into(),
        symbols_lt: "A quantidade de símbolos deve ser menor que %value%".into(),
        symbols_le: "A quantidade de símbolos deve ser menor ou igual a %value%".into(),
        symbols_btwn: "A quantidade de símbolos deve estar entre %value_a% e %value_b%".into(),
    }
}

pub fn locale_es_long() -> Locale {
    Locale {
        required: "Se requiere".into(),
        u64: "Debe ser un número entero sin signo".into(),
        i64: "Debe ser un número entero".into(),
        f64: "Debe ser un número de punto flotante".into(),
        usize: "".into(),
        isize: "".into(),
        bool: "Debe ser un booleano".into(),
        str: "Debe ser una cadena".into(),
        email: "Debe ser un correo electrónico".into(),
        date: "Debe ser una fecha".into(),
        time: "Debe ser una hora".into(),
        date_time: "Debe ser una fecha y hora".into(),
        eq: "Debe ser igual a %value%".into(),
        ne: "Debe ser diferente de %value%".into(),
        gt: "Debe ser mayor que %value%".into(),
        ge: "Debe ser mayor o igual a %value%".into(),
        lt: "Debe ser menor que %value%".into(),
        le: "Debe ser menor o igual a %value%".into(),
        btwn: "Debe estar entre %value_a% y %value_b%".into(),
        bytes_len_eq: "La cantidad de bytes debe ser igual a %value%".into(),
        bytes_len_ne: "La cantidad de bytes debe ser diferente de %value%".into(),
        bytes_len_gt: "La cantidad de bytes debe ser mayor que %value%".into(),
        bytes_len_ge: "La cantidad de bytes debe ser mayor o igual a %value%".into(),
        bytes_len_lt: "La cantidad de bytes debe ser menor que %value%".into(),
        bytes_len_le: "La cantidad de bytes debe ser menor o igual a %value%".into(),
        bytes_len_btwn: "La cantidad de bytes debe estar entre %value_a% y %value_b%".into(),
        chars_len_eq: "La cantidad de caracteres debe ser igual a %value%".into(),
        chars_len_ne: "La cantidad de caracteres debe ser diferente de %value%".into(),
        chars_len_gt: "La cantidad de caracteres debe ser mayor que %value%".into(),
        chars_len_ge: "La cantidad de caracteres debe ser mayor o igual a %value%".into(),
        chars_len_lt: "La cantidad de caracteres debe ser menor que %value%".into(),
        chars_len_le: "La cantidad de caracteres debe ser menor o igual a %value%".into(),
        chars_len_btwn: "La cantidad de caracteres debe estar entre %value_a% y %value_b%".into(),
        graphemes_len_eq: "La cantidad de grafemas debe ser igual a %value%".into(),
        graphemes_len_ne: "La cantidad de grafemas debe ser diferente de %value%".into(),
        graphemes_len_gt: "La cantidad de grafemas debe ser mayor que %value%".into(),
        graphemes_len_ge: "La cantidad de grafemas debe ser mayor o igual a %value%".into(),
        graphemes_len_lt: "La cantidad de grafemas debe ser menor que %value%".into(),
        graphemes_len_le: "La cantidad de grafemas debe ser menor o igual a %value%".into(),
        graphemes_len_btwn: "La cantidad de grafemas debe estar entre %value_a% y %value_b%".into(),
        lowercase_len_eq: "La cantidad de caracteres en minúsculas debe ser igual a %value%".into(),
        lowercase_len_ne: "La cantidad de caracteres en minúsculas debe ser diferente de %value%".into(),
        lowercase_len_gt: "La cantidad de caracteres en minúsculas debe ser mayor que %value%".into(),
        lowercase_len_ge: "La cantidad de caracteres en minúsculas debe ser mayor o igual a %value%".into(),
        lowercase_len_lt: "La cantidad de caracteres en minúsculas debe ser menor que %value%".into(),
        lowercase_len_le: "La cantidad de caracteres en minúsculas debe ser menor o igual a %value%".into(),
        lowercase_len_btwn: "La cantidad de caracteres en minúsculas debe estar entre %value_a% y %value_b%".into(),
        uppercase_len_eq: "La cantidad de caracteres en mayúsculas debe ser igual a %value%".into(),
        uppercase_len_ne: "La cantidad de caracteres en mayúsculas debe ser diferente de %value%".into(),
        uppercase_len_gt: "La cantidad de caracteres en mayúsculas debe ser mayor que %value%".into(),
        uppercase_len_ge: "La cantidad de caracteres en mayúsculas debe ser mayor o igual a %value%".into(),
        uppercase_len_lt: "La cantidad de caracteres en mayúsculas debe ser menor que %value%".into(),
        uppercase_len_le: "La cantidad de caracteres en mayúsculas debe ser menor o igual a %value%".into(),
        uppercase_len_btwn: "La cantidad de caracteres en mayúsculas debe estar entre %value_a% y %value_b%".into(),
        number_len_eq: "La cantidad de números debe ser igual a %value%".into(),
        number_len_ne: "La cantidad de números debe ser diferente de %value%".into(),
        number_len_gt: "La cantidad de números debe ser mayor que %value%".into(),
        number_len_ge: "La cantidad de números debe ser mayor o igual a %value%".into(),
        number_len_lt: "La cantidad de números debe ser menor que %value%".into(),
        number_len_le: "La cantidad de números debe ser menor o igual a %value%".into(),
        number_len_btwn: "La cantidad de números debe estar entre %value_a% y %value_b%".into(),
        symbols_eq: "La cantidad de símbolos debe ser igual a %value%".into(),
        symbols_ne: "La cantidad de símbolos debe ser diferente de %value%".into(),
        symbols_gt: "La cantidad de símbolos debe ser mayor que %value%".into(),
        symbols_ge: "La cantidad de símbolos debe ser mayor o igual a %value%".into(),
        symbols_lt: "La cantidad de símbolos debe ser menor que %value%".into(),
        symbols_le: "La cantidad de símbolos debe ser menor o igual a %value%".into(),
        symbols_btwn: "La cantidad de símbolos debe estar entre %value_a% y %value_b%".into(),
    }
}

pub fn locale_en_long() -> Locale {
    Locale {
        required: "Is required".into(),
        u64: "Must be an unsigned integer".into(),
        i64: "Must be an integer".into(),
        f64: "Must be a float".into(),
        usize: "".into(),
        isize: "".into(),
        bool: "Must be a boolean".into(),
        str: "Must be a string".into(),
        email: "Must be an e-mail".into(),
        date: "Must be a date".into(),
        time: "Must be a time".into(),
        date_time: "Must be a date and time".into(),
        eq: "Must be equals to %value%".into(),
        ne: "Must be different from %value%".into(),
        gt: "Must be greater than %value%".into(),
        ge: "Must be greater than or equals to %value%".into(),
        lt: "Must be smaller than %value%".into(),
        le: "Must be smaller than or equals to %value%".into(),
        btwn: "Must be between %value_a% and %value_b%".into(),
        bytes_len_eq: "The length of bytes must be equals to %value%".into(),
        bytes_len_ne: "The length of bytes must be different from %value%".into(),
        bytes_len_gt: "The length of bytes must be greater than %value%".into(),
        bytes_len_ge: "The length of bytes must be greater than or equals to %value%".into(),
        bytes_len_lt: "The length of bytes must be smaller than %value%".into(),
        bytes_len_le: "The length of bytes must be smaller than or equals to %value%".into(),
        bytes_len_btwn: "The length of bytes must be between %value_a% and %value_b%".into(),
        chars_len_eq: "The length of characters must be equals to %value%".into(),
        chars_len_ne: "The length of characters must be different from %value%".into(),
        chars_len_gt: "The length of characters must be greater than %value%".into(),
        chars_len_ge: "The length of characters must be greater than or equals to %value%".into(),
        chars_len_lt: "The length of characters must be smaller than %value%".into(),
        chars_len_le: "The length of characters must be smaller than or equals to %value%".into(),
        chars_len_btwn: "The length of characters must be between %value_a% and %value_b%".into(),
        graphemes_len_eq: "The length of graphemes must be equals to %value%".into(),
        graphemes_len_ne: "The length of graphemes must be different from %value%".into(),
        graphemes_len_gt: "The length of graphemes must be greater than %value%".into(),
        graphemes_len_ge: "The length of graphemes must be greater than or equals to %value%".into(),
        graphemes_len_lt: "The length of graphemes must be smaller than %value%".into(),
        graphemes_len_le: "The length of graphemes must be smaller than or equals to %value%".into(),
        graphemes_len_btwn: "The length of graphemes must be between %value_a% and %value_b%".into(),
        lowercase_len_eq: "The length of lowercase characters must be equals to %value%".into(),
        lowercase_len_ne: "The length of lowercase characters must be different from %value%".into(),
        lowercase_len_gt: "The length of lowercase characters must be greater than %value%".into(),
        lowercase_len_ge: "The length of lowercase characters must be greater than or equals to %value%".into(),
        lowercase_len_lt: "The length of lowercase characters must be smaller than %value%".into(),
        lowercase_len_le: "The length of lowercase characters must be smaller than or equals to %value%".into(),
        lowercase_len_btwn: "The length of lowercase characters must be between %value_a% and %value_b%".into(),
        uppercase_len_eq: "The length of uppercase characters must be equals to %value%".into(),
        uppercase_len_ne: "The length of uppercase characters must be different from %value%".into(),
        uppercase_len_gt: "The length of uppercase characters must be greater than %value%".into(),
        uppercase_len_ge: "The length of uppercase characters must be greater than or equals to %value%".into(),
        uppercase_len_lt: "The length of uppercase characters must be smaller than %value%".into(),
        uppercase_len_le: "The length of uppercase characters must be smaller than or equals to %value%".into(),
        uppercase_len_btwn: "The length of uppercase characters must be between %value_a% and %value_b%".into(),
        number_len_eq: "The length of numbers must be equals to %value%".into(),
        number_len_ne: "The length of numbers must be different from %value%".into(),
        number_len_gt: "The length of numbers must be greater than %value%".into(),
        number_len_ge: "The length of numbers must be greater than or equals to %value%".into(),
        number_len_lt: "The length of numbers must be smaller than %value%".into(),
        number_len_le: "The length of numbers must be smaller than or equals to %value%".into(),
        number_len_btwn: "The length of numbers must be between %value_a% and %value_b%".into(),
        symbols_eq: "The length of symbols must be equals to %value%".into(),
        symbols_ne: "The length of symbols must be different from %value%".into(),
        symbols_gt: "The length of symbols must be greater than %value%".into(),
        symbols_ge: "The length of symbols must be greater than or equals to %value%".into(),
        symbols_lt: "The length of symbols must be smaller than %value%".into(),
        symbols_le: "The length of symbols must be smaller than or equals to %value%".into(),
        symbols_btwn: "The length of symbols must be between %value_a% and %value_b%".into(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
    };

    use super::{SchemaLocalizedErr, locale_en_long, locale_es_long, locale_pt_long, localize_schema_err, validation_err_to_locale};

    const REQUIRED: ValidationErr = ValidationErr::Required;
    const U64: ValidationErr = ValidationErr::U64;
    const I64: ValidationErr = ValidationErr::I64;
    const F64: ValidationErr = ValidationErr::F64;
    const USIZE: ValidationErr = ValidationErr::USize;
    const ISIZE: ValidationErr = ValidationErr::ISize;
    const BOOL: ValidationErr = ValidationErr::Bool;
    const STR: ValidationErr = ValidationErr::Str;
    const EMAIL: ValidationErr = ValidationErr::Email;
    const DATE: ValidationErr = ValidationErr::Date;
    const TIME: ValidationErr = ValidationErr::Time;
    const DATE_TIME: ValidationErr = ValidationErr::DateTime;

    const U64_VALUE_A: Operand = Operand::Value(OperandValue::U64(34));
    const U64_VALUE_B: Operand = Operand::Value(OperandValue::U64(43));
    const I64_VALUE_A: Operand = Operand::Value(OperandValue::I64(-4));
    const I64_VALUE_B: Operand = Operand::Value(OperandValue::I64(4));
    const F64_VALUE_A: Operand = Operand::Value(OperandValue::F64(-4.6));
    const F64_VALUE_B: Operand = Operand::Value(OperandValue::F64(-2.4));
    const USIZE_VALUE_A: Operand = Operand::Value(OperandValue::USize(27));
    const USIZE_VALUE_B: Operand = Operand::Value(OperandValue::USize(39));
    const BOOL_VALUE_A: Operand = Operand::Value(OperandValue::Bool(false));
    const BOOL_VALUE_B: Operand = Operand::Value(OperandValue::Bool(true));
    const OPERATION_U64_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(U64_VALUE_A));
    const OPERATION_U64_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(U64_VALUE_A));
    const OPERATION_U64_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(U64_VALUE_A));
    const OPERATION_U64_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(U64_VALUE_A));
    const OPERATION_U64_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(U64_VALUE_A));
    const OPERATION_U64_LE: ValidationErr = ValidationErr::Operation(Operation::Le(U64_VALUE_A));
    const OPERATION_U64_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(U64_VALUE_A, U64_VALUE_B));
    const OPERATION_I64_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(I64_VALUE_A));
    const OPERATION_I64_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(I64_VALUE_A));
    const OPERATION_I64_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(I64_VALUE_A));
    const OPERATION_I64_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(I64_VALUE_A));
    const OPERATION_I64_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(I64_VALUE_A));
    const OPERATION_I64_LE: ValidationErr = ValidationErr::Operation(Operation::Le(I64_VALUE_A));
    const OPERATION_I64_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(I64_VALUE_A, I64_VALUE_B));
    const OPERATION_F64_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(F64_VALUE_A));
    const OPERATION_F64_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(F64_VALUE_A));
    const OPERATION_F64_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(F64_VALUE_A));
    const OPERATION_F64_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(F64_VALUE_A));
    const OPERATION_F64_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(F64_VALUE_A));
    const OPERATION_F64_LE: ValidationErr = ValidationErr::Operation(Operation::Le(F64_VALUE_A));
    const OPERATION_F64_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(F64_VALUE_A, F64_VALUE_B));
    const OPERATION_BOOL_EQ: ValidationErr = ValidationErr::Operation(Operation::Eq(BOOL_VALUE_A));
    const OPERATION_BOOL_NE: ValidationErr = ValidationErr::Operation(Operation::Ne(BOOL_VALUE_A));
    const OPERATION_BOOL_GT: ValidationErr = ValidationErr::Operation(Operation::Gt(BOOL_VALUE_A));
    const OPERATION_BOOL_GE: ValidationErr = ValidationErr::Operation(Operation::Ge(BOOL_VALUE_A));
    const OPERATION_BOOL_LT: ValidationErr = ValidationErr::Operation(Operation::Lt(BOOL_VALUE_A));
    const OPERATION_BOOL_LE: ValidationErr = ValidationErr::Operation(Operation::Le(BOOL_VALUE_A));
    const OPERATION_BOOL_BTWN: ValidationErr = ValidationErr::Operation(Operation::Btwn(BOOL_VALUE_A, BOOL_VALUE_B));
    const BYTES_LEN_EQ: ValidationErr = ValidationErr::BytesLen(Operation::Eq(USIZE_VALUE_A));
    const BYTES_LEN_NE: ValidationErr = ValidationErr::BytesLen(Operation::Ne(USIZE_VALUE_A));
    const BYTES_LEN_GT: ValidationErr = ValidationErr::BytesLen(Operation::Gt(USIZE_VALUE_A));
    const BYTES_LEN_GE: ValidationErr = ValidationErr::BytesLen(Operation::Ge(USIZE_VALUE_A));
    const BYTES_LEN_LT: ValidationErr = ValidationErr::BytesLen(Operation::Lt(USIZE_VALUE_A));
    const BYTES_LEN_LE: ValidationErr = ValidationErr::BytesLen(Operation::Le(USIZE_VALUE_A));
    const BYTES_LEN_BTWN: ValidationErr = ValidationErr::BytesLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));
    const CHARS_LEN_EQ: ValidationErr = ValidationErr::CharsLen(Operation::Eq(USIZE_VALUE_A));
    const CHARS_LEN_NE: ValidationErr = ValidationErr::CharsLen(Operation::Ne(USIZE_VALUE_A));
    const CHARS_LEN_GT: ValidationErr = ValidationErr::CharsLen(Operation::Gt(USIZE_VALUE_A));
    const CHARS_LEN_GE: ValidationErr = ValidationErr::CharsLen(Operation::Ge(USIZE_VALUE_A));
    const CHARS_LEN_LT: ValidationErr = ValidationErr::CharsLen(Operation::Lt(USIZE_VALUE_A));
    const CHARS_LEN_LE: ValidationErr = ValidationErr::CharsLen(Operation::Le(USIZE_VALUE_A));
    const CHARS_LEN_BTWN: ValidationErr = ValidationErr::CharsLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));
    const GRAPHEMES_LEN_EQ: ValidationErr = ValidationErr::GraphemesLen(Operation::Eq(USIZE_VALUE_A));
    const GRAPHEMES_LEN_NE: ValidationErr = ValidationErr::GraphemesLen(Operation::Ne(USIZE_VALUE_A));
    const GRAPHEMES_LEN_GT: ValidationErr = ValidationErr::GraphemesLen(Operation::Gt(USIZE_VALUE_A));
    const GRAPHEMES_LEN_GE: ValidationErr = ValidationErr::GraphemesLen(Operation::Ge(USIZE_VALUE_A));
    const GRAPHEMES_LEN_LT: ValidationErr = ValidationErr::GraphemesLen(Operation::Lt(USIZE_VALUE_A));
    const GRAPHEMES_LEN_LE: ValidationErr = ValidationErr::GraphemesLen(Operation::Le(USIZE_VALUE_A));
    const GRAPHEMES_LEN_BTWN: ValidationErr = ValidationErr::GraphemesLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));
    const LOWERCASE_LEN_EQ: ValidationErr = ValidationErr::LowercaseLen(Operation::Eq(USIZE_VALUE_A));
    const LOWERCASE_LEN_NE: ValidationErr = ValidationErr::LowercaseLen(Operation::Ne(USIZE_VALUE_A));
    const LOWERCASE_LEN_GT: ValidationErr = ValidationErr::LowercaseLen(Operation::Gt(USIZE_VALUE_A));
    const LOWERCASE_LEN_GE: ValidationErr = ValidationErr::LowercaseLen(Operation::Ge(USIZE_VALUE_A));
    const LOWERCASE_LEN_LT: ValidationErr = ValidationErr::LowercaseLen(Operation::Lt(USIZE_VALUE_A));
    const LOWERCASE_LEN_LE: ValidationErr = ValidationErr::LowercaseLen(Operation::Le(USIZE_VALUE_A));
    const LOWERCASE_LEN_BTWN: ValidationErr = ValidationErr::LowercaseLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));
    const UPPERCASE_LEN_EQ: ValidationErr = ValidationErr::UppercaseLen(Operation::Eq(USIZE_VALUE_A));
    const UPPERCASE_LEN_NE: ValidationErr = ValidationErr::UppercaseLen(Operation::Ne(USIZE_VALUE_A));
    const UPPERCASE_LEN_GT: ValidationErr = ValidationErr::UppercaseLen(Operation::Gt(USIZE_VALUE_A));
    const UPPERCASE_LEN_GE: ValidationErr = ValidationErr::UppercaseLen(Operation::Ge(USIZE_VALUE_A));
    const UPPERCASE_LEN_LT: ValidationErr = ValidationErr::UppercaseLen(Operation::Lt(USIZE_VALUE_A));
    const UPPERCASE_LEN_LE: ValidationErr = ValidationErr::UppercaseLen(Operation::Le(USIZE_VALUE_A));
    const UPPERCASE_LEN_BTWN: ValidationErr = ValidationErr::UppercaseLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));
    const NUMBERS_LEN_EQ: ValidationErr = ValidationErr::NumbersLen(Operation::Eq(USIZE_VALUE_A));
    const NUMBERS_LEN_NE: ValidationErr = ValidationErr::NumbersLen(Operation::Ne(USIZE_VALUE_A));
    const NUMBERS_LEN_GT: ValidationErr = ValidationErr::NumbersLen(Operation::Gt(USIZE_VALUE_A));
    const NUMBERS_LEN_GE: ValidationErr = ValidationErr::NumbersLen(Operation::Ge(USIZE_VALUE_A));
    const NUMBERS_LEN_LT: ValidationErr = ValidationErr::NumbersLen(Operation::Lt(USIZE_VALUE_A));
    const NUMBERS_LEN_LE: ValidationErr = ValidationErr::NumbersLen(Operation::Le(USIZE_VALUE_A));
    const NUMBERS_LEN_BTWN: ValidationErr = ValidationErr::NumbersLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));
    const SYMBOLS_LEN_EQ: ValidationErr = ValidationErr::SymbolsLen(Operation::Eq(USIZE_VALUE_A));
    const SYMBOLS_LEN_NE: ValidationErr = ValidationErr::SymbolsLen(Operation::Ne(USIZE_VALUE_A));
    const SYMBOLS_LEN_GT: ValidationErr = ValidationErr::SymbolsLen(Operation::Gt(USIZE_VALUE_A));
    const SYMBOLS_LEN_GE: ValidationErr = ValidationErr::SymbolsLen(Operation::Ge(USIZE_VALUE_A));
    const SYMBOLS_LEN_LT: ValidationErr = ValidationErr::SymbolsLen(Operation::Lt(USIZE_VALUE_A));
    const SYMBOLS_LEN_LE: ValidationErr = ValidationErr::SymbolsLen(Operation::Le(USIZE_VALUE_A));
    const SYMBOLS_LEN_BTWN: ValidationErr = ValidationErr::SymbolsLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));

    #[test]
    fn test_schema_err_arr_to_locale() {
        let locale = locale_pt_long();
        let err = SchemaErr::arr([REQUIRED, BOOL, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(true))))]);
        let localized_err = SchemaLocalizedErr::Arr(vec!["É obrigatório".into(), "Deve ser um booleano".into(), "Deve ser igual a true".into()]);
        assert_eq!(localize_schema_err(&err, &locale), localized_err);
    }

    #[test]
    fn test_schema_err_obj_to_locale() {
        let locale = locale_pt_long();
        let err = SchemaErr::Obj(BTreeMap::from([
            (
                "name".into(),
                SchemaErr::arr([REQUIRED, STR, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("Paul McCartney"))))]),
            ),
            (
                "birthdate".into(),
                SchemaErr::arr([REQUIRED, STR, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("1942-06-18"))))]),
            ),
            ("alive".into(), SchemaErr::arr([REQUIRED, BOOL, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(true))))])),
            (
                "bands".into(),
                SchemaErr::arr([REQUIRED, STR, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("The Beatles"))))]),
            ),
        ]));
        let localized_err = SchemaLocalizedErr::Obj(BTreeMap::from([
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
        assert_eq!(localize_schema_err(&err, &locale), localized_err);
    }

    // TODO REMOVE FROM HERE
    #[test]
    fn test_localize_schema_err_to_locale() {
        assert_eq!(
            serde_json::to_string(&SchemaLocalizedErr::Obj(BTreeMap::from([
                ( "name".into(), SchemaLocalizedErr::Arr(vec![ "É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "Paul McCartney""#.into()])),
                ( "birthdate".into(), SchemaLocalizedErr::Arr(vec![ "É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "1942-06-18""#.into()])),
                ( "alive".into(), SchemaLocalizedErr::Arr(vec![ "É obrigatório".into(), "Deve ser um booleano".into(), "Deve ser igual a true".into()])),
                ( "bands".into(), SchemaLocalizedErr::Arr(vec![ "É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "The Beatles""#.into()])),
            ]))).unwrap(),
                r#"{"alive":["É obrigatório","Deve ser um booleano","Deve ser igual a true"],"bands":["É obrigatório","Deve ser uma string","Deve ser igual a \"The Beatles\""],"birthdate":["É obrigatório","Deve ser uma string","Deve ser igual a \"1942-06-18\""],"name":["É obrigatório","Deve ser uma string","Deve ser igual a \"Paul McCartney\""]}"#.to_string());
    }

    #[test]
    fn validation_err_to_locale_locale_pt_long() {
        let locale = locale_pt_long();

        let str_value_a = Operand::Value(OperandValue::from("aurorae"));
        let str_value_b = Operand::Value(OperandValue::from("crespúculum"));

        let operation_str_eq = ValidationErr::Operation(Operation::Eq(str_value_a.clone()));
        let operation_str_ne = ValidationErr::Operation(Operation::Ne(str_value_a.clone()));
        let operation_str_gt = ValidationErr::Operation(Operation::Gt(str_value_a.clone()));
        let operation_str_ge = ValidationErr::Operation(Operation::Ge(str_value_a.clone()));
        let operation_str_lt = ValidationErr::Operation(Operation::Lt(str_value_a.clone()));
        let operation_str_le = ValidationErr::Operation(Operation::Le(str_value_a.clone()));
        let operation_str_btwn = ValidationErr::Operation(Operation::Btwn(str_value_a, str_value_b));

        assert_eq!(validation_err_to_locale(&REQUIRED, &locale), "É obrigatório".to_string());
        assert_eq!(validation_err_to_locale(&U64, &locale), "Deve ser um número inteiro sem sinal".to_string());
        assert_eq!(validation_err_to_locale(&I64, &locale), "Deve ser um número inteiro".to_string());
        assert_eq!(validation_err_to_locale(&F64, &locale), "Deve ser um número com ponto flutuante".to_string());
        assert_eq!(validation_err_to_locale(&BOOL, &locale), "Deve ser um booleano".to_string());
        assert_eq!(validation_err_to_locale(&STR, &locale), "Deve ser uma string".to_string());
        assert_eq!(validation_err_to_locale(&EMAIL, &locale), "Deve ser um e-mail".to_string());
        assert_eq!(validation_err_to_locale(&DATE, &locale), "Deve ser uma data".to_string());
        assert_eq!(validation_err_to_locale(&TIME, &locale), "Deve ser uma hora".to_string());
        assert_eq!(validation_err_to_locale(&DATE_TIME, &locale), "Deve ser uma data e hora".to_string());

        assert_eq!(validation_err_to_locale(&OPERATION_U64_EQ, &locale), "Deve ser igual a 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_NE, &locale), "Deve ser diferente de 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_GT, &locale), "Deve ser maior que 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_GE, &locale), "Deve ser maior ou igual a 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_LT, &locale), "Deve ser menor que 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_LE, &locale), "Deve ser menor ou igual a 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_BTWN, &locale), "Deve estar entre 34 e 43".to_string());

        assert_eq!(validation_err_to_locale(&OPERATION_I64_EQ, &locale), "Deve ser igual a -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_NE, &locale), "Deve ser diferente de -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_GT, &locale), "Deve ser maior que -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_GE, &locale), "Deve ser maior ou igual a -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_LT, &locale), "Deve ser menor que -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_LE, &locale), "Deve ser menor ou igual a -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_BTWN, &locale), "Deve estar entre -4 e 4".to_string());

        assert_eq!(validation_err_to_locale(&OPERATION_F64_EQ, &locale), "Deve ser igual a -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_NE, &locale), "Deve ser diferente de -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_GT, &locale), "Deve ser maior que -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_GE, &locale), "Deve ser maior ou igual a -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_LT, &locale), "Deve ser menor que -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_LE, &locale), "Deve ser menor ou igual a -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_BTWN, &locale), "Deve estar entre -4.6 e -2.4".to_string());

        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_EQ, &locale), "Deve ser igual a false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_NE, &locale), "Deve ser diferente de false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_GT, &locale), "Deve ser maior que false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_GE, &locale), "Deve ser maior ou igual a false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_LT, &locale), "Deve ser menor que false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_LE, &locale), "Deve ser menor ou igual a false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_BTWN, &locale), "Deve estar entre false e true".to_string());

        assert_eq!(validation_err_to_locale(&operation_str_eq, &locale), r#"Deve ser igual a "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_ne, &locale), r#"Deve ser diferente de "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_gt, &locale), r#"Deve ser maior que "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_ge, &locale), r#"Deve ser maior ou igual a "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_lt, &locale), r#"Deve ser menor que "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_le, &locale), r#"Deve ser menor ou igual a "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_btwn, &locale), r#"Deve estar entre "aurorae" e "crespúculum""#.to_string());

        assert_eq!(validation_err_to_locale(&BYTES_LEN_EQ, &locale), "A quantidade de bytes deve ser igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_NE, &locale), "A quantidade de bytes deve ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_GT, &locale), "A quantidade de bytes deve ser maior que 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_GE, &locale), "A quantidade de bytes deve ser maior ou igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_LT, &locale), "A quantidade de bytes deve ser menor que 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_LE, &locale), "A quantidade de bytes deve ser menor ou igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_BTWN, &locale), "A quantidade de bytes deve estar entre 27 e 39".to_string());

        assert_eq!(validation_err_to_locale(&CHARS_LEN_EQ, &locale), "A quantidade de caracteres deve ser igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_NE, &locale), "A quantidade de caracteres deve ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_GT, &locale), "A quantidade de caracteres deve ser maior que 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_GE, &locale), "A quantidade de caracteres deve ser maior ou igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_LT, &locale), "A quantidade de caracteres deve ser menor que 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_LE, &locale), "A quantidade de caracteres deve ser menor ou igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_BTWN, &locale), "A quantidade de caracteres deve estar entre 27 e 39".to_string());

        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_EQ, &locale), "A quantidade de grafemas deve ser igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_NE, &locale), "A quantidade de grafemas deve ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_GT, &locale), "A quantidade de grafemas deve ser maior que 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_GE, &locale), "A quantidade de grafemas deve ser maior ou igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_LT, &locale), "A quantidade de grafemas deve ser menor que 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_LE, &locale), "A quantidade de grafemas deve ser menor ou igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_BTWN, &locale), "A quantidade de grafemas deve estar entre 27 e 39".to_string());

        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_EQ, &locale), "A quantidade de caracteres minúsculos deve ser igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_NE, &locale), "A quantidade de caracteres minúsculos deve ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_GT, &locale), "A quantidade de caracteres minúsculos deve ser maior que 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_GE, &locale), "A quantidade de caracteres minúsculos deve ser maior ou igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_LT, &locale), "A quantidade de caracteres minúsculos deve ser menor que 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_LE, &locale), "A quantidade de caracteres minúsculos deve ser menor ou igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_BTWN, &locale), "A quantidade de caracteres minúsculos deve estar entre 27 e 39".to_string());

        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_EQ, &locale), "A quantidade de caracteres maiúsculos deve ser igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_NE, &locale), "A quantidade de caracteres maiúsculos deve ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_GT, &locale), "A quantidade de caracteres maiúsculos deve ser maior que 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_GE, &locale), "A quantidade de caracteres maiúsculos deve ser maior ou igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_LT, &locale), "A quantidade de caracteres maiúsculos deve ser menor que 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_LE, &locale), "A quantidade de caracteres maiúsculos deve ser menor ou igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_BTWN, &locale), "A quantidade de caracteres maiúsculos deve estar entre 27 e 39".to_string());

        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_EQ, &locale), "A quantidade de números deve ser igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_NE, &locale), "A quantidade de números deve ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_GT, &locale), "A quantidade de números deve ser maior que 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_GE, &locale), "A quantidade de números deve ser maior ou igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_LT, &locale), "A quantidade de números deve ser menor que 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_LE, &locale), "A quantidade de números deve ser menor ou igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_BTWN, &locale), "A quantidade de números deve estar entre 27 e 39".to_string());

        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_EQ, &locale), "A quantidade de símbolos deve ser igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_NE, &locale), "A quantidade de símbolos deve ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_GT, &locale), "A quantidade de símbolos deve ser maior que 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_GE, &locale), "A quantidade de símbolos deve ser maior ou igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_LT, &locale), "A quantidade de símbolos deve ser menor que 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_LE, &locale), "A quantidade de símbolos deve ser menor ou igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_BTWN, &locale), "A quantidade de símbolos deve estar entre 27 e 39".to_string());
    }

    #[test]
    fn validation_err_to_locale_locale_es_long() {
        let locale = locale_es_long();

        let str_value_a = Operand::Value(OperandValue::from("aurorae"));
        let str_value_b = Operand::Value(OperandValue::from("crespúculum"));

        let operation_str_eq = ValidationErr::Operation(Operation::Eq(str_value_a.clone()));
        let operation_str_ne = ValidationErr::Operation(Operation::Ne(str_value_a.clone()));
        let operation_str_gt = ValidationErr::Operation(Operation::Gt(str_value_a.clone()));
        let operation_str_ge = ValidationErr::Operation(Operation::Ge(str_value_a.clone()));
        let operation_str_lt = ValidationErr::Operation(Operation::Lt(str_value_a.clone()));
        let operation_str_le = ValidationErr::Operation(Operation::Le(str_value_a.clone()));
        let operation_str_btwn = ValidationErr::Operation(Operation::Btwn(str_value_a, str_value_b));

        assert_eq!(validation_err_to_locale(&REQUIRED, &locale), "Se requiere".to_string());
        assert_eq!(validation_err_to_locale(&U64, &locale), "Debe ser un número entero sin signo".to_string());
        assert_eq!(validation_err_to_locale(&I64, &locale), "Debe ser un número entero".to_string());
        assert_eq!(validation_err_to_locale(&F64, &locale), "Debe ser un número de punto flotante".to_string());
        assert_eq!(validation_err_to_locale(&BOOL, &locale), "Debe ser un booleano".to_string());
        assert_eq!(validation_err_to_locale(&STR, &locale), "Debe ser una cadena".to_string());
        assert_eq!(validation_err_to_locale(&EMAIL, &locale), "Debe ser un correo electrónico".to_string());
        assert_eq!(validation_err_to_locale(&DATE, &locale), "Debe ser una fecha".to_string());
        assert_eq!(validation_err_to_locale(&TIME, &locale), "Debe ser una hora".to_string());
        assert_eq!(validation_err_to_locale(&DATE_TIME, &locale), "Debe ser una fecha y hora".to_string());

        assert_eq!(validation_err_to_locale(&OPERATION_U64_EQ, &locale), "Debe ser igual a 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_NE, &locale), "Debe ser diferente de 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_GT, &locale), "Debe ser mayor que 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_GE, &locale), "Debe ser mayor o igual a 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_LT, &locale), "Debe ser menor que 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_LE, &locale), "Debe ser menor o igual a 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_BTWN, &locale), "Debe estar entre 34 y 43".to_string());

        assert_eq!(validation_err_to_locale(&OPERATION_I64_EQ, &locale), "Debe ser igual a -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_NE, &locale), "Debe ser diferente de -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_GT, &locale), "Debe ser mayor que -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_GE, &locale), "Debe ser mayor o igual a -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_LT, &locale), "Debe ser menor que -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_LE, &locale), "Debe ser menor o igual a -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_BTWN, &locale), "Debe estar entre -4 y 4".to_string());

        assert_eq!(validation_err_to_locale(&OPERATION_F64_EQ, &locale), "Debe ser igual a -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_NE, &locale), "Debe ser diferente de -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_GT, &locale), "Debe ser mayor que -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_GE, &locale), "Debe ser mayor o igual a -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_LT, &locale), "Debe ser menor que -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_LE, &locale), "Debe ser menor o igual a -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_BTWN, &locale), "Debe estar entre -4.6 y -2.4".to_string());

        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_EQ, &locale), "Debe ser igual a false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_NE, &locale), "Debe ser diferente de false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_GT, &locale), "Debe ser mayor que false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_GE, &locale), "Debe ser mayor o igual a false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_LT, &locale), "Debe ser menor que false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_LE, &locale), "Debe ser menor o igual a false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_BTWN, &locale), "Debe estar entre false y true".to_string());

        assert_eq!(validation_err_to_locale(&operation_str_eq, &locale), r#"Debe ser igual a "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_ne, &locale), r#"Debe ser diferente de "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_gt, &locale), r#"Debe ser mayor que "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_ge, &locale), r#"Debe ser mayor o igual a "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_lt, &locale), r#"Debe ser menor que "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_le, &locale), r#"Debe ser menor o igual a "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_btwn, &locale), r#"Debe estar entre "aurorae" y "crespúculum""#.to_string());

        assert_eq!(validation_err_to_locale(&BYTES_LEN_EQ, &locale), "La cantidad de bytes debe ser igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_NE, &locale), "La cantidad de bytes debe ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_GT, &locale), "La cantidad de bytes debe ser mayor que 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_GE, &locale), "La cantidad de bytes debe ser mayor o igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_LT, &locale), "La cantidad de bytes debe ser menor que 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_LE, &locale), "La cantidad de bytes debe ser menor o igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_BTWN, &locale), "La cantidad de bytes debe estar entre 27 y 39".to_string());

        assert_eq!(validation_err_to_locale(&CHARS_LEN_EQ, &locale), "La cantidad de caracteres debe ser igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_NE, &locale), "La cantidad de caracteres debe ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_GT, &locale), "La cantidad de caracteres debe ser mayor que 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_GE, &locale), "La cantidad de caracteres debe ser mayor o igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_LT, &locale), "La cantidad de caracteres debe ser menor que 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_LE, &locale), "La cantidad de caracteres debe ser menor o igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_BTWN, &locale), "La cantidad de caracteres debe estar entre 27 y 39".to_string());

        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_EQ, &locale), "La cantidad de grafemas debe ser igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_NE, &locale), "La cantidad de grafemas debe ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_GT, &locale), "La cantidad de grafemas debe ser mayor que 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_GE, &locale), "La cantidad de grafemas debe ser mayor o igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_LT, &locale), "La cantidad de grafemas debe ser menor que 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_LE, &locale), "La cantidad de grafemas debe ser menor o igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_BTWN, &locale), "La cantidad de grafemas debe estar entre 27 y 39".to_string());

        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_EQ , &locale), "La cantidad de caracteres en minúsculas debe ser igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_NE , &locale), "La cantidad de caracteres en minúsculas debe ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_GT , &locale), "La cantidad de caracteres en minúsculas debe ser mayor que 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_GE , &locale), "La cantidad de caracteres en minúsculas debe ser mayor o igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_LT , &locale), "La cantidad de caracteres en minúsculas debe ser menor que 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_LE , &locale), "La cantidad de caracteres en minúsculas debe ser menor o igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_BTWN , &locale), "La cantidad de caracteres en minúsculas debe estar entre 27 y 39".to_string());

        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_EQ , &locale), "La cantidad de caracteres en mayúsculas debe ser igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_NE , &locale), "La cantidad de caracteres en mayúsculas debe ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_GT , &locale), "La cantidad de caracteres en mayúsculas debe ser mayor que 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_GE , &locale), "La cantidad de caracteres en mayúsculas debe ser mayor o igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_LT , &locale), "La cantidad de caracteres en mayúsculas debe ser menor que 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_LE , &locale), "La cantidad de caracteres en mayúsculas debe ser menor o igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_BTWN , &locale), "La cantidad de caracteres en mayúsculas debe estar entre 27 y 39".to_string());

        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_EQ, &locale), "La cantidad de números debe ser igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_NE, &locale), "La cantidad de números debe ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_GT, &locale), "La cantidad de números debe ser mayor que 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_GE, &locale), "La cantidad de números debe ser mayor o igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_LT, &locale), "La cantidad de números debe ser menor que 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_LE, &locale), "La cantidad de números debe ser menor o igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_BTWN, &locale), "La cantidad de números debe estar entre 27 y 39".to_string());

        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_EQ, &locale), "La cantidad de símbolos debe ser igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_NE, &locale), "La cantidad de símbolos debe ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_GT, &locale), "La cantidad de símbolos debe ser mayor que 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_GE, &locale), "La cantidad de símbolos debe ser mayor o igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_LT, &locale), "La cantidad de símbolos debe ser menor que 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_LE, &locale), "La cantidad de símbolos debe ser menor o igual a 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_BTWN, &locale), "La cantidad de símbolos debe estar entre 27 y 39".to_string());
    }

    #[test]
    fn validation_err_to_locale_locale_en_long() {
        let locale = locale_en_long();

        let str_value_a = Operand::Value(OperandValue::from("aurorae"));
        let str_value_b = Operand::Value(OperandValue::from("crespúculum"));

        let operation_str_eq = ValidationErr::Operation(Operation::Eq(str_value_a.clone()));
        let operation_str_ne = ValidationErr::Operation(Operation::Ne(str_value_a.clone()));
        let operation_str_gt = ValidationErr::Operation(Operation::Gt(str_value_a.clone()));
        let operation_str_ge = ValidationErr::Operation(Operation::Ge(str_value_a.clone()));
        let operation_str_lt = ValidationErr::Operation(Operation::Lt(str_value_a.clone()));
        let operation_str_le = ValidationErr::Operation(Operation::Le(str_value_a.clone()));
        let operation_str_btwn = ValidationErr::Operation(Operation::Btwn(str_value_a, str_value_b));

        assert_eq!(validation_err_to_locale(&REQUIRED, &locale), "Is required".to_string());
        assert_eq!(validation_err_to_locale(&U64, &locale), "Must be an unsigned integer".to_string());
        assert_eq!(validation_err_to_locale(&I64, &locale), "Must be an integer".to_string());
        assert_eq!(validation_err_to_locale(&F64, &locale), "Must be a float".to_string());
        assert_eq!(validation_err_to_locale(&BOOL, &locale), "Must be a boolean".to_string());
        assert_eq!(validation_err_to_locale(&STR, &locale), "Must be a string".to_string());
        assert_eq!(validation_err_to_locale(&EMAIL, &locale), "Must be an e-mail".to_string());
        assert_eq!(validation_err_to_locale(&DATE, &locale), "Must be a date".to_string());
        assert_eq!(validation_err_to_locale(&TIME, &locale), "Must be a time".to_string());
        assert_eq!(validation_err_to_locale(&DATE_TIME, &locale), "Must be a date and time".to_string());

        assert_eq!(validation_err_to_locale(&OPERATION_U64_EQ, &locale), "Must be equals to 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_NE, &locale), "Must be different from 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_GT, &locale), "Must be greater than 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_GE, &locale), "Must be greater than or equals to 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_LT, &locale), "Must be smaller than 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_LE, &locale), "Must be smaller than or equals to 34".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_U64_BTWN, &locale), "Must be between 34 and 43".to_string());

        assert_eq!(validation_err_to_locale(&OPERATION_I64_EQ, &locale), "Must be equals to -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_NE, &locale), "Must be different from -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_GT, &locale), "Must be greater than -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_GE, &locale), "Must be greater than or equals to -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_LT, &locale), "Must be smaller than -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_LE, &locale), "Must be smaller than or equals to -4".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_I64_BTWN, &locale), "Must be between -4 and 4".to_string());

        assert_eq!(validation_err_to_locale(&OPERATION_F64_EQ, &locale), "Must be equals to -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_NE, &locale), "Must be different from -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_GT, &locale), "Must be greater than -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_GE, &locale), "Must be greater than or equals to -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_LT, &locale), "Must be smaller than -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_LE, &locale), "Must be smaller than or equals to -4.6".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_F64_BTWN, &locale), "Must be between -4.6 and -2.4".to_string());

        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_EQ, &locale), "Must be equals to false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_NE, &locale), "Must be different from false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_GT, &locale), "Must be greater than false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_GE, &locale), "Must be greater than or equals to false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_LT, &locale), "Must be smaller than false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_LE, &locale), "Must be smaller than or equals to false".to_string());
        assert_eq!(validation_err_to_locale(&OPERATION_BOOL_BTWN, &locale), "Must be between false and true".to_string());

        assert_eq!(validation_err_to_locale(&operation_str_eq, &locale), r#"Must be equals to "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_ne, &locale), r#"Must be different from "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_gt, &locale), r#"Must be greater than "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_ge, &locale), r#"Must be greater than or equals to "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_lt, &locale), r#"Must be smaller than "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_le, &locale), r#"Must be smaller than or equals to "aurorae""#.to_string());
        assert_eq!(validation_err_to_locale(&operation_str_btwn, &locale), r#"Must be between "aurorae" and "crespúculum""#.to_string());

        assert_eq!(validation_err_to_locale(&BYTES_LEN_EQ, &locale), "The length of bytes must be equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_NE, &locale), "The length of bytes must be different from 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_GT, &locale), "The length of bytes must be greater than 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_GE, &locale), "The length of bytes must be greater than or equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_LT, &locale), "The length of bytes must be smaller than 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_LE, &locale), "The length of bytes must be smaller than or equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&BYTES_LEN_BTWN, &locale), "The length of bytes must be between 27 and 39".to_string());

        assert_eq!(validation_err_to_locale(&CHARS_LEN_EQ, &locale), "The length of characters must be equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_NE, &locale), "The length of characters must be different from 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_GT, &locale), "The length of characters must be greater than 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_GE, &locale), "The length of characters must be greater than or equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_LT, &locale), "The length of characters must be smaller than 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_LE, &locale), "The length of characters must be smaller than or equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&CHARS_LEN_BTWN, &locale), "The length of characters must be between 27 and 39".to_string());

        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_EQ, &locale), "The length of graphemes must be equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_NE, &locale), "The length of graphemes must be different from 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_GT, &locale), "The length of graphemes must be greater than 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_GE, &locale), "The length of graphemes must be greater than or equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_LT, &locale), "The length of graphemes must be smaller than 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_LE, &locale), "The length of graphemes must be smaller than or equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&GRAPHEMES_LEN_BTWN, &locale), "The length of graphemes must be between 27 and 39".to_string());

        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_EQ, &locale), "The length of lowercase characters must be equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_NE, &locale), "The length of lowercase characters must be different from 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_GT, &locale), "The length of lowercase characters must be greater than 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_GE, &locale), "The length of lowercase characters must be greater than or equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_LT, &locale), "The length of lowercase characters must be smaller than 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_LE, &locale), "The length of lowercase characters must be smaller than or equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&LOWERCASE_LEN_BTWN, &locale), "The length of lowercase characters must be between 27 and 39".to_string());

        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_EQ, &locale), "The length of uppercase characters must be equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_NE, &locale), "The length of uppercase characters must be different from 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_GT, &locale), "The length of uppercase characters must be greater than 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_GE, &locale), "The length of uppercase characters must be greater than or equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_LT, &locale), "The length of uppercase characters must be smaller than 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_LE, &locale), "The length of uppercase characters must be smaller than or equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&UPPERCASE_LEN_BTWN, &locale), "The length of uppercase characters must be between 27 and 39".to_string());

        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_EQ, &locale), "The length of numbers must be equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_NE, &locale), "The length of numbers must be different from 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_GT, &locale), "The length of numbers must be greater than 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_GE, &locale), "The length of numbers must be greater than or equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_LT, &locale), "The length of numbers must be smaller than 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_LE, &locale), "The length of numbers must be smaller than or equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&NUMBERS_LEN_BTWN, &locale), "The length of numbers must be between 27 and 39".to_string());

        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_EQ, &locale), "The length of symbols must be equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_NE, &locale), "The length of symbols must be different from 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_GT, &locale), "The length of symbols must be greater than 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_GE, &locale), "The length of symbols must be greater than or equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_LT, &locale), "The length of symbols must be smaller than 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_LE, &locale), "The length of symbols must be smaller than or equals to 27".to_string());
        assert_eq!(validation_err_to_locale(&SYMBOLS_LEN_BTWN, &locale), "The length of symbols must be between 27 and 39".to_string());
    }
}

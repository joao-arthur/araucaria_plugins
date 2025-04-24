use serde::{Deserialize, Serialize, Serializer};
use std::collections::BTreeMap;

use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{Operand, OperandValue, Operation},
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

pub fn schema_err_to_locale(err: &SchemaErr, locale: &Locale) -> SchemaLocalizedErr {
    match err {
        SchemaErr::Validation(arr) => SchemaLocalizedErr::Arr(arr.iter().map(|item| validation_err_to_locale(item, locale)).collect()),
        SchemaErr::Obj(obj) => {
            let mut result: BTreeMap<String, SchemaLocalizedErr> = BTreeMap::new();
            for (key, item) in obj {
                result.insert(key.clone(), schema_err_to_locale(item, locale));
            }
            SchemaLocalizedErr::Obj(result)
        }
    }
}

pub fn operand_value_to_string(value: &OperandValue) -> String {
    match value {
        OperandValue::U64(val) => val.to_string(),
        OperandValue::I64(val) => val.to_string(),
        OperandValue::F64(val) => val.to_string(),
        OperandValue::USize(val) => val.to_string(),
        OperandValue::Bool(val) => val.to_string(),
        OperandValue::Str(val) => "\"".to_string() + val + "\"",
    }
}

pub fn operand_to_string(operand: &Operand) -> String {
    match operand {
        Operand::Value(value) => operand_value_to_string(value),
        Operand::FieldPath(path) => "\"".to_string() + path + "\"",
    }
}

pub fn validation_err_to_locale(error: &ValidationErr, locale: &Locale) -> String {
    match error {
        ValidationErr::Required => locale.required.clone(),
        ValidationErr::U64 => locale.num_u.clone(),
        ValidationErr::I64 => locale.num_i.clone(),
        ValidationErr::F64 => locale.num_f.clone(),
        ValidationErr::Bool => locale.bool.clone(),
        ValidationErr::Str => locale.str.clone(),
        ValidationErr::Email => locale.email.clone(),
        ValidationErr::Date => locale.date.clone(),
        ValidationErr::Time => locale.time.clone(),
        ValidationErr::DateTime => locale.date_time.clone(),
        ValidationErr::Operation(operation) => match operation {
            Operation::Eq(v) => locale.eq.replace("%value%", &operand_to_string(v)),
            Operation::Ne(v) => locale.ne.replace("%value%", &operand_to_string(v)),
            Operation::Gt(v) => locale.gt.replace("%value%", &operand_to_string(v)),
            Operation::Ge(v) => locale.ge.replace("%value%", &operand_to_string(v)),
            Operation::Lt(v) => locale.lt.replace("%value%", &operand_to_string(v)),
            Operation::Le(v) => locale.le.replace("%value%", &operand_to_string(v)),
            Operation::Btwn(a, b) => locale.btwn.replace("%value_a%", &operand_to_string(a)).replace("%value_b%", &operand_to_string(b)),
        },
        ValidationErr::BytesLen(operation) => match operation {
            Operation::Eq(v) => locale.bytes_len_eq.replace("%value%", &operand_to_string(v)),
            Operation::Ne(v) => locale.bytes_len_ne.replace("%value%", &operand_to_string(v)),
            Operation::Gt(v) => locale.bytes_len_gt.replace("%value%", &operand_to_string(v)),
            Operation::Ge(v) => locale.bytes_len_ge.replace("%value%", &operand_to_string(v)),
            Operation::Lt(v) => locale.bytes_len_lt.replace("%value%", &operand_to_string(v)),
            Operation::Le(v) => locale.bytes_len_le.replace("%value%", &operand_to_string(v)),
            Operation::Btwn(a, b) => locale.bytes_len_btwn.replace("%value_a%", &operand_to_string(a)).replace("%value_b%", &operand_to_string(b)),
        },
        ValidationErr::CharsLen(operation) => match operation {
            Operation::Eq(v) => locale.chars_len_eq.replace("%value%", &operand_to_string(v)),
            Operation::Ne(v) => locale.chars_len_ne.replace("%value%", &operand_to_string(v)),
            Operation::Gt(v) => locale.chars_len_gt.replace("%value%", &operand_to_string(v)),
            Operation::Ge(v) => locale.chars_len_ge.replace("%value%", &operand_to_string(v)),
            Operation::Lt(v) => locale.chars_len_lt.replace("%value%", &operand_to_string(v)),
            Operation::Le(v) => locale.chars_len_le.replace("%value%", &operand_to_string(v)),
            Operation::Btwn(a, b) => locale.chars_len_btwn.replace("%value_a%", &operand_to_string(a)).replace("%value_b%", &operand_to_string(b)),
        },
        ValidationErr::GraphemesLen(operation) => match operation {
            Operation::Eq(v) => locale.graphemes_len_eq.replace("%value%", &operand_to_string(v)),
            Operation::Ne(v) => locale.graphemes_len_ne.replace("%value%", &operand_to_string(v)),
            Operation::Gt(v) => locale.graphemes_len_gt.replace("%value%", &operand_to_string(v)),
            Operation::Ge(v) => locale.graphemes_len_ge.replace("%value%", &operand_to_string(v)),
            Operation::Lt(v) => locale.graphemes_len_lt.replace("%value%", &operand_to_string(v)),
            Operation::Le(v) => locale.graphemes_len_le.replace("%value%", &operand_to_string(v)),
            Operation::Btwn(a, b) => {
                locale.graphemes_len_btwn.replace("%value_a%", &operand_to_string(a)).replace("%value_b%", &operand_to_string(b))
            }
        },
        ValidationErr::LowercaseLen(operation) => match operation {
            Operation::Eq(v) => locale.lowercase_len_eq.replace("%value%", &operand_to_string(v)),
            Operation::Ne(v) => locale.lowercase_len_ne.replace("%value%", &operand_to_string(v)),
            Operation::Gt(v) => locale.lowercase_len_gt.replace("%value%", &operand_to_string(v)),
            Operation::Ge(v) => locale.lowercase_len_ge.replace("%value%", &operand_to_string(v)),
            Operation::Lt(v) => locale.lowercase_len_lt.replace("%value%", &operand_to_string(v)),
            Operation::Le(v) => locale.lowercase_len_le.replace("%value%", &operand_to_string(v)),
            Operation::Btwn(a, b) => {
                locale.lowercase_len_btwn.replace("%value_a%", &operand_to_string(a)).replace("%value_b%", &operand_to_string(b))
            }
        },
        ValidationErr::UppercaseLen(operation) => match operation {
            Operation::Eq(v) => locale.uppercase_len_eq.replace("%value%", &operand_to_string(v)),
            Operation::Ne(v) => locale.uppercase_len_ne.replace("%value%", &operand_to_string(v)),
            Operation::Gt(v) => locale.uppercase_len_gt.replace("%value%", &operand_to_string(v)),
            Operation::Ge(v) => locale.uppercase_len_ge.replace("%value%", &operand_to_string(v)),
            Operation::Lt(v) => locale.uppercase_len_lt.replace("%value%", &operand_to_string(v)),
            Operation::Le(v) => locale.uppercase_len_le.replace("%value%", &operand_to_string(v)),
            Operation::Btwn(a, b) => {
                locale.uppercase_len_btwn.replace("%value_a%", &operand_to_string(a)).replace("%value_b%", &operand_to_string(b))
            }
        },
        ValidationErr::NumbersLen(operation) => match operation {
            Operation::Eq(v) => locale.number_len_eq.replace("%value%", &operand_to_string(v)),
            Operation::Ne(v) => locale.number_len_ne.replace("%value%", &operand_to_string(v)),
            Operation::Gt(v) => locale.number_len_gt.replace("%value%", &operand_to_string(v)),
            Operation::Ge(v) => locale.number_len_ge.replace("%value%", &operand_to_string(v)),
            Operation::Lt(v) => locale.number_len_lt.replace("%value%", &operand_to_string(v)),
            Operation::Le(v) => locale.number_len_le.replace("%value%", &operand_to_string(v)),
            Operation::Btwn(a, b) => locale.number_len_btwn.replace("%value_a%", &operand_to_string(a)).replace("%value_b%", &operand_to_string(b)),
        },
        ValidationErr::SymbolsLen(operation) => match operation {
            Operation::Eq(v) => locale.symbols_eq.replace("%value%", &operand_to_string(v)),
            Operation::Ne(v) => locale.symbols_ne.replace("%value%", &operand_to_string(v)),
            Operation::Gt(v) => locale.symbols_gt.replace("%value%", &operand_to_string(v)),
            Operation::Ge(v) => locale.symbols_ge.replace("%value%", &operand_to_string(v)),
            Operation::Lt(v) => locale.symbols_lt.replace("%value%", &operand_to_string(v)),
            Operation::Le(v) => locale.symbols_le.replace("%value%", &operand_to_string(v)),
            Operation::Btwn(a, b) => locale.symbols_btwn.replace("%value_a%", &operand_to_string(a)).replace("%value_b%", &operand_to_string(b)),
        },
    }
}

pub fn locale_pt_long() -> Locale {
    Locale {
        required: "É obrigatório".into(),
        num_u: "Deve ser um número inteiro sem sinal".into(),
        num_i: "Deve ser um número inteiro".into(),
        num_f: "Deve ser um número com ponto flutuante".into(),
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
        num_u: "Debe ser un número entero sin signo".into(),
        num_i: "Debe ser un número entero".into(),
        num_f: "Debe ser un número de punto flotante".into(),
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
        num_u: "Must be an unsigned integer".into(),
        num_i: "Must be an integer".into(),
        num_f: "Must be a float".into(),
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
mod test {
    use std::collections::BTreeMap;

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
    };

    use super::{SchemaLocalizedErr, locale_en_long, locale_es_long, locale_pt_long, schema_err_to_locale, validation_err_to_locale};

    #[test]
    fn test_schema_err_to_locale() {
        let locale = locale_pt_long();
        assert_eq!(
            schema_err_to_locale(
                &SchemaErr::Validation(vec![
                    ValidationErr::Required,
                    ValidationErr::Bool,
                    ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(true))))
                ]),
                &locale
            ),
            SchemaLocalizedErr::Arr(vec!["É obrigatório".into(), "Deve ser um booleano".into(), "Deve ser igual a true".into()])
        );
        assert_eq!(
            schema_err_to_locale(
                &SchemaErr::Obj(BTreeMap::from([
                    (
                        "name".into(),
                        SchemaErr::Validation(vec![
                            ValidationErr::Required,
                            ValidationErr::Str,
                            ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("Paul McCartney"))))
                        ])
                    ),
                    (
                        "birthdate".into(),
                        SchemaErr::Validation(vec![
                            ValidationErr::Required,
                            ValidationErr::Str,
                            ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("1942-06-18"))))
                        ])
                    ),
                    (
                        "alive".into(),
                        SchemaErr::Validation(vec![
                            ValidationErr::Required,
                            ValidationErr::Bool,
                            ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(true))))
                        ])
                    ),
                    (
                        "bands".into(),
                        SchemaErr::Validation(vec![
                            ValidationErr::Required,
                            ValidationErr::Str,
                            ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("The Beatles"))))
                        ])
                    ),
                ])),
                &locale
            ),
            SchemaLocalizedErr::Obj(BTreeMap::from([
                (
                    "name".into(),
                    SchemaLocalizedErr::Arr(vec![
                        "É obrigatório".into(),
                        "Deve ser uma string".into(),
                        r#"Deve ser igual a "Paul McCartney""#.into()
                    ])
                ),
                (
                    "birthdate".into(),
                    SchemaLocalizedErr::Arr(vec!["É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "1942-06-18""#.into()])
                ),
                (
                    "alive".into(),
                    SchemaLocalizedErr::Arr(vec!["É obrigatório".into(), "Deve ser um booleano".into(), "Deve ser igual a true".into()])
                ),
                (
                    "bands".into(),
                    SchemaLocalizedErr::Arr(vec!["É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "The Beatles""#.into()])
                ),
            ]))
        );
        assert_eq!(
            serde_json::to_string(&SchemaLocalizedErr::Obj(BTreeMap::from([
                ( "name".into(), SchemaLocalizedErr::Arr(vec![ "É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "Paul McCartney""#.into()])),
                ( "birthdate".into(), SchemaLocalizedErr::Arr(vec![ "É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "1942-06-18""#.into()])),
                ( "alive".into(), SchemaLocalizedErr::Arr(vec![ "É obrigatório".into(), "Deve ser um booleano".into(), "Deve ser igual a true".into()])),
                ( "bands".into(), SchemaLocalizedErr::Arr(vec![ "É obrigatório".into(), "Deve ser uma string".into(), r#"Deve ser igual a "The Beatles""#.into()])),
            ]))).unwrap(),
                r#"{"alive":["É obrigatório","Deve ser um booleano","Deve ser igual a true"],"bands":["É obrigatório","Deve ser uma string","Deve ser igual a \"The Beatles\""],"birthdate":["É obrigatório","Deve ser uma string","Deve ser igual a \"1942-06-18\""],"name":["É obrigatório","Deve ser uma string","Deve ser igual a \"Paul McCartney\""]}"#.to_string()
        );
    }

    #[test]
    fn test_locale_pt_long() {
        let locale = locale_pt_long();
        assert_eq!(validation_err_to_locale(&ValidationErr::Required, &locale), "É obrigatório".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::U64, &locale), "Deve ser um número inteiro sem sinal".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::I64, &locale), "Deve ser um número inteiro".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::F64, &locale), "Deve ser um número com ponto flutuante".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Bool, &locale), "Deve ser um booleano".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Str, &locale), "Deve ser uma string".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Email, &locale), "Deve ser um e-mail".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Date, &locale), "Deve ser uma data".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Time, &locale), "Deve ser uma hora".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::DateTime, &locale), "Deve ser uma data e hora".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(34)))), &locale), "Deve ser igual a 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::U64(34)))), &locale), "Deve ser diferente de 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::U64(34)))), &locale), "Deve ser maior que 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::U64(34)))), &locale), "Deve ser maior ou igual a 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::U64(34)))), &locale), "Deve ser menor que 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::U64(34)))), &locale), "Deve ser menor ou igual a 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::U64(34)), Operand::Value(OperandValue::U64(43)))), &locale), "Deve estar entre 34 e 43".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::I64(-4)))), &locale), "Deve ser igual a -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::I64(-4)))), &locale), "Deve ser diferente de -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::I64(-4)))), &locale), "Deve ser maior que -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::I64(-4)))), &locale), "Deve ser maior ou igual a -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::I64(-4)))), &locale), "Deve ser menor que -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::I64(-4)))), &locale), "Deve ser menor ou igual a -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::I64(-4)), Operand::Value(OperandValue::I64(4)))), &locale), "Deve estar entre -4 e 4".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Deve ser igual a -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Deve ser diferente de -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Deve ser maior que -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Deve ser maior ou igual a -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Deve ser menor que -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Deve ser menor ou igual a -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::F64(-4.6)), Operand::Value(OperandValue::F64(-2.4)))), &locale), "Deve estar entre -4.6 e -2.4".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false)))), &locale), "Deve ser igual a false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Bool(false)))), &locale), "Deve ser diferente de false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::Bool(false)))), &locale), "Deve ser maior que false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::Bool(false)))), &locale), "Deve ser maior ou igual a false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::Bool(false)))), &locale), "Deve ser menor que false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::Bool(false)))), &locale), "Deve ser menor ou igual a false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::Bool(false)), Operand::Value(OperandValue::Bool(true)))), &locale), "Deve estar entre false e true".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("aurorae")))), &locale), "Deve ser igual a \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::from("aurorae")))), &locale), "Deve ser diferente de \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::from("aurorae")))), &locale), "Deve ser maior que \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::from("aurorae")))), &locale), "Deve ser maior ou igual a \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::from("aurorae")))), &locale), "Deve ser menor que \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::from("aurorae")))), &locale), "Deve ser menor ou igual a \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::from("aurorae")), Operand::Value(OperandValue::from("crespúculum")))), &locale), "Deve estar entre \"aurorae\" e \"crespúculum\"".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(10)))), &locale), "A quantidade de bytes deve ser igual a 10".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Ne(Operand::Value(OperandValue::USize(11)))), &locale), "A quantidade de bytes deve ser diferente de 11".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(12)))), &locale), "A quantidade de bytes deve ser maior que 12".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(13)))), &locale), "A quantidade de bytes deve ser maior ou igual a 13".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(14)))), &locale), "A quantidade de bytes deve ser menor que 14".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(15)))), &locale), "A quantidade de bytes deve ser menor ou igual a 15".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(16)), Operand::Value(OperandValue::USize(17)))), &locale), "A quantidade de bytes deve estar entre 16 e 17".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Eq(Operand::Value(OperandValue::USize(18)))), &locale), "A quantidade de caracteres deve ser igual a 18".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(19)))), &locale), "A quantidade de caracteres deve ser diferente de 19".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Gt(Operand::Value(OperandValue::USize(20)))), &locale), "A quantidade de caracteres deve ser maior que 20".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Ge(Operand::Value(OperandValue::USize(21)))), &locale), "A quantidade de caracteres deve ser maior ou igual a 21".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Lt(Operand::Value(OperandValue::USize(22)))), &locale), "A quantidade de caracteres deve ser menor que 22".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Le(Operand::Value(OperandValue::USize(23)))), &locale), "A quantidade de caracteres deve ser menor ou igual a 23".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Btwn(Operand::Value(OperandValue::USize(24)), Operand::Value(OperandValue::USize(25)))), &locale), "A quantidade de caracteres deve estar entre 24 e 25".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Eq(Operand::Value(OperandValue::USize(26)))), &locale), "A quantidade de grafemas deve ser igual a 26".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Ne(Operand::Value(OperandValue::USize(27)))), &locale), "A quantidade de grafemas deve ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(28)))), &locale), "A quantidade de grafemas deve ser maior que 28".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Ge(Operand::Value(OperandValue::USize(29)))), &locale), "A quantidade de grafemas deve ser maior ou igual a 29".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Lt(Operand::Value(OperandValue::USize(30)))), &locale), "A quantidade de grafemas deve ser menor que 30".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Le(Operand::Value(OperandValue::USize(31)))), &locale), "A quantidade de grafemas deve ser menor ou igual a 31".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Btwn(Operand::Value(OperandValue::USize(32)), Operand::Value(OperandValue::USize(33)))), &locale), "A quantidade de grafemas deve estar entre 32 e 33".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(42)))), &locale), "A quantidade de caracteres minúsculos deve ser igual a 42".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(43)))), &locale), "A quantidade de caracteres minúsculos deve ser diferente de 43".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(44)))), &locale), "A quantidade de caracteres minúsculos deve ser maior que 44".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(45)))), &locale), "A quantidade de caracteres minúsculos deve ser maior ou igual a 45".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(46)))), &locale), "A quantidade de caracteres minúsculos deve ser menor que 46".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Le(Operand::Value(OperandValue::USize(47)))), &locale), "A quantidade de caracteres minúsculos deve ser menor ou igual a 47".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(48)), Operand::Value(OperandValue::USize(49)))), &locale), "A quantidade de caracteres minúsculos deve estar entre 48 e 49".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(34)))), &locale), "A quantidade de caracteres maiúsculos deve ser igual a 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(35)))), &locale), "A quantidade de caracteres maiúsculos deve ser diferente de 35".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(36)))), &locale), "A quantidade de caracteres maiúsculos deve ser maior que 36".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(37)))), &locale), "A quantidade de caracteres maiúsculos deve ser maior ou igual a 37".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(38)))), &locale), "A quantidade de caracteres maiúsculos deve ser menor que 38".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Le(Operand::Value(OperandValue::USize(39)))), &locale), "A quantidade de caracteres maiúsculos deve ser menor ou igual a 39".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(40)), Operand::Value(OperandValue::USize(41)))), &locale), "A quantidade de caracteres maiúsculos deve estar entre 40 e 41".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Eq(Operand::Value(OperandValue::USize(50)))), &locale), "A quantidade de números deve ser igual a 50".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Ne(Operand::Value(OperandValue::USize(51)))), &locale), "A quantidade de números deve ser diferente de 51".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Gt(Operand::Value(OperandValue::USize(52)))), &locale), "A quantidade de números deve ser maior que 52".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Ge(Operand::Value(OperandValue::USize(53)))), &locale), "A quantidade de números deve ser maior ou igual a 53".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Lt(Operand::Value(OperandValue::USize(54)))), &locale), "A quantidade de números deve ser menor que 54".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(55)))), &locale), "A quantidade de números deve ser menor ou igual a 55".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Btwn(Operand::Value(OperandValue::USize(56)), Operand::Value(OperandValue::USize(57)))), &locale), "A quantidade de números deve estar entre 56 e 57".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Eq(Operand::Value(OperandValue::USize(58)))), &locale), "A quantidade de símbolos deve ser igual a 58".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Ne(Operand::Value(OperandValue::USize(59)))), &locale), "A quantidade de símbolos deve ser diferente de 59".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Gt(Operand::Value(OperandValue::USize(60)))), &locale), "A quantidade de símbolos deve ser maior que 60".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Ge(Operand::Value(OperandValue::USize(61)))), &locale), "A quantidade de símbolos deve ser maior ou igual a 61".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Lt(Operand::Value(OperandValue::USize(62)))), &locale), "A quantidade de símbolos deve ser menor que 62".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Le(Operand::Value(OperandValue::USize(63)))), &locale), "A quantidade de símbolos deve ser menor ou igual a 63".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Btwn(Operand::Value(OperandValue::USize(64)), Operand::Value(OperandValue::USize(65)))), &locale), "A quantidade de símbolos deve estar entre 64 e 65".to_string());
    }

    #[test]
    fn test_locale_es_long() {
        let locale = locale_es_long();
        assert_eq!(validation_err_to_locale(&ValidationErr::Required, &locale), "Se requiere".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::U64, &locale), "Debe ser un número entero sin signo".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::I64, &locale), "Debe ser un número entero".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::F64, &locale), "Debe ser un número de punto flotante".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Bool, &locale), "Debe ser un booleano".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Str, &locale), "Debe ser una cadena".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Email, &locale), "Debe ser un correo electrónico".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Date, &locale), "Debe ser una fecha".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Time, &locale), "Debe ser una hora".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::DateTime, &locale), "Debe ser una fecha y hora".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(34)))), &locale), "Debe ser igual a 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::U64(34)))), &locale), "Debe ser diferente de 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::U64(34)))), &locale), "Debe ser mayor que 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::U64(34)))), &locale), "Debe ser mayor o igual a 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::U64(34)))), &locale), "Debe ser menor que 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::U64(34)))), &locale), "Debe ser menor o igual a 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::U64(34)), Operand::Value(OperandValue::U64(43)))), &locale), "Debe estar entre 34 y 43".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::I64(-4)))), &locale), "Debe ser igual a -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::I64(-4)))), &locale), "Debe ser diferente de -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::I64(-4)))), &locale), "Debe ser mayor que -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::I64(-4)))), &locale), "Debe ser mayor o igual a -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::I64(-4)))), &locale), "Debe ser menor que -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::I64(-4)))), &locale), "Debe ser menor o igual a -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::I64(-4)), Operand::Value(OperandValue::I64(4)))), &locale), "Debe estar entre -4 y 4".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Debe ser igual a -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Debe ser diferente de -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Debe ser mayor que -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Debe ser mayor o igual a -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Debe ser menor que -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Debe ser menor o igual a -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::F64(-4.6)), Operand::Value(OperandValue::F64(-2.4)))), &locale), "Debe estar entre -4.6 y -2.4".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false)))), &locale), "Debe ser igual a false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Bool(false)))), &locale), "Debe ser diferente de false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::Bool(false)))), &locale), "Debe ser mayor que false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::Bool(false)))), &locale), "Debe ser mayor o igual a false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::Bool(false)))), &locale), "Debe ser menor que false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::Bool(false)))), &locale), "Debe ser menor o igual a false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::Bool(false)), Operand::Value(OperandValue::Bool(true)))), &locale), "Debe estar entre false y true".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("aurorae")))), &locale), "Debe ser igual a \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::from("aurorae")))), &locale), "Debe ser diferente de \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::from("aurorae")))), &locale), "Debe ser mayor que \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::from("aurorae")))), &locale), "Debe ser mayor o igual a \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::from("aurorae")))), &locale), "Debe ser menor que \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::from("aurorae")))), &locale), "Debe ser menor o igual a \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::from("aurorae")), Operand::Value(OperandValue::from("crespúculum")))), &locale), "Debe estar entre \"aurorae\" y \"crespúculum\"".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(10)))), &locale), "La cantidad de bytes debe ser igual a 10".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Ne(Operand::Value(OperandValue::USize(11)))), &locale), "La cantidad de bytes debe ser diferente de 11".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(12)))), &locale), "La cantidad de bytes debe ser mayor que 12".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(13)))), &locale), "La cantidad de bytes debe ser mayor o igual a 13".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(14)))), &locale), "La cantidad de bytes debe ser menor que 14".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(15)))), &locale), "La cantidad de bytes debe ser menor o igual a 15".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(16)), Operand::Value(OperandValue::USize(17)))), &locale), "La cantidad de bytes debe estar entre 16 y 17".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Eq(Operand::Value(OperandValue::USize(18)))), &locale), "La cantidad de caracteres debe ser igual a 18".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(19)))), &locale), "La cantidad de caracteres debe ser diferente de 19".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Gt(Operand::Value(OperandValue::USize(20)))), &locale), "La cantidad de caracteres debe ser mayor que 20".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Ge(Operand::Value(OperandValue::USize(21)))), &locale), "La cantidad de caracteres debe ser mayor o igual a 21".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Lt(Operand::Value(OperandValue::USize(22)))), &locale), "La cantidad de caracteres debe ser menor que 22".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Le(Operand::Value(OperandValue::USize(23)))), &locale), "La cantidad de caracteres debe ser menor o igual a 23".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Btwn(Operand::Value(OperandValue::USize(24)), Operand::Value(OperandValue::USize(25)))), &locale), "La cantidad de caracteres debe estar entre 24 y 25".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Eq(Operand::Value(OperandValue::USize(26)))), &locale), "La cantidad de grafemas debe ser igual a 26".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Ne(Operand::Value(OperandValue::USize(27)))), &locale), "La cantidad de grafemas debe ser diferente de 27".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(28)))), &locale), "La cantidad de grafemas debe ser mayor que 28".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Ge(Operand::Value(OperandValue::USize(29)))), &locale), "La cantidad de grafemas debe ser mayor o igual a 29".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Lt(Operand::Value(OperandValue::USize(30)))), &locale), "La cantidad de grafemas debe ser menor que 30".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Le(Operand::Value(OperandValue::USize(31)))), &locale), "La cantidad de grafemas debe ser menor o igual a 31".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Btwn(Operand::Value(OperandValue::USize(32)), Operand::Value(OperandValue::USize(33)))), &locale), "La cantidad de grafemas debe estar entre 32 y 33".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(42)))), &locale), "La cantidad de caracteres en minúsculas debe ser igual a 42".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(43)))), &locale), "La cantidad de caracteres en minúsculas debe ser diferente de 43".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(44)))), &locale), "La cantidad de caracteres en minúsculas debe ser mayor que 44".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(45)))), &locale), "La cantidad de caracteres en minúsculas debe ser mayor o igual a 45".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(46)))), &locale), "La cantidad de caracteres en minúsculas debe ser menor que 46".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Le(Operand::Value(OperandValue::USize(47)))), &locale), "La cantidad de caracteres en minúsculas debe ser menor o igual a 47".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(48)), Operand::Value(OperandValue::USize(49)))), &locale), "La cantidad de caracteres en minúsculas debe estar entre 48 y 49".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(34)))), &locale), "La cantidad de caracteres en mayúsculas debe ser igual a 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(35)))), &locale), "La cantidad de caracteres en mayúsculas debe ser diferente de 35".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(36)))), &locale), "La cantidad de caracteres en mayúsculas debe ser mayor que 36".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(37)))), &locale), "La cantidad de caracteres en mayúsculas debe ser mayor o igual a 37".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(38)))), &locale), "La cantidad de caracteres en mayúsculas debe ser menor que 38".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Le(Operand::Value(OperandValue::USize(39)))), &locale), "La cantidad de caracteres en mayúsculas debe ser menor o igual a 39".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(40)), Operand::Value(OperandValue::USize(41)))), &locale), "La cantidad de caracteres en mayúsculas debe estar entre 40 y 41".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Eq(Operand::Value(OperandValue::USize(50)))), &locale), "La cantidad de números debe ser igual a 50".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Ne(Operand::Value(OperandValue::USize(51)))), &locale), "La cantidad de números debe ser diferente de 51".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Gt(Operand::Value(OperandValue::USize(52)))), &locale), "La cantidad de números debe ser mayor que 52".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Ge(Operand::Value(OperandValue::USize(53)))), &locale), "La cantidad de números debe ser mayor o igual a 53".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Lt(Operand::Value(OperandValue::USize(54)))), &locale), "La cantidad de números debe ser menor que 54".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(55)))), &locale), "La cantidad de números debe ser menor o igual a 55".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Btwn(Operand::Value(OperandValue::USize(56)), Operand::Value(OperandValue::USize(57)))), &locale), "La cantidad de números debe estar entre 56 y 57".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Eq(Operand::Value(OperandValue::USize(58)))), &locale), "La cantidad de símbolos debe ser igual a 58".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Ne(Operand::Value(OperandValue::USize(59)))), &locale), "La cantidad de símbolos debe ser diferente de 59".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Gt(Operand::Value(OperandValue::USize(60)))), &locale), "La cantidad de símbolos debe ser mayor que 60".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Ge(Operand::Value(OperandValue::USize(61)))), &locale), "La cantidad de símbolos debe ser mayor o igual a 61".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Lt(Operand::Value(OperandValue::USize(62)))), &locale), "La cantidad de símbolos debe ser menor que 62".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Le(Operand::Value(OperandValue::USize(63)))), &locale), "La cantidad de símbolos debe ser menor o igual a 63".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Btwn(Operand::Value(OperandValue::USize(64)), Operand::Value(OperandValue::USize(65)))), &locale), "La cantidad de símbolos debe estar entre 64 y 65".to_string());
    }

    #[test]
    fn test_locale_en_long() {
        let locale = locale_en_long();
        assert_eq!(validation_err_to_locale(&ValidationErr::Required, &locale), "Is required".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::U64, &locale), "Must be an unsigned integer".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::I64, &locale), "Must be an integer".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::F64, &locale), "Must be a float".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Bool, &locale), "Must be a boolean".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Str, &locale), "Must be a string".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Email, &locale), "Must be an e-mail".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Date, &locale), "Must be a date".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Time, &locale), "Must be a time".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::DateTime, &locale), "Must be a date and time".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(34)))), &locale), "Must be equals to 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::U64(34)))), &locale), "Must be different from 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::U64(34)))), &locale), "Must be greater than 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::U64(34)))), &locale), "Must be greater than or equals to 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::U64(34)))), &locale), "Must be smaller than 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::U64(34)))), &locale), "Must be smaller than or equals to 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::U64(34)), Operand::Value(OperandValue::U64(43)))), &locale), "Must be between 34 and 43".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::I64(-4)))), &locale), "Must be equals to -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::I64(-4)))), &locale), "Must be different from -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::I64(-4)))), &locale), "Must be greater than -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::I64(-4)))), &locale), "Must be greater than or equals to -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::I64(-4)))), &locale), "Must be smaller than -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::I64(-4)))), &locale), "Must be smaller than or equals to -4".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::I64(-4)), Operand::Value(OperandValue::I64(4)))), &locale), "Must be between -4 and 4".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Must be equals to -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Must be different from -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Must be greater than -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Must be greater than or equals to -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Must be smaller than -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::F64(-4.6)))), &locale), "Must be smaller than or equals to -4.6".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::F64(-4.6)), Operand::Value(OperandValue::F64(-2.4)))), &locale), "Must be between -4.6 and -2.4".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false)))), &locale), "Must be equals to false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Bool(false)))), &locale), "Must be different from false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::Bool(false)))), &locale), "Must be greater than false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::Bool(false)))), &locale), "Must be greater than or equals to false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::Bool(false)))), &locale), "Must be smaller than false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::Bool(false)))), &locale), "Must be smaller than or equals to false".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::Bool(false)), Operand::Value(OperandValue::Bool(true)))), &locale), "Must be between false and true".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("aurorae")))), &locale), "Must be equals to \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::from("aurorae")))), &locale), "Must be different from \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::from("aurorae")))), &locale), "Must be greater than \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::from("aurorae")))), &locale), "Must be greater than or equals to \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::from("aurorae")))), &locale), "Must be smaller than \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::from("aurorae")))), &locale), "Must be smaller than or equals to \"aurorae\"".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::from("aurorae")), Operand::Value(OperandValue::from("crespúculum")))), &locale), "Must be between \"aurorae\" and \"crespúculum\"".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(10)))), &locale), "The length of bytes must be equals to 10".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Ne(Operand::Value(OperandValue::USize(11)))), &locale), "The length of bytes must be different from 11".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(12)))), &locale), "The length of bytes must be greater than 12".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(13)))), &locale), "The length of bytes must be greater than or equals to 13".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(14)))), &locale), "The length of bytes must be smaller than 14".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(15)))), &locale), "The length of bytes must be smaller than or equals to 15".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(16)), Operand::Value(OperandValue::USize(17)))), &locale), "The length of bytes must be between 16 and 17".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Eq(Operand::Value(OperandValue::USize(18)))), &locale), "The length of characters must be equals to 18".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(19)))), &locale), "The length of characters must be different from 19".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Gt(Operand::Value(OperandValue::USize(20)))), &locale), "The length of characters must be greater than 20".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Ge(Operand::Value(OperandValue::USize(21)))), &locale), "The length of characters must be greater than or equals to 21".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Lt(Operand::Value(OperandValue::USize(22)))), &locale), "The length of characters must be smaller than 22".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Le(Operand::Value(OperandValue::USize(23)))), &locale), "The length of characters must be smaller than or equals to 23".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Btwn(Operand::Value(OperandValue::USize(24)), Operand::Value(OperandValue::USize(25)))), &locale), "The length of characters must be between 24 and 25".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Eq(Operand::Value(OperandValue::USize(26)))), &locale), "The length of graphemes must be equals to 26".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Ne(Operand::Value(OperandValue::USize(27)))), &locale), "The length of graphemes must be different from 27".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(28)))), &locale), "The length of graphemes must be greater than 28".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Ge(Operand::Value(OperandValue::USize(29)))), &locale), "The length of graphemes must be greater than or equals to 29".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Lt(Operand::Value(OperandValue::USize(30)))), &locale), "The length of graphemes must be smaller than 30".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Le(Operand::Value(OperandValue::USize(31)))), &locale), "The length of graphemes must be smaller than or equals to 31".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Btwn(Operand::Value(OperandValue::USize(32)), Operand::Value(OperandValue::USize(33)))), &locale), "The length of graphemes must be between 32 and 33".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(42)))), &locale), "The length of lowercase characters must be equals to 42".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(43)))), &locale), "The length of lowercase characters must be different from 43".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(44)))), &locale), "The length of lowercase characters must be greater than 44".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(45)))), &locale), "The length of lowercase characters must be greater than or equals to 45".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(46)))), &locale), "The length of lowercase characters must be smaller than 46".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Le(Operand::Value(OperandValue::USize(47)))), &locale), "The length of lowercase characters must be smaller than or equals to 47".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(48)), Operand::Value(OperandValue::USize(49)))), &locale), "The length of lowercase characters must be between 48 and 49".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(34)))), &locale), "The length of uppercase characters must be equals to 34".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(35)))), &locale), "The length of uppercase characters must be different from 35".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(36)))), &locale), "The length of uppercase characters must be greater than 36".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(37)))), &locale), "The length of uppercase characters must be greater than or equals to 37".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(38)))), &locale), "The length of uppercase characters must be smaller than 38".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Le(Operand::Value(OperandValue::USize(39)))), &locale), "The length of uppercase characters must be smaller than or equals to 39".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(40)), Operand::Value(OperandValue::USize(41)))), &locale), "The length of uppercase characters must be between 40 and 41".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Eq(Operand::Value(OperandValue::USize(50)))), &locale), "The length of numbers must be equals to 50".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Ne(Operand::Value(OperandValue::USize(51)))), &locale), "The length of numbers must be different from 51".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Gt(Operand::Value(OperandValue::USize(52)))), &locale), "The length of numbers must be greater than 52".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Ge(Operand::Value(OperandValue::USize(53)))), &locale), "The length of numbers must be greater than or equals to 53".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Lt(Operand::Value(OperandValue::USize(54)))), &locale), "The length of numbers must be smaller than 54".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(55)))), &locale), "The length of numbers must be smaller than or equals to 55".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Btwn(Operand::Value(OperandValue::USize(56)), Operand::Value(OperandValue::USize(57)))), &locale), "The length of numbers must be between 56 and 57".to_string());

        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Eq(Operand::Value(OperandValue::USize(58)))), &locale), "The length of symbols must be equals to 58".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Ne(Operand::Value(OperandValue::USize(59)))), &locale), "The length of symbols must be different from 59".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Gt(Operand::Value(OperandValue::USize(60)))), &locale), "The length of symbols must be greater than 60".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Ge(Operand::Value(OperandValue::USize(61)))), &locale), "The length of symbols must be greater than or equals to 61".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Lt(Operand::Value(OperandValue::USize(62)))), &locale), "The length of symbols must be smaller than 62".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Le(Operand::Value(OperandValue::USize(63)))), &locale), "The length of symbols must be smaller than or equals to 63".to_string());
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Btwn(Operand::Value(OperandValue::USize(64)), Operand::Value(OperandValue::USize(65)))), &locale), "The length of symbols must be between 64 and 65".to_string());
    }
}

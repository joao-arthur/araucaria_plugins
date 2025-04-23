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
        OperandValue::Str(val) => String::from("\"") + val + "\"",
    }
}

pub fn operand_to_string(operand: &Operand) -> String {
    match operand {
        Operand::Value(value) => operand_value_to_string(value),
        Operand::FieldPath(path) => String::from("\"") + path + "\"",
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
        required: String::from("É obrigatório"),
        num_u: String::from("Deve ser um número inteiro sem sinal"),
        num_i: String::from("Deve ser um número inteiro"),
        num_f: String::from("Deve ser um número com ponto flutuante"),
        bool: String::from("Deve ser um booleano"),
        str: String::from("Deve ser uma string"),
        email: String::from("Deve ser um e-mail"),
        date: String::from("Deve ser uma data"),
        time: String::from("Deve ser uma hora"),
        date_time: String::from("Deve ser uma data e hora"),
        eq: String::from("Deve ser igual a %value%"),
        ne: String::from("Deve ser diferente de %value%"),
        gt: String::from("Deve ser maior que %value%"),
        ge: String::from("Deve ser maior ou igual a %value%"),
        lt: String::from("Deve ser menor que %value%"),
        le: String::from("Deve ser menor ou igual a %value%"),
        btwn: String::from("Deve estar entre %value_a% e %value_b%"),
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

pub fn locale_es_long() -> Locale {
    Locale {
        required: String::from("Se requiere"),
        num_u: String::from("Debe ser un número entero sin signo"),
        num_i: String::from("Debe ser un número entero"),
        num_f: String::from("Debe ser un número de punto flotante"),
        bool: String::from("Debe ser un booleano"),
        str: String::from("Debe ser una cadena"),
        email: String::from("Debe ser un correo electrónico"),
        date: String::from("Debe ser una fecha"),
        time: String::from("Debe ser una hora"),
        date_time: String::from("Debe ser una fecha y hora"),
        eq: String::from("Debe ser igual a %value%"),
        ne: String::from("Debe ser diferente de %value%"),
        gt: String::from("Debe ser mayor que %value%"),
        ge: String::from("Debe ser mayor o igual a %value%"),
        lt: String::from("Debe ser menor que %value%"),
        le: String::from("Debe ser menor o igual a %value%"),
        btwn: String::from("Debe estar entre %value_a% y %value_b%"),
        bytes_len_eq: String::from("La cantidad de bytes debe ser igual a %value%"),
        bytes_len_ne: String::from("La cantidad de bytes debe ser diferente de %value%"),
        bytes_len_gt: String::from("La cantidad de bytes debe ser mayor que %value%"),
        bytes_len_ge: String::from("La cantidad de bytes debe ser mayor o igual a %value%"),
        bytes_len_lt: String::from("La cantidad de bytes debe ser menor que %value%"),
        bytes_len_le: String::from("La cantidad de bytes debe ser menor o igual a %value%"),
        bytes_len_btwn: String::from("La cantidad de bytes debe estar entre %value_a% y %value_b%"),
        chars_len_eq: String::from("La cantidad de caracteres debe ser igual a %value%"),
        chars_len_ne: String::from("La cantidad de caracteres debe ser diferente de %value%"),
        chars_len_gt: String::from("La cantidad de caracteres debe ser mayor que %value%"),
        chars_len_ge: String::from("La cantidad de caracteres debe ser mayor o igual a %value%"),
        chars_len_lt: String::from("La cantidad de caracteres debe ser menor que %value%"),
        chars_len_le: String::from("La cantidad de caracteres debe ser menor o igual a %value%"),
        chars_len_btwn: String::from("La cantidad de caracteres debe estar entre %value_a% y %value_b%"),
        graphemes_len_eq: String::from("La cantidad de grafemas debe ser igual a %value%"),
        graphemes_len_ne: String::from("La cantidad de grafemas debe ser diferente de %value%"),
        graphemes_len_gt: String::from("La cantidad de grafemas debe ser mayor que %value%"),
        graphemes_len_ge: String::from("La cantidad de grafemas debe ser mayor o igual a %value%"),
        graphemes_len_lt: String::from("La cantidad de grafemas debe ser menor que %value%"),
        graphemes_len_le: String::from("La cantidad de grafemas debe ser menor o igual a %value%"),
        graphemes_len_btwn: String::from("La cantidad de grafemas debe estar entre %value_a% y %value_b%"),
        lowercase_len_eq: String::from("La cantidad de caracteres en minúsculas debe ser igual a %value%"),
        lowercase_len_ne: String::from("La cantidad de caracteres en minúsculas debe ser diferente de %value%"),
        lowercase_len_gt: String::from("La cantidad de caracteres en minúsculas debe ser mayor que %value%"),
        lowercase_len_ge: String::from("La cantidad de caracteres en minúsculas debe ser mayor o igual a %value%"),
        lowercase_len_lt: String::from("La cantidad de caracteres en minúsculas debe ser menor que %value%"),
        lowercase_len_le: String::from("La cantidad de caracteres en minúsculas debe ser menor o igual a %value%"),
        lowercase_len_btwn: String::from("La cantidad de caracteres en minúsculas debe estar entre %value_a% y %value_b%"),
        uppercase_len_eq: String::from("La cantidad de caracteres en mayúsculas debe ser igual a %value%"),
        uppercase_len_ne: String::from("La cantidad de caracteres en mayúsculas debe ser diferente de %value%"),
        uppercase_len_gt: String::from("La cantidad de caracteres en mayúsculas debe ser mayor que %value%"),
        uppercase_len_ge: String::from("La cantidad de caracteres en mayúsculas debe ser mayor o igual a %value%"),
        uppercase_len_lt: String::from("La cantidad de caracteres en mayúsculas debe ser menor que %value%"),
        uppercase_len_le: String::from("La cantidad de caracteres en mayúsculas debe ser menor o igual a %value%"),
        uppercase_len_btwn: String::from("La cantidad de caracteres en mayúsculas debe estar entre %value_a% y %value_b%"),
        number_len_eq: String::from("La cantidad de números debe ser igual a %value%"),
        number_len_ne: String::from("La cantidad de números debe ser diferente de %value%"),
        number_len_gt: String::from("La cantidad de números debe ser mayor que %value%"),
        number_len_ge: String::from("La cantidad de números debe ser mayor o igual a %value%"),
        number_len_lt: String::from("La cantidad de números debe ser menor que %value%"),
        number_len_le: String::from("La cantidad de números debe ser menor o igual a %value%"),
        number_len_btwn: String::from("La cantidad de números debe estar entre %value_a% y %value_b%"),
        symbols_eq: String::from("La cantidad de símbolos debe ser igual a %value%"),
        symbols_ne: String::from("La cantidad de símbolos debe ser diferente de %value%"),
        symbols_gt: String::from("La cantidad de símbolos debe ser mayor que %value%"),
        symbols_ge: String::from("La cantidad de símbolos debe ser mayor o igual a %value%"),
        symbols_lt: String::from("La cantidad de símbolos debe ser menor que %value%"),
        symbols_le: String::from("La cantidad de símbolos debe ser menor o igual a %value%"),
        symbols_btwn: String::from("La cantidad de símbolos debe estar entre %value_a% y %value_b%"),
    }
}

pub fn locale_en_long() -> Locale {
    Locale {
        required: String::from("Is required"),
        num_u: String::from("Must be an unsigned integer"),
        num_i: String::from("Must be an integer"),
        num_f: String::from("Must be a float"),
        bool: String::from("Must be a boolean"),
        str: String::from("Must be a string"),
        email: String::from("Must be an e-mail"),
        date: String::from("Must be a date"),
        time: String::from("Must be a time"),
        date_time: String::from("Must be a date and time"),
        eq: String::from("Must be equals to %value%"),
        ne: String::from("Must be different from %value%"),
        gt: String::from("Must be greater than %value%"),
        ge: String::from("Must be greater than or equals to %value%"),
        lt: String::from("Must be smaller than %value%"),
        le: String::from("Must be smaller than or equals to %value%"),
        btwn: String::from("Must be between %value_a% and %value_b%"),
        bytes_len_eq: String::from("The length of bytes must be equals to %value%"),
        bytes_len_ne: String::from("The length of bytes must be different from %value%"),
        bytes_len_gt: String::from("The length of bytes must be greater than %value%"),
        bytes_len_ge: String::from("The length of bytes must be greater than or equals to %value%"),
        bytes_len_lt: String::from("The length of bytes must be smaller than %value%"),
        bytes_len_le: String::from("The length of bytes must be smaller than or equals to %value%"),
        bytes_len_btwn: String::from("The length of bytes must be between %value_a% and %value_b%"),
        chars_len_eq: String::from("The length of characters must be equals to %value%"),
        chars_len_ne: String::from("The length of characters must be different from %value%"),
        chars_len_gt: String::from("The length of characters must be greater than %value%"),
        chars_len_ge: String::from("The length of characters must be greater than or equals to %value%"),
        chars_len_lt: String::from("The length of characters must be smaller than %value%"),
        chars_len_le: String::from("The length of characters must be smaller than or equals to %value%"),
        chars_len_btwn: String::from("The length of characters must be between %value_a% and %value_b%"),
        graphemes_len_eq: String::from("The length of graphemes must be equals to %value%"),
        graphemes_len_ne: String::from("The length of graphemes must be different from %value%"),
        graphemes_len_gt: String::from("The length of graphemes must be greater than %value%"),
        graphemes_len_ge: String::from("The length of graphemes must be greater than or equals to %value%"),
        graphemes_len_lt: String::from("The length of graphemes must be smaller than %value%"),
        graphemes_len_le: String::from("The length of graphemes must be smaller than or equals to %value%"),
        graphemes_len_btwn: String::from("The length of graphemes must be between %value_a% and %value_b%"),
        lowercase_len_eq: String::from("The length of lowercase characters must be equals to %value%"),
        lowercase_len_ne: String::from("The length of lowercase characters must be different from %value%"),
        lowercase_len_gt: String::from("The length of lowercase characters must be greater than %value%"),
        lowercase_len_ge: String::from("The length of lowercase characters must be greater than or equals to %value%"),
        lowercase_len_lt: String::from("The length of lowercase characters must be smaller than %value%"),
        lowercase_len_le: String::from("The length of lowercase characters must be smaller than or equals to %value%"),
        lowercase_len_btwn: String::from("The length of lowercase characters must be between %value_a% and %value_b%"),
        uppercase_len_eq: String::from("The length of uppercase characters must be equals to %value%"),
        uppercase_len_ne: String::from("The length of uppercase characters must be different from %value%"),
        uppercase_len_gt: String::from("The length of uppercase characters must be greater than %value%"),
        uppercase_len_ge: String::from("The length of uppercase characters must be greater than or equals to %value%"),
        uppercase_len_lt: String::from("The length of uppercase characters must be smaller than %value%"),
        uppercase_len_le: String::from("The length of uppercase characters must be smaller than or equals to %value%"),
        uppercase_len_btwn: String::from("The length of uppercase characters must be between %value_a% and %value_b%"),
        number_len_eq: String::from("The length of numbers must be equals to %value%"),
        number_len_ne: String::from("The length of numbers must be different from %value%"),
        number_len_gt: String::from("The length of numbers must be greater than %value%"),
        number_len_ge: String::from("The length of numbers must be greater than or equals to %value%"),
        number_len_lt: String::from("The length of numbers must be smaller than %value%"),
        number_len_le: String::from("The length of numbers must be smaller than or equals to %value%"),
        number_len_btwn: String::from("The length of numbers must be between %value_a% and %value_b%"),
        symbols_eq: String::from("The length of symbols must be equals to %value%"),
        symbols_ne: String::from("The length of symbols must be different from %value%"),
        symbols_gt: String::from("The length of symbols must be greater than %value%"),
        symbols_ge: String::from("The length of symbols must be greater than or equals to %value%"),
        symbols_lt: String::from("The length of symbols must be smaller than %value%"),
        symbols_le: String::from("The length of symbols must be smaller than or equals to %value%"),
        symbols_btwn: String::from("The length of symbols must be between %value_a% and %value_b%"),
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
            SchemaLocalizedErr::Arr(vec![String::from("É obrigatório"), String::from("Deve ser um booleano"), String::from("Deve ser igual a true")])
        );
        assert_eq!(
            schema_err_to_locale(
                &SchemaErr::Obj(BTreeMap::from([
                    (
                        String::from("name"),
                        SchemaErr::Validation(vec![
                            ValidationErr::Required,
                            ValidationErr::Str,
                            ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Str(String::from("Paul McCartney")))))
                        ])
                    ),
                    (
                        String::from("birthdate"),
                        SchemaErr::Validation(vec![
                            ValidationErr::Required,
                            ValidationErr::Str,
                            ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Str(String::from("1942-06-18")))))
                        ])
                    ),
                    (
                        String::from("alive"),
                        SchemaErr::Validation(vec![
                            ValidationErr::Required,
                            ValidationErr::Bool,
                            ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(true))))
                        ])
                    ),
                    (
                        String::from("bands"),
                        SchemaErr::Validation(vec![
                            ValidationErr::Required,
                            ValidationErr::Str,
                            ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Str(String::from("The Beatles")))))
                        ])
                    ),
                ])),
                &locale
            ),
            SchemaLocalizedErr::Obj(BTreeMap::from([
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
            serde_json::to_string(&SchemaLocalizedErr::Obj(BTreeMap::from([
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
                r#"{"alive":["É obrigatório","Deve ser um booleano","Deve ser igual a true"],"bands":["É obrigatório","Deve ser uma string","Deve ser igual a \"The Beatles\""],"birthdate":["É obrigatório","Deve ser uma string","Deve ser igual a \"1942-06-18\""],"name":["É obrigatório","Deve ser uma string","Deve ser igual a \"Paul McCartney\""]}"#
            )
        );
    }

    #[test]
    fn test_locale_pt_long() {
        let locale = locale_pt_long();
        assert_eq!(validation_err_to_locale(&ValidationErr::Required, &locale), String::from("É obrigatório"));
        assert_eq!(validation_err_to_locale(&ValidationErr::U64, &locale), String::from("Deve ser um número inteiro sem sinal"));
        assert_eq!(validation_err_to_locale(&ValidationErr::I64, &locale), String::from("Deve ser um número inteiro"));
        assert_eq!(validation_err_to_locale(&ValidationErr::F64, &locale), String::from("Deve ser um número com ponto flutuante"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Bool, &locale), String::from("Deve ser um booleano"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Str, &locale), String::from("Deve ser uma string"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Email, &locale), String::from("Deve ser um e-mail"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Date, &locale), String::from("Deve ser uma data"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Time, &locale), String::from("Deve ser uma hora"));
        assert_eq!(validation_err_to_locale(&ValidationErr::DateTime, &locale), String::from("Deve ser uma data e hora"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Deve ser igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Deve ser diferente de 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Deve ser maior que 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Deve ser maior ou igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Deve ser menor que 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Deve ser menor ou igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::U64(34)), Operand::Value(OperandValue::U64(43)))), &locale), String::from("Deve estar entre 34 e 43"));
        
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Deve ser igual a -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Deve ser diferente de -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Deve ser maior que -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Deve ser maior ou igual a -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Deve ser menor que -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Deve ser menor ou igual a -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::I64(-4)), Operand::Value(OperandValue::I64(4)))), &locale), String::from("Deve estar entre -4 e 4"));
        
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Deve ser igual a -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Deve ser diferente de -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Deve ser maior que -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Deve ser maior ou igual a -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Deve ser menor que -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Deve ser menor ou igual a -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::F64(-4.6)), Operand::Value(OperandValue::F64(-2.4)))), &locale), String::from("Deve estar entre -4.6 e -2.4"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Deve ser igual a false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Deve ser diferente de false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Deve ser maior que false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Deve ser maior ou igual a false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Deve ser menor que false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Deve ser menor ou igual a false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::Bool(false)), Operand::Value(OperandValue::Bool(true)))), &locale), String::from("Deve estar entre false e true"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Deve ser igual a \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Deve ser diferente de \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Deve ser maior que \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Deve ser maior ou igual a \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Deve ser menor que \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Deve ser menor ou igual a \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::Str(String::from("aurorae"))), Operand::Value(OperandValue::Str(String::from("crespúculum"))))), &locale), String::from("Deve estar entre \"aurorae\" e \"crespúculum\""));

        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(10)))), &locale), String::from("A quantidade de bytes deve ser igual a 10"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Ne(Operand::Value(OperandValue::USize(11)))), &locale), String::from("A quantidade de bytes deve ser diferente de 11"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(12)))), &locale), String::from("A quantidade de bytes deve ser maior que 12"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(13)))), &locale), String::from("A quantidade de bytes deve ser maior ou igual a 13"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(14)))), &locale), String::from("A quantidade de bytes deve ser menor que 14"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(15)))), &locale), String::from("A quantidade de bytes deve ser menor ou igual a 15"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(16)), Operand::Value(OperandValue::USize(17)))), &locale), String::from("A quantidade de bytes deve estar entre 16 e 17"));

        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Eq(Operand::Value(OperandValue::USize(18)))), &locale), String::from("A quantidade de caracteres deve ser igual a 18"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(19)))), &locale), String::from("A quantidade de caracteres deve ser diferente de 19"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Gt(Operand::Value(OperandValue::USize(20)))), &locale), String::from("A quantidade de caracteres deve ser maior que 20"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Ge(Operand::Value(OperandValue::USize(21)))), &locale), String::from("A quantidade de caracteres deve ser maior ou igual a 21"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Lt(Operand::Value(OperandValue::USize(22)))), &locale), String::from("A quantidade de caracteres deve ser menor que 22"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Le(Operand::Value(OperandValue::USize(23)))), &locale), String::from("A quantidade de caracteres deve ser menor ou igual a 23"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Btwn(Operand::Value(OperandValue::USize(24)), Operand::Value(OperandValue::USize(25)))), &locale), String::from("A quantidade de caracteres deve estar entre 24 e 25"));

        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Eq(Operand::Value(OperandValue::USize(26)))), &locale), String::from("A quantidade de grafemas deve ser igual a 26"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Ne(Operand::Value(OperandValue::USize(27)))), &locale), String::from("A quantidade de grafemas deve ser diferente de 27"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(28)))), &locale), String::from("A quantidade de grafemas deve ser maior que 28"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Ge(Operand::Value(OperandValue::USize(29)))), &locale), String::from("A quantidade de grafemas deve ser maior ou igual a 29"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Lt(Operand::Value(OperandValue::USize(30)))), &locale), String::from("A quantidade de grafemas deve ser menor que 30"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Le(Operand::Value(OperandValue::USize(31)))), &locale), String::from("A quantidade de grafemas deve ser menor ou igual a 31"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Btwn(Operand::Value(OperandValue::USize(32)), Operand::Value(OperandValue::USize(33)))), &locale), String::from("A quantidade de grafemas deve estar entre 32 e 33"));

        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(42)))), &locale), String::from("A quantidade de caracteres minúsculos deve ser igual a 42"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(43)))), &locale), String::from("A quantidade de caracteres minúsculos deve ser diferente de 43"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(44)))), &locale), String::from("A quantidade de caracteres minúsculos deve ser maior que 44"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(45)))), &locale), String::from("A quantidade de caracteres minúsculos deve ser maior ou igual a 45"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(46)))), &locale), String::from("A quantidade de caracteres minúsculos deve ser menor que 46"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Le(Operand::Value(OperandValue::USize(47)))), &locale), String::from("A quantidade de caracteres minúsculos deve ser menor ou igual a 47"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(48)), Operand::Value(OperandValue::USize(49)))), &locale), String::from("A quantidade de caracteres minúsculos deve estar entre 48 e 49"));

        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(34)))), &locale), String::from("A quantidade de caracteres maiúsculos deve ser igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(35)))), &locale), String::from("A quantidade de caracteres maiúsculos deve ser diferente de 35"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(36)))), &locale), String::from("A quantidade de caracteres maiúsculos deve ser maior que 36"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(37)))), &locale), String::from("A quantidade de caracteres maiúsculos deve ser maior ou igual a 37"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(38)))), &locale), String::from("A quantidade de caracteres maiúsculos deve ser menor que 38"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Le(Operand::Value(OperandValue::USize(39)))), &locale), String::from("A quantidade de caracteres maiúsculos deve ser menor ou igual a 39"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(40)), Operand::Value(OperandValue::USize(41)))), &locale), String::from("A quantidade de caracteres maiúsculos deve estar entre 40 e 41"));

        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Eq(Operand::Value(OperandValue::USize(50)))), &locale), String::from("A quantidade de números deve ser igual a 50"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Ne(Operand::Value(OperandValue::USize(51)))), &locale), String::from("A quantidade de números deve ser diferente de 51"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Gt(Operand::Value(OperandValue::USize(52)))), &locale), String::from("A quantidade de números deve ser maior que 52"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Ge(Operand::Value(OperandValue::USize(53)))), &locale), String::from("A quantidade de números deve ser maior ou igual a 53"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Lt(Operand::Value(OperandValue::USize(54)))), &locale), String::from("A quantidade de números deve ser menor que 54"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(55)))), &locale), String::from("A quantidade de números deve ser menor ou igual a 55"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Btwn(Operand::Value(OperandValue::USize(56)), Operand::Value(OperandValue::USize(57)))), &locale), String::from("A quantidade de números deve estar entre 56 e 57"));

        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Eq(Operand::Value(OperandValue::USize(58)))), &locale), String::from("A quantidade de símbolos deve ser igual a 58"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Ne(Operand::Value(OperandValue::USize(59)))), &locale), String::from("A quantidade de símbolos deve ser diferente de 59"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Gt(Operand::Value(OperandValue::USize(60)))), &locale), String::from("A quantidade de símbolos deve ser maior que 60"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Ge(Operand::Value(OperandValue::USize(61)))), &locale), String::from("A quantidade de símbolos deve ser maior ou igual a 61"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Lt(Operand::Value(OperandValue::USize(62)))), &locale), String::from("A quantidade de símbolos deve ser menor que 62"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Le(Operand::Value(OperandValue::USize(63)))), &locale), String::from("A quantidade de símbolos deve ser menor ou igual a 63"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Btwn(Operand::Value(OperandValue::USize(64)), Operand::Value(OperandValue::USize(65)))), &locale), String::from("A quantidade de símbolos deve estar entre 64 e 65"));
    }

    #[test]
    fn test_locale_es_long() {
        let locale = locale_es_long();
        assert_eq!(validation_err_to_locale(&ValidationErr::Required, &locale), String::from("Se requiere"));
        assert_eq!(validation_err_to_locale(&ValidationErr::U64, &locale), String::from("Debe ser un número entero sin signo"));
        assert_eq!(validation_err_to_locale(&ValidationErr::I64, &locale), String::from("Debe ser un número entero"));
        assert_eq!(validation_err_to_locale(&ValidationErr::F64, &locale), String::from("Debe ser un número de punto flotante"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Bool, &locale), String::from("Debe ser un booleano"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Str, &locale), String::from("Debe ser una cadena"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Email, &locale), String::from("Debe ser un correo electrónico"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Date, &locale), String::from("Debe ser una fecha"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Time, &locale), String::from("Debe ser una hora"));
        assert_eq!(validation_err_to_locale(&ValidationErr::DateTime, &locale), String::from("Debe ser una fecha y hora"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Debe ser igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Debe ser diferente de 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Debe ser mayor que 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Debe ser mayor o igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Debe ser menor que 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Debe ser menor o igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::U64(34)), Operand::Value(OperandValue::U64(43)))), &locale), String::from("Debe estar entre 34 y 43"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Debe ser igual a -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Debe ser diferente de -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Debe ser mayor que -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Debe ser mayor o igual a -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Debe ser menor que -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Debe ser menor o igual a -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::I64(-4)), Operand::Value(OperandValue::I64(4)))), &locale), String::from("Debe estar entre -4 y 4"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Debe ser igual a -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Debe ser diferente de -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Debe ser mayor que -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Debe ser mayor o igual a -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Debe ser menor que -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Debe ser menor o igual a -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::F64(-4.6)), Operand::Value(OperandValue::F64(-2.4)))), &locale), String::from("Debe estar entre -4.6 y -2.4"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Debe ser igual a false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Debe ser diferente de false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Debe ser mayor que false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Debe ser mayor o igual a false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Debe ser menor que false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Debe ser menor o igual a false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::Bool(false)), Operand::Value(OperandValue::Bool(true)))), &locale), String::from("Debe estar entre false y true"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Debe ser igual a \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Debe ser diferente de \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Debe ser mayor que \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Debe ser mayor o igual a \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Debe ser menor que \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Debe ser menor o igual a \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::Str(String::from("aurorae"))), Operand::Value(OperandValue::Str(String::from("crespúculum"))))), &locale), String::from("Debe estar entre \"aurorae\" y \"crespúculum\""));

        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(10)))), &locale), String::from("La cantidad de bytes debe ser igual a 10"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Ne(Operand::Value(OperandValue::USize(11)))), &locale), String::from("La cantidad de bytes debe ser diferente de 11"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(12)))), &locale), String::from("La cantidad de bytes debe ser mayor que 12"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(13)))), &locale), String::from("La cantidad de bytes debe ser mayor o igual a 13"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(14)))), &locale), String::from("La cantidad de bytes debe ser menor que 14"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(15)))), &locale), String::from("La cantidad de bytes debe ser menor o igual a 15"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(16)), Operand::Value(OperandValue::USize(17)))), &locale), String::from("La cantidad de bytes debe estar entre 16 y 17"));

        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Eq(Operand::Value(OperandValue::USize(18)))), &locale), String::from("La cantidad de caracteres debe ser igual a 18"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(19)))), &locale), String::from("La cantidad de caracteres debe ser diferente de 19"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Gt(Operand::Value(OperandValue::USize(20)))), &locale), String::from("La cantidad de caracteres debe ser mayor que 20"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Ge(Operand::Value(OperandValue::USize(21)))), &locale), String::from("La cantidad de caracteres debe ser mayor o igual a 21"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Lt(Operand::Value(OperandValue::USize(22)))), &locale), String::from("La cantidad de caracteres debe ser menor que 22"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Le(Operand::Value(OperandValue::USize(23)))), &locale), String::from("La cantidad de caracteres debe ser menor o igual a 23"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Btwn(Operand::Value(OperandValue::USize(24)), Operand::Value(OperandValue::USize(25)))), &locale), String::from("La cantidad de caracteres debe estar entre 24 y 25"));

        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Eq(Operand::Value(OperandValue::USize(26)))), &locale), String::from("La cantidad de grafemas debe ser igual a 26"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Ne(Operand::Value(OperandValue::USize(27)))), &locale), String::from("La cantidad de grafemas debe ser diferente de 27"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(28)))), &locale), String::from("La cantidad de grafemas debe ser mayor que 28"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Ge(Operand::Value(OperandValue::USize(29)))), &locale), String::from("La cantidad de grafemas debe ser mayor o igual a 29"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Lt(Operand::Value(OperandValue::USize(30)))), &locale), String::from("La cantidad de grafemas debe ser menor que 30"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Le(Operand::Value(OperandValue::USize(31)))), &locale), String::from("La cantidad de grafemas debe ser menor o igual a 31"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Btwn(Operand::Value(OperandValue::USize(32)), Operand::Value(OperandValue::USize(33)))), &locale), String::from("La cantidad de grafemas debe estar entre 32 y 33"));

        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(42)))), &locale), String::from("La cantidad de caracteres en minúsculas debe ser igual a 42"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(43)))), &locale), String::from("La cantidad de caracteres en minúsculas debe ser diferente de 43"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(44)))), &locale), String::from("La cantidad de caracteres en minúsculas debe ser mayor que 44"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(45)))), &locale), String::from("La cantidad de caracteres en minúsculas debe ser mayor o igual a 45"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(46)))), &locale), String::from("La cantidad de caracteres en minúsculas debe ser menor que 46"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Le(Operand::Value(OperandValue::USize(47)))), &locale), String::from("La cantidad de caracteres en minúsculas debe ser menor o igual a 47"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(48)), Operand::Value(OperandValue::USize(49)))), &locale), String::from("La cantidad de caracteres en minúsculas debe estar entre 48 y 49"));

        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(34)))), &locale), String::from("La cantidad de caracteres en mayúsculas debe ser igual a 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(35)))), &locale), String::from("La cantidad de caracteres en mayúsculas debe ser diferente de 35"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(36)))), &locale), String::from("La cantidad de caracteres en mayúsculas debe ser mayor que 36"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(37)))), &locale), String::from("La cantidad de caracteres en mayúsculas debe ser mayor o igual a 37"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(38)))), &locale), String::from("La cantidad de caracteres en mayúsculas debe ser menor que 38"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Le(Operand::Value(OperandValue::USize(39)))), &locale), String::from("La cantidad de caracteres en mayúsculas debe ser menor o igual a 39"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(40)), Operand::Value(OperandValue::USize(41)))), &locale), String::from("La cantidad de caracteres en mayúsculas debe estar entre 40 y 41"));

        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Eq(Operand::Value(OperandValue::USize(50)))), &locale), String::from("La cantidad de números debe ser igual a 50"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Ne(Operand::Value(OperandValue::USize(51)))), &locale), String::from("La cantidad de números debe ser diferente de 51"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Gt(Operand::Value(OperandValue::USize(52)))), &locale), String::from("La cantidad de números debe ser mayor que 52"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Ge(Operand::Value(OperandValue::USize(53)))), &locale), String::from("La cantidad de números debe ser mayor o igual a 53"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Lt(Operand::Value(OperandValue::USize(54)))), &locale), String::from("La cantidad de números debe ser menor que 54"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(55)))), &locale), String::from("La cantidad de números debe ser menor o igual a 55"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Btwn(Operand::Value(OperandValue::USize(56)), Operand::Value(OperandValue::USize(57)))), &locale), String::from("La cantidad de números debe estar entre 56 y 57"));

        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Eq(Operand::Value(OperandValue::USize(58)))), &locale), String::from("La cantidad de símbolos debe ser igual a 58"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Ne(Operand::Value(OperandValue::USize(59)))), &locale), String::from("La cantidad de símbolos debe ser diferente de 59"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Gt(Operand::Value(OperandValue::USize(60)))), &locale), String::from("La cantidad de símbolos debe ser mayor que 60"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Ge(Operand::Value(OperandValue::USize(61)))), &locale), String::from("La cantidad de símbolos debe ser mayor o igual a 61"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Lt(Operand::Value(OperandValue::USize(62)))), &locale), String::from("La cantidad de símbolos debe ser menor que 62"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Le(Operand::Value(OperandValue::USize(63)))), &locale), String::from("La cantidad de símbolos debe ser menor o igual a 63"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Btwn(Operand::Value(OperandValue::USize(64)), Operand::Value(OperandValue::USize(65)))), &locale), String::from("La cantidad de símbolos debe estar entre 64 y 65"));
    }

    #[test]
    fn test_locale_en_long() {
        let locale = locale_en_long();
        assert_eq!(validation_err_to_locale(&ValidationErr::Required, &locale), String::from("Is required"));
        assert_eq!(validation_err_to_locale(&ValidationErr::U64, &locale), String::from("Must be an unsigned integer"));
        assert_eq!(validation_err_to_locale(&ValidationErr::I64, &locale), String::from("Must be an integer"));
        assert_eq!(validation_err_to_locale(&ValidationErr::F64, &locale), String::from("Must be a float"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Bool, &locale), String::from("Must be a boolean"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Str, &locale), String::from("Must be a string"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Email, &locale), String::from("Must be an e-mail"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Date, &locale), String::from("Must be a date"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Time, &locale), String::from("Must be a time"));
        assert_eq!(validation_err_to_locale(&ValidationErr::DateTime, &locale), String::from("Must be a date and time"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Must be equals to 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Must be different from 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Must be greater than 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Must be greater than or equals to 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Must be smaller than 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::U64(34)))), &locale), String::from("Must be smaller than or equals to 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::U64(34)), Operand::Value(OperandValue::U64(43)))), &locale), String::from("Must be between 34 and 43"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Must be equals to -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Must be different from -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Must be greater than -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Must be greater than or equals to -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Must be smaller than -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::I64(-4)))), &locale), String::from("Must be smaller than or equals to -4"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::I64(-4)), Operand::Value(OperandValue::I64(4)))), &locale), String::from("Must be between -4 and 4"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Must be equals to -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Must be different from -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Must be greater than -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Must be greater than or equals to -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Must be smaller than -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::F64(-4.6)))), &locale), String::from("Must be smaller than or equals to -4.6"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::F64(-4.6)), Operand::Value(OperandValue::F64(-2.4)))), &locale), String::from("Must be between -4.6 and -2.4"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Must be equals to false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Must be different from false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Must be greater than false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Must be greater than or equals to false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Must be smaller than false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::Bool(false)))), &locale), String::from("Must be smaller than or equals to false"));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::Bool(false)), Operand::Value(OperandValue::Bool(true)))), &locale), String::from("Must be between false and true"));

        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Must be equals to \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Must be different from \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Must be greater than \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Must be greater than or equals to \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Must be smaller than \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::Str(String::from("aurorae"))))), &locale), String::from("Must be smaller than or equals to \"aurorae\""));
        assert_eq!(validation_err_to_locale(&ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::Str(String::from("aurorae"))), Operand::Value(OperandValue::Str(String::from("crespúculum"))))), &locale), String::from("Must be between \"aurorae\" and \"crespúculum\""));

        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(10)))), &locale), String::from("The length of bytes must be equals to 10"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Ne(Operand::Value(OperandValue::USize(11)))), &locale), String::from("The length of bytes must be different from 11"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(12)))), &locale), String::from("The length of bytes must be greater than 12"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(13)))), &locale), String::from("The length of bytes must be greater than or equals to 13"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(14)))), &locale), String::from("The length of bytes must be smaller than 14"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(15)))), &locale), String::from("The length of bytes must be smaller than or equals to 15"));
        assert_eq!(validation_err_to_locale(&ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(16)), Operand::Value(OperandValue::USize(17)))), &locale), String::from("The length of bytes must be between 16 and 17"));

        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Eq(Operand::Value(OperandValue::USize(18)))), &locale), String::from("The length of characters must be equals to 18"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(19)))), &locale), String::from("The length of characters must be different from 19"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Gt(Operand::Value(OperandValue::USize(20)))), &locale), String::from("The length of characters must be greater than 20"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Ge(Operand::Value(OperandValue::USize(21)))), &locale), String::from("The length of characters must be greater than or equals to 21"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Lt(Operand::Value(OperandValue::USize(22)))), &locale), String::from("The length of characters must be smaller than 22"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Le(Operand::Value(OperandValue::USize(23)))), &locale), String::from("The length of characters must be smaller than or equals to 23"));
        assert_eq!(validation_err_to_locale(&ValidationErr::CharsLen(Operation::Btwn(Operand::Value(OperandValue::USize(24)), Operand::Value(OperandValue::USize(25)))), &locale), String::from("The length of characters must be between 24 and 25"));

        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Eq(Operand::Value(OperandValue::USize(26)))), &locale), String::from("The length of graphemes must be equals to 26"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Ne(Operand::Value(OperandValue::USize(27)))), &locale), String::from("The length of graphemes must be different from 27"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(28)))), &locale), String::from("The length of graphemes must be greater than 28"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Ge(Operand::Value(OperandValue::USize(29)))), &locale), String::from("The length of graphemes must be greater than or equals to 29"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Lt(Operand::Value(OperandValue::USize(30)))), &locale), String::from("The length of graphemes must be smaller than 30"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Le(Operand::Value(OperandValue::USize(31)))), &locale), String::from("The length of graphemes must be smaller than or equals to 31"));
        assert_eq!(validation_err_to_locale(&ValidationErr::GraphemesLen(Operation::Btwn(Operand::Value(OperandValue::USize(32)), Operand::Value(OperandValue::USize(33)))), &locale), String::from("The length of graphemes must be between 32 and 33"));

        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(42)))), &locale), String::from("The length of lowercase characters must be equals to 42"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(43)))), &locale), String::from("The length of lowercase characters must be different from 43"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(44)))), &locale), String::from("The length of lowercase characters must be greater than 44"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(45)))), &locale), String::from("The length of lowercase characters must be greater than or equals to 45"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(46)))), &locale), String::from("The length of lowercase characters must be smaller than 46"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Le(Operand::Value(OperandValue::USize(47)))), &locale), String::from("The length of lowercase characters must be smaller than or equals to 47"));
        assert_eq!(validation_err_to_locale(&ValidationErr::LowercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(48)), Operand::Value(OperandValue::USize(49)))), &locale), String::from("The length of lowercase characters must be between 48 and 49"));

        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(34)))), &locale), String::from("The length of uppercase characters must be equals to 34"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(35)))), &locale), String::from("The length of uppercase characters must be different from 35"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(36)))), &locale), String::from("The length of uppercase characters must be greater than 36"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(37)))), &locale), String::from("The length of uppercase characters must be greater than or equals to 37"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(38)))), &locale), String::from("The length of uppercase characters must be smaller than 38"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Le(Operand::Value(OperandValue::USize(39)))), &locale), String::from("The length of uppercase characters must be smaller than or equals to 39"));
        assert_eq!(validation_err_to_locale(&ValidationErr::UppercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(40)), Operand::Value(OperandValue::USize(41)))), &locale), String::from("The length of uppercase characters must be between 40 and 41"));

        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Eq(Operand::Value(OperandValue::USize(50)))), &locale), String::from("The length of numbers must be equals to 50"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Ne(Operand::Value(OperandValue::USize(51)))), &locale), String::from("The length of numbers must be different from 51"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Gt(Operand::Value(OperandValue::USize(52)))), &locale), String::from("The length of numbers must be greater than 52"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Ge(Operand::Value(OperandValue::USize(53)))), &locale), String::from("The length of numbers must be greater than or equals to 53"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Lt(Operand::Value(OperandValue::USize(54)))), &locale), String::from("The length of numbers must be smaller than 54"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(55)))), &locale), String::from("The length of numbers must be smaller than or equals to 55"));
        assert_eq!(validation_err_to_locale(&ValidationErr::NumbersLen(Operation::Btwn(Operand::Value(OperandValue::USize(56)), Operand::Value(OperandValue::USize(57)))), &locale), String::from("The length of numbers must be between 56 and 57"));

        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Eq(Operand::Value(OperandValue::USize(58)))), &locale), String::from("The length of symbols must be equals to 58"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Ne(Operand::Value(OperandValue::USize(59)))), &locale), String::from("The length of symbols must be different from 59"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Gt(Operand::Value(OperandValue::USize(60)))), &locale), String::from("The length of symbols must be greater than 60"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Ge(Operand::Value(OperandValue::USize(61)))), &locale), String::from("The length of symbols must be greater than or equals to 61"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Lt(Operand::Value(OperandValue::USize(62)))), &locale), String::from("The length of symbols must be smaller than 62"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Le(Operand::Value(OperandValue::USize(63)))), &locale), String::from("The length of symbols must be smaller than or equals to 63"));
        assert_eq!(validation_err_to_locale(&ValidationErr::SymbolsLen(Operation::Btwn(Operand::Value(OperandValue::USize(64)), Operand::Value(OperandValue::USize(65)))), &locale), String::from("The length of symbols must be between 64 and 65"));
    }
}

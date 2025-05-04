use araucaria::locale::Locale;

pub fn locale_pt_long() -> Locale {
    Locale {
        required: "É obrigatório".into(),
        u64: "Deve ser um número inteiro sem sinal de 64 bits".into(),
        i64: "Deve ser um número inteiro de 64 bits".into(),
        f64: "Deve ser um número com ponto flutuante de 64 bits".into(),
        usize: "Deve ser um número inteiro sem sinal".into(),
        isize: "Deve ser um número inteiro".into(),
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
        eq_field: "Deve ser igual ao campo %value%".into(),
        ne_field: "Deve ser diferente do campo %value%".into(),
        gt_field: "Deve ser maior que o campo %value%".into(),
        ge_field: "Deve ser maior ou igual ao campo %value%".into(),
        lt_field: "Deve ser menor que o campo %value%".into(),
        le_field: "Deve ser menor ou igual ao campo %value%".into(),
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
        enumerated: "Deve ser um dos valores %value%".into(),
    }
}

pub fn locale_es_long() -> Locale {
    Locale {
        required: "Se requiere".into(),
        u64: "Debe ser un número entero sin signo de 64 bits".into(),
        i64: "Debe ser un número entero de 64 bits".into(),
        f64: "Debe ser un número de punto flotante de 64 bits".into(),
        usize: "Debe ser un número entero sin signo".into(),
        isize: "Debe ser un número entero".into(),
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
        eq_field: "Debe ser igual al campo %value%".into(),
        ne_field: "Debe ser diferente del campo %value%".into(),
        gt_field: "Debe ser mayor que el campo %value%".into(),
        ge_field: "Debe ser mayor o igual al campo %value%".into(),
        lt_field: "Debe ser menor que el campo %value%".into(),
        le_field: "Debe ser menor o igual al campo %value%".into(),
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
        enumerated: "Debe ser uno de los valores %value%".into(),
    }
}

pub fn locale_en_long() -> Locale {
    Locale {
        required: "Is required".into(),
        u64: "Must be an 64 bits unsigned integer".into(),
        i64: "Must be an 64 bits integer".into(),
        f64: "Must be a 64 bits float".into(),
        usize: "Must be an unsigned integer".into(),
        isize: "Must be an integer".into(),
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
        eq_field: "Must be equals to the field %value%".into(),
        ne_field: "Must be different from the field %value%".into(),
        gt_field: "Must be greater than the field %value%".into(),
        ge_field: "Must be greater than or equals to the field %value%".into(),
        lt_field: "Must be smaller than the field %value%".into(),
        le_field: "Must be smaller than or equals to the field %value%".into(),
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
        enumerated: "Must be one of the values %value%".into(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use araucaria::{
        error::ValidationErr,
        locale::localize_validation_err,
        operation::{Operand, OperandValue, Operation},
        schema::EnumValues,
    };

    use super::{locale_en_long, locale_es_long, locale_pt_long};

    const USIZE_VALUES: [usize; 6] = [0, 1, 2, 3, 4, 5];
    const ISIZE_VALUES: [isize; 5] = [-2, -1, 0, 1, 2];
    const STR_VALUES: [&str; 3] = ["APPLE", "GRAPE", "PEAR"];

    static STR_VALUE_A: LazyLock<Operand> = LazyLock::new(|| Operand::Value(OperandValue::from("aurorae")));
    static STR_VALUE_B: LazyLock<Operand> = LazyLock::new(|| Operand::Value(OperandValue::from("crespúculum")));

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

    const U64_VALUE: Operand = Operand::Value(OperandValue::U64(34));
    const I64_VALUE: Operand = Operand::Value(OperandValue::I64(-4));
    const F64_VALUE: Operand = Operand::Value(OperandValue::F64(-4.6));
    const USIZE_VALUE_A: Operand = Operand::Value(OperandValue::USize(27));
    const USIZE_VALUE_B: Operand = Operand::Value(OperandValue::USize(39));
    const ISIZE_VALUE_A: Operand = Operand::Value(OperandValue::ISize(-93));
    const BOOL_VALUE: Operand = Operand::Value(OperandValue::Bool(false));
    const OP_U64: ValidationErr = ValidationErr::Operation(Operation::Eq(U64_VALUE));
    const OP_I64: ValidationErr = ValidationErr::Operation(Operation::Ne(I64_VALUE));
    const OP_F64: ValidationErr = ValidationErr::Operation(Operation::Gt(F64_VALUE));
    const OP_USIZE: ValidationErr = ValidationErr::Operation(Operation::Ge(USIZE_VALUE_A));
    const OP_ISIZE: ValidationErr = ValidationErr::Operation(Operation::Lt(ISIZE_VALUE_A));
    const OP_BOOL: ValidationErr = ValidationErr::Operation(Operation::Le(BOOL_VALUE));
    static OP_STR: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Operation(Operation::Btwn(STR_VALUE_A.clone(), STR_VALUE_B.clone())));

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

    const LOWER_LEN_EQ: ValidationErr = ValidationErr::LowercaseLen(Operation::Eq(USIZE_VALUE_A));
    const LOWER_LEN_NE: ValidationErr = ValidationErr::LowercaseLen(Operation::Ne(USIZE_VALUE_A));
    const LOWER_LEN_GT: ValidationErr = ValidationErr::LowercaseLen(Operation::Gt(USIZE_VALUE_A));
    const LOWER_LEN_GE: ValidationErr = ValidationErr::LowercaseLen(Operation::Ge(USIZE_VALUE_A));
    const LOWER_LEN_LT: ValidationErr = ValidationErr::LowercaseLen(Operation::Lt(USIZE_VALUE_A));
    const LOWER_LEN_LE: ValidationErr = ValidationErr::LowercaseLen(Operation::Le(USIZE_VALUE_A));
    const LOWER_LEN_BTWN: ValidationErr = ValidationErr::LowercaseLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));

    const UPPER_LEN_EQ: ValidationErr = ValidationErr::UppercaseLen(Operation::Eq(USIZE_VALUE_A));
    const UPPER_LEN_NE: ValidationErr = ValidationErr::UppercaseLen(Operation::Ne(USIZE_VALUE_A));
    const UPPER_LEN_GT: ValidationErr = ValidationErr::UppercaseLen(Operation::Gt(USIZE_VALUE_A));
    const UPPER_LEN_GE: ValidationErr = ValidationErr::UppercaseLen(Operation::Ge(USIZE_VALUE_A));
    const UPPER_LEN_LT: ValidationErr = ValidationErr::UppercaseLen(Operation::Lt(USIZE_VALUE_A));
    const UPPER_LEN_LE: ValidationErr = ValidationErr::UppercaseLen(Operation::Le(USIZE_VALUE_A));
    const UPPER_LEN_BTWN: ValidationErr = ValidationErr::UppercaseLen(Operation::Btwn(USIZE_VALUE_A, USIZE_VALUE_B));

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

    static ENUM_USIZE: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Enumerated(EnumValues::from(USIZE_VALUES)));
    static ENUM_ISIZE: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Enumerated(EnumValues::from(ISIZE_VALUES)));
    static ENUM_STR: LazyLock<ValidationErr> = LazyLock::new(|| ValidationErr::Enumerated(EnumValues::from(STR_VALUES)));

    #[test]
    fn validation_err_to_locale_locale_pt_long() {
        let l = locale_pt_long();

        assert_eq!(localize_validation_err(&REQUIRED, &l), "É obrigatório".to_string());
        assert_eq!(localize_validation_err(&U64, &l), "Deve ser um número inteiro sem sinal de 64 bits".to_string());
        assert_eq!(localize_validation_err(&I64, &l), "Deve ser um número inteiro de 64 bits".to_string());
        assert_eq!(localize_validation_err(&F64, &l), "Deve ser um número com ponto flutuante de 64 bits".to_string());
        assert_eq!(localize_validation_err(&USIZE, &l), "Deve ser um número inteiro sem sinal".to_string());
        assert_eq!(localize_validation_err(&ISIZE, &l), "Deve ser um número inteiro".to_string());
        assert_eq!(localize_validation_err(&BOOL, &l), "Deve ser um booleano".to_string());
        assert_eq!(localize_validation_err(&STR, &l), "Deve ser uma string".to_string());
        assert_eq!(localize_validation_err(&EMAIL, &l), "Deve ser um e-mail".to_string());
        assert_eq!(localize_validation_err(&DATE, &l), "Deve ser uma data".to_string());
        assert_eq!(localize_validation_err(&TIME, &l), "Deve ser uma hora".to_string());
        assert_eq!(localize_validation_err(&DATE_TIME, &l), "Deve ser uma data e hora".to_string());

        assert_eq!(localize_validation_err(&OP_U64, &l), "Deve ser igual a 34".to_string());
        assert_eq!(localize_validation_err(&OP_I64, &l), "Deve ser diferente de -4".to_string());
        assert_eq!(localize_validation_err(&OP_F64, &l), "Deve ser maior que -4.6".to_string());
        assert_eq!(localize_validation_err(&OP_USIZE, &l), "Deve ser maior ou igual a 27".to_string());
        assert_eq!(localize_validation_err(&OP_ISIZE, &l), "Deve ser menor que -93".to_string());
        assert_eq!(localize_validation_err(&OP_BOOL, &l), "Deve ser menor ou igual a false".to_string());
        assert_eq!(localize_validation_err(&OP_STR, &l), r#"Deve estar entre "aurorae" e "crespúculum""#.to_string());

        assert_eq!(localize_validation_err(&BYTES_LEN_EQ, &l), "A quantidade de bytes deve ser igual a 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_NE, &l), "A quantidade de bytes deve ser diferente de 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_GT, &l), "A quantidade de bytes deve ser maior que 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_GE, &l), "A quantidade de bytes deve ser maior ou igual a 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_LT, &l), "A quantidade de bytes deve ser menor que 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_LE, &l), "A quantidade de bytes deve ser menor ou igual a 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_BTWN, &l), "A quantidade de bytes deve estar entre 27 e 39".to_string());

        assert_eq!(localize_validation_err(&CHARS_LEN_EQ, &l), "A quantidade de caracteres deve ser igual a 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_NE, &l), "A quantidade de caracteres deve ser diferente de 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_GT, &l), "A quantidade de caracteres deve ser maior que 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_GE, &l), "A quantidade de caracteres deve ser maior ou igual a 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_LT, &l), "A quantidade de caracteres deve ser menor que 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_LE, &l), "A quantidade de caracteres deve ser menor ou igual a 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_BTWN, &l), "A quantidade de caracteres deve estar entre 27 e 39".to_string());

        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_EQ, &l), "A quantidade de grafemas deve ser igual a 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_NE, &l), "A quantidade de grafemas deve ser diferente de 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_GT, &l), "A quantidade de grafemas deve ser maior que 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_GE, &l), "A quantidade de grafemas deve ser maior ou igual a 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_LT, &l), "A quantidade de grafemas deve ser menor que 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_LE, &l), "A quantidade de grafemas deve ser menor ou igual a 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_BTWN, &l), "A quantidade de grafemas deve estar entre 27 e 39".to_string());

        assert_eq!(localize_validation_err(&LOWER_LEN_EQ, &l), "A quantidade de caracteres minúsculos deve ser igual a 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_NE, &l), "A quantidade de caracteres minúsculos deve ser diferente de 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_GT, &l), "A quantidade de caracteres minúsculos deve ser maior que 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_GE, &l), "A quantidade de caracteres minúsculos deve ser maior ou igual a 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_LT, &l), "A quantidade de caracteres minúsculos deve ser menor que 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_LE, &l), "A quantidade de caracteres minúsculos deve ser menor ou igual a 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_BTWN, &l), "A quantidade de caracteres minúsculos deve estar entre 27 e 39".to_string());

        assert_eq!(localize_validation_err(&UPPER_LEN_EQ, &l), "A quantidade de caracteres maiúsculos deve ser igual a 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_NE, &l), "A quantidade de caracteres maiúsculos deve ser diferente de 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_GT, &l), "A quantidade de caracteres maiúsculos deve ser maior que 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_GE, &l), "A quantidade de caracteres maiúsculos deve ser maior ou igual a 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_LT, &l), "A quantidade de caracteres maiúsculos deve ser menor que 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_LE, &l), "A quantidade de caracteres maiúsculos deve ser menor ou igual a 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_BTWN, &l), "A quantidade de caracteres maiúsculos deve estar entre 27 e 39".to_string());

        assert_eq!(localize_validation_err(&NUMBERS_LEN_EQ, &l), "A quantidade de números deve ser igual a 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_NE, &l), "A quantidade de números deve ser diferente de 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_GT, &l), "A quantidade de números deve ser maior que 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_GE, &l), "A quantidade de números deve ser maior ou igual a 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_LT, &l), "A quantidade de números deve ser menor que 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_LE, &l), "A quantidade de números deve ser menor ou igual a 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_BTWN, &l), "A quantidade de números deve estar entre 27 e 39".to_string());

        assert_eq!(localize_validation_err(&SYMBOLS_LEN_EQ, &l), "A quantidade de símbolos deve ser igual a 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_NE, &l), "A quantidade de símbolos deve ser diferente de 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_GT, &l), "A quantidade de símbolos deve ser maior que 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_GE, &l), "A quantidade de símbolos deve ser maior ou igual a 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_LT, &l), "A quantidade de símbolos deve ser menor que 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_LE, &l), "A quantidade de símbolos deve ser menor ou igual a 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_BTWN, &l), "A quantidade de símbolos deve estar entre 27 e 39".to_string());

        assert_eq!(localize_validation_err(&ENUM_USIZE, &l), "Deve ser um dos valores [ 0, 1, 2, 3, 4, 5 ]".to_string());
        assert_eq!(localize_validation_err(&ENUM_ISIZE, &l), "Deve ser um dos valores [ -2, -1, 0, 1, 2 ]".to_string());
        assert_eq!(localize_validation_err(&ENUM_STR, &l), r#"Deve ser um dos valores [ "APPLE", "GRAPE", "PEAR" ]"#.to_string());
    }

    #[test]
    fn validation_err_to_locale_locale_es_long() {
        let l = locale_es_long();

        assert_eq!(localize_validation_err(&REQUIRED, &l), "Se requiere".to_string());
        assert_eq!(localize_validation_err(&U64, &l), "Debe ser un número entero sin signo de 64 bits".to_string());
        assert_eq!(localize_validation_err(&I64, &l), "Debe ser un número entero de 64 bits".to_string());
        assert_eq!(localize_validation_err(&F64, &l), "Debe ser un número de punto flotante de 64 bits".to_string());
        assert_eq!(localize_validation_err(&USIZE, &l), "Debe ser un número entero sin signo".to_string());
        assert_eq!(localize_validation_err(&ISIZE, &l), "Debe ser un número entero".to_string());
        assert_eq!(localize_validation_err(&BOOL, &l), "Debe ser un booleano".to_string());
        assert_eq!(localize_validation_err(&STR, &l), "Debe ser una cadena".to_string());
        assert_eq!(localize_validation_err(&EMAIL, &l), "Debe ser un correo electrónico".to_string());
        assert_eq!(localize_validation_err(&DATE, &l), "Debe ser una fecha".to_string());
        assert_eq!(localize_validation_err(&TIME, &l), "Debe ser una hora".to_string());
        assert_eq!(localize_validation_err(&DATE_TIME, &l), "Debe ser una fecha y hora".to_string());

        assert_eq!(localize_validation_err(&OP_U64, &l), "Debe ser igual a 34".to_string());
        assert_eq!(localize_validation_err(&OP_I64, &l), "Debe ser diferente de -4".to_string());
        assert_eq!(localize_validation_err(&OP_F64, &l), "Debe ser mayor que -4.6".to_string());
        assert_eq!(localize_validation_err(&OP_USIZE, &l), "Debe ser mayor o igual a 27".to_string());
        assert_eq!(localize_validation_err(&OP_ISIZE, &l), "Debe ser menor que -93".to_string());
        assert_eq!(localize_validation_err(&OP_BOOL, &l), "Debe ser menor o igual a false".to_string());
        assert_eq!(localize_validation_err(&OP_STR, &l), r#"Debe estar entre "aurorae" y "crespúculum""#.to_string());

        assert_eq!(localize_validation_err(&BYTES_LEN_EQ, &l), "La cantidad de bytes debe ser igual a 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_NE, &l), "La cantidad de bytes debe ser diferente de 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_GT, &l), "La cantidad de bytes debe ser mayor que 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_GE, &l), "La cantidad de bytes debe ser mayor o igual a 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_LT, &l), "La cantidad de bytes debe ser menor que 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_LE, &l), "La cantidad de bytes debe ser menor o igual a 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_BTWN, &l), "La cantidad de bytes debe estar entre 27 y 39".to_string());

        assert_eq!(localize_validation_err(&CHARS_LEN_EQ, &l), "La cantidad de caracteres debe ser igual a 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_NE, &l), "La cantidad de caracteres debe ser diferente de 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_GT, &l), "La cantidad de caracteres debe ser mayor que 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_GE, &l), "La cantidad de caracteres debe ser mayor o igual a 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_LT, &l), "La cantidad de caracteres debe ser menor que 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_LE, &l), "La cantidad de caracteres debe ser menor o igual a 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_BTWN, &l), "La cantidad de caracteres debe estar entre 27 y 39".to_string());

        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_EQ, &l), "La cantidad de grafemas debe ser igual a 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_NE, &l), "La cantidad de grafemas debe ser diferente de 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_GT, &l), "La cantidad de grafemas debe ser mayor que 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_GE, &l), "La cantidad de grafemas debe ser mayor o igual a 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_LT, &l), "La cantidad de grafemas debe ser menor que 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_LE, &l), "La cantidad de grafemas debe ser menor o igual a 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_BTWN, &l), "La cantidad de grafemas debe estar entre 27 y 39".to_string());

        assert_eq!(localize_validation_err(&LOWER_LEN_EQ, &l), "La cantidad de caracteres en minúsculas debe ser igual a 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_NE, &l), "La cantidad de caracteres en minúsculas debe ser diferente de 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_GT, &l), "La cantidad de caracteres en minúsculas debe ser mayor que 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_GE, &l), "La cantidad de caracteres en minúsculas debe ser mayor o igual a 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_LT, &l), "La cantidad de caracteres en minúsculas debe ser menor que 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_LE, &l), "La cantidad de caracteres en minúsculas debe ser menor o igual a 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_BTWN, &l), "La cantidad de caracteres en minúsculas debe estar entre 27 y 39".to_string());

        assert_eq!(localize_validation_err(&UPPER_LEN_EQ, &l), "La cantidad de caracteres en mayúsculas debe ser igual a 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_NE, &l), "La cantidad de caracteres en mayúsculas debe ser diferente de 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_GT, &l), "La cantidad de caracteres en mayúsculas debe ser mayor que 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_GE, &l), "La cantidad de caracteres en mayúsculas debe ser mayor o igual a 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_LT, &l), "La cantidad de caracteres en mayúsculas debe ser menor que 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_LE, &l), "La cantidad de caracteres en mayúsculas debe ser menor o igual a 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_BTWN, &l), "La cantidad de caracteres en mayúsculas debe estar entre 27 y 39".to_string());

        assert_eq!(localize_validation_err(&NUMBERS_LEN_EQ, &l), "La cantidad de números debe ser igual a 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_NE, &l), "La cantidad de números debe ser diferente de 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_GT, &l), "La cantidad de números debe ser mayor que 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_GE, &l), "La cantidad de números debe ser mayor o igual a 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_LT, &l), "La cantidad de números debe ser menor que 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_LE, &l), "La cantidad de números debe ser menor o igual a 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_BTWN, &l), "La cantidad de números debe estar entre 27 y 39".to_string());

        assert_eq!(localize_validation_err(&SYMBOLS_LEN_EQ, &l), "La cantidad de símbolos debe ser igual a 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_NE, &l), "La cantidad de símbolos debe ser diferente de 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_GT, &l), "La cantidad de símbolos debe ser mayor que 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_GE, &l), "La cantidad de símbolos debe ser mayor o igual a 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_LT, &l), "La cantidad de símbolos debe ser menor que 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_LE, &l), "La cantidad de símbolos debe ser menor o igual a 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_BTWN, &l), "La cantidad de símbolos debe estar entre 27 y 39".to_string());

        assert_eq!(localize_validation_err(&ENUM_USIZE, &l), "Debe ser uno de los valores [ 0, 1, 2, 3, 4, 5 ]".to_string());
        assert_eq!(localize_validation_err(&ENUM_ISIZE, &l), "Debe ser uno de los valores [ -2, -1, 0, 1, 2 ]".to_string());
        assert_eq!(localize_validation_err(&ENUM_STR, &l), r#"Debe ser uno de los valores [ "APPLE", "GRAPE", "PEAR" ]"#.to_string());
    }

    #[test]
    fn validation_err_to_locale_locale_en_long() {
        let l = locale_en_long();

        assert_eq!(localize_validation_err(&REQUIRED, &l), "Is required".to_string());
        assert_eq!(localize_validation_err(&U64, &l), "Must be an 64 bits unsigned integer".to_string());
        assert_eq!(localize_validation_err(&I64, &l), "Must be an 64 bits integer".to_string());
        assert_eq!(localize_validation_err(&F64, &l), "Must be a 64 bits float".to_string());
        assert_eq!(localize_validation_err(&USIZE, &l), "Must be an unsigned integer".to_string());
        assert_eq!(localize_validation_err(&ISIZE, &l), "Must be an integer".to_string());
        assert_eq!(localize_validation_err(&BOOL, &l), "Must be a boolean".to_string());
        assert_eq!(localize_validation_err(&STR, &l), "Must be a string".to_string());
        assert_eq!(localize_validation_err(&EMAIL, &l), "Must be an e-mail".to_string());
        assert_eq!(localize_validation_err(&DATE, &l), "Must be a date".to_string());
        assert_eq!(localize_validation_err(&TIME, &l), "Must be a time".to_string());
        assert_eq!(localize_validation_err(&DATE_TIME, &l), "Must be a date and time".to_string());

        assert_eq!(localize_validation_err(&OP_U64, &l), "Must be equals to 34".to_string());
        assert_eq!(localize_validation_err(&OP_I64, &l), "Must be different from -4".to_string());
        assert_eq!(localize_validation_err(&OP_F64, &l), "Must be greater than -4.6".to_string());
        assert_eq!(localize_validation_err(&OP_USIZE, &l), "Must be greater than or equals to 27".to_string());
        assert_eq!(localize_validation_err(&OP_ISIZE, &l), "Must be smaller than -93".to_string());
        assert_eq!(localize_validation_err(&OP_BOOL, &l), "Must be smaller than or equals to false".to_string());
        assert_eq!(localize_validation_err(&OP_STR, &l), r#"Must be between "aurorae" and "crespúculum""#.to_string());

        assert_eq!(localize_validation_err(&BYTES_LEN_EQ, &l), "The length of bytes must be equals to 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_NE, &l), "The length of bytes must be different from 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_GT, &l), "The length of bytes must be greater than 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_GE, &l), "The length of bytes must be greater than or equals to 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_LT, &l), "The length of bytes must be smaller than 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_LE, &l), "The length of bytes must be smaller than or equals to 27".to_string());
        assert_eq!(localize_validation_err(&BYTES_LEN_BTWN, &l), "The length of bytes must be between 27 and 39".to_string());

        assert_eq!(localize_validation_err(&CHARS_LEN_EQ, &l), "The length of characters must be equals to 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_NE, &l), "The length of characters must be different from 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_GT, &l), "The length of characters must be greater than 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_GE, &l), "The length of characters must be greater than or equals to 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_LT, &l), "The length of characters must be smaller than 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_LE, &l), "The length of characters must be smaller than or equals to 27".to_string());
        assert_eq!(localize_validation_err(&CHARS_LEN_BTWN, &l), "The length of characters must be between 27 and 39".to_string());

        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_EQ, &l), "The length of graphemes must be equals to 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_NE, &l), "The length of graphemes must be different from 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_GT, &l), "The length of graphemes must be greater than 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_GE, &l), "The length of graphemes must be greater than or equals to 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_LT, &l), "The length of graphemes must be smaller than 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_LE, &l), "The length of graphemes must be smaller than or equals to 27".to_string());
        assert_eq!(localize_validation_err(&GRAPHEMES_LEN_BTWN, &l), "The length of graphemes must be between 27 and 39".to_string());

        assert_eq!(localize_validation_err(&LOWER_LEN_EQ, &l), "The length of lowercase characters must be equals to 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_NE, &l), "The length of lowercase characters must be different from 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_GT, &l), "The length of lowercase characters must be greater than 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_GE, &l), "The length of lowercase characters must be greater than or equals to 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_LT, &l), "The length of lowercase characters must be smaller than 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_LE, &l), "The length of lowercase characters must be smaller than or equals to 27".to_string());
        assert_eq!(localize_validation_err(&LOWER_LEN_BTWN, &l), "The length of lowercase characters must be between 27 and 39".to_string());

        assert_eq!(localize_validation_err(&UPPER_LEN_EQ, &l), "The length of uppercase characters must be equals to 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_NE, &l), "The length of uppercase characters must be different from 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_GT, &l), "The length of uppercase characters must be greater than 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_GE, &l), "The length of uppercase characters must be greater than or equals to 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_LT, &l), "The length of uppercase characters must be smaller than 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_LE, &l), "The length of uppercase characters must be smaller than or equals to 27".to_string());
        assert_eq!(localize_validation_err(&UPPER_LEN_BTWN, &l), "The length of uppercase characters must be between 27 and 39".to_string());

        assert_eq!(localize_validation_err(&NUMBERS_LEN_EQ, &l), "The length of numbers must be equals to 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_NE, &l), "The length of numbers must be different from 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_GT, &l), "The length of numbers must be greater than 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_GE, &l), "The length of numbers must be greater than or equals to 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_LT, &l), "The length of numbers must be smaller than 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_LE, &l), "The length of numbers must be smaller than or equals to 27".to_string());
        assert_eq!(localize_validation_err(&NUMBERS_LEN_BTWN, &l), "The length of numbers must be between 27 and 39".to_string());

        assert_eq!(localize_validation_err(&SYMBOLS_LEN_EQ, &l), "The length of symbols must be equals to 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_NE, &l), "The length of symbols must be different from 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_GT, &l), "The length of symbols must be greater than 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_GE, &l), "The length of symbols must be greater than or equals to 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_LT, &l), "The length of symbols must be smaller than 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_LE, &l), "The length of symbols must be smaller than or equals to 27".to_string());
        assert_eq!(localize_validation_err(&SYMBOLS_LEN_BTWN, &l), "The length of symbols must be between 27 and 39".to_string());

        assert_eq!(localize_validation_err(&ENUM_USIZE, &l), "Must be one of the values [ 0, 1, 2, 3, 4, 5 ]".to_string());
        assert_eq!(localize_validation_err(&ENUM_ISIZE, &l), "Must be one of the values [ -2, -1, 0, 1, 2 ]".to_string());
        assert_eq!(localize_validation_err(&ENUM_STR, &l), r#"Must be one of the values [ "APPLE", "GRAPE", "PEAR" ]"#.to_string());
    }
}

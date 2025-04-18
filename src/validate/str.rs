use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{compare, OperandValue},
    validation::str::StrValidation,
    value::Value,
};
use unicode_segmentation::UnicodeSegmentation;

fn bytes_len(str_value: &String) -> usize {
    str_value.len()
}

fn chars_len(str_value: &String) -> usize {
    str_value.chars().count()
}

fn graphemes_len(str_value: &String) -> usize {
    str_value.graphemes(true).collect::<Vec<&str>>().len()
}

fn lowercase_len(str_value: &String) -> usize {
    str_value.chars().filter(|c| c.is_lowercase()).count()
}

fn uppercase_len(str_value: &String) -> usize {
    str_value.chars().filter(|c| c.is_uppercase()).count()
}

fn numbers_len(str_value: &String) -> usize {
    str_value.chars().filter(|c| c.is_ascii_digit()).count()
}

fn symbols_len(str_value: &String) -> usize {
    str_value.chars().filter(|c| c.is_ascii_punctuation()).count()
}

pub fn validate_str(validation: &StrValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            if let Some(operation) = &validation.operation {
                if let Some(Err(())) = compare(operation, &OperandValue::Str(str_value.clone())) {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
            }
            if let Some(bytes_len_operation) = &validation.bytes_len {
                let len = bytes_len(str_value);
                if let Some(Err(())) = compare(bytes_len_operation, &OperandValue::USize(len)) {
                    base.push(ValidationErr::BytesLen(bytes_len_operation.clone()))
                }
            }
            if let Some(chars_len_operation) = &validation.chars_len {
                let len = chars_len(str_value);
                if let Some(Err(())) = compare(chars_len_operation, &OperandValue::USize(len)) {
                    base.push(ValidationErr::CharsLen(chars_len_operation.clone()))
                }
            }
            if let Some(graphemes_len_operation) = &validation.graphemes_len {
                let len = graphemes_len(str_value);
                if let Some(Err(())) = compare(graphemes_len_operation, &OperandValue::USize(len)) {
                    base.push(ValidationErr::GraphemesLen(graphemes_len_operation.clone()))
                }
            }
            if let Some(lowercase_len_operation) = &validation.lowercase_len {
                let len = lowercase_len(str_value);
                if let Some(Err(())) = compare(lowercase_len_operation, &OperandValue::USize(len)) {
                    base.push(ValidationErr::LowercaseLen(lowercase_len_operation.clone()))
                }
            }
            if let Some(uppercase_len_operation) = &validation.uppercase_len {
                let len = uppercase_len(str_value);
                if let Some(Err(())) = compare(uppercase_len_operation, &OperandValue::USize(len)) {
                    base.push(ValidationErr::UppercaseLen(uppercase_len_operation.clone()))
                }
            }
            if let Some(numbers_len_operation) = &validation.numbers_len {
                let len = numbers_len(str_value);
                if let Some(Err(())) = compare(numbers_len_operation, &OperandValue::USize(len)) {
                    base.push(ValidationErr::NumbersLen(numbers_len_operation.clone()))
                }
            }
            if let Some(symbols_len_operation) = &validation.symbols_len {
                let len = symbols_len(str_value);
                if let Some(Err(())) = compare(symbols_len_operation, &OperandValue::USize(len)) {
                    base.push(ValidationErr::SymbolsLen(symbols_len_operation.clone()))
                }
            }
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::Str);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
            if let Some(bytes_len_operation) = &validation.bytes_len {
                base.push(ValidationErr::BytesLen(bytes_len_operation.clone()))
            }
            if let Some(chars_len_operation) = &validation.chars_len {
                base.push(ValidationErr::CharsLen(chars_len_operation.clone()))
            }
            if let Some(graphemes_len_operation) = &validation.graphemes_len {
                base.push(ValidationErr::GraphemesLen(graphemes_len_operation.clone()))
            }
            if let Some(lowercase_len_operation) = &validation.lowercase_len {
                base.push(ValidationErr::LowercaseLen(lowercase_len_operation.clone()))
            }
            if let Some(uppercase_len_operation) = &validation.uppercase_len {
                base.push(ValidationErr::UppercaseLen(uppercase_len_operation.clone()))
            }
            if let Some(numbers_len_operation) = &validation.numbers_len {
                base.push(ValidationErr::NumbersLen(numbers_len_operation.clone()))
            }
            if let Some(symbols_len_operation) = &validation.symbols_len {
                base.push(ValidationErr::SymbolsLen(symbols_len_operation.clone()))
            }
        }
        _ => {
            base.push(ValidationErr::Str);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
            if let Some(bytes_len_operation) = &validation.bytes_len {
                base.push(ValidationErr::BytesLen(bytes_len_operation.clone()))
            }
            if let Some(chars_len_operation) = &validation.chars_len {
                base.push(ValidationErr::CharsLen(chars_len_operation.clone()))
            }
            if let Some(graphemes_len_operation) = &validation.graphemes_len {
                base.push(ValidationErr::GraphemesLen(graphemes_len_operation.clone()))
            }
            if let Some(lowercase_len_operation) = &validation.lowercase_len {
                base.push(ValidationErr::LowercaseLen(lowercase_len_operation.clone()))
            }
            if let Some(uppercase_len_operation) = &validation.uppercase_len {
                base.push(ValidationErr::UppercaseLen(uppercase_len_operation.clone()))
            }
            if let Some(numbers_len_operation) = &validation.numbers_len {
                base.push(ValidationErr::NumbersLen(numbers_len_operation.clone()))
            }
            if let Some(symbols_len_operation) = &validation.symbols_len {
                base.push(ValidationErr::SymbolsLen(symbols_len_operation.clone()))
            }
        }
    }
    if !base.is_empty() {
        Err(SchemaErr::Validation(base))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::str::StrValidation,
        value::{stub::num_u_stub, Value},
    };

    use super::{bytes_len, chars_len, graphemes_len, lowercase_len, numbers_len, symbols_len, uppercase_len, validate_str};

    #[test]
    fn test_validate_str_default() {
        let v = StrValidation::default();
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum")), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str])));
    }

    #[test]
    fn test_validate_str_optional() {
        let v = StrValidation::default().optional();
        assert_eq!(validate_str(&v, &Value::Str(String::from("Cogito ergo sum"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Str])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str])));
    }

    #[test]
    fn test_validate_str_eq() {
        let v = StrValidation::default().eq(String::from("Cogito ergo sum"));
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Str(String::from("Cogito ergo sum")))));
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum")), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("Memento mori")), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_str_ne() {
        let v = StrValidation::default().ne(String::from("Cogito ergo sum"));
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Str(String::from("Cogito ergo sum")))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("Memento mori"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("Cogito ergo sum"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_str_gt() {
        let v = StrValidation::default().gt(String::from("j"));
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::Str(String::from("j")))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("a"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("j"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("z"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_str_ge() {
        let v = StrValidation::default().ge(String::from("j"));
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::Str(String::from("j")))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("a"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("j"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("z"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_str_lt() {
        let v = StrValidation::default().lt(String::from("j"));
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::Str(String::from("j")))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("a"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("j"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_str_le() {
        let v = StrValidation::default().le(String::from("j"));
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::Str(String::from("j")))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("a"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("j"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("z"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_str_btwn() {
        let v = StrValidation::default().btwn(String::from("f"), String::from("l"));
        let op_err = ValidationErr::Operation(Operation::Btwn(
            Operand::Value(OperandValue::Str(String::from("f"))),
            Operand::Value(OperandValue::Str(String::from("l"))),
        ));
        assert_eq!(validate_str(&v, &Value::Str(String::from("e"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("f"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("i"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("l"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("m"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_bytes_len_eq() {
        let v = StrValidation::default().bytes_len_eq(16);
        let op_err = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(16))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_bytes_len_ne() {
        let v = StrValidation::default().bytes_len_ne(16);
        let op_err = ValidationErr::BytesLen(Operation::Ne(Operand::Value(OperandValue::USize(16))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_bytes_len_gt() {
        let v = StrValidation::default().bytes_len_gt(23);
        let op_err = ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(23))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_bytes_len_ge() {
        let v = StrValidation::default().bytes_len_ge(23);
        let op_err = ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(23))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_bytes_len_lt() {
        let v = StrValidation::default().bytes_len_lt(23);
        let op_err = ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(23))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_bytes_len_le() {
        let v = StrValidation::default().bytes_len_le(23);
        let op_err = ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(23))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_bytes_len_btwn() {
        let v = StrValidation::default().bytes_len_btwn(23, 29);
        let op_err = ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(23)), Operand::Value(OperandValue::USize(29))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_chars_len_eq() {
        let v = StrValidation::default().chars_len_eq(16);
        let op_err = ValidationErr::CharsLen(Operation::Eq(Operand::Value(OperandValue::USize(16))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_chars_len_ne() {
        let v = StrValidation::default().chars_len_ne(16);
        let op_err = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(16))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_chars_len_gt() {
        let v = StrValidation::default().chars_len_gt(12);
        let op_err = ValidationErr::CharsLen(Operation::Gt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_chars_len_ge() {
        let v = StrValidation::default().chars_len_ge(12);
        let op_err = ValidationErr::CharsLen(Operation::Ge(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_chars_len_lt() {
        let v = StrValidation::default().chars_len_lt(12);
        let op_err = ValidationErr::CharsLen(Operation::Lt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_chars_len_le() {
        let v = StrValidation::default().chars_len_le(12);
        let op_err = ValidationErr::CharsLen(Operation::Le(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_chars_len_btwn() {
        let v = StrValidation::default().chars_len_btwn(8, 12);
        let op_err = ValidationErr::CharsLen(Operation::Btwn(Operand::Value(OperandValue::USize(8)), Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ओंकार"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_graphemes_len_eq() {
        let v = StrValidation::default().graphemes_len_eq(16);
        let op_err = ValidationErr::GraphemesLen(Operation::Eq(Operand::Value(OperandValue::USize(16))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_graphemes_len_ne() {
        let v = StrValidation::default().graphemes_len_ne(16);
        let op_err = ValidationErr::GraphemesLen(Operation::Ne(Operand::Value(OperandValue::USize(16))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_graphemes_len_gt() {
        let v = StrValidation::default().graphemes_len_gt(12);
        let op_err = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_graphemes_len_ge() {
        let v = StrValidation::default().graphemes_len_ge(12);
        let op_err = ValidationErr::GraphemesLen(Operation::Ge(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_graphemes_len_lt() {
        let v = StrValidation::default().graphemes_len_lt(12);
        let op_err = ValidationErr::GraphemesLen(Operation::Lt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_graphemes_len_le() {
        let v = StrValidation::default().graphemes_len_le(12);
        let op_err = ValidationErr::GraphemesLen(Operation::Le(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_graphemes_len_btwn() {
        let v = StrValidation::default().graphemes_len_btwn(8, 12);
        let op_err = ValidationErr::GraphemesLen(Operation::Btwn(Operand::Value(OperandValue::USize(8)), Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ओंकार"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_lowercase_len_eq() {
        let v = StrValidation::default().lowercase_len_eq(12);
        let op_err = ValidationErr::LowercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_lowercase_len_ne() {
        let v = StrValidation::default().lowercase_len_ne(12);
        let op_err = ValidationErr::LowercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_lowercase_len_gt() {
        let v = StrValidation::default().lowercase_len_gt(12);
        let op_err = ValidationErr::LowercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_lowercase_len_ge() {
        let v = StrValidation::default().lowercase_len_ge(12);
        let op_err = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_lowercase_len_lt() {
        let v = StrValidation::default().lowercase_len_lt(12);
        let op_err = ValidationErr::LowercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_lowercase_len_le() {
        let v = StrValidation::default().lowercase_len_le(12);
        let op_err = ValidationErr::LowercaseLen(Operation::Le(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_lowercase_len_btwn() {
        let v = StrValidation::default().lowercase_len_btwn(11, 12);
        let op_err = ValidationErr::LowercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(11)), Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_uppercase_len_eq() {
        let v = StrValidation::default().uppercase_len_eq(12);
        let op_err = ValidationErr::UppercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("VENI, VIDI, VICI"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ГРУППА КРОВИ"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_uppercase_len_ne() {
        let v = StrValidation::default().uppercase_len_ne(12);
        let op_err = ValidationErr::UppercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("VENI, VIDI, VICI"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ГРУППА КРОВИ"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_uppercase_len_gt() {
        let v = StrValidation::default().uppercase_len_gt(12);
        let op_err = ValidationErr::UppercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ГРУППА КРОВИ"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("VENI, VIDI, VICI"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_uppercase_len_ge() {
        let v = StrValidation::default().uppercase_len_ge(12);
        let op_err = ValidationErr::UppercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ГРУППА КРОВИ"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("VENI, VIDI, VICI"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_uppercase_len_lt() {
        let v = StrValidation::default().uppercase_len_lt(12);
        let op_err = ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ГРУППА КРОВИ"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("VENI, VIDI, VICI"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_uppercase_len_le() {
        let v = StrValidation::default().uppercase_len_le(12);
        let op_err = ValidationErr::UppercaseLen(Operation::Le(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ГРУППА КРОВИ"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("VENI, VIDI, VICI"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_uppercase_len_btwn() {
        let v = StrValidation::default().uppercase_len_btwn(11, 12);
        let op_err = ValidationErr::UppercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(11)), Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ГРУППА КРОВИ"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("VENI, VIDI, VICI"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_numbers_len_eq() {
        let v = StrValidation::default().numbers_len_eq(2);
        let op_err = ValidationErr::NumbersLen(Operation::Eq(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("1"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("22"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("333"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_numbers_len_ne() {
        let v = StrValidation::default().numbers_len_ne(2);
        let op_err = ValidationErr::NumbersLen(Operation::Ne(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("1"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("22"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("333"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_numbers_len_gt() {
        let v = StrValidation::default().numbers_len_gt(2);
        let op_err = ValidationErr::NumbersLen(Operation::Gt(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("1"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("22"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("333"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_numbers_len_ge() {
        let v = StrValidation::default().numbers_len_ge(2);
        let op_err = ValidationErr::NumbersLen(Operation::Ge(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("1"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("22"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("333"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_numbers_len_lt() {
        let v = StrValidation::default().numbers_len_lt(2);
        let op_err = ValidationErr::NumbersLen(Operation::Lt(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("1"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("22"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("333"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_numbers_len_le() {
        let v = StrValidation::default().numbers_len_le(2);
        let op_err = ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("1"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("22"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("333"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_numbers_len_btwn() {
        let v = StrValidation::default().numbers_len_btwn(2, 3);
        let op_err = ValidationErr::NumbersLen(Operation::Btwn(Operand::Value(OperandValue::USize(2)), Operand::Value(OperandValue::USize(3))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("1"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("22"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("333"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("4444"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_symbols_len_eq() {
        let v = StrValidation::default().symbols_len_eq(2);
        let op_err = ValidationErr::SymbolsLen(Operation::Eq(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("!"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("@#"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("$%^"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_symbols_len_ne() {
        let v = StrValidation::default().symbols_len_ne(2);
        let op_err = ValidationErr::SymbolsLen(Operation::Ne(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("!"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("@#"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("$%^"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_symbols_len_gt() {
        let v = StrValidation::default().symbols_len_gt(2);
        let op_err = ValidationErr::SymbolsLen(Operation::Gt(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("!"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("@#"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("$%^"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_symbols_len_ge() {
        let v = StrValidation::default().symbols_len_ge(2);
        let op_err = ValidationErr::SymbolsLen(Operation::Ge(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("!"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("@#"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("$%^"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_symbols_len_lt() {
        let v = StrValidation::default().symbols_len_lt(2);
        let op_err = ValidationErr::SymbolsLen(Operation::Lt(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("!"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("@#"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("$%^"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_symbols_len_le() {
        let v = StrValidation::default().symbols_len_le(2);
        let op_err = ValidationErr::SymbolsLen(Operation::Le(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("!"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("@#"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("$%^"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_validate_symbols_len_btwn() {
        let v = StrValidation::default().symbols_len_btwn(2, 3);
        let op_err = ValidationErr::SymbolsLen(Operation::Btwn(Operand::Value(OperandValue::USize(2)), Operand::Value(OperandValue::USize(3))));
        assert_eq!(validate_str(&v, &Value::Str(String::from("!"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::Str(String::from("@#"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("$%^"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("&*()"))), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn test_bytes_len() {
        assert_eq!(bytes_len(&String::from("veni, vidi, vici")), 16);
        assert_eq!(bytes_len(&String::from("ὅσον ζῇς, φαίνου")), 31);
        assert_eq!(bytes_len(&String::from("группа крови")), 23);
        assert_eq!(bytes_len(&String::from("ओंकार")), 15);
        assert_eq!(bytes_len(&String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕")), 29);
    }

    #[test]
    fn test_bytes_len_emoji() {
        assert_eq!(bytes_len(&String::from("👩‍👩‍👧‍👧")), 25);
        assert_eq!(bytes_len(&String::from("👩‍👩‍👧")), 18);
    }

    #[test]
    fn test_chars_len() {
        assert_eq!(chars_len(&String::from("veni, vidi, vici")), 16);
        assert_eq!(chars_len(&String::from("ὅσον ζῇς, φαίνου")), 16);
        assert_eq!(chars_len(&String::from("группа крови")), 12);
        assert_eq!(chars_len(&String::from("ओंकार")), 5);
        assert_eq!(chars_len(&String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕")), 8);
    }

    #[test]
    fn test_chars_len_emoji() {
        assert_eq!(chars_len(&String::from("👩‍👩‍👧‍👧")), 7);
        assert_eq!(chars_len(&String::from("👩‍👩‍👧")), 5);
    }

    #[test]
    fn test_graphemes_len() {
        assert_eq!(graphemes_len(&String::from("veni, vidi, vici")), 16);
        assert_eq!(graphemes_len(&String::from("ὅσον ζῇς, φαίνου")), 16);
        assert_eq!(graphemes_len(&String::from("группа крови")), 12);
        assert_eq!(graphemes_len(&String::from("ओंकार")), 3);
        assert_eq!(graphemes_len(&String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕")), 8);
    }

    #[test]
    fn test_graphemes_len_emoji() {
        assert_eq!(graphemes_len(&String::from("👩‍👩‍👧‍👧")), 1);
        assert_eq!(graphemes_len(&String::from("👩‍👩‍👧")), 1);
    }

    #[test]
    fn test_lowercase_len_lowercase() {
        assert_eq!(lowercase_len(&String::from("группа крови")), 11);
        assert_eq!(lowercase_len(&String::from("veni, vidi, vici")), 12);
        assert_eq!(lowercase_len(&String::from("ὅσον ζῇς, φαίνου")), 13);
    }

    #[test]
    fn test_lowercase_len_uppercase() {
        assert_eq!(lowercase_len(&String::from("ГРУППА КРОВИ")), 0);
        assert_eq!(lowercase_len(&String::from("VENI, VIDI, VICI")), 0);
        assert_eq!(lowercase_len(&String::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ")), 0);
    }

    #[test]
    fn test_lowercase_len_not_applyable() {
        assert_eq!(lowercase_len(&String::from("👩‍👩‍👧‍👧")), 0);
        assert_eq!(lowercase_len(&String::from("👩‍👩‍👧")), 0);
        assert_eq!(lowercase_len(&String::from("ओंकार")), 0);
        assert_eq!(lowercase_len(&String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕")), 0);
    }

    #[test]
    fn test_uppercase_len_lowercase() {
        assert_eq!(uppercase_len(&String::from("группа крови")), 0);
        assert_eq!(uppercase_len(&String::from("veni, vidi, vici")), 0);
        assert_eq!(uppercase_len(&String::from("ὅσον ζῇς, φαίνου")), 0);
    }

    #[test]
    fn test_uppercase_len_uppercase() {
        assert_eq!(uppercase_len(&String::from("ГРУППА КРОВИ")), 11);
        assert_eq!(uppercase_len(&String::from("VENI, VIDI, VICI")), 12);
        assert_eq!(uppercase_len(&String::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ")), 14);
    }

    #[test]
    fn test_uppercase_len_not_applyable() {
        assert_eq!(uppercase_len(&String::from("👩‍👩‍👧‍👧")), 0);
        assert_eq!(uppercase_len(&String::from("👩‍👩‍👧")), 0);
        assert_eq!(uppercase_len(&String::from("ओंकार")), 0);
        assert_eq!(uppercase_len(&String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕")), 0);
    }

    #[test]
    fn test_numbers_len() {
        assert_eq!(numbers_len(&String::from("veni, vidi, vici")), 0);
        assert_eq!(numbers_len(&String::from("ὅσον ζῇς, φαίνου")), 0);
        assert_eq!(numbers_len(&String::from("группа крови")), 0);
        assert_eq!(numbers_len(&String::from("ओंकार")), 0);
        assert_eq!(numbers_len(&String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕")), 0);
        assert_eq!(numbers_len(&String::from("👩‍👩‍👧‍👧")), 0);
        assert_eq!(numbers_len(&String::from("0123456789")), 10);
    }

    #[test]
    fn test_symbols_len() {
        assert_eq!(symbols_len(&String::from("veni, vidi, vici")), 2);
        assert_eq!(symbols_len(&String::from("ὅσον ζῇς, φαίνου")), 1);
        assert_eq!(symbols_len(&String::from("группа крови")), 0);
        assert_eq!(symbols_len(&String::from("ओंकार")), 0);
        assert_eq!(symbols_len(&String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕")), 0);
        assert_eq!(symbols_len(&String::from("👩‍👩‍👧‍👧")), 0);
        assert_eq!(symbols_len(&String::from("!\"#$%&'()*+,-./")), 15);
        assert_eq!(symbols_len(&String::from(":;<=>?@")), 7);
        assert_eq!(symbols_len(&String::from("[\\]^_`")), 6);
        assert_eq!(symbols_len(&String::from("{|}~")), 4);
    }
}

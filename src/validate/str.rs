use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{compare, OperandValue},
    validation::str::StrValidation,
    value::Value,
};
use unicode_segmentation::UnicodeSegmentation;

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
                let len = str_value.len();
                if let Some(Err(())) = compare(bytes_len_operation, &OperandValue::USize(len)) {
                    base.push(ValidationErr::BytesLen(bytes_len_operation.clone()))
                }
            }
            if let Some(chars_len_operation) = &validation.chars_len {
                let len = str_value.chars().count();
                if let Some(Err(())) = compare(chars_len_operation, &OperandValue::USize(len)) {
                    base.push(ValidationErr::CharsLen(chars_len_operation.clone()))
                }
            }
            if let Some(graphemes_len_operation) = &validation.graphemes_len {
                let len = str_value.graphemes(true).collect::<Vec<&str>>().len();
                if let Some(Err(())) = compare(graphemes_len_operation, &OperandValue::USize(len)) {
                    base.push(ValidationErr::GraphemesLen(graphemes_len_operation.clone()))
                }
            }
            if let Some(lowercase_len_operation) = &validation.lowercase_len {
                let len = str_value.chars().filter(|c| c.is_alphabetic() && c.is_lowercase()).count();
                if let Some(Err(())) = compare(lowercase_len_operation, &OperandValue::USize(len)) {
                    base.push(ValidationErr::LowercaseLen(lowercase_len_operation.clone()))
                }
            }
            if let Some(uppercase_len_operation) = &validation.uppercase_len {
                let len = str_value.chars().filter(|c| c.is_alphabetic() && c.is_uppercase()).count();
                if let Some(Err(())) = compare(uppercase_len_operation, &OperandValue::USize(len)) {
                    base.push(ValidationErr::UppercaseLen(uppercase_len_operation.clone()))
                }
            }
            if let Some(numbers_len_operation) = &validation.numbers_len {
                let len = str_value.chars().filter(|c| c.is_ascii_digit()).count();
                if let Some(Err(())) = compare(numbers_len_operation, &OperandValue::USize(len)) {
                    base.push(ValidationErr::NumbersLen(numbers_len_operation.clone()))
                }
            }
            if let Some(symbols_len_operation) = &validation.symbols_len {
                let len = str_value.chars().filter(|c| c.is_ascii_punctuation()).count();
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
        }
        _ => {
            base.push(ValidationErr::Str);
            if let Some(operation) = &validation.operation {
                base.push(ValidationErr::Operation(operation.clone()));
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
        value::{stub::num_u_stub, Value},
    };

    use super::{validate_str, StrValidation};

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
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum")), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("Memento mori")), Err(SchemaErr::validation([ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Str(String::from("Cogito ergo sum")))))])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Str(String::from("Cogito ergo sum")))))])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::Str(String::from("Cogito ergo sum")))))])));
    }

    #[test]
    fn test_validate_str_ne() {
        let v = StrValidation::default().ne(String::from("Cogito ergo sum"));
        assert_eq!(validate_str(&v, &Value::Str(String::from("Memento mori"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("Cogito ergo sum"))), Err(SchemaErr::validation([ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Str(String::from("Cogito ergo sum")))))])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Str(String::from("Cogito ergo sum")))))])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::Str(String::from("Cogito ergo sum")))))])));
    }

    #[test]
    fn test_validate_bytes_len_latin() {
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(15), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(15))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(16), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(17), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(17))))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(15), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(16), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ne(Operand::Value(OperandValue::USize(16))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(17), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(14), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(15), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(16), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(16))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(17), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(17))))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(15), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(16), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(17), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(17))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(18), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(18))))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(15), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(15))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(16), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(16))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(17), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(18), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(14), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(14))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(15), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(15))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(16), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(17), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(13, 15), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(13)), Operand::Value(OperandValue::USize(15))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(14, 16), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(15, 17), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(16, 18), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(17, 19), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(17)), Operand::Value(OperandValue::USize(19))))])));
    }

    #[test]
    fn test_validate_bytes_len_greek() {
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(30), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(30))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(31), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(32), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(32))))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(30), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(31), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ne(Operand::Value(OperandValue::USize(31))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(32), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(29), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(30), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(31), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(31))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(32), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(32))))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(30), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(31), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(32), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(32))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(33), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(33))))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(30), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(30))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(31), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(31))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(32), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(33), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(29), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(29))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(30), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(30))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(31), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(32), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(28, 30), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(28)), Operand::Value(OperandValue::USize(30))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(29, 31), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(30, 32), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(31, 33), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(32, 34), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(32)), Operand::Value(OperandValue::USize(34))))])));
    }

    #[test]
    fn test_validate_bytes_len_cyrillic() {
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(22), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(22))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(23), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(24), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(24))))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(22), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(23), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ne(Operand::Value(OperandValue::USize(23))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(24), &Value::Str(String::from("группа крови"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(21), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(22), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(23), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(23))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(24), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(24))))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(22), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(23), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(24), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(24))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(25), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(25))))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(22), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(22))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(23), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(23))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(24), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(25), &Value::Str(String::from("группа крови"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(21), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(21))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(22), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(22))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(23), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(24), &Value::Str(String::from("группа крови"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(20, 22), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(20)), Operand::Value(OperandValue::USize(22))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(21, 23), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(22, 24), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(23, 25), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(24, 26), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(24)), Operand::Value(OperandValue::USize(26))))])));
    }

    #[test]
    fn test_validate_bytes_len_cuneiform() {
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(28), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(28))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(29), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(30), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(30))))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(28), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(29), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ne(Operand::Value(OperandValue::USize(29))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(30), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(27), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(28), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(29), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(29))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(30), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(30))))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(28), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(29), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(30), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(30))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(31), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(31))))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(28), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(28))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(29), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(29))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(30), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(31), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(27), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(27))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(28), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(28))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(29), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(30), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(26, 28), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(26)), Operand::Value(OperandValue::USize(28))))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(27, 29), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(28, 30), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(29, 31), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(30, 32), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(30)), Operand::Value(OperandValue::USize(32))))])));
    }
}

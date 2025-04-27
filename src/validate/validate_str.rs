use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::StrValidation,
    value::Value,
};

use crate::utils::string::{bytes_len, chars_len, graphemes_len, lowercase_len, normalize_nfc, numbers_len, symbols_len, uppercase_len};

pub fn validate_str(validation: &StrValidation, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            let str_value = normalize_nfc(str_value);
            if let Some(operation) = &validation.operation {
                if let Some(Err(())) = compare(operation, &OperandValue::Str(str_value.clone()), root) {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
            }
            if let Some(bytes_len_operation) = &validation.bytes_len {
                let len = bytes_len(&str_value);
                if let Some(Err(())) = compare(bytes_len_operation, &OperandValue::USize(len), root) {
                    base.push(ValidationErr::BytesLen(bytes_len_operation.clone()))
                }
            }
            if let Some(chars_len_operation) = &validation.chars_len {
                let len = chars_len(&str_value);
                if let Some(Err(())) = compare(chars_len_operation, &OperandValue::USize(len), root) {
                    base.push(ValidationErr::CharsLen(chars_len_operation.clone()))
                }
            }
            if let Some(graphemes_len_operation) = &validation.graphemes_len {
                let len = graphemes_len(&str_value);
                if let Some(Err(())) = compare(graphemes_len_operation, &OperandValue::USize(len), root) {
                    base.push(ValidationErr::GraphemesLen(graphemes_len_operation.clone()))
                }
            }
            if let Some(lowercase_len_operation) = &validation.lowercase_len {
                let len = lowercase_len(&str_value);
                if let Some(Err(())) = compare(lowercase_len_operation, &OperandValue::USize(len), root) {
                    base.push(ValidationErr::LowercaseLen(lowercase_len_operation.clone()))
                }
            }
            if let Some(uppercase_len_operation) = &validation.uppercase_len {
                let len = uppercase_len(&str_value);
                if let Some(Err(())) = compare(uppercase_len_operation, &OperandValue::USize(len), root) {
                    base.push(ValidationErr::UppercaseLen(uppercase_len_operation.clone()))
                }
            }
            if let Some(numbers_len_operation) = &validation.numbers_len {
                let len = numbers_len(&str_value);
                if let Some(Err(())) = compare(numbers_len_operation, &OperandValue::USize(len), root) {
                    base.push(ValidationErr::NumbersLen(numbers_len_operation.clone()))
                }
            }
            if let Some(symbols_len_operation) = &validation.symbols_len {
                let len = symbols_len(&str_value);
                if let Some(Err(())) = compare(symbols_len_operation, &OperandValue::USize(len), root) {
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
    if !base.is_empty() { Err(SchemaErr::Validation(base)) } else { Ok(()) }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, sync::LazyLock};

    use araucaria::{
        error::{SchemaErr, ValidationErr},
        operation::{Operand, OperandValue, Operation},
        validation::StrValidation,
        value::{Value, stub::u64_stub},
    };

    use super::validate_str;

    static ROOT: LazyLock<Value> = LazyLock::new(|| {
        Value::Obj(BTreeMap::from([
            (
                "str".into(),
                Value::Arr(vec![
                    Value::Obj(BTreeMap::from([("value".into(), Value::from("d"))])),
                    Value::Obj(BTreeMap::from([("value".into(), Value::from("f"))])),
                    Value::Obj(BTreeMap::from([("value".into(), Value::from("l"))])),
                    Value::Obj(BTreeMap::from([("value".into(), Value::from("j"))])),
                ]),
            ),
            (
                "usize".into(),
                Value::Obj(BTreeMap::from([(
                    "values".into(),
                    Value::Obj(BTreeMap::from([(
                        "nums".into(),
                        Value::Arr(vec![
                            Value::USize(0),
                            Value::USize(1),
                            Value::USize(2),
                            Value::USize(3),
                            Value::USize(4),
                            Value::USize(5),
                            Value::USize(6),
                            Value::USize(7),
                            Value::USize(8),
                            Value::USize(9),
                            Value::USize(10),
                            Value::USize(11),
                            Value::USize(12),
                            Value::USize(13),
                            Value::USize(14),
                            Value::USize(15),
                            Value::USize(16),
                            Value::USize(17),
                            Value::USize(18),
                            Value::USize(19),
                            Value::USize(20),
                            Value::USize(21),
                            Value::USize(22),
                            Value::USize(23),
                            Value::USize(24),
                            Value::USize(25),
                            Value::USize(26),
                            Value::USize(27),
                            Value::USize(28),
                            Value::USize(29),
                        ]),
                    )])),
                )])),
            ),
        ]))
    });

    #[test]
    fn validate_str_default() {
        let v = StrValidation::default();
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str])));
    }

    #[test]
    fn validate_str_optional() {
        let v = StrValidation::default().optional();
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Str])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str])));
    }

    #[test]
    fn validate_str_eq_value() {
        let v = StrValidation::default().eq("Cogito ergo sum".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("Cogito ergo sum"))));
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("Memento mori"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_str_ne_value() {
        let v = StrValidation::default().ne("Cogito ergo sum".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::Value(OperandValue::from("Cogito ergo sum"))));
        assert_eq!(validate_str(&v, &Value::from("Memento mori"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_str_gt_value() {
        let v = StrValidation::default().gt("j".into());
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::Value(OperandValue::from("j"))));
        assert_eq!(validate_str(&v, &Value::from("a"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("j"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("z"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_str_ge_value() {
        let v = StrValidation::default().ge("j".into());
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::Value(OperandValue::from("j"))));
        assert_eq!(validate_str(&v, &Value::from("a"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("j"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("z"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_str_lt_value() {
        let v = StrValidation::default().lt("j".into());
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::Value(OperandValue::from("j"))));
        assert_eq!(validate_str(&v, &Value::from("a"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("j"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_str_le_value() {
        let v = StrValidation::default().le("j".into());
        let op_err = ValidationErr::Operation(Operation::Le(Operand::Value(OperandValue::from("j"))));
        assert_eq!(validate_str(&v, &Value::from("a"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("j"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_str_btwn_value() {
        let v = StrValidation::default().btwn("f".into(), "l".into());
        let op_err = ValidationErr::Operation(Operation::Btwn(Operand::Value(OperandValue::from("f")), Operand::Value(OperandValue::from("l"))));
        assert_eq!(validate_str(&v, &Value::from("e"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("f"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("i"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("l"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("m"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_eq_value() {
        let v = StrValidation::default().bytes_len_eq(16);
        let op_err = ValidationErr::BytesLen(Operation::Eq(Operand::Value(OperandValue::USize(16))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_ne_value() {
        let v = StrValidation::default().bytes_len_ne(16);
        let op_err = ValidationErr::BytesLen(Operation::Ne(Operand::Value(OperandValue::USize(16))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_gt_value() {
        let v = StrValidation::default().bytes_len_gt(23);
        let op_err = ValidationErr::BytesLen(Operation::Gt(Operand::Value(OperandValue::USize(23))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_ge_value() {
        let v = StrValidation::default().bytes_len_ge(23);
        let op_err = ValidationErr::BytesLen(Operation::Ge(Operand::Value(OperandValue::USize(23))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_lt_value() {
        let v = StrValidation::default().bytes_len_lt(23);
        let op_err = ValidationErr::BytesLen(Operation::Lt(Operand::Value(OperandValue::USize(23))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_le_value() {
        let v = StrValidation::default().bytes_len_le(23);
        let op_err = ValidationErr::BytesLen(Operation::Le(Operand::Value(OperandValue::USize(23))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_btwn_value() {
        let v = StrValidation::default().bytes_len_btwn(23, 29);
        let op_err = ValidationErr::BytesLen(Operation::Btwn(Operand::Value(OperandValue::USize(23)), Operand::Value(OperandValue::USize(29))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_eq_value() {
        let v = StrValidation::default().chars_len_eq(16);
        let op_err = ValidationErr::CharsLen(Operation::Eq(Operand::Value(OperandValue::USize(16))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_ne_value() {
        let v = StrValidation::default().chars_len_ne(16);
        let op_err = ValidationErr::CharsLen(Operation::Ne(Operand::Value(OperandValue::USize(16))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_gt_value() {
        let v = StrValidation::default().chars_len_gt(12);
        let op_err = ValidationErr::CharsLen(Operation::Gt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_ge_value() {
        let v = StrValidation::default().chars_len_ge(12);
        let op_err = ValidationErr::CharsLen(Operation::Ge(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_lt_value() {
        let v = StrValidation::default().chars_len_lt(12);
        let op_err = ValidationErr::CharsLen(Operation::Lt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_le_value() {
        let v = StrValidation::default().chars_len_le(12);
        let op_err = ValidationErr::CharsLen(Operation::Le(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_btwn_value() {
        let v = StrValidation::default().chars_len_btwn(8, 12);
        let op_err = ValidationErr::CharsLen(Operation::Btwn(Operand::Value(OperandValue::USize(8)), Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("ओंकार"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_eq_value() {
        let v = StrValidation::default().graphemes_len_eq(16);
        let op_err = ValidationErr::GraphemesLen(Operation::Eq(Operand::Value(OperandValue::USize(16))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_ne_value() {
        let v = StrValidation::default().graphemes_len_ne(16);
        let op_err = ValidationErr::GraphemesLen(Operation::Ne(Operand::Value(OperandValue::USize(16))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_gt_value() {
        let v = StrValidation::default().graphemes_len_gt(12);
        let op_err = ValidationErr::GraphemesLen(Operation::Gt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_ge_value() {
        let v = StrValidation::default().graphemes_len_ge(12);
        let op_err = ValidationErr::GraphemesLen(Operation::Ge(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_lt_value() {
        let v = StrValidation::default().graphemes_len_lt(12);
        let op_err = ValidationErr::GraphemesLen(Operation::Lt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_le_value() {
        let v = StrValidation::default().graphemes_len_le(12);
        let op_err = ValidationErr::GraphemesLen(Operation::Le(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_btwn_value() {
        let v = StrValidation::default().graphemes_len_btwn(8, 12);
        let op_err = ValidationErr::GraphemesLen(Operation::Btwn(Operand::Value(OperandValue::USize(8)), Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("ओंकार"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_eq_value() {
        let v = StrValidation::default().lowercase_len_eq(12);
        let op_err = ValidationErr::LowercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_ne_value() {
        let v = StrValidation::default().lowercase_len_ne(12);
        let op_err = ValidationErr::LowercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_gt_value() {
        let v = StrValidation::default().lowercase_len_gt(12);
        let op_err = ValidationErr::LowercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_ge_value() {
        let v = StrValidation::default().lowercase_len_ge(12);
        let op_err = ValidationErr::LowercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_lt_value() {
        let v = StrValidation::default().lowercase_len_lt(12);
        let op_err = ValidationErr::LowercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_le_value() {
        let v = StrValidation::default().lowercase_len_le(12);
        let op_err = ValidationErr::LowercaseLen(Operation::Le(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_btwn_value() {
        let v = StrValidation::default().lowercase_len_btwn(11, 12);
        let op_err = ValidationErr::LowercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(11)), Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_eq_value() {
        let v = StrValidation::default().uppercase_len_eq(12);
        let op_err = ValidationErr::UppercaseLen(Operation::Eq(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ГРУППА КРОВИ"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_ne_value() {
        let v = StrValidation::default().uppercase_len_ne(12);
        let op_err = ValidationErr::UppercaseLen(Operation::Ne(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ГРУППА КРОВИ"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_gt_value() {
        let v = StrValidation::default().uppercase_len_gt(12);
        let op_err = ValidationErr::UppercaseLen(Operation::Gt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("ГРУППА КРОВИ"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_ge_value() {
        let v = StrValidation::default().uppercase_len_ge(12);
        let op_err = ValidationErr::UppercaseLen(Operation::Ge(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("ГРУППА КРОВИ"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_lt_value() {
        let v = StrValidation::default().uppercase_len_lt(12);
        let op_err = ValidationErr::UppercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("ГРУППА КРОВИ"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_le_value() {
        let v = StrValidation::default().uppercase_len_le(12);
        let op_err = ValidationErr::UppercaseLen(Operation::Le(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("ГРУППА КРОВИ"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_btwn_value() {
        let v = StrValidation::default().uppercase_len_btwn(11, 12);
        let op_err = ValidationErr::UppercaseLen(Operation::Btwn(Operand::Value(OperandValue::USize(11)), Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("ГРУППА КРОВИ"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_eq_value() {
        let v = StrValidation::default().numbers_len_eq(2);
        let op_err = ValidationErr::NumbersLen(Operation::Eq(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::from("1"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_ne_value() {
        let v = StrValidation::default().numbers_len_ne(2);
        let op_err = ValidationErr::NumbersLen(Operation::Ne(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::from("1"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_gt_value() {
        let v = StrValidation::default().numbers_len_gt(2);
        let op_err = ValidationErr::NumbersLen(Operation::Gt(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::from("1"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_ge_value() {
        let v = StrValidation::default().numbers_len_ge(2);
        let op_err = ValidationErr::NumbersLen(Operation::Ge(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::from("1"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_lt_value() {
        let v = StrValidation::default().numbers_len_lt(2);
        let op_err = ValidationErr::NumbersLen(Operation::Lt(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::from("1"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_le_value() {
        let v = StrValidation::default().numbers_len_le(2);
        let op_err = ValidationErr::NumbersLen(Operation::Le(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::from("1"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_btwn_value() {
        let v = StrValidation::default().numbers_len_btwn(2, 3);
        let op_err = ValidationErr::NumbersLen(Operation::Btwn(Operand::Value(OperandValue::USize(2)), Operand::Value(OperandValue::USize(3))));
        assert_eq!(validate_str(&v, &Value::from("1"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("4444"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_eq_value() {
        let v = StrValidation::default().symbols_len_eq(2);
        let op_err = ValidationErr::SymbolsLen(Operation::Eq(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_ne_value() {
        let v = StrValidation::default().symbols_len_ne(2);
        let op_err = ValidationErr::SymbolsLen(Operation::Ne(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_gt_value() {
        let v = StrValidation::default().symbols_len_gt(2);
        let op_err = ValidationErr::SymbolsLen(Operation::Gt(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_ge_value() {
        let v = StrValidation::default().symbols_len_ge(2);
        let op_err = ValidationErr::SymbolsLen(Operation::Ge(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_lt_value() {
        let v = StrValidation::default().symbols_len_lt(2);
        let op_err = ValidationErr::SymbolsLen(Operation::Lt(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_le_value() {
        let v = StrValidation::default().symbols_len_le(2);
        let op_err = ValidationErr::SymbolsLen(Operation::Le(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_btwn_value() {
        let v = StrValidation::default().symbols_len_btwn(2, 3);
        let op_err = ValidationErr::SymbolsLen(Operation::Btwn(Operand::Value(OperandValue::USize(2)), Operand::Value(OperandValue::USize(3))));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("&*()"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_str_eq_field() {
        let v = StrValidation::default().eq_field("str.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::FieldPath("str.3.value".into())));
        assert_eq!(validate_str(&v, &Value::from("j"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("Memento mori"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_str_ne_field() {
        let v = StrValidation::default().ne_field("str.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("str.3.value".into())));
        assert_eq!(validate_str(&v, &Value::from("Memento mori"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("j"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_str_gt_field() {
        let v = StrValidation::default().gt_field("str.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Gt(Operand::FieldPath("str.3.value".into())));
        assert_eq!(validate_str(&v, &Value::from("a"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("j"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("z"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_str_ge_field() {
        let v = StrValidation::default().ge_field("str.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Ge(Operand::FieldPath("str.3.value".into())));
        assert_eq!(validate_str(&v, &Value::from("a"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("j"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("z"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_str_lt_field() {
        let v = StrValidation::default().lt_field("str.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Lt(Operand::FieldPath("str.3.value".into())));
        assert_eq!(validate_str(&v, &Value::from("a"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("j"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_str_le_field() {
        let v = StrValidation::default().le_field("str.3.value".into());
        let op_err = ValidationErr::Operation(Operation::Le(Operand::FieldPath("str.3.value".into())));
        assert_eq!(validate_str(&v, &Value::from("a"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("j"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("z"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_str_btwn_field() {
        let v = StrValidation::default().btwn_field("str.1.value".into(), "str.2.value".into());
        let op_err = ValidationErr::Operation(Operation::Btwn(Operand::FieldPath("str.1.value".into()), Operand::FieldPath("str.2.value".into())));
        assert_eq!(validate_str(&v, &Value::from("e"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("f"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("i"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("l"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("m"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_eq_field() {
        let v = StrValidation::default().bytes_len_eq_field("usize.values.nums.16".into());
        let op_err = ValidationErr::BytesLen(Operation::Eq(Operand::FieldPath("usize.values.nums.16".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_ne_field() {
        let v = StrValidation::default().bytes_len_ne_field("usize.values.nums.16".into());
        let op_err = ValidationErr::BytesLen(Operation::Ne(Operand::FieldPath("usize.values.nums.16".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_gt_field() {
        let v = StrValidation::default().bytes_len_gt_field("usize.values.nums.23".into());
        let op_err = ValidationErr::BytesLen(Operation::Gt(Operand::FieldPath("usize.values.nums.23".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_ge_field() {
        let v = StrValidation::default().bytes_len_ge_field("usize.values.nums.23".into());
        let op_err = ValidationErr::BytesLen(Operation::Ge(Operand::FieldPath("usize.values.nums.23".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_lt_field() {
        let v = StrValidation::default().bytes_len_lt_field("usize.values.nums.23".into());
        let op_err = ValidationErr::BytesLen(Operation::Lt(Operand::FieldPath("usize.values.nums.23".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_le_field() {
        let v = StrValidation::default().bytes_len_le_field("usize.values.nums.23".into());
        let op_err = ValidationErr::BytesLen(Operation::Le(Operand::FieldPath("usize.values.nums.23".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_btwn_field() {
        let v = StrValidation::default().bytes_len_btwn_field("usize.values.nums.23".into(), "usize.values.nums.29".into());
        let op_err = ValidationErr::BytesLen(Operation::Btwn(
            Operand::FieldPath("usize.values.nums.23".into()),
            Operand::FieldPath("usize.values.nums.29".into()),
        ));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_eq_field() {
        let v = StrValidation::default().chars_len_eq_field("usize.values.nums.16".into());
        let op_err = ValidationErr::CharsLen(Operation::Eq(Operand::FieldPath("usize.values.nums.16".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_ne_field() {
        let v = StrValidation::default().chars_len_ne_field("usize.values.nums.16".into());
        let op_err = ValidationErr::CharsLen(Operation::Ne(Operand::FieldPath("usize.values.nums.16".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_gt_field() {
        let v = StrValidation::default().chars_len_gt_field("usize.values.nums.12".into());
        let op_err = ValidationErr::CharsLen(Operation::Gt(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_ge_field() {
        let v = StrValidation::default().chars_len_ge_field("usize.values.nums.12".into());
        let op_err = ValidationErr::CharsLen(Operation::Ge(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_lt_field() {
        let v = StrValidation::default().chars_len_lt_field("usize.values.nums.12".into());
        let op_err = ValidationErr::CharsLen(Operation::Lt(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_le_field() {
        let v = StrValidation::default().chars_len_le_field("usize.values.nums.12".into());
        let op_err = ValidationErr::CharsLen(Operation::Le(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_btwn_field() {
        let v = StrValidation::default().chars_len_btwn_field("usize.values.nums.8".into(), "usize.values.nums.12".into());
        let op_err = ValidationErr::CharsLen(Operation::Btwn(
            Operand::FieldPath("usize.values.nums.8".into()),
            Operand::FieldPath("usize.values.nums.12".into()),
        ));
        assert_eq!(validate_str(&v, &Value::from("ओंकार"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_eq_field() {
        let v = StrValidation::default().graphemes_len_eq_field("usize.values.nums.16".into());
        let op_err = ValidationErr::GraphemesLen(Operation::Eq(Operand::FieldPath("usize.values.nums.16".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_ne_field() {
        let v = StrValidation::default().graphemes_len_ne_field("usize.values.nums.16".into());
        let op_err = ValidationErr::GraphemesLen(Operation::Ne(Operand::FieldPath("usize.values.nums.16".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_gt_field() {
        let v = StrValidation::default().graphemes_len_gt_field("usize.values.nums.12".into());
        let op_err = ValidationErr::GraphemesLen(Operation::Gt(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_ge_field() {
        let v = StrValidation::default().graphemes_len_ge_field("usize.values.nums.12".into());
        let op_err = ValidationErr::GraphemesLen(Operation::Ge(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_lt_field() {
        let v = StrValidation::default().graphemes_len_lt_field("usize.values.nums.12".into());
        let op_err = ValidationErr::GraphemesLen(Operation::Lt(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_le_field() {
        let v = StrValidation::default().graphemes_len_le_field("usize.values.nums.12".into());
        let op_err = ValidationErr::GraphemesLen(Operation::Le(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_btwn_field() {
        let v = StrValidation::default().graphemes_len_btwn_field("usize.values.nums.8".into(), "usize.values.nums.12".into());
        let op_err = ValidationErr::GraphemesLen(Operation::Btwn(
            Operand::FieldPath("usize.values.nums.8".into()),
            Operand::FieldPath("usize.values.nums.12".into()),
        ));
        assert_eq!(validate_str(&v, &Value::from("ओंकार"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_eq_field() {
        let v = StrValidation::default().lowercase_len_eq_field("usize.values.nums.12".into());
        let op_err = ValidationErr::LowercaseLen(Operation::Eq(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_ne_field() {
        let v = StrValidation::default().lowercase_len_ne_field("usize.values.nums.12".into());
        let op_err = ValidationErr::LowercaseLen(Operation::Ne(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_gt_field() {
        let v = StrValidation::default().lowercase_len_gt_field("usize.values.nums.12".into());
        let op_err = ValidationErr::LowercaseLen(Operation::Gt(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_ge_field() {
        let v = StrValidation::default().lowercase_len_ge_field("usize.values.nums.12".into());
        let op_err = ValidationErr::LowercaseLen(Operation::Ge(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_lt_field() {
        let v = StrValidation::default().lowercase_len_lt_field("usize.values.nums.12".into());
        let op_err = ValidationErr::LowercaseLen(Operation::Lt(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_le_field() {
        let v = StrValidation::default().lowercase_len_le_field("usize.values.nums.12".into());
        let op_err = ValidationErr::LowercaseLen(Operation::Le(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_btwn_field() {
        let v = StrValidation::default().lowercase_len_btwn_field("usize.values.nums.11".into(), "usize.values.nums.12".into());
        let op_err = ValidationErr::LowercaseLen(Operation::Btwn(
            Operand::FieldPath("usize.values.nums.11".into()),
            Operand::FieldPath("usize.values.nums.12".into()),
        ));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_eq_field() {
        let v = StrValidation::default().uppercase_len_eq_field("usize.values.nums.12".into());
        let op_err = ValidationErr::UppercaseLen(Operation::Eq(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ГРУППА КРОВИ"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_ne_field() {
        let v = StrValidation::default().uppercase_len_ne_field("usize.values.nums.12".into());
        let op_err = ValidationErr::UppercaseLen(Operation::Ne(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ГРУППА КРОВИ"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_gt_field() {
        let v = StrValidation::default().uppercase_len_gt_field("usize.values.nums.12".into());
        let op_err = ValidationErr::UppercaseLen(Operation::Gt(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("ГРУППА КРОВИ"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_ge_field() {
        let v = StrValidation::default().uppercase_len_ge_field("usize.values.nums.12".into());
        let op_err = ValidationErr::UppercaseLen(Operation::Ge(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("ГРУППА КРОВИ"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_lt_field() {
        let v = StrValidation::default().uppercase_len_lt_field("usize.values.nums.12".into());
        let op_err = ValidationErr::UppercaseLen(Operation::Lt(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("ГРУППА КРОВИ"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_le_field() {
        let v = StrValidation::default().uppercase_len_le_field("usize.values.nums.12".into());
        let op_err = ValidationErr::UppercaseLen(Operation::Le(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("ГРУППА КРОВИ"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_btwn_field() {
        let v = StrValidation::default().uppercase_len_btwn_field("usize.values.nums.11".into(), "usize.values.nums.12".into());
        let op_err = ValidationErr::UppercaseLen(Operation::Btwn(
            Operand::FieldPath("usize.values.nums.11".into()),
            Operand::FieldPath("usize.values.nums.12".into()),
        ));
        assert_eq!(validate_str(&v, &Value::from("ГРУППА КРОВИ"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_eq_field() {
        let v = StrValidation::default().numbers_len_eq_field("usize.values.nums.2".into());
        let op_err = ValidationErr::NumbersLen(Operation::Eq(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("1"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_ne_field() {
        let v = StrValidation::default().numbers_len_ne_field("usize.values.nums.2".into());
        let op_err = ValidationErr::NumbersLen(Operation::Ne(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("1"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_gt_field() {
        let v = StrValidation::default().numbers_len_gt_field("usize.values.nums.2".into());
        let op_err = ValidationErr::NumbersLen(Operation::Gt(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("1"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_ge_field() {
        let v = StrValidation::default().numbers_len_ge_field("usize.values.nums.2".into());
        let op_err = ValidationErr::NumbersLen(Operation::Ge(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("1"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_lt_field() {
        let v = StrValidation::default().numbers_len_lt_field("usize.values.nums.2".into());
        let op_err = ValidationErr::NumbersLen(Operation::Lt(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("1"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_le_field() {
        let v = StrValidation::default().numbers_len_le_field("usize.values.nums.2".into());
        let op_err = ValidationErr::NumbersLen(Operation::Le(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("1"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_btwn_field() {
        let v = StrValidation::default().numbers_len_btwn_field("usize.values.nums.2".into(), "usize.values.nums.3".into());
        let op_err = ValidationErr::NumbersLen(Operation::Btwn(
            Operand::FieldPath("usize.values.nums.2".into()),
            Operand::FieldPath("usize.values.nums.3".into()),
        ));
        assert_eq!(validate_str(&v, &Value::from("1"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("4444"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_eq_field() {
        let v = StrValidation::default().symbols_len_eq_field("usize.values.nums.2".into());
        let op_err = ValidationErr::SymbolsLen(Operation::Eq(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_ne_field() {
        let v = StrValidation::default().symbols_len_ne_field("usize.values.nums.2".into());
        let op_err = ValidationErr::SymbolsLen(Operation::Ne(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_gt_field() {
        let v = StrValidation::default().symbols_len_gt_field("usize.values.nums.2".into());
        let op_err = ValidationErr::SymbolsLen(Operation::Gt(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_ge_field() {
        let v = StrValidation::default().symbols_len_ge_field("usize.values.nums.2".into());
        let op_err = ValidationErr::SymbolsLen(Operation::Ge(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_lt_field() {
        let v = StrValidation::default().symbols_len_lt_field("usize.values.nums.2".into());
        let op_err = ValidationErr::SymbolsLen(Operation::Lt(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_le_field() {
        let v = StrValidation::default().symbols_len_le_field("usize.values.nums.2".into());
        let op_err = ValidationErr::SymbolsLen(Operation::Le(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_btwn_field() {
        let v = StrValidation::default().symbols_len_btwn_field("usize.values.nums.2".into(), "usize.values.nums.3".into());
        let op_err = ValidationErr::SymbolsLen(Operation::Btwn(
            Operand::FieldPath("usize.values.nums.2".into()),
            Operand::FieldPath("usize.values.nums.3".into()),
        ));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("&*()"), &ROOT), Err(SchemaErr::validation([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::validation([ValidationErr::Str, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_eq_normalized() {
        let v = StrValidation::default().chars_len_eq(5);

        let a_upper_composed = "ÀÁÂÃÄ";
        let e_upper_composed = "ÈÉÊẼË";
        let i_upper_composed = "ÌÍÎĨÏ";
        let o_upper_composed = "ÒÓÔÕÖ";
        let u_upper_composed = "ÙÚÛŨÜ";

        let a_lower_composed = "àáâãä";
        let e_lower_composed = "èéêẽë";
        let i_lower_composed = "ìíîĩï";
        let o_lower_composed = "òóôõö";
        let u_lower_composed = "ùúûũü";

        let a_upper_decomposed = "A\u{300}A\u{301}A\u{302}A\u{303}A\u{308}";
        let e_upper_decomposed = "E\u{300}E\u{301}E\u{302}E\u{303}E\u{308}";
        let i_upper_decomposed = "I\u{300}I\u{301}I\u{302}I\u{303}I\u{308}";
        let o_upper_decomposed = "O\u{300}O\u{301}O\u{302}O\u{303}O\u{308}";
        let u_upper_decomposed = "U\u{300}U\u{301}U\u{302}U\u{303}U\u{308}";

        let a_lower_decomposed = "a\u{300}a\u{301}a\u{302}a\u{303}a\u{308}";
        let e_lower_decomposed = "e\u{300}e\u{301}e\u{302}e\u{303}e\u{308}";
        let i_lower_decomposed = "i\u{300}i\u{301}i\u{302}i\u{303}i\u{308}";
        let o_lower_decomposed = "o\u{300}o\u{301}o\u{302}o\u{303}o\u{308}";
        let u_lower_decomposed = "u\u{300}u\u{301}u\u{302}u\u{303}u\u{308}";

        assert_eq!(validate_str(&v, &Value::from(a_upper_composed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(e_upper_composed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(i_upper_composed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(o_upper_composed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(u_upper_composed), &ROOT), Ok(()));

        assert_eq!(validate_str(&v, &Value::from(a_lower_composed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(e_lower_composed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(i_lower_composed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(o_lower_composed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(u_lower_composed), &ROOT), Ok(()));

        assert_eq!(validate_str(&v, &Value::from(a_upper_decomposed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(e_upper_decomposed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(i_upper_decomposed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(o_upper_decomposed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(u_upper_decomposed), &ROOT), Ok(()));

        assert_eq!(validate_str(&v, &Value::from(a_lower_decomposed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(e_lower_decomposed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(i_lower_decomposed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(o_lower_decomposed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(u_lower_decomposed), &ROOT), Ok(()));
    }
}

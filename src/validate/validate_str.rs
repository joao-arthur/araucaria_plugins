use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    validation::StrValidation,
    value::Value,
};

use crate::utils::string::{bytes_len, chars_len, graphemes_len, lowercase_len, normalize_nfc, numbers_len, symbols_len, uppercase_len};

pub fn validate_str(validation: &StrValidation, value: &Value, root: &Value, enforce_optional: bool) -> Result<(), SchemaErr> {
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
            if enforce_optional {
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
            } else {
                if validation.required {
                    base.push(ValidationErr::Required);
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
    if !base.is_empty() { Err(SchemaErr::Arr(base)) } else { Ok(()) }
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
            ("str".into(), Value::from("j")),
            (
                "usize".into(),
                Value::Obj(BTreeMap::from([(
                    "values".into(),
                    Value::Obj(BTreeMap::from([(
                        "nums".into(),
                        Value::Obj(BTreeMap::from([
                            ("2".into(), Value::USize(2)),
                            ("11".into(), Value::USize(11)),
                            ("12".into(), Value::USize(12)),
                            ("23".into(), Value::USize(23)),
                        ])),
                    )])),
                )])),
            ),
        ]))
    });
    const REQUIRED: ValidationErr = ValidationErr::Required;
    const STR: ValidationErr = ValidationErr::Str;

    #[test]
    fn validate_str_default() {
        let v = StrValidation::default();
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR])));
    }

    #[test]
    fn validate_str_optional() {
        let v = StrValidation::default().optional();
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([STR])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR])));
    }

    #[test]
    fn validate_str_operation_value() {
        let v = StrValidation::default().eq("Cogito ergo sum".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("Cogito ergo sum"))));
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("Memento mori"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }

    #[test]
    fn validate_str_operation_field() {
        let v = StrValidation::default().ne_field("str".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("str".into())));
        assert_eq!(validate_str(&v, &Value::from("Memento mori"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("j"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_value() {
        let v = StrValidation::default().bytes_len_ne(16);
        let op_err = ValidationErr::BytesLen(Operation::Ne(Operand::Value(OperandValue::USize(16))));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_field() {
        let v = StrValidation::default().bytes_len_gt_field("usize.values.nums.23".into());
        let op_err = ValidationErr::BytesLen(Operation::Gt(Operand::FieldPath("usize.values.nums.23".into())));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_value() {
        let v = StrValidation::default().chars_len_gt(12);
        let op_err = ValidationErr::CharsLen(Operation::Gt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_field() {
        let v = StrValidation::default().chars_len_ge_field("usize.values.nums.12".into());
        let op_err = ValidationErr::CharsLen(Operation::Ge(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_normalized() {
        let v = StrValidation::default().chars_len_eq(5);
        let a_upper_decomposed = "A\u{300}A\u{301}A\u{302}A\u{303}A\u{308}";
        let e_lower_decomposed = "e\u{300}e\u{301}e\u{302}e\u{303}e\u{308}";
        assert_eq!(validate_str(&v, &Value::from(a_upper_decomposed), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(e_lower_decomposed), &ROOT, false), Ok(()));
    }

    #[test]
    fn validate_graphemes_len_value() {
        let v = StrValidation::default().graphemes_len_ge(12);
        let op_err = ValidationErr::GraphemesLen(Operation::Ge(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_field() {
        let v = StrValidation::default().graphemes_len_lt_field("usize.values.nums.12".into());
        let op_err = ValidationErr::GraphemesLen(Operation::Lt(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_value() {
        let v = StrValidation::default().lowercase_len_lt(12);
        let op_err = ValidationErr::LowercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_field() {
        let v = StrValidation::default().lowercase_len_le_field("usize.values.nums.12".into());
        let op_err = ValidationErr::LowercaseLen(Operation::Le(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_value() {
        let v = StrValidation::default().uppercase_len_le(12);
        let op_err = ValidationErr::UppercaseLen(Operation::Le(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_field() {
        let v = StrValidation::default().uppercase_len_btwn_field("usize.values.nums.11".into(), "usize.values.nums.12".into());
        let op_err = ValidationErr::UppercaseLen(Operation::Btwn(
            Operand::FieldPath("usize.values.nums.11".into()),
            Operand::FieldPath("usize.values.nums.12".into()),
        ));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_value() {
        let v = StrValidation::default().numbers_len_btwn(2, 3);
        let op_err = ValidationErr::NumbersLen(Operation::Btwn(Operand::Value(OperandValue::USize(2)), Operand::Value(OperandValue::USize(3))));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("4444"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_field() {
        let v = StrValidation::default().numbers_len_eq_field("usize.values.nums.2".into());
        let op_err = ValidationErr::NumbersLen(Operation::Eq(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_value() {
        let v = StrValidation::default().symbols_len_eq(2);
        let op_err = ValidationErr::SymbolsLen(Operation::Eq(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_field() {
        let v = StrValidation::default().symbols_len_ne_field("usize.values.nums.2".into());
        let op_err = ValidationErr::SymbolsLen(Operation::Ne(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT, false), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT, false), Err(SchemaErr::arr([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, true), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT, false), Err(SchemaErr::arr([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT, false), Err(SchemaErr::arr([STR, op_err.clone()])));
    }
}

use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{OperandValue, compare},
    schema::StrSchema,
    value::Value,
};

use crate::utils::string::{bytes_len, chars_len, graphemes_len, lowercase_len, normalize_nfc, numbers_len, symbols_len, uppercase_len};

pub fn validate_str(schema: &StrSchema, value: &Value, root: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            let str_value = normalize_nfc(str_value);
            if let Some(operation) = &schema.operation {
                if let Some(Err(())) = compare(operation, &OperandValue::Str(str_value.clone()), root) {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
            }
            if let Some(bytes_len_operation) = &schema.bytes_len {
                let len = bytes_len(&str_value);
                if let Some(Err(())) = compare(bytes_len_operation, &OperandValue::USize(len), root) {
                    base.push(ValidationErr::BytesLen(bytes_len_operation.clone()))
                }
            }
            if let Some(chars_len_operation) = &schema.chars_len {
                let len = chars_len(&str_value);
                if let Some(Err(())) = compare(chars_len_operation, &OperandValue::USize(len), root) {
                    base.push(ValidationErr::CharsLen(chars_len_operation.clone()))
                }
            }
            if let Some(graphemes_len_operation) = &schema.graphemes_len {
                let len = graphemes_len(&str_value);
                if let Some(Err(())) = compare(graphemes_len_operation, &OperandValue::USize(len), root) {
                    base.push(ValidationErr::GraphemesLen(graphemes_len_operation.clone()))
                }
            }
            if let Some(lowercase_len_operation) = &schema.lowercase_len {
                let len = lowercase_len(&str_value);
                if let Some(Err(())) = compare(lowercase_len_operation, &OperandValue::USize(len), root) {
                    base.push(ValidationErr::LowercaseLen(lowercase_len_operation.clone()))
                }
            }
            if let Some(uppercase_len_operation) = &schema.uppercase_len {
                let len = uppercase_len(&str_value);
                if let Some(Err(())) = compare(uppercase_len_operation, &OperandValue::USize(len), root) {
                    base.push(ValidationErr::UppercaseLen(uppercase_len_operation.clone()))
                }
            }
            if let Some(numbers_len_operation) = &schema.numbers_len {
                let len = numbers_len(&str_value);
                if let Some(Err(())) = compare(numbers_len_operation, &OperandValue::USize(len), root) {
                    base.push(ValidationErr::NumbersLen(numbers_len_operation.clone()))
                }
            }
            if let Some(symbols_len_operation) = &schema.symbols_len {
                let len = symbols_len(&str_value);
                if let Some(Err(())) = compare(symbols_len_operation, &OperandValue::USize(len), root) {
                    base.push(ValidationErr::SymbolsLen(symbols_len_operation.clone()))
                }
            }
        }
        Value::None => {
            if schema.required {
                base.push(ValidationErr::Required);
                base.push(ValidationErr::Str);
                if let Some(operation) = &schema.operation {
                    base.push(ValidationErr::Operation(operation.clone()));
                }
                if let Some(bytes_len_operation) = &schema.bytes_len {
                    base.push(ValidationErr::BytesLen(bytes_len_operation.clone()))
                }
                if let Some(chars_len_operation) = &schema.chars_len {
                    base.push(ValidationErr::CharsLen(chars_len_operation.clone()))
                }
                if let Some(graphemes_len_operation) = &schema.graphemes_len {
                    base.push(ValidationErr::GraphemesLen(graphemes_len_operation.clone()))
                }
                if let Some(lowercase_len_operation) = &schema.lowercase_len {
                    base.push(ValidationErr::LowercaseLen(lowercase_len_operation.clone()))
                }
                if let Some(uppercase_len_operation) = &schema.uppercase_len {
                    base.push(ValidationErr::UppercaseLen(uppercase_len_operation.clone()))
                }
                if let Some(numbers_len_operation) = &schema.numbers_len {
                    base.push(ValidationErr::NumbersLen(numbers_len_operation.clone()))
                }
                if let Some(symbols_len_operation) = &schema.symbols_len {
                    base.push(ValidationErr::SymbolsLen(symbols_len_operation.clone()))
                }
            }
        }
        _ => {
            base.push(ValidationErr::Str);
            if let Some(operation) = &schema.operation {
                base.push(ValidationErr::Operation(operation.clone()));
            }
            if let Some(bytes_len_operation) = &schema.bytes_len {
                base.push(ValidationErr::BytesLen(bytes_len_operation.clone()))
            }
            if let Some(chars_len_operation) = &schema.chars_len {
                base.push(ValidationErr::CharsLen(chars_len_operation.clone()))
            }
            if let Some(graphemes_len_operation) = &schema.graphemes_len {
                base.push(ValidationErr::GraphemesLen(graphemes_len_operation.clone()))
            }
            if let Some(lowercase_len_operation) = &schema.lowercase_len {
                base.push(ValidationErr::LowercaseLen(lowercase_len_operation.clone()))
            }
            if let Some(uppercase_len_operation) = &schema.uppercase_len {
                base.push(ValidationErr::UppercaseLen(uppercase_len_operation.clone()))
            }
            if let Some(numbers_len_operation) = &schema.numbers_len {
                base.push(ValidationErr::NumbersLen(numbers_len_operation.clone()))
            }
            if let Some(symbols_len_operation) = &schema.symbols_len {
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
        schema::StrSchema,
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
        let v = StrSchema::default();
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR])));
    }

    #[test]
    fn validate_str_optional() {
        let v = StrSchema::default().optional();
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR])));
    }

    #[test]
    fn validate_str_operation_value() {
        let v = StrSchema::default().eq("Cogito ergo sum".into());
        let op_err = ValidationErr::Operation(Operation::Eq(Operand::Value(OperandValue::from("Cogito ergo sum"))));
        assert_eq!(validate_str(&v, &Value::from("Cogito ergo sum"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("Memento mori"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }

    #[test]
    fn validate_str_operation_field() {
        let v = StrSchema::default().ne_field("str".into());
        let op_err = ValidationErr::Operation(Operation::Ne(Operand::FieldPath("str".into())));
        assert_eq!(validate_str(&v, &Value::from("Memento mori"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("j"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_value() {
        let v = StrSchema::default().bytes_len_ne(16);
        let op_err = ValidationErr::BytesLen(Operation::Ne(Operand::Value(OperandValue::USize(16))));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }

    #[test]
    fn validate_bytes_len_field() {
        let v = StrSchema::default().bytes_len_gt_field("usize.values.nums.23".into());
        let op_err = ValidationErr::BytesLen(Operation::Gt(Operand::FieldPath("usize.values.nums.23".into())));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_value() {
        let v = StrSchema::default().chars_len_gt(12);
        let op_err = ValidationErr::CharsLen(Operation::Gt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_field() {
        let v = StrSchema::default().chars_len_ge_field("usize.values.nums.12".into());
        let op_err = ValidationErr::CharsLen(Operation::Ge(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }

    #[test]
    fn validate_chars_len_normalized() {
        let v = StrSchema::default().chars_len_eq(5);
        let a_upper_decomposed = "A\u{300}A\u{301}A\u{302}A\u{303}A\u{308}";
        let e_lower_decomposed = "e\u{300}e\u{301}e\u{302}e\u{303}e\u{308}";
        assert_eq!(validate_str(&v, &Value::from(a_upper_decomposed), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from(e_lower_decomposed), &ROOT), Ok(()));
    }

    #[test]
    fn validate_graphemes_len_value() {
        let v = StrSchema::default().graphemes_len_ge(12);
        let op_err = ValidationErr::GraphemesLen(Operation::Ge(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }

    #[test]
    fn validate_graphemes_len_field() {
        let v = StrSchema::default().graphemes_len_lt_field("usize.values.nums.12".into());
        let op_err = ValidationErr::GraphemesLen(Operation::Lt(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_value() {
        let v = StrSchema::default().lowercase_len_lt(12);
        let op_err = ValidationErr::LowercaseLen(Operation::Lt(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("группа крови"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }

    #[test]
    fn validate_lowercase_len_field() {
        let v = StrSchema::default().lowercase_len_le_field("usize.values.nums.12".into());
        let op_err = ValidationErr::LowercaseLen(Operation::Le(Operand::FieldPath("usize.values.nums.12".into())));
        assert_eq!(validate_str(&v, &Value::from("veni, vidi, vici"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὅσον ζῇς, φαίνου"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_value() {
        let v = StrSchema::default().uppercase_len_le(12);
        let op_err = ValidationErr::UppercaseLen(Operation::Le(Operand::Value(OperandValue::USize(12))));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }

    #[test]
    fn validate_uppercase_len_field() {
        let v = StrSchema::default().uppercase_len_btwn_field("usize.values.nums.11".into(), "usize.values.nums.12".into());
        let op_err = ValidationErr::UppercaseLen(Operation::Btwn(
            Operand::FieldPath("usize.values.nums.11".into()),
            Operand::FieldPath("usize.values.nums.12".into()),
        ));
        assert_eq!(validate_str(&v, &Value::from("VENI, VIDI, VICI"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_value() {
        let v = StrSchema::default().numbers_len_btwn(2, 3);
        let op_err = ValidationErr::NumbersLen(Operation::Btwn(Operand::Value(OperandValue::USize(2)), Operand::Value(OperandValue::USize(3))));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("4444"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }

    #[test]
    fn validate_numbers_len_field() {
        let v = StrSchema::default().numbers_len_eq_field("usize.values.nums.2".into());
        let op_err = ValidationErr::NumbersLen(Operation::Eq(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("22"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("333"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_value() {
        let v = StrSchema::default().symbols_len_eq(2);
        let op_err = ValidationErr::SymbolsLen(Operation::Eq(Operand::Value(OperandValue::USize(2))));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("$%^"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }

    #[test]
    fn validate_symbols_len_field() {
        let v = StrSchema::default().symbols_len_ne_field("usize.values.nums.2".into());
        let op_err = ValidationErr::SymbolsLen(Operation::Ne(Operand::FieldPath("usize.values.nums.2".into())));
        assert_eq!(validate_str(&v, &Value::from("!"), &ROOT), Ok(()));
        assert_eq!(validate_str(&v, &Value::from("@#"), &ROOT), Err(SchemaErr::from([op_err.clone()])));
        assert_eq!(validate_str(&v, &Value::None, &ROOT), Err(SchemaErr::from([REQUIRED, STR, op_err.clone()])));
        assert_eq!(validate_str(&v, &u64_stub(), &ROOT), Err(SchemaErr::from([STR, op_err.clone()])));
    }
}

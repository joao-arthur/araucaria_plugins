use araucaria::{
    error::{SchemaErr, ValidationErr},
    operation::{compare, Operation},
    validation::str::StrValidation,
    value::Value,
};
use unicode_segmentation::UnicodeSegmentation;

pub fn validate_str(validation: &StrValidation, value: &Value) -> Result<(), SchemaErr> {
    let mut base = vec![];
    match value {
        Value::Str(str_value) => {
            if let Some(v) = &validation.eq {
                if str_value != v {
                    base.push(ValidationErr::Eq(Value::Str(v.clone())));
                }
            }
            if let Some(v) = &validation.ne {
                if str_value == v {
                    base.push(ValidationErr::Ne(Value::Str(v.clone())));
                }
            }
            if let Some(op) = &validation.bytes_len {
                let len = str_value.as_bytes().len();
                if let Err(op_err) = compare(op, &len) {
                    base.push(ValidationErr::BytesLen(op_err))
                }
            }
            // let len = str_value.graphemes(true).collect::<Vec<&str>>().len();
        }
        Value::None => {
            if validation.required {
                base.push(ValidationErr::Required);
            }
            base.push(ValidationErr::Str);
            if let Some(v) = &validation.eq {
                base.push(ValidationErr::Eq(Value::Str(v.clone())));
            }
            if let Some(v) = &validation.ne {
                base.push(ValidationErr::Ne(Value::Str(v.clone())));
            }
        }
        _ => {
            base.push(ValidationErr::Str);
            if let Some(v) = &validation.eq {
                base.push(ValidationErr::Eq(Value::Str(v.clone())));
            }
            if let Some(v) = &validation.ne {
                base.push(ValidationErr::Ne(Value::Str(v.clone())));
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
    use araucaria::value::stub::num_u_stub;

    use super::*;

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
        assert_eq!(validate_str(&v, &Value::from("Memento mori")), Err(SchemaErr::validation([ValidationErr::Eq(Value::Str(String::from("Cogito ergo sum")))])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, ValidationErr::Eq(Value::Str(String::from("Cogito ergo sum")))])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, ValidationErr::Eq(Value::Str(String::from("Cogito ergo sum")))])));
    }

    #[test]
    fn test_validate_str_ne() {
        let v = StrValidation::default().ne(String::from("Cogito ergo sum"));
        assert_eq!(validate_str(&v, &Value::Str(String::from("Memento mori"))), Ok(()));
        assert_eq!(validate_str(&v, &Value::Str(String::from("Cogito ergo sum"))), Err(SchemaErr::validation([ValidationErr::Ne(Value::Str(String::from("Cogito ergo sum")))])));
        assert_eq!(validate_str(&v, &Value::None), Err(SchemaErr::validation([ValidationErr::Required, ValidationErr::Str, ValidationErr::Ne(Value::Str(String::from("Cogito ergo sum")))])));
        assert_eq!(validate_str(&v, &num_u_stub()), Err(SchemaErr::validation([ValidationErr::Str, ValidationErr::Ne(Value::Str(String::from("Cogito ergo sum")))])));
    }

    #[test]
    fn test_validate_bytes_len_latin() {
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(15), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(15))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(16), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(17), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(17))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(15), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(16), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ne(16))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(17), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(14), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(15), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(16), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(16))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(17), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(17))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(15), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(16), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(17), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(17))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(18), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(18))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(15), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(15))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(16), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(16))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(17), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(18), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(14), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(14))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(15), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(15))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(16), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(17), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(13, 15), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(13, 15))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(14, 16), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(15, 17), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(16, 18), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(17, 19), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(17, 19))])));
    }

    #[test]
    fn test_validate_bytes_len_greek() {
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(30), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(30))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(31), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(32), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(32))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(30), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(31), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ne(31))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(32), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(29), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(30), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(31), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(31))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(32), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(32))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(30), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(31), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(32), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(32))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(33), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(33))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(30), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(30))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(31), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(31))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(32), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(33), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(29), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(29))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(30), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(30))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(31), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(32), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(28, 30), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(28, 30))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(29, 31), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(30, 32), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(31, 33), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(32, 34), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(32, 34))])));
    }

    #[test]
    fn test_validate_bytes_len_cyrillic() {
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(22), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(22))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(23), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(24), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(24))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(22), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(23), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ne(23))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(24), &Value::Str(String::from("группа крови"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(21), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(22), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(23), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(23))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(24), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(24))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(22), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(23), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(24), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(24))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(25), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(25))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(22), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(22))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(23), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(23))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(24), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(25), &Value::Str(String::from("группа крови"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(21), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(21))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(22), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(22))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(23), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(24), &Value::Str(String::from("группа крови"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(20, 22), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(20, 22))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(21, 23), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(22, 24), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(23, 25), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(24, 26), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(24, 26))])));
    }

    #[test]
    fn test_validate_bytes_len_cuneiform() {
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(28), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(28))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(29), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_eq(30), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(30))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(28), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(29), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ne(29))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ne(30), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(27), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(28), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(29), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(29))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_gt(30), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(30))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(28), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(29), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(30), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(30))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_ge(31), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(31))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(28), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(28))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(29), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(29))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(30), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_lt(31), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(27), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(27))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(28), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(28))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(29), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_le(30), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(26, 28), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(26, 28))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(27, 29), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(28, 30), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(29, 31), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len_btwn(30, 32), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(30, 32))])));
    }
}

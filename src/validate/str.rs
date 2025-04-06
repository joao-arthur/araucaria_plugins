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
            // let len = str_value.graphemes(true).collect::<Vec<&str>>().len() ;
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
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Eq(15)), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(15))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Eq(16)), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Eq(17)), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(17))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ne(15)), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ne(16)), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ne(16))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ne(17)), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(14)), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(15)), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(16)), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(16))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(17)), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(17))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(15)), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(16)), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(17)), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(17))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(18)), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(18))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(15)), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(15))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(16)), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(16))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(17)), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(18)), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(14)), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(14))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(15)), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(15))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(16)), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(17)), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(13, 15)), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(13, 15))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(14, 16)), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(15, 17)), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(16, 18)), &Value::Str(String::from("veni, vidi, vici"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(17, 19)), &Value::Str(String::from("veni, vidi, vici"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(17, 19))])));
    }

    #[test]
    fn test_validate_bytes_len_greek() {
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Eq(30)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(30))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Eq(31)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Eq(32)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(32))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ne(30)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ne(31)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ne(31))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ne(32)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(29)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(30)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(31)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(31))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(32)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(32))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(30)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(31)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(32)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(32))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(33)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(33))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(30)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(30))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(31)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(31))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(32)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(33)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(29)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(29))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(30)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(30))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(31)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(32)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(28, 30)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(28, 30))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(29, 31)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(30, 32)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(31, 33)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(32, 34)), &Value::Str(String::from("ὅσον ζῇς, φαίνου"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(32, 34))])));
    }

    #[test]
    fn test_validate_bytes_len_cyrillic() {
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Eq(22)), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(22))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Eq(23)), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Eq(24)), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(24))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ne(22)), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ne(23)), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ne(23))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ne(24)), &Value::Str(String::from("группа крови"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(21)), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(22)), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(23)), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(23))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(24)), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(24))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(22)), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(23)), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(24)), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(24))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(25)), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(25))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(22)), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(22))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(23)), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(23))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(24)), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(25)), &Value::Str(String::from("группа крови"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(21)), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(21))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(22)), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(22))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(23)), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(24)), &Value::Str(String::from("группа крови"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(20, 22)), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(20, 22))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(21, 23)), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(22, 24)), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(23, 25)), &Value::Str(String::from("группа крови"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(24, 26)), &Value::Str(String::from("группа крови"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(24, 26))])));
    }

    #[test]
    fn test_validate_bytes_len_cuneiform() {
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Eq(28)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(28))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Eq(29)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Eq(30)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Eq(30))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ne(28)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ne(29)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ne(29))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ne(30)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(27)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(28)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(29)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(29))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Gt(30)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Gt(30))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(28)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(29)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(30)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(30))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Ge(31)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Ge(31))])));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(28)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(28))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(29)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Lt(29))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(30)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Lt(31)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(27)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(27))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(28)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Le(28))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(29)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Le(30)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));

        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(26, 28)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(26, 28))])));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(27, 29)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(28, 30)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(29, 31)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Ok(()));
        assert_eq!(validate_str(&StrValidation::default().bytes_len(Operation::Btwn(30, 32)), &Value::Str(String::from("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"))), Err(SchemaErr::validation([ValidationErr::BytesLen(Operation::Btwn(30, 32))])));
    }
}

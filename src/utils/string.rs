use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

pub fn bytes_len(str_value: &String) -> usize {
    str_value.len()
}

pub fn chars_len(str_value: &String) -> usize {
    str_value.chars().count()
}

pub fn graphemes_len(str_value: &String) -> usize {
    str_value.graphemes(true).collect::<Vec<&str>>().len()
}

pub fn lowercase_len(str_value: &String) -> usize {
    str_value.chars().filter(|c| c.is_lowercase()).count()
}

pub fn uppercase_len(str_value: &String) -> usize {
    str_value.chars().filter(|c| c.is_uppercase()).count()
}

pub fn numbers_len(str_value: &String) -> usize {
    str_value.chars().filter(|c| c.is_ascii_digit()).count()
}

pub fn symbols_len(str_value: &String) -> usize {
    str_value.chars().filter(|c| c.is_ascii_punctuation()).count()
}

pub fn normalize_nfc(str_value: &String) -> String {
    str_value.nfc().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::{bytes_len, chars_len, graphemes_len, lowercase_len, normalize_nfc, numbers_len, symbols_len, uppercase_len};

    #[test]
    fn test_bytes_len() {
        assert_eq!(bytes_len(&"veni, vidi, vici".into()), 16);
        assert_eq!(bytes_len(&"ὅσον ζῇς, φαίνου".into()), 31);
        assert_eq!(bytes_len(&"группа крови".into()), 23);
        assert_eq!(bytes_len(&"ओंकार".into()), 15);
        assert_eq!(bytes_len(&"𒀀𒈾 𒂍𒀀𒈾𒍢𒅕".into()), 29);
    }

    #[test]
    fn bytes_len_emoji() {
        assert_eq!(bytes_len(&"👩‍👩‍👧‍👧".into()), 25);
        assert_eq!(bytes_len(&"👩‍👩‍👧".into()), 18);
    }

    #[test]
    fn test_chars_len() {
        assert_eq!(chars_len(&"veni, vidi, vici".into()), 16);
        assert_eq!(chars_len(&"ὅσον ζῇς, φαίνου".into()), 16);
        assert_eq!(chars_len(&"группа крови".into()), 12);
        assert_eq!(chars_len(&"ओंकार".into()), 5);
        assert_eq!(chars_len(&"𒀀𒈾 𒂍𒀀𒈾𒍢𒅕".into()), 8);
    }

    #[test]
    fn chars_len_emoji() {
        assert_eq!(chars_len(&"👩‍👩‍👧‍👧".into()), 7);
        assert_eq!(chars_len(&"👩‍👩‍👧".into()), 5);
    }

    #[test]
    fn test_graphemes_len() {
        assert_eq!(graphemes_len(&"veni, vidi, vici".into()), 16);
        assert_eq!(graphemes_len(&"ὅσον ζῇς, φαίνου".into()), 16);
        assert_eq!(graphemes_len(&"группа крови".into()), 12);
        assert_eq!(graphemes_len(&"ओंकार".into()), 3);
        assert_eq!(graphemes_len(&"𒀀𒈾 𒂍𒀀𒈾𒍢𒅕".into()), 8);
    }

    #[test]
    fn graphemes_len_emoji() {
        assert_eq!(graphemes_len(&"👩‍👩‍👧‍👧".into()), 1);
        assert_eq!(graphemes_len(&"👩‍👩‍👧".into()), 1);
    }

    #[test]
    fn lowercase_len_lowercase() {
        assert_eq!(lowercase_len(&"группа крови".into()), 11);
        assert_eq!(lowercase_len(&"veni, vidi, vici".into()), 12);
        assert_eq!(lowercase_len(&"ὅσον ζῇς, φαίνου".into()), 13);
    }

    #[test]
    fn lowercase_len_uppercase() {
        assert_eq!(lowercase_len(&"ГРУППА КРОВИ".into()), 0);
        assert_eq!(lowercase_len(&"VENI, VIDI, VICI".into()), 0);
        assert_eq!(lowercase_len(&"ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ".into()), 0);
    }

    #[test]
    fn lowercase_len_not_applyable() {
        assert_eq!(lowercase_len(&"👩‍👩‍👧‍👧".into()), 0);
        assert_eq!(lowercase_len(&"👩‍👩‍👧".into()), 0);
        assert_eq!(lowercase_len(&"ओंकार".into()), 0);
        assert_eq!(lowercase_len(&"𒀀𒈾 𒂍𒀀𒈾𒍢𒅕".into()), 0);
    }

    #[test]
    fn uppercase_len_lowercase() {
        assert_eq!(uppercase_len(&"группа крови".into()), 0);
        assert_eq!(uppercase_len(&"veni, vidi, vici".into()), 0);
        assert_eq!(uppercase_len(&"ὅσον ζῇς, φαίνου".into()), 0);
    }

    #[test]
    fn uppercase_len_uppercase() {
        assert_eq!(uppercase_len(&"ГРУППА КРОВИ".into()), 11);
        assert_eq!(uppercase_len(&"VENI, VIDI, VICI".into()), 12);
        assert_eq!(uppercase_len(&"ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ".into()), 14);
    }

    #[test]
    fn uppercase_len_not_applyable() {
        assert_eq!(uppercase_len(&"👩‍👩‍👧‍👧".into()), 0);
        assert_eq!(uppercase_len(&"👩‍👩‍👧".into()), 0);
        assert_eq!(uppercase_len(&"ओंकार".into()), 0);
        assert_eq!(uppercase_len(&"𒀀𒈾 𒂍𒀀𒈾𒍢𒅕".into()), 0);
    }

    #[test]
    fn test_numbers_len() {
        assert_eq!(numbers_len(&"veni, vidi, vici".into()), 0);
        assert_eq!(numbers_len(&"ὅσον ζῇς, φαίνου".into()), 0);
        assert_eq!(numbers_len(&"группа крови".into()), 0);
        assert_eq!(numbers_len(&"ओंकार".into()), 0);
        assert_eq!(numbers_len(&"𒀀𒈾 𒂍𒀀𒈾𒍢𒅕".into()), 0);
        assert_eq!(numbers_len(&"👩‍👩‍👧‍👧".into()), 0);
        assert_eq!(numbers_len(&"0123456789".into()), 10);
    }

    #[test]
    fn test_symbols_len() {
        assert_eq!(symbols_len(&"veni, vidi, vici".into()), 2);
        assert_eq!(symbols_len(&"ὅσον ζῇς, φαίνου".into()), 1);
        assert_eq!(symbols_len(&"группа крови".into()), 0);
        assert_eq!(symbols_len(&"ओंकार".into()), 0);
        assert_eq!(symbols_len(&"𒀀𒈾 𒂍𒀀𒈾𒍢𒅕".into()), 0);
        assert_eq!(symbols_len(&"👩‍👩‍👧‍👧".into()), 0);
        assert_eq!(symbols_len(&"!\"#$%&'()*+,-./".into()), 15);
        assert_eq!(symbols_len(&":;<=>?@".into()), 7);
        assert_eq!(symbols_len(&"[\\]^_`".into()), 6);
        assert_eq!(symbols_len(&"{|}~".into()), 4);
    }

    #[test]
    fn validate_chars_len_eq_normalized() {
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

        assert_eq!(normalize_nfc(&a_upper_decomposed.into()), a_upper_composed);
        assert_eq!(normalize_nfc(&e_upper_decomposed.into()), e_upper_composed);
        assert_eq!(normalize_nfc(&i_upper_decomposed.into()), i_upper_composed);
        assert_eq!(normalize_nfc(&o_upper_decomposed.into()), o_upper_composed);
        assert_eq!(normalize_nfc(&u_upper_decomposed.into()), u_upper_composed);

        assert_eq!(normalize_nfc(&a_lower_decomposed.into()), a_lower_composed);
        assert_eq!(normalize_nfc(&e_lower_decomposed.into()), e_lower_composed);
        assert_eq!(normalize_nfc(&i_lower_decomposed.into()), i_lower_composed);
        assert_eq!(normalize_nfc(&o_lower_decomposed.into()), o_lower_composed);
        assert_eq!(normalize_nfc(&u_lower_decomposed.into()), u_lower_composed);
    }
}

use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

pub fn bytes_len(str_value: &str) -> usize {
    str_value.len()
}

pub fn chars_len(str_value: &str) -> usize {
    str_value.chars().count()
}

pub fn graphemes_len(str_value: &str) -> usize {
    str_value.graphemes(true).collect::<Vec<&str>>().len()
}

pub fn lowercase_len(str_value: &str) -> usize {
    str_value.chars().filter(|c| c.is_lowercase()).count()
}

pub fn uppercase_len(str_value: &str) -> usize {
    str_value.chars().filter(|c| c.is_uppercase()).count()
}

pub fn numbers_len(str_value: &str) -> usize {
    str_value.chars().filter(|c| c.is_ascii_digit()).count()
}

pub fn symbols_len(str_value: &str) -> usize {
    str_value.chars().filter(|c| c.is_ascii_punctuation()).count()
}

pub fn normalize_nfc(str_value: &str) -> String {
    str_value.nfc().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::{bytes_len, chars_len, graphemes_len, lowercase_len, normalize_nfc, numbers_len, symbols_len, uppercase_len};

    #[test]
    fn bytes_len_multiple_scripts() {
        assert_eq!(bytes_len("veni, vidi, vici"), 16);
        assert_eq!(bytes_len("ὅσον ζῇς, φαίνου"), 31);
        assert_eq!(bytes_len("группа крови"), 23);
        assert_eq!(bytes_len("ओंकार"), 15);
        assert_eq!(bytes_len("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), 29);
    }

    #[test]
    fn bytes_len_emoji() {
        assert_eq!(bytes_len("👩‍👩‍👧‍👧"), 25);
        assert_eq!(bytes_len("👩‍👩‍👧"), 18);
    }

    #[test]
    fn chars_len_multiple_scripts() {
        assert_eq!(chars_len("veni, vidi, vici"), 16);
        assert_eq!(chars_len("ὅσον ζῇς, φαίνου"), 16);
        assert_eq!(chars_len("группа крови"), 12);
        assert_eq!(chars_len("ओंकार"), 5);
        assert_eq!(chars_len("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), 8);
    }

    #[test]
    fn chars_len_emoji() {
        assert_eq!(chars_len("👩‍👩‍👧‍👧"), 7);
        assert_eq!(chars_len("👩‍👩‍👧"), 5);
    }

    #[test]
    fn graphemes_len_multiple_scripts() {
        assert_eq!(graphemes_len("veni, vidi, vici"), 16);
        assert_eq!(graphemes_len("ὅσον ζῇς, φαίνου"), 16);
        assert_eq!(graphemes_len("группа крови"), 12);
        assert_eq!(graphemes_len("ओंकार"), 3);
        assert_eq!(graphemes_len("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), 8);
    }

    #[test]
    fn graphemes_len_emoji() {
        assert_eq!(graphemes_len("👩‍👩‍👧‍👧"), 1);
        assert_eq!(graphemes_len("👩‍👩‍👧"), 1);
    }

    #[test]
    fn lowercase_len_lowercase_multiple_scripts() {
        assert_eq!(lowercase_len("группа крови"), 11);
        assert_eq!(lowercase_len("veni, vidi, vici"), 12);
        assert_eq!(lowercase_len("ὅσον ζῇς, φαίνου"), 13);
    }

    #[test]
    fn lowercase_len_uppercase_multiple_scripts() {
        assert_eq!(lowercase_len("ГРУППА КРОВИ"), 0);
        assert_eq!(lowercase_len("VENI, VIDI, VICI"), 0);
        assert_eq!(lowercase_len("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), 0);
    }

    #[test]
    fn lowercase_len_not_applyable() {
        assert_eq!(lowercase_len("👩‍👩‍👧‍👧"), 0);
        assert_eq!(lowercase_len("👩‍👩‍👧"), 0);
        assert_eq!(lowercase_len("ओंकार"), 0);
        assert_eq!(lowercase_len("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), 0);
    }

    #[test]
    fn uppercase_len_lowercase_multiple_scripts() {
        assert_eq!(uppercase_len("группа крови"), 0);
        assert_eq!(uppercase_len("veni, vidi, vici"), 0);
        assert_eq!(uppercase_len("ὅσον ζῇς, φαίνου"), 0);
    }

    #[test]
    fn uppercase_len_uppercase_multiple_scripts() {
        assert_eq!(uppercase_len("ГРУППА КРОВИ"), 11);
        assert_eq!(uppercase_len("VENI, VIDI, VICI"), 12);
        assert_eq!(uppercase_len("ὍΣΟΝ ΖΗ͂ΙΣ, ΦΑΊΝΟΥ"), 14);
    }

    #[test]
    fn uppercase_len_not_applyable() {
        assert_eq!(uppercase_len("👩‍👩‍👧‍👧"), 0);
        assert_eq!(uppercase_len("👩‍👩‍👧"), 0);
        assert_eq!(uppercase_len("ओंकार"), 0);
        assert_eq!(uppercase_len("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), 0);
    }

    #[test]
    fn numbers_len_multiple_scripts() {
        assert_eq!(numbers_len("veni, vidi, vici"), 0);
        assert_eq!(numbers_len("ὅσον ζῇς, φαίνου"), 0);
        assert_eq!(numbers_len("группа крови"), 0);
        assert_eq!(numbers_len("ओंकार"), 0);
        assert_eq!(numbers_len("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), 0);
        assert_eq!(numbers_len("👩‍👩‍👧‍👧"), 0);
        assert_eq!(numbers_len("0123456789"), 10);
    }

    #[test]
    fn symbols_len_multiple_scripts() {
        assert_eq!(symbols_len("veni, vidi, vici"), 2);
        assert_eq!(symbols_len("ὅσον ζῇς, φαίνου"), 1);
        assert_eq!(symbols_len("группа крови"), 0);
        assert_eq!(symbols_len("ओंकार"), 0);
        assert_eq!(symbols_len("𒀀𒈾 𒂍𒀀𒈾𒍢𒅕"), 0);
        assert_eq!(symbols_len("👩‍👩‍👧‍👧"), 0);
        assert_eq!(symbols_len("!\"#$%&'()*+,-./"), 15);
        assert_eq!(symbols_len(":;<=>?@"), 7);
        assert_eq!(symbols_len("[\\]^_`"), 6);
        assert_eq!(symbols_len("{|}~"), 4);
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

        assert_eq!(normalize_nfc(a_upper_decomposed), a_upper_composed);
        assert_eq!(normalize_nfc(e_upper_decomposed), e_upper_composed);
        assert_eq!(normalize_nfc(i_upper_decomposed), i_upper_composed);
        assert_eq!(normalize_nfc(o_upper_decomposed), o_upper_composed);
        assert_eq!(normalize_nfc(u_upper_decomposed), u_upper_composed);

        assert_eq!(normalize_nfc(a_lower_decomposed), a_lower_composed);
        assert_eq!(normalize_nfc(e_lower_decomposed), e_lower_composed);
        assert_eq!(normalize_nfc(i_lower_decomposed), i_lower_composed);
        assert_eq!(normalize_nfc(o_lower_decomposed), o_lower_composed);
        assert_eq!(normalize_nfc(u_lower_decomposed), u_lower_composed);
    }
}

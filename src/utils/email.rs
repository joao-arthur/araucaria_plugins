use email_address::EmailAddress;

pub fn email_is_valid(email: &str) -> bool {
    EmailAddress::is_valid(email)
}

#[cfg(test)]
mod tests {
    use super::email_is_valid;

    #[test]
    fn email_is_valid_true() {
        assert!(email_is_valid("john.lennon@gmail.com"));
        assert!(email_is_valid("paul_macca@hotmail.com"));
        assert!(email_is_valid("ringo-starr@outlook.com"));
        assert!(email_is_valid("GeorgeHarrison@live.com"));
    }

    #[test]
    fn email_is_valid_false() {
        assert!(!email_is_valid("paullivecom"));
        assert!(!email_is_valid("paullive.com"));
        assert!(!email_is_valid("paul@liv@e.com"));
        assert!(!email_is_valid("live.com"));
        assert!(!email_is_valid("@live.com"));
    }
}

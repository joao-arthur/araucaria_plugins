use std::sync::LazyLock;

use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Tm(pub u8, pub u8);

static TM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([0-9]{2}):([0-9]{2})$").unwrap());

pub fn parse_time(s: &str) -> Result<Tm, ()> {
    let caps = TM_REGEX.captures(s).ok_or(())?;
    let (_, [h, m]) = caps.extract();
    let h = h.parse::<u8>().map_err(|_| ())?;
    let m = m.parse::<u8>().map_err(|_| ())?;
    if h > 23 || m > 59 {
        return Err(());
    }
    Ok(Tm(h, m))
}

#[cfg(test)]
mod tests {
    use super::{Tm, parse_time};

    #[test]
    fn parse_time_ok() {
        assert_eq!(parse_time("06:11"), Ok(Tm(6, 11)));
    }

    #[test]
    fn parse_time_invalid_format() {
        assert_eq!(parse_time("10:27:23.235"), Err(()));
        assert_eq!(parse_time("10:27:24"), Err(()));
        assert_eq!(parse_time("1061"), Err(()));
        assert_eq!(parse_time("106"), Err(()));
        assert_eq!(parse_time("10"), Err(()));
        assert_eq!(parse_time("1"), Err(()));
    }

    #[test]
    fn parse_time_invalid_value() {
        assert_eq!(parse_time("24:00"), Err(()));
        assert_eq!(parse_time("00:60"), Err(()));
        assert_eq!(parse_time("24:20"), Err(()));
        assert_eq!(parse_time("04:99"), Err(()));
        assert_eq!(parse_time("72:93"), Err(()));
    }
}

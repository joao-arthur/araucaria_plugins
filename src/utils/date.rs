use std::sync::LazyLock;

use chrono::NaiveDate;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Dt(pub u32, pub u8, pub u8);

static DT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$").unwrap());

pub fn parse_date(s: &str) -> Result<Dt, ()> {
    let caps = DT_REGEX.captures(s).ok_or(())?;
    let (_, [yyyy, mm, dd]) = caps.extract();
    let yyyy = yyyy.parse::<u32>().map_err(|_| ())?;
    let mm = mm.parse::<u8>().map_err(|_| ())?;
    let dd = dd.parse::<u8>().map_err(|_| ())?;
    NaiveDate::from_ymd_opt(yyyy as i32, mm.into(), dd.into()).ok_or(())?;
    Ok(Dt(yyyy, mm, dd))
}

#[cfg(test)]
mod tests {
    use super::{Dt, parse_date};

    #[test]
    fn parse_date_ok() {
        assert_eq!(parse_date("2029-12-31".into()), Ok(Dt(2029, 12, 31)));
        assert_eq!(parse_date("2024-02-29".into()), Ok(Dt(2024, 2, 29)));
    }

    #[test]
    fn parse_date_invalid_format() {
        assert_eq!(parse_date("10-10-2026"), Err(()));
        assert_eq!(parse_date("10-2026-10"), Err(()));
        assert_eq!(parse_date("2026/10/28"), Err(()));
        assert_eq!(parse_date("28/10/2026"), Err(()));
        assert_eq!(parse_date("20261028"), Err(()));
        assert_eq!(parse_date("28102026"), Err(()));
    }

    #[test]
    fn parse_date_invalid_value() {
        assert_eq!(parse_date("2029-12-00"), Err(()));
        assert_eq!(parse_date("2029-11-31"), Err(()));
        assert_eq!(parse_date("2029-12-32"), Err(()));
        assert_eq!(parse_date("2029-00-26"), Err(()));
        assert_eq!(parse_date("2029-13-26"), Err(()));
        assert_eq!(parse_date("2029-17-83"), Err(()));
    }
}

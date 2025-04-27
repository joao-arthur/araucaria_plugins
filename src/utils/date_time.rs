use std::sync::LazyLock;

use chrono::NaiveDate;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct DtTm(pub u32, pub u16, pub u16, pub u8, pub u8);

static DT_TM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2})Z$").unwrap());

pub fn parse_date_time(s: &str) -> Result<DtTm, ()> {
    let caps = DT_TM_REGEX.captures(s).ok_or(())?;
    let (_, [yyyy, mm, dd, h, m]) = caps.extract();
    let yyyy = yyyy.parse::<u32>().map_err(|_| ())?;
    let mm = mm.parse::<u16>().map_err(|_| ())?;
    let dd = dd.parse::<u16>().map_err(|_| ())?;
    let h = h.parse::<u8>().map_err(|_| ())?;
    let m = m.parse::<u8>().map_err(|_| ())?;
    NaiveDate::from_ymd_opt(yyyy as i32, mm.into(), dd.into()).ok_or(())?;
    if h > 23 || m > 59 {
        return Err(());
    }
    Ok(DtTm(yyyy, mm, dd, h, m))
}

#[cfg(test)]
mod tests {
    use super::{DtTm, parse_date_time};

    #[test]
    fn parse_date_time_ok() {
        assert_eq!(parse_date_time("2029-12-31T06:11Z".into()), Ok(DtTm(2029, 12, 31, 6, 11)));
    }

    #[test]
    fn parse_date_time_invalid_format() {
        assert_eq!(parse_date_time("10-10-2026"), Err(()));
        assert_eq!(parse_date_time("10-2026-10"), Err(()));
        assert_eq!(parse_date_time("2026/10/28"), Err(()));
        assert_eq!(parse_date_time("28/10/2026"), Err(()));
        assert_eq!(parse_date_time("20261028"), Err(()));
        assert_eq!(parse_date_time("28102026"), Err(()));
        assert_eq!(parse_date_time("10:27:23.235"), Err(()));
        assert_eq!(parse_date_time("10:27:24"), Err(()));
        assert_eq!(parse_date_time("1061"), Err(()));
        assert_eq!(parse_date_time("106"), Err(()));
        assert_eq!(parse_date_time("10"), Err(()));
        assert_eq!(parse_date_time("1"), Err(()));
        assert_eq!(parse_date_time("2026-10-28T10:27:29Z"), Err(()));
        assert_eq!(parse_date_time("2026-10-28T10:27:29.973Z"), Err(()));
        assert_eq!(parse_date_time("10-2026-28T10:27:29.973Z"), Err(()));
        assert_eq!(parse_date_time("28-10-2026T10:27:29.973Z"), Err(()));
    }

    #[test]
    fn parse_date_time_invalid_value() {
        assert_eq!(parse_date_time("2029-12-00T20:42Z"), Err(()));
        assert_eq!(parse_date_time("2029-11-31T20:42Z"), Err(()));
        assert_eq!(parse_date_time("2029-12-32T20:42Z"), Err(()));
        assert_eq!(parse_date_time("2029-00-26T20:42Z"), Err(()));
        assert_eq!(parse_date_time("2029-13-26T20:42Z"), Err(()));
        assert_eq!(parse_date_time("2029-17-83T20:42Z"), Err(()));
        assert_eq!(parse_date_time("2024-04-26T24:00Z"), Err(()));
        assert_eq!(parse_date_time("2024-04-26T00:60Z"), Err(()));
        assert_eq!(parse_date_time("2024-04-26T24:20Z"), Err(()));
        assert_eq!(parse_date_time("2024-04-26T04:99Z"), Err(()));
        assert_eq!(parse_date_time("2024-04-26T72:93Z"), Err(()));
    }
}

//! Helper structs to assist deserialising common data types
use nom::{bytes::complete::take, IResult};
use rand::Rng;
use std::fmt::Display;

/// BSB number struct to represent XXX-XXX where Xs are numerics
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Bsb {
    pub first: String,
    pub sep: String,
    pub second: String,
}

impl Bsb {
    pub fn deserialise(i: &str) -> IResult<&str, Self> {
        let (i, first) = take(3u8)(i)?;
        let (i, sep) = take(1u8)(i)?;
        let (i, second) = take(3u8)(i)?;

        let bsb = Self {
            first: first.to_string(),
            sep: sep.to_string(),
            second: second.to_string(),
        };

        Ok((i, bsb))
    }
}

impl Display for Bsb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}{}{}", self.first, self.sep, self.second)
    }
}

/// The .aba file uses non-standard date format of DDMMYY.
/// Caveat is that the leap year validation is capped at year 9999.
/// This is a workaround that uses random two digits to make up a 4-digit year.
/// Please refer to `<https://www.ietf.org/rfc/rfc3339.txt>` for more info
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct TwoDigitYears {
    pub day: String,
    pub month: String,
    pub year: String,
}

impl TwoDigitYears {
    pub fn deserialise(i: &str) -> IResult<&str, String> {
        let mut rng = rand::thread_rng();

        let (i, day) = take(2u8)(i)?;
        let (i, month) = take(2u8)(i)?;
        let (i, year) = take(2u8)(i)?;

        Ok((
            i,
            format!(
                "{}{}{}{}{}",
                day,
                month,
                rng.gen_range(1i32..=9i32),
                rng.gen_range(1i32..=9i32),
                year
            ),
        ))
    }
}

impl Display for TwoDigitYears {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}{}{}", self.day, self.month, self.year)
    }
}

/// The total field is represented as a Total-Credit-Debit triplet
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct TotalField {
    pub total: String,
    pub credit: String,
    pub debit: String,
}

impl TotalField {
    pub fn deserialise(i: &str) -> IResult<&str, Self> {
        let (i, total) = take(10u8)(i)?;
        let (i, credit) = take(10u8)(i)?;
        let (i, debit) = take(10u8)(i)?;

        let total_field = Self {
            total: total.to_string(),
            credit: credit.to_string(),
            debit: debit.to_string(),
        };

        Ok((i, total_field))
    }
}

impl Display for TotalField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}{}{}", self.total, self.credit, self.debit)
    }
}

#[test]
fn test_bsb() {
    let bsb: &str = "063-000";
    let (_, result) = Bsb::deserialise(&bsb).unwrap();
    assert_eq!(result.first, "063");
}

#[test]
fn test_2dy() {
    let date: &str = "280222";
    let (_, result) = TwoDigitYears::deserialise(&date).unwrap();
    let (first, second) = result.split_at(4);
    let (_, last) = second.split_at(2);
    assert_eq!(first, "2802");
    assert_eq!(last, "22");
}

#[test]
fn test_total_field() {
    let total: &str = "000312924700031292470000000000";
    let (_, result) = TotalField::deserialise(&total).unwrap();
    assert_eq!(result.total, "0003129247");
}

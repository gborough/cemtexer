//! Total block is always the last line of a valid .aba file
use nom::bytes::complete::take;
use nom::IResult;
use std::{fmt::Display, fmt::Write};

use crate::csv::*;
use crate::errors::*;
use crate::parser_utils::*;
use crate::types::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref BLANK_1: String = " ".repeat(12);
    static ref BLANK_2: String = " ".repeat(24);
    static ref BLANK_3: String = " ".repeat(40);
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct TotalBlock {
    pub record_type: String,
    pub bsb_filler: String,
    pub blank_1: String,
    pub total_field: String,
    pub blank_2: String,
    pub record_count: String,
    pub blank_3: String,
}

impl TotalBlock {
    pub fn deserialise(i: &str) -> IResult<&str, Self> {
        let (i, record_type) = take(1u8)(i)?;
        let (i, bsb_filler) = take(7u8)(i)?;
        let (i, blank_1) = take(12u8)(i)?;
        let (i, total_field) = take(30u8)(i)?;
        let (i, blank_2) = take(24u8)(i)?;
        let (i, record_count) = take(6u8)(i)?;
        let (i, blank_3) = take(40u8)(i)?;

        let total = Self {
            record_type: record_type.to_owned(),
            bsb_filler: bsb_filler.to_owned(),
            blank_1: blank_1.to_owned(),
            total_field: total_field.to_owned(),
            blank_2: blank_2.to_owned(),
            record_count: record_count.to_owned(),
            blank_3: blank_3.to_owned(),
        };

        Ok((i, total))
    }

    pub async fn validate(&self, line_count: &u32) -> Result<String, LineParseError> {
        let mut res: String = String::new();

        let _res = validate_generic_filler_str(
            self.record_type.clone(),
            "7".to_owned(),
            ValidationType::TotalTypeSeven,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_generic_filler_str(
            self.bsb_filler.clone(),
            "999-999".to_owned(),
            ValidationType::TotalBsbFiller,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_generic_filler_str(
            self.blank_1.clone(),
            BLANK_1.to_owned(),
            ValidationType::TotalBlankOne,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res =
            validate_total_field(self.total_field.clone(), TotalAmountType::Total).map_err(|e| {
                res.write_str(&e.to_string()).unwrap();
            });

        let _res =
            validate_total_field(self.total_field.clone(), TotalAmountType::Credit).map_err(|e| {
                res.write_str(&e.to_string()).unwrap();
            });

        let _res =
            validate_total_field(self.total_field.clone(), TotalAmountType::Debit).map_err(|e| {
                res.write_str(&e.to_string()).unwrap();
            });

        let _res = validate_generic_filler_str(
            self.blank_2.clone(),
            BLANK_2.to_owned(),
            ValidationType::TotalBlankTwo,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_record_type_count(self.record_count.clone(), line_count).map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_generic_filler_str(
            self.blank_3.clone(),
            BLANK_3.to_owned(),
            ValidationType::TotalBlankThree,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        Ok(res)
    }
}

impl Display for TotalBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}",
            self.record_type,
            self.bsb_filler,
            self.blank_1,
            self.total_field,
            self.blank_2,
            self.record_count,
            self.blank_3
        )
    }
}

impl From<TotalRecord> for TotalBlock {
    fn from(tr: TotalRecord) -> Self {
        Self {
            record_type: "7".to_owned(),
            bsb_filler: "999-999".to_owned(),
            blank_1: BLANK_1.to_owned(),
            total_field: format!("{}{}0000000000", tr.total, tr.total),
            blank_2: BLANK_2.to_owned(),
            record_count: tr.line_count,
            blank_3: BLANK_3.to_owned(),
        }
    }
}

#[test]
fn test_total_deser() {
    let total: &'static str = "7999-999            000312924700031292470000000000                        000004                                        ";
    let (_, result) = TotalBlock::deserialise(&total).unwrap();
    assert_eq!(result.bsb_filler, "999-999")
}

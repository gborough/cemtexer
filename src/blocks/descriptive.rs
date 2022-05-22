//! Descriptive block is always the first line of a valid .aba file
use nom::bytes::complete::take;
use nom::IResult;
use std::{fmt::Display, fmt::Write};

use crate::csv::*;
use crate::errors::*;
use crate::parser_utils::*;
use crate::types::*;

/// Fillers for fixed blank position 1
const BLANK_1: &str = "                 ";
/// Fillers for fixed blank position 2
const BLANK_2: &str = "       ";
/// Fillers for fixed blank position 3
const BLANK_3: &str = "                                        ";

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct DescriptiveBlock {
    pub record_type: String,
    pub blank_1: String,
    pub reel_seq: String,
    pub bank_name: String,
    pub blank_2: String,
    pub user_name: String,
    pub apca_number: String,
    pub file_description: String,
    pub settle_date: String,
    pub blank_3: String,
}

impl DescriptiveBlock {
    pub fn deserialise(i: &str) -> IResult<&str, Self> {
        let (i, record_type) = take(1u8)(i)?;
        let (i, blank_1) = take(17u8)(i)?;
        let (i, reel_seq) = take(2u8)(i)?;
        let (i, bank_name) = take(3u8)(i)?;
        let (i, blank_2) = take(7u8)(i)?;
        let (i, user_name) = take(26u8)(i)?;
        let (i, apca_number) = take(6u8)(i)?;
        let (i, file_description) = take(12u8)(i)?;
        let (i, settle_date) = take(6u8)(i)?;
        let (i, blank_3) = take(40u8)(i)?;

        let descriptive = Self {
            record_type: record_type.to_string(),
            blank_1: blank_1.to_string(),
            reel_seq: reel_seq.to_string(),
            bank_name: bank_name.to_string(),
            blank_2: blank_2.to_string(),
            user_name: user_name.to_string(),
            apca_number: apca_number.to_string(),
            file_description: file_description.to_string(),
            settle_date: settle_date.to_string(),
            blank_3: blank_3.to_string(),
        };

        Ok((i, descriptive))
    }

    pub fn validate(&self) -> Result<String, LineParseError> {
        let mut res: String = String::new();

        let _res = validate_generic_filler_str(
            self.record_type.clone(),
            "0".to_string(),
            ValidationType::DescriptiveRecordTypeZero,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_generic_filler_str(
            self.blank_1.clone(),
            BLANK_1.to_string(),
            ValidationType::DescriptiveBlankOne,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_generic_filler_str(
            self.reel_seq.clone(),
            "01".to_string(),
            ValidationType::DescriptiveReelSequence,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_user_name(self.bank_name.clone(), ValidationType::DescriptiveUsername)
            .map_err(|e| {
                res.write_str(&e.to_string()).unwrap();
            });

        let _res = validate_generic_filler_str(
            self.blank_2.clone(),
            BLANK_2.to_string(),
            ValidationType::DescriptiveBlankTwo,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_self_name(self.user_name.clone(), ValidationType::DescriptiveSelfName)
            .map_err(|e| {
                res.write_str(&e.to_string()).unwrap();
            });

        let _res = validate_apca_ident(self.apca_number.clone(), ValidationType::DescriptiveApca)
            .map_err(|e| {
                res.write_str(&e.to_string()).unwrap();
            });

        let _res = validate_file_entry(
            self.file_description.clone(),
            ValidationType::DescriptiveEntry,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_date(self.settle_date.clone(), ValidationType::DescriptiveDate)
            .map_err(|e| {
                res.write_str(&e.to_string()).unwrap();
            });

        let _res = validate_generic_filler_str(
            self.blank_3.clone(),
            BLANK_3.to_string(),
            ValidationType::DescriptiveBlankThree,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        Ok(res)
    }
}

impl Display for DescriptiveBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}",
            self.record_type,
            self.blank_1,
            self.reel_seq,
            self.bank_name,
            self.blank_2,
            self.user_name,
            self.apca_number,
            self.file_description,
            self.settle_date,
            self.blank_3
        )
    }
}

impl From<SettlementSettings> for DescriptiveBlock {
    fn from(desc_settings: SettlementSettings) -> Self {
        Self {
            record_type: "0".to_string(),
            blank_1: BLANK_1.to_string(),
            reel_seq: "01".to_string(),
            bank_name: desc_settings.bank_name,
            blank_2: BLANK_2.to_string(),
            user_name: left_adjust(&desc_settings.user_name, 26usize, FillStrategy::Blank),
            apca_number: right_adjust(&desc_settings.apca_number, 6usize, FillStrategy::Zero),
            file_description: left_adjust(
                &desc_settings.file_description,
                12usize,
                FillStrategy::Blank,
            ),
            settle_date: desc_settings.settle_date,
            blank_3: BLANK_3.to_string(),
        }
    }
}

#[test]
fn test_descriptive() {
    let desc: &str = "0                 01BQL       MY NAME                   1111111004231633  230410                                        ";
    let (_, result) = DescriptiveBlock::deserialise(&desc).unwrap();
    assert_eq!(result.blank_3, BLANK_3.to_string())
}

//! Validation utility function for various .aba data fields
use lazy_static::lazy_static;
use nom::character::is_digit;
use regex::Regex;
use std::ops::Sub;
use time::{format_description as Fd, Date as Td};

use crate::errors::*;
use crate::helper::*;
use crate::types::*;

/// Indicators for transaction types, rarely used in day to day banking unless required
const INDICATOR: [&str; 5] = ["N", "W", "X", "Y", " "];
/// Transaction codes, 53 is used most of the time unless otherwise required
const TRANS_CODE: [&str; 9] = ["13", "50", "51", "52", "53", "54", "55", "56", "57"];

lazy_static! {
    static ref RE_BLANK: Regex = Regex::new("^\\s*$").unwrap();
}

pub fn validate_generic_filler_str(
    i: String,
    pattern: String,
    fill_type: ValidationType,
) -> Result<(), LineParseError> {
    match fill_type {
        ValidationType::DescriptiveRecordTypeZero => {
            if !i.eq(&pattern) {
                Err(LineParseError::DescriptiveRecordTypeZero(i))
            } else {
                Ok(())
            }
        }
        ValidationType::DescriptiveBlankOne => {
            if !i.eq(&pattern) {
                Err(LineParseError::DescriptiveBlankOne)
            } else {
                Ok(())
            }
        }
        ValidationType::DescriptiveReelSequence => {
            if !i.eq(&pattern) {
                Err(LineParseError::DescriptiveReelSequence(i))
            } else {
                Ok(())
            }
        }
        ValidationType::DescriptiveBlankTwo => {
            if !i.eq(&pattern) {
                Err(LineParseError::DescriptiveBlankTwo)
            } else {
                Ok(())
            }
        }
        ValidationType::DescriptiveBlankThree => {
            if !i.eq(&pattern) {
                Err(LineParseError::DescriptiveBlankThree)
            } else {
                Ok(())
            }
        }
        ValidationType::TotalTypeSeven => {
            if !i.eq(&pattern) {
                Err(LineParseError::TotalTypeSeven(i))
            } else {
                Ok(())
            }
        }
        ValidationType::TotalBsbFiller => {
            if !i.eq(&pattern) {
                Err(LineParseError::TotalBsbFiller(i))
            } else {
                Ok(())
            }
        }
        ValidationType::TotalBlankOne => {
            if !i.eq(&pattern) {
                Err(LineParseError::TotalBlankOne)
            } else {
                Ok(())
            }
        }
        ValidationType::TotalBlankTwo => {
            if !i.eq(&pattern) {
                Err(LineParseError::TotalBlankTwo)
            } else {
                Ok(())
            }
        }
        ValidationType::TotalBlankThree => {
            if !i.eq(&pattern) {
                Err(LineParseError::TotalBlankThree)
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_filler_str_line_counted(
    i: String,
    pattern: String,
    fill_type: ValidationType,
    line_count: &u32,
) -> Result<(), LineParseError> {
    match fill_type {
        ValidationType::DetailRecordTypeOne => {
            if !i.eq(&pattern) {
                Err(LineParseError::DetailRecordTypeOne(*line_count, i))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_user_name(i: String, fill_type: ValidationType) -> Result<(), LineParseError> {
    let file = include_str!("data/institution");

    match fill_type {
        ValidationType::DescriptiveUsername => {
            if !file.contains(&i) {
                Err(LineParseError::DescriptiveUserName)
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_self_name(i: String, fill_type: ValidationType) -> Result<(), LineParseError> {
    match fill_type {
        ValidationType::DescriptiveSelfName => {
            if validate_blank(&i) || i.starts_with(' ') {
                Err(LineParseError::DescriptiveSelfName)
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_apca_ident(i: String, fill_type: ValidationType) -> Result<(), LineParseError> {
    let i = i.trim_start_matches(' ');
    match fill_type {
        ValidationType::DescriptiveApca => {
            if !validate_number(i) || i.ends_with(' ') {
                Err(LineParseError::DescriptiveApca)
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_file_entry(i: String, fill_type: ValidationType) -> Result<(), LineParseError> {
    match fill_type {
        ValidationType::DescriptiveEntry => {
            if validate_blank(&i) || i.starts_with(' ') {
                Err(LineParseError::DescriptiveFileEntry)
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_date(i: String, fill_type: ValidationType) -> Result<(), LineParseError> {
    match fill_type {
        ValidationType::DescriptiveDate => {
            if !validate_date_format(&i) {
                Err(LineParseError::DescriptiveDate)
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_bsb_client(
    i: String,
    fill_type: BsbType,
    line_count: &u32,
) -> Result<(), LineParseError> {
    let (_, bsb) = Bsb::deserialise(&i).unwrap();
    let file = include_str!("data/bsb");

    match fill_type {
        BsbType::DetailBsb => {
            if !file.contains(&bsb.to_string()) {
                Err(LineParseError::DetailBsbClient(
                    *line_count,
                    bsb.to_string(),
                ))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_dest_acct(
    i: String,
    fill_type: ValidationType,
    line_count: &u32,
) -> Result<(), LineParseError> {
    let i = i.trim_start_matches(' ');
    match fill_type {
        ValidationType::DetailDestAccount => {
            if !validate_number(i) || i.ends_with(' ') {
                Err(LineParseError::DetailDestAccount(*line_count))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_indicator(
    i: String,
    fill_type: ValidationType,
    line_count: &u32,
) -> Result<(), LineParseError> {
    match fill_type {
        ValidationType::DetailIndicator => {
            if !INDICATOR.contains(&&*i) {
                Err(LineParseError::DetailIndicator(*line_count))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_trans_code(
    i: String,
    fill_type: ValidationType,
    line_count: &u32,
) -> Result<(), LineParseError> {
    match fill_type {
        ValidationType::DetailTransCode => {
            if !TRANS_CODE.contains(&&*i) {
                Err(LineParseError::DetailTransCode(*line_count))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_amount(
    i: String,
    fill_type: ValidationType,
    line_count: &u32,
) -> Result<(), LineParseError> {
    let i = i.trim_start_matches(' ');
    match fill_type {
        ValidationType::DetailAmount => {
            if !validate_number(i) || i.ends_with(' ') {
                Err(LineParseError::DetailAmount(*line_count))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_client_name(
    i: String,
    fill_type: ValidationType,
    line_count: &u32,
) -> Result<(), LineParseError> {
    match fill_type {
        ValidationType::DetailClientName => {
            if i.starts_with(' ') {
                Err(LineParseError::DetailClientName(*line_count))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_lodge_ref(
    i: String,
    fill_type: ValidationType,
    line_count: &u32,
) -> Result<(), LineParseError> {
    match fill_type {
        ValidationType::DetailLodgeRef => {
            if i.starts_with('0') || i.starts_with('-') {
                Err(LineParseError::DetailLodgeRef(*line_count))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_bsb_trace(
    i: String,
    fill_type: BsbType,
    line_count: &u32,
) -> Result<(), LineParseError> {
    let (_, bsb) = Bsb::deserialise(&i).unwrap();
    let file = include_str!("data/bsb");

    match fill_type {
        BsbType::DetailTraceBsb => {
            if !file.contains(&bsb.to_string()) {
                Err(LineParseError::DetailBsbTrace(*line_count, bsb.to_string()))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_src_acct(
    i: String,
    fill_type: ValidationType,
    line_count: &u32,
) -> Result<(), LineParseError> {
    let i = i.trim_start_matches(' ');
    match fill_type {
        ValidationType::DetailSrcAccount => {
            if !validate_number(i) || i.ends_with(' ') {
                Err(LineParseError::DetailSrcAccount(*line_count))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_remitter(
    i: String,
    fill_type: ValidationType,
    line_count: &u32,
) -> Result<(), LineParseError> {
    match fill_type {
        ValidationType::DetailRemitter => {
            if validate_blank(&i) || i.starts_with(' ') {
                Err(LineParseError::DetailRemitter(*line_count))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_tax_withhold(
    i: String,
    fill_type: ValidationType,
    line_count: &u32,
) -> Result<(), LineParseError> {
    match fill_type {
        ValidationType::DetailTaxWithhold => {
            if !validate_number(&i) || i.ends_with(' ') {
                Err(LineParseError::DetailTaxWithhold(*line_count))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

pub fn validate_total_field(i: String, amount_type: TotalAmountType) -> Result<(), LineParseError> {
    let (_, total_field) = TotalField::deserialise(&i).unwrap();

    match amount_type {
        TotalAmountType::Credit => {
            if !validate_number(&total_field.credit) {
                Err(LineParseError::TotalCredit)
            } else {
                Ok(())
            }
        }
        TotalAmountType::Debit => {
            if !validate_number(&total_field.debit) {
                Err(LineParseError::TotalDebit)
            } else {
                Ok(())
            }
        }
        TotalAmountType::Total => {
            if !validate_number(&total_field.total) {
                Err(LineParseError::TotalField)
            } else if !validate_number(&total_field.credit) || !validate_number(&total_field.debit)
            {
                Err(LineParseError::TotalMalformedCreditDebit)
            } else if !validate_trim_then_compute(
                &total_field.credit,
                &total_field.debit,
                &total_field.total,
            ) {
                Err(LineParseError::TotalCompute)
            } else {
                Ok(())
            }
        }
    }
}

pub fn validate_record_type_count(i: String, line_count: &u32) -> Result<(), LineParseError> {
    match validate_number(&i) {
        true => {
            let record_count = i.trim_start_matches('0').parse::<u32>().unwrap();
            if !record_count.eq(line_count) {
                Err(LineParseError::TotalCount(record_count, *line_count))
            } else {
                Ok(())
            }
        }
        false => Err(LineParseError::TotalNonNumeric),
    }
}

pub fn validate_trim_then_compute(credit: &str, debit: &str, total: &str) -> bool {
    let credit = validate_nonzero_str(credit);
    let debit = validate_nonzero_str(debit);
    let total = validate_nonzero_str(total);

    credit.sub(debit).eq(&total)
}

pub fn validate_nonzero_str(i: &str) -> u32 {
    match i.trim_start_matches('0').eq("") {
        true => 0u32,
        false => return i.trim_start_matches('0').parse::<u32>().unwrap(),
    }
}

pub fn validate_date_format(i: &str) -> bool {
    let (_, date) = TwoDigitYears::deserialise(i).unwrap();
    let format = Fd::parse("[day][month][year]").unwrap();

    Td::parse(&date, &format).is_ok()
}

pub fn validate_number(i: &str) -> bool {
    for byte in i.as_bytes() {
        if !is_digit(*byte) {
            return false;
        }
    }
    true
}

pub fn validate_blank(i: &str) -> bool {
    RE_BLANK.is_match(i)
}

#[test]
fn test_validate_trim_then_compute() {
    assert_eq!(validate_trim_then_compute("00032", "00000", "00031"), false)
}

#[test]
fn test_validate_non_zero_str() {
    let i: &str = "00000";
    assert_eq!(validate_nonzero_str(i), 0u32)
}

#[test]
fn test_validate_date_format() {
    let ddmmyy: &str = "300220";
    assert_eq!(validate_date_format(ddmmyy), false)
}

#[test]
fn test_validate_number() {
    let i: &str = "abcde";
    assert_eq!(validate_number(i), false)
}

#[test]
fn test_validate_blank() {
    let i: &str = "oolala";
    assert_eq!(validate_blank(i), false)
}

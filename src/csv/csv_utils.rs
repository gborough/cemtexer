#![allow(clippy::single_char_pattern)]

//! Validation utility functions for csv files
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer};
use std::ops::Sub;

use crate::parser_utils::*;
use crate::types::*;

lazy_static! {
    static ref RE_AMOUNT: Regex = Regex::new(r"^[[:digit:]]{1,8}\.[[:digit:]]{2}$").unwrap();
}

/// Custom deserialisation function for the comment field, blank filled
pub fn optional_comment<'de, D>(de: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
    Option<String>: Deserialize<'de>,
{
    match Option::<String>::deserialize(de) {
        Ok(Some(comment)) => Ok(Some(comment)),
        Ok(None) => Ok(Some("                  ".to_string())),
        _ => unreachable!(),
    }
}

/// Custom deserialisation function for the tax withold field, zero filled
pub fn optional_tax_withhold<'de, D>(de: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
    Option<String>: Deserialize<'de>,
{
    match Option::<String>::deserialize(de) {
        Ok(Some(comment)) => Ok(Some(comment)),
        Ok(None) => Ok(Some("00000000".to_string())),
        _ => unreachable!(),
    }
}

/// Left adjust any field as mandated by .aba format
pub fn left_adjust(i: &str, size: usize, strat: FillStrategy) -> String {
    let mut res: String = String::new();

    match strat {
        FillStrategy::Zero => {
            if i.len().lt(&size) {
                let fill = zero_fill(i, size);
                res.push_str(i);
                res.push_str(&fill);
                res
            } else {
                i.to_string()
            }
        }
        FillStrategy::Blank => {
            if i.len().lt(&size) {
                let fill = blank_fill(i, size);
                res.push_str(i);
                res.push_str(&fill);
                res
            } else {
                i.to_string()
            }
        }
    }
}

/// Right adjust any field as mandated by .aba format
pub fn right_adjust(i: &str, size: usize, strat: FillStrategy) -> String {
    let mut res: String = String::new();

    match strat {
        FillStrategy::Zero => {
            if i.len().lt(&size) {
                let fill = zero_fill(i, size);
                res.push_str(&fill);
                res.push_str(i);
                res
            } else {
                i.to_string()
            }
        }
        FillStrategy::Blank => {
            if i.len().lt(&size) {
                let fill = blank_fill(i, size);
                res.push_str(&fill);
                res.push_str(i);
                res
            } else {
                i.to_string()
            }
        }
    }
}

fn zero_fill(i: &str, size: usize) -> String {
    let remaining = size.sub(i.len());
    "0".repeat(remaining)
}

fn blank_fill(i: &str, size: usize) -> String {
    let remaining = size.sub(i.len());
    " ".repeat(remaining)
}

pub fn normalise_amount(i: &str) -> String {
    if i.contains(".") && RE_AMOUNT.is_match(&i) {
        i.replace(".", "")
    } else {
        i.to_owned()
    }
}

pub fn validate_csv_bank_name(i: &str, res: &mut Vec<&str>) -> bool {
    if i.is_empty() || i.len().gt(&3usize) {
        res.push(
            "Bank name field must be in the excat format of 3 upppercase characters",
        );
        return false;
    }

    let file = include_str!("../data/institution");
    if !file.contains(i) {
        res.push("Bank name field is not valid");
        return false;
    }
    true
}

pub fn validate_csv_user_name(i: &str, res: &mut Vec<&str>) -> bool {
    if i.is_empty() || i.len().gt(&26usize) {
        res.push("User name field must not be empty and exceed 26 characters");
        return false;
    }
    true
}

pub fn validate_csv_apca_number(i: &str, res: &mut Vec<&str>) -> bool {
    if i.len().gt(&6usize) || !validate_number(i) {
        res.push("APCA code field must be in the excat format of 6 digits");
        return false;
    }
    true
}

pub fn validate_csv_file_description(i: &str, res: &mut Vec<&str>) -> bool {
    if i.is_empty() || i.len().gt(&12usize) {
        res.push("File description field must not be empty and exceed 12 characters");
        return false;
    }
    true
}

pub fn validate_csv_settle_date(i: &str, res: &mut Vec<&str>) -> bool {
    if i.is_empty() || i.len().gt(&6usize) || !validate_date_format(i) {
        res.push("Settlement date field must be in the exact format of DDMMYY");
        return false;
    }
    true
}

pub fn validate_bsb(i: &str, res: &mut Vec<&str>, bsb_type: BsbType) -> bool {
    if i.is_empty() || !i.len().eq(&7usize) {
        match bsb_type {
            BsbType::DetailBsb => res
                .push("BSB field must be in the format of xxx-xxx where x are digits"),
            BsbType::DetailTraceBsb => res.push(
                "Trace BSB code field must be in the excat format of xxx-xxx where x are digits",
            ),
        }
        return false;
    }

    let file = include_str!("../data/bsb");
    if !file.contains(i) {
        res.push("BSB code is not valid");
        return false;
    }
    true
}

pub fn validate_account_number(i: &str, res: &mut Vec<&str>, bsb_type: BsbType) -> bool {
    if i.is_empty() || i.len().gt(&9usize) || !validate_number(i.trim_start_matches(' ')) {
        match bsb_type {
            BsbType::DetailBsb => {
                res.push("Account number field must not be empty and exceed 9 digits")
            }
            BsbType::DetailTraceBsb => res.push(
                "Trace account number field must not be empty and exceed 9 digits"),
        }
        return false;
    }
    true
}

pub fn validate_csv_trace_account_name(i: &str, res: &mut Vec<&str>) -> bool {
    if i.is_empty() || i.len().gt(&16usize) {
        res.push("Trace ccount name field must not be empty and exceed 12 characters");
        return false;
    }
    true
}

pub fn validate_csv_client_name(i: &str, res: &mut Vec<&str>) -> bool {
    if i.is_empty() || i.len().gt(&32usize) {
        res.push("Account name field must not be empty and exceed 32 characters");
        return false;
    }
    true
}

pub fn validate_csv_amount(i: &str, res: &mut Vec<&str>) -> bool {
    if i.is_empty()
        || i.replace(".", "").len().gt(&10usize)
        || !validate_number(&i.replace(".", ""))
    {
        res.push(
            "Amount field must be digits and must not be empty and exceed 10 digits");
        false
    } else if i.contains(".") {
        match RE_AMOUNT.is_match(i) {
            true => true,
            false => {
                res.push("Amount field must be in valid two deicmal format");
                false
            }
        }
    } else {
        true
    }
}

pub fn validate_csv_comment(i: &str, res: &mut Vec<&str>) -> bool {
    if i.len().gt(&18usize) || i.starts_with('0') || i.starts_with('-') {
        res.push("Comment field must not exceed 18 characters or start with 0 and -");
        return false;
    }
    true
}

pub fn validate_csv_tax_withhold(i: &str, res: &mut Vec<&str>) -> bool {
    if i.replace(".", "").len().gt(&8usize) || !validate_number(&i.replace(".", "")) {
        res.push("Tax withold field must be digits and must not exceed 8 digits");
        false
    } else if i.contains(".") {
        match RE_AMOUNT.is_match(i) {
            true => true,
            false => {
                res.push("Tax withold field must be in valid two deicmal format");
                false
            }
        }
    } else {
        true
    }
}

#[test]
fn test_normalise_amount() {
    let i = "123.45";
    assert_eq!(normalise_amount(i), "12345");
}

#[test]
fn test_left_adjust() {
    let i = "      ";
    assert_eq!(left_adjust(i, 7usize, FillStrategy::Blank), "       ")
}

#[test]
fn test_right_adjust() {
    let i = "000000".to_string();
    assert_eq!(right_adjust(&i, 7usize, FillStrategy::Zero), "0000000")
}

#[test]
fn test_zero_fill() {
    let i = "1";
    assert_eq!(zero_fill(i, 7usize), "000000")
}

#[test]
fn test_blank_fill() {
    let i = "a";
    assert_eq!(blank_fill(i, 7usize), "      ")
}

#[test]
fn test_validate_csv_bank_name() {
    let blank: &str = "";
    let too_long: &str = "lolol";
    let non_exist: &str = "lol";
    let mut res: Vec<&str> = Vec::new();
    assert_eq!(validate_csv_bank_name(blank, &mut res), false);
    assert_eq!(validate_csv_bank_name(too_long, &mut res), false);
    assert_eq!(validate_csv_bank_name(non_exist, &mut res), false)
}

#[test]
fn test_validate_csv_user_name() {
    let blank: &str = "";
    let too_long: &str = "lololololololololololololol";
    let mut res: Vec<&str> = Vec::new();
    assert_eq!(validate_csv_user_name(blank, &mut res), false);
    assert_eq!(validate_csv_user_name(too_long, &mut res), false)
}

#[test]
fn test_validate_csv_apca_number() {
    let non_digits: &str = "lololo";
    let too_long: &str = "1234567";
    let mut res: Vec<&str> = Vec::new();
    assert_eq!(validate_csv_apca_number(non_digits, &mut res), false);
    assert_eq!(validate_csv_apca_number(too_long, &mut res), false)
}

#[test]
fn test_validate_csv_file_description() {
    let blank: &str = "";
    let too_long: &str = "lolololololol";
    let mut res: Vec<&str> = Vec::new();
    assert_eq!(validate_csv_file_description(blank, &mut res), false);
    assert_eq!(validate_csv_file_description(too_long, &mut res), false)
}

#[test]
fn test_validate_csv_settle_date() {
    let blank: &str = "";
    let too_long: &str = "1111111";
    let non_date: &str = "300220";
    let mut res: Vec<&str> = Vec::new();
    assert_eq!(validate_csv_settle_date(blank, &mut res), false);
    assert_eq!(validate_csv_settle_date(too_long, &mut res), false);
    assert_eq!(validate_csv_settle_date(non_date, &mut res), false)
}

#[test]
fn test_validate_csv_trace() {
    let blank: &str = "";
    let invalid_bsb: &str = "123-456";
    let too_longbsb: &str = "123-4567";
    let acct: &str = "1234567890";
    let bad_acct: &str = "lol";
    let too_long_trace: &str = "lolololololololol";
    let too_long_client: &str = "lolololololololololololololololol";
    let mut res: Vec<&str> = Vec::new();
    assert_eq!(validate_bsb(blank, &mut res, BsbType::DetailBsb), false);
    assert_eq!(
        validate_bsb(invalid_bsb, &mut res, BsbType::DetailBsb),
        false
    );
    assert_eq!(
        validate_bsb(too_longbsb, &mut res, BsbType::DetailBsb),
        false
    );
    assert_eq!(
        validate_account_number(blank, &mut res, BsbType::DetailBsb),
        false
    );
    assert_eq!(
        validate_account_number(acct, &mut res, BsbType::DetailBsb),
        false
    );
    assert_eq!(
        validate_account_number(bad_acct, &mut res, BsbType::DetailBsb),
        false
    );
    assert_eq!(validate_csv_trace_account_name(blank, &mut res), false);
    assert_eq!(
        validate_csv_trace_account_name(too_long_trace, &mut res),
        false
    );
    assert_eq!(validate_csv_client_name(blank, &mut res), false);
    assert_eq!(validate_csv_client_name(too_long_client, &mut res), false);
}

#[test]
fn test_validate_csv_amount() {
    let blank: &str = "";
    let too_long: &str = "11111111111";
    let non_digits: &str = "lol";
    let three_decimal: &str = "123.456";
    let mut res: Vec<&str> = Vec::new();
    assert_eq!(validate_csv_amount(blank, &mut res), false);
    assert_eq!(validate_csv_amount(too_long, &mut res), false);
    assert_eq!(validate_csv_amount(non_digits, &mut res), false);
    assert_eq!(validate_csv_amount(three_decimal, &mut res), false);
}

#[test]
fn test_validate_csv_comment() {
    let start_with_hyphen: &str = "-lol";
    let start_with_zero: &str = "0lol";
    let too_long: &str = "lololololololololol";
    let mut res: Vec<&str> = Vec::new();
    assert_eq!(validate_csv_comment(start_with_hyphen, &mut res), false);
    assert_eq!(validate_csv_comment(start_with_zero, &mut res), false);
    assert_eq!(validate_csv_comment(too_long, &mut res), false)
}

#[test]
fn test_validate_csv_tax_withhold() {
    let non_digits: &str = "lol";
    let three_decimal: &str = "123.456";
    let too_long: &str = "111111111";
    let mut res: Vec<&str> = Vec::new();
    assert_eq!(validate_csv_tax_withhold(non_digits, &mut res), false);
    assert_eq!(validate_csv_tax_withhold(three_decimal, &mut res), false);
    assert_eq!(validate_csv_tax_withhold(too_long, &mut res), false)
}

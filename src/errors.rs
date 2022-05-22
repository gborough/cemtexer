//! Error types used for .aba file validation
use std::fmt::Display;
use thiserror::Error;

/// Pre-filled error messages for line counts and data fields which are fixed in the blocks structures
#[non_exhaustive]
#[derive(Error, Debug, Clone)]
pub enum LineParseError {
    #[error(
        "- At line 1 in the descriptive block at character position 1, it must be 0 not {0}\n"
    )]
    DescriptiveRecordTypeZero(String),
    #[error("- At line 1 in the descriptive block between character position 2 - 18, all must be 17 blanks\n")]
    DescriptiveBlankOne,
    #[error("- At line 1 in the descriptive block between character position 19 - 20, all must be 01 not {0}\n")]
    DescriptiveReelSequence(String),
    #[error("- At line 1 in the descriptive block between character position 21 - 23, it must contain valid 3 character bank code\n")]
    DescriptiveUserName,
    #[error("- At line 1 in the descriptive block between character position 24 - 30, all must be 7 blanks\n")]
    DescriptiveBlankTwo,
    #[error("- At line 1 in the descriptive block between character position 31 - 56, it must be left adjusted and must not be all blank\n")]
    DescriptiveSelfName,
    #[error("- At line 1 in the descriptive block between character position 57 - 62, all must be numerics\n")]
    DescriptiveApca,
    #[error("- At line 1 in the descriptive block between character position 63 - 74, it must be left adjusted and must not be all blank\n")]
    DescriptiveFileEntry,
    #[error("- At line 1 in the descriptive block between character position 75 - 80, the date format must be in DDMMYY\n")]
    DescriptiveDate,
    #[error("- At line 1 in the descriptive block between character position 81 - 120, all must be 40 blanks\n")]
    DescriptiveBlankThree,
    #[error("- At line {0} in the detail block at character position 1, it must be 1 not {1}\n")]
    DetailRecordTypeOne(u32, String),
    #[error("- At line {0} in the detail block between character position 2 - 7, it must contain valid BSB number, but you have {1}\n")]
    DetailBsbClient(u32, String),
    #[error("- At line {0} in the detail block between character position 9 - 17, all must be numerics and right adjusted\n")]
    DetailDestAccount(u32),
    #[error("- At line {0} in the detail block at character position 18, the indicator must be one of N, W, X, Y, or blank\n")]
    DetailIndicator(u32),
    #[error("- At line {0} in the detail block between character position 19 - 20, the transaction code must be one of 13, 50, 51, 52, 53, 54, 55, 56, 57\n")]
    DetailTransCode(u32),
    #[error("- At line {0} in the detail block between character position 21 -30, the amount must be right justified\n")]
    DetailAmount(u32),
    #[error("- At line {0} in the detail block between character position 31 -62, the name must be left justified\n")]
    DetailClientName(u32),
    #[error("- At line {0} in the detail block between character position 63 - 80, the lodge reference must be left justifed and must not start zeros and hyphens\n")]
    DetailLodgeRef(u32),
    #[error("- At line {0} in the detail block between character position 81 - 87, it must contain valid BSB number, but you have {1}\n")]
    DetailBsbTrace(u32, String),
    #[error("- At line {0} in the detail block between character position 88 - 96, the account number must be right justified\n")]
    DetailSrcAccount(u32),
    #[error("- At line {0} in the detail block between character position 97 - 112, the name must be left justified and must not be all blank\n")]
    DetailRemitter(u32),
    #[error("- At line {0} in the detail block between character position 113 - 120, the amount must be right justified\n")]
    DetailTaxWithhold(u32),
    #[error(
        "- At the last line in the total block at character position 1, it must be 7 not {0}\n"
    )]
    TotalTypeSeven(String),
    #[error("- At the last line in the total block between character position 2 - 8, it must be 999-999 not {0}\n")]
    TotalBsbFiller(String),
    #[error("- At the last line in the total block between character position 9 - 20, all must be 12 blanks\n")]
    TotalBlankOne,
    #[error("- At the last line in the total block between character position 21 - 30, all must be numerics\n")]
    TotalField,
    #[error("- At the last line in the total block between character position 21 - 30, total is not equal to credit - debit\n")]
    TotalCompute,
    #[error("- At the last line in the total block between character position 21 - 30, the credit/debit field failed validation, see next error(s)\n")]
    TotalMalformedCreditDebit,
    #[error("- At the last line in the total block between character position 31 - 40, all must be numerics\n")]
    TotalCredit,
    #[error("- At the last line in the total block between character position 41 - 50, all must be numerics\n")]
    TotalDebit,
    #[error("- At the last line in the total block between character position 51 - 74, all must be 24 blanks\n")]
    TotalBlankTwo,
    #[error("- At the last line in the total block between character position 75 - 80, all must be numerics\n")]
    TotalNonNumeric,
    #[error("- At the last line in the total block between character position 75 - 80, the record count {0} is not equal to the detail line count {1}\n")]
    TotalCount(u32, u32),
    #[error("- At the last line in the total block between character position 81 - 120, all must be 40 blanks\n")]
    TotalBlankThree,
}

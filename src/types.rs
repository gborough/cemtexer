//! Various dispatch helpers for data validations that share common functions

/// Dispatch types for .aba file validation that have one-to-one mapping
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ValidationType {
    DescriptiveRecordTypeZero,
    DescriptiveBlankOne,
    DescriptiveReelSequence,
    DescriptiveUsername,
    DescriptiveSelfName,
    DescriptiveApca,
    DescriptiveEntry,
    DescriptiveBlankTwo,
    DescriptiveBlankThree,
    DescriptiveDate,
    DetailRecordTypeOne,
    DetailDestAccount,
    DetailIndicator,
    DetailTransCode,
    DetailAmount,
    DetailClientName,
    DetailLodgeRef,
    DetailSrcAccount,
    DetailRemitter,
    DetailTaxWithhold,
    TotalTypeSeven,
    TotalBsbFiller,
    TotalBlankOne,
    TotalBlankTwo,
    TotalBlankThree,
}

/// Dispatch types for left/right adjustment functions
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FillStrategy {
    Zero,
    Blank,
}

/// Dispatch types for BSB number validation
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BsbType {
    DetailBsb,
    DetailTraceBsb,
}

/// Dispatch types for the Total-Credit-Debit triplet validation in the total block
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TotalAmountType {
    Total,
    Credit,
    Debit,
}

//! Detail block starts from the second line to the second last line of a valid .aba file
use nom::bytes::complete::take;
use nom::IResult;
use std::{fmt::Display, fmt::Write};

use crate::csv::*;
use crate::errors::*;
use crate::parser_utils::*;
use crate::types::*;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct DetailBlock {
    pub record_type: String,
    pub bsb_number: String,
    pub dest_acct: String,
    pub indicator: String,
    pub trans_code: String,
    pub amount: String,
    pub client_name: String,
    pub lodge_ref: String,
    pub trace_bsb: String,
    pub src_acct: String,
    pub account_name: String,
    pub tax_withhold: String,
}

impl DetailBlock {
    pub fn deserialise(i: &str) -> IResult<&str, Self> {
        let (i, record_type) = take(1u8)(i)?;
        let (i, bsb_number) = take(7u8)(i)?;
        let (i, dest_acct) = take(9u8)(i)?;
        let (i, indicator) = take(1u8)(i)?;
        let (i, trans_code) = take(2u8)(i)?;
        let (i, amount) = take(10u8)(i)?;
        let (i, client_name) = take(32u8)(i)?;
        let (i, lodge_ref) = take(18u8)(i)?;
        let (i, trace_bsb) = take(7u8)(i)?;
        let (i, src_acct) = take(9u8)(i)?;
        let (i, account_name) = take(16u8)(i)?;
        let (i, tax_withhold) = take(8u8)(i)?;

        let detail = Self {
            record_type: record_type.to_owned(),
            bsb_number: bsb_number.to_owned(),
            dest_acct: dest_acct.to_owned(),
            indicator: indicator.to_owned(),
            trans_code: trans_code.to_owned(),
            amount: amount.to_owned(),
            client_name: client_name.to_owned(),
            lodge_ref: lodge_ref.to_owned(),
            trace_bsb: trace_bsb.to_owned(),
            src_acct: src_acct.to_owned(),
            account_name: account_name.to_owned(),
            tax_withhold: tax_withhold.to_owned(),
        };

        Ok((i, detail))
    }

    pub async fn validate(&self, line_count: &u32) -> Result<String, LineParseError> {
        let mut res: String = String::new();

        let _res = validate_filler_str_line_counted(
            self.record_type.clone(),
            "1".to_owned(),
            ValidationType::DetailRecordTypeOne,
            line_count,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_bsb_client(self.bsb_number.clone(), BsbType::DetailBsb, line_count)
            .map_err(|e| {
                res.write_str(&e.to_string()).unwrap();
            });

        let _res = validate_dest_acct(
            self.dest_acct.clone(),
            ValidationType::DetailDestAccount,
            line_count,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_indicator(
            self.indicator.clone(),
            ValidationType::DetailIndicator,
            line_count,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_trans_code(
            self.trans_code.clone(),
            ValidationType::DetailTransCode,
            line_count,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_amount(
            self.amount.clone(),
            ValidationType::DetailAmount,
            line_count,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_client_name(
            self.client_name.clone(),
            ValidationType::DetailClientName,
            line_count,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_lodge_ref(
            self.lodge_ref.clone(),
            ValidationType::DetailLodgeRef,
            line_count,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_bsb_trace(self.trace_bsb.clone(), BsbType::DetailTraceBsb, line_count)
            .map_err(|e| {
                res.write_str(&e.to_string()).unwrap();
            });

        let _res = validate_src_acct(
            self.src_acct.clone(),
            ValidationType::DetailSrcAccount,
            line_count,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_remitter(
            self.account_name.clone(),
            ValidationType::DetailRemitter,
            line_count,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        let _res = validate_tax_withhold(
            self.tax_withhold.clone(),
            ValidationType::DetailTaxWithhold,
            line_count,
        )
        .map_err(|e| {
            res.write_str(&e.to_string()).unwrap();
        });

        Ok(res)
    }
}

impl Display for DetailBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}{}{}",
            self.record_type,
            self.bsb_number,
            self.dest_acct,
            self.indicator,
            self.trans_code,
            self.amount,
            self.client_name,
            self.lodge_ref,
            self.trace_bsb,
            self.src_acct,
            self.account_name,
            self.tax_withhold
        )
    }
}

impl From<RecordWithConf> for DetailBlock {
    fn from(rec_conf: RecordWithConf) -> Self {
        Self {
            record_type: "1".to_owned(),
            bsb_number: rec_conf.rec.bsb,
            dest_acct: right_adjust(&rec_conf.rec.account_number, 9usize, FillStrategy::Blank),
            indicator: " ".to_owned(),
            trans_code: "53".to_owned(),
            amount: right_adjust(&rec_conf.rec.amount, 10usize, FillStrategy::Zero),
            client_name: left_adjust(&rec_conf.rec.client_name, 32usize, FillStrategy::Blank),
            lodge_ref: left_adjust(&rec_conf.rec.comment, 18usize, FillStrategy::Blank),
            trace_bsb: rec_conf.conf.trace_bsb,
            src_acct: right_adjust(
                &rec_conf.conf.trace_account_number,
                9usize,
                FillStrategy::Blank,
            ),
            account_name: left_adjust(
                &rec_conf.conf.trace_account_name,
                16usize,
                FillStrategy::Blank,
            ),
            tax_withhold: right_adjust(&rec_conf.rec.tax_withhold, 8usize, FillStrategy::Zero),
        }
    }
}

#[test]
fn test_detail_deser() {
    let detail: &'static str = "1123-456157108231 530000001234S R SMITH                       TEST BATCH        062-000 12223123MY ACCOUNT      00001200";
    let (_, result) = DetailBlock::deserialise(&detail).unwrap();
    assert_eq!(result.bsb_number, "123-456")
}

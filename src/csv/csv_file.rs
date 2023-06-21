//! Various structs to represent csv compliant format
use config::{Config, File as ConfFile};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::{
    collections::HashMap, convert::AsRef, error::Error, ffi::OsStr, fmt::Display, path::Path,
    process::exit,
};

use crate::csv::*;
use crate::types::*;

/// Actual csv file struct used for deserialisation.
/// For the purpose of self integration the format is made rigid
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Deserialize)]
pub struct CsvRecord {
    pub bsb: String,
    pub account_number: String,
    pub client_name: String,
    pub amount: String,
    #[serde(deserialize_with = "optional_comment")]
    pub comment: Option<String>,
    #[serde(deserialize_with = "optional_tax_withhold")]
    pub tax_withhold: Option<String>,
}

impl CsvRecord {
    pub fn read(path: impl AsRef<Path>) -> Result<Vec<CsvRecord>, Box<dyn Error>> {
        let mut rdr = match ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(path)
        {
            Ok(rdr) => rdr,
            Err(_) => {
                println!("Unable to open the csv file. Program Aborted");
                exit(1);
            }
        };

        let mut col: Vec<CsvRecord> = Vec::new();

        for res in rdr.deserialize() {
            let rec: CsvRecord = match res {
                Ok(rec) => rec,
                Err(_) => {
                    print!("Invalid csv file format, most likely a missing comma to denote a field, please refer to self integration guide. Program aborted");
                    exit(1);
                }
            };
            col.push(rec);
        }

        Ok(col)
    }
}

/// Template file struct used in self integration
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct SettlementSettings {
    pub bank_name: String,
    pub user_name: String,
    pub apca_number: String,
    pub file_description: String,
    pub settle_date: String,
    pub trace_bsb: String,
    pub trace_account_number: String,
    pub trace_account_name: String,
}

impl SettlementSettings {
    pub async fn new(path: impl AsRef<Path> + AsRef<OsStr>) -> Self {
        let settings = match Config::builder()
            .add_source(ConfFile::from(Path::new(&path)))
            .build()
        {
            Ok(settings) => settings,
            Err(_) => {
                println!("Unable to open the settings file. Program Aborted");
                exit(1);
            }
        };

        let settings = settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap();

        let bank_name = match settings.get("bank_name") {
            Some(bank_name) => bank_name.trim(),
            None => {
                println!("Cannot find value key: user_name...most likely you have accidentally modified the key name, please fix the keyname or regenerate the template and try again");
                exit(1)
            }
        };

        let user_name = match settings.get("user_name") {
            Some(user_name) => user_name.trim(),
            None => {
                println!("Cannot find value key: user_name...most likely you have accidentally modified the key name, please fix the keyname or regenerate the template and try again");
                exit(1)
            }
        };

        let apca_number = match settings.get("apca_number") {
            Some(apca_number) => apca_number.trim(),
            None => {
                println!("Cannot find value key: apca_number...most likely you have accidentally modified the key name, please fix the keyname or regenerate the template and try again");
                exit(1)
            }
        };

        let file_description = match settings.get("file_description") {
            Some(file_description) => file_description.trim(),
            None => {
                println!("Cannot find value key: file_description...most likely you have accidentally modified the key name, please fix the keyname or regenerate the template and try again");
                exit(1)
            }
        };

        let settle_date = match settings.get("settle_date") {
            Some(settle_date) => settle_date.trim(),
            None => {
                println!("Cannot find value key: settle_date...most likely you have accidentally modified the key name, please fix the keyname or regenerate the template and try again");
                exit(1)
            }
        };

        let trace_bsb = match settings.get("trace_bsb") {
            Some(trace_bsb) => trace_bsb.trim(),
            None => {
                println!("Cannot find value key: trace_bsb...most likely you have accidentally modified the key name, please fix the keyname or regenerate the template and try again");
                exit(1)
            }
        };

        let trace_account_number = match settings.get("trace_account_number") {
            Some(trace_account_number) => trace_account_number.trim(),
            None => {
                println!("Cannot find value key: trace_account_number...most likely you have accidentally modified the key name, please fix the keyname or regenerate the template and try again");
                exit(1)
            }
        };

        let trace_account_name = match settings.get("trace_account_name") {
            Some(trace_account_name) => trace_account_name.trim(),
            None => {
                println!("Cannot find value key: trace_account_name...most likely you have accidentally modified the key name, please fix the keyname or regenerate the template and try again");
                exit(1)
            }
        };

        Self {
            bank_name: bank_name.to_owned(),
            user_name: user_name.to_owned(),
            apca_number: apca_number.to_owned(),
            file_description: file_description.to_owned(),
            settle_date: settle_date.to_owned(),
            trace_bsb: trace_bsb.to_owned(),
            trace_account_number: trace_account_number.to_owned(),
            trace_account_name: trace_account_name.to_owned(),
        }
    }

    pub async fn validate(&self) {
        let mut res: Vec<&str> = Vec::new();

        let _ = validate_csv_bank_name(&self.bank_name, &mut res);
        let _ = validate_csv_user_name(&self.user_name, &mut res);
        let _ = validate_csv_apca_number(&self.apca_number, &mut res);
        let _ = validate_csv_file_description(&self.file_description, &mut res);
        let _ = validate_csv_settle_date(&self.settle_date, &mut res);
        let _ = validate_bsb(&self.trace_bsb, &mut res, BsbType::DetailTraceBsb);
        let _ = validate_account_number(
            &self.trace_account_number,
            &mut res,
            BsbType::DetailTraceBsb,
        );
        let _ = validate_csv_trace_account_name(&self.trace_account_name, &mut res);

        if !res.is_empty() {
            println!("The follow error(s) are detected in the template:");
            println!("{:?}\n Program Aborted", &res);
            exit(1)
        }
    }
}

impl Display for SettlementSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{}{}{}{}{}{}",
            self.bank_name,
            self.user_name,
            self.apca_number,
            self.file_description,
            self.settle_date,
            self.trace_bsb,
            self.trace_account_number,
            self.trace_account_name,
        )
    }
}

/// Flattened struct for csv data collected
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct RecordFlatten {
    pub bsb: String,
    pub account_number: String,
    pub client_name: String,
    pub amount: String,
    pub comment: String,
    pub tax_withhold: String,
}

impl RecordFlatten {
    pub async fn new(rec: &CsvRecord) -> Self {
        Self {
            bsb: rec.bsb.trim().to_owned(),
            account_number: rec.account_number.trim().to_owned(),
            client_name: rec.client_name.trim().to_owned(),
            amount: normalise_amount(rec.amount.trim().trim_start_matches('$')).to_owned(),
            comment: rec.comment.as_ref().unwrap().trim().to_owned(),
            tax_withhold: normalise_amount(
                rec.tax_withhold
                    .as_ref()
                    .unwrap()
                    .trim()
                    .trim_start_matches('$'),
            ).to_owned(),
        }
    }
}

/// Settlement settings and csv data are flushed to a new struct.
/// It is used for converting into descriptive and detail data blocks
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct RecordWithConf {
    pub rec: RecordFlatten,
    pub conf: SettlementSettings,
}

impl RecordWithConf {
    pub async fn new(csv_rec: &CsvRecord, conf: SettlementSettings) -> Self {
        Self {
            rec: RecordFlatten::new(csv_rec).await,
            conf,
        }
    }

    pub async fn validate(&self, line_count: &mut u32, err_count: &mut u32) {
        let mut res: Vec<&str> = Vec::new();

        let _ = validate_bsb(&self.rec.bsb, &mut res, BsbType::DetailBsb);
        let _ = validate_account_number(&self.rec.account_number, &mut res, BsbType::DetailBsb);
        let _ = validate_csv_client_name(&self.rec.client_name, &mut res);
        let _ = validate_csv_amount(&self.rec.amount, &mut res);
        let _ = validate_csv_comment(&self.rec.comment, &mut res);
        let _ = validate_csv_tax_withhold(&self.rec.tax_withhold, &mut res);

        if !res.is_empty() {
            *err_count += 1u32;
            println!(
                "The follow error(s) are detected at line {}: {:?}",
                line_count, &res
            );
        }
    }
}

/// Helper struct for calculation detail block line count and total amount
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct TotalRecord {
    pub line_count: String,
    pub total: String,
}

impl TotalRecord {
    pub async fn new(line_count: String, total: String) -> Self {
        Self {
            line_count: right_adjust(&line_count, 6, FillStrategy::Zero),
            total: right_adjust(&total, 10, FillStrategy::Zero),
        }
    }
}

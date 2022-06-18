use std::{error::Error, fmt::Write as fw, fs::File, io::Write, path::Path, process::exit};

use crate::blocks::*;
use crate::cemtex::*;
use crate::cli::*;
use crate::csv::*;

/// Subcommand to print out example template
pub fn print_example_template() -> std::io::Result<()> {
    let tpl = include_str!("../data/example");
    print!("{}", tpl);

    Ok(())
}

/// Subcommand to generate template to designated location
pub fn generate_template(path: impl AsRef<Path>) -> std::io::Result<()> {
    let mut buf = match File::create(path) {
        Ok(buf) => buf,
        Err(_) => {
            println!("Unable to create file at this location. Program aborted");
            exit(1);
        }
    };
    buf.write_all(include_str!("../data/example").as_bytes())?;

    Ok(())
}

/// Subcommand to generate .aba file to designated location
pub fn aba_gen(path: AbagenSub) -> std::io::Result<()> {
    let settle_setting = SettlementSettings::new(path.template);
    SettlementSettings::validate(&settle_setting);
    let desc_block = DescriptiveBlock::from(settle_setting.clone());

    let mut total = 0u32;
    let mut line_count = 0u32;

    let rec = CsvRecord::read(path.csv);
    let detailvec = read_settings(settle_setting, rec, &mut line_count, &mut total);

    let total_record = TotalRecord::new(line_count.to_string(), total.to_string());
    let total_block = TotalBlock::from(total_record);

    let mut buf = match File::create(&path.aba) {
        Ok(buf) => buf,
        Err(_) => {
            println!("Unable to create file at this location. Program aborted");
            exit(1);
        }
    };

    let mut aba = String::new();
    writeln!(&mut aba, "{}", desc_block).unwrap();
    for detail in detailvec {
        writeln!(&mut aba, "{}", detail).unwrap();
    }
    writeln!(&mut aba, "{}", total_block).unwrap();

    buf.write_all(aba.to_string().as_bytes()).unwrap();
    println!(".aba file succefully generate at location {}", &path.aba);

    Ok(())
}

/// Subcommand to validation existing .aba file
pub fn aba_check(path: AbacheckSub) -> std::io::Result<()> {
    println!("Checking file located at {}\n", path.aba);
    let aba = Cemtex::new(path.aba);
    let _ = Cemtex::validate(&aba, path.report);

    Ok(())
}

fn read_settings(
    settle_setting: SettlementSettings,
    rec: Result<Vec<CsvRecord>, Box<dyn Error>>,
    line_count: &mut u32,
    total: &mut u32,
) -> Vec<DetailBlock> {
    let mut err_count = 0u32;
    let mut rec_conf: Vec<RecordWithConf> = Vec::new();
    let mut detailvec: Vec<DetailBlock> = Vec::new();

    for line in rec.as_ref().unwrap() {
        *line_count += 1u32;
        let rec_conf_temp = RecordWithConf::new(line, settle_setting.clone());
        RecordWithConf::validate(&rec_conf_temp, line_count, &mut err_count);
        rec_conf.push(rec_conf_temp);
    }

    if err_count.gt(&0u32) {
        println!("CSV file validation failed. Program aborted");
        exit(1);
    } else {
        for recs in rec_conf.into_iter() {
            *total += recs
                .rec
                .amount
                .trim_start_matches('0')
                .parse::<u32>()
                .unwrap();

            let detail_block = DetailBlock::from(recs);
            detailvec.push(detail_block);
        }
    }

    detailvec
}

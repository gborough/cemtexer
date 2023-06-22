//! Various struct for reading .aba file format
use std::{fmt::Display, path::Path, process::exit};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};

use crate::blocks::*;
use crate::errors::*;

/// Cemtex representation
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Cemtex {
    pub inner: CemtexInner,
}

impl Cemtex {
    pub async fn new(path: impl AsRef<Path>) -> Cemtex {
        Self {
            inner: CemtexInner::new(path).await,
        }
    }

    pub async fn validate(&self, path: impl AsRef<Path> + Display) {
        let _ = CemtexInner::validate_inner(&self.inner, path).await;
    }
}

/// Inner presentation for Cemtex
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CemtexInner {
    pub descriptive: String,
    pub detail: Vec<String>,
    pub total: String,
    pub line_count: u32,
}

impl CemtexInner {
    pub async fn new(path: impl AsRef<Path>) -> CemtexInner {
        let file = match File::open(path).await {
            Ok(file) => file,
            Err(_) => {
                println!("Cannot locate file, program aborting");
                exit(1)
            }
        };
        let buf = BufReader::new(file);

        let mut line_count: u32 = 0u32;
        let mut err_count: u32 = 0u32;
        let mut err: Vec<String> = Vec::new();
        let entries = read_buf(buf, &mut line_count, &mut err_count, &mut err).await;

        if line_count.lt(&3u32) {
            println!("Sorry: The total number of line entries should be at least 3 line in order to form a valid .aba file, instead it has `{0}` lines. Program aborted", line_count);
            exit(1);
        }

        if err_count.gt(&0u32) {
            println!("{:?}", err);
            println!("Character count error. Program aborted");
            exit(1);
        }

        println!("This file format is valid, starting content validation.....\n");

        Self {
            descriptive: entries[0usize].clone(),
            detail: entries[1usize..=(line_count - 2u32) as usize].to_vec(),
            total: entries[(line_count - 1u32) as usize].clone(),
            line_count,
        }
    }

    pub async fn validate_inner(
        &self,
        path: impl AsRef<Path> + Display,
    ) -> Result<(), LineParseError> {
        let mut detail_line_count = 1u32;
        let mut error_count = 0u32;

        let mut buf = match File::create(&path).await {
            Ok(buf) => buf,
            Err(_) => {
                println!("Unable to create file at this location. Program aborted");
                exit(1);
            }
        };

        let (_, descriptive) = DescriptiveBlock::deserialise(&self.descriptive).unwrap();
        let desc_res = DescriptiveBlock::validate(&descriptive).await?;
        if !desc_res.is_empty() {
            error_count += 1;
            buf.write_all(desc_res.as_bytes()).await.unwrap();
        }

        for line in self.detail.iter() {
            detail_line_count += 1u32;
            let (_, detail_total) = DetailBlock::deserialise(line).unwrap();
            let detail_res = DetailBlock::validate(&detail_total, &detail_line_count).await?;
            if !detail_res.is_empty() {
                error_count += 1;
                buf.write_all(detail_res.as_bytes()).await.unwrap();
            }
        }

        let (_, block_total) = TotalBlock::deserialise(&self.total).unwrap();
        let total_res = TotalBlock::validate(&block_total, &(self.line_count - 2u32)).await?;
        if !total_res.is_empty() {
            error_count += 1;
            buf.write_all(total_res.as_bytes()).await.unwrap();
        }

        if error_count.eq(&0u32) {
            println!("File content validation successful!");
            buf.write_all(b"No errors detected").await.unwrap();
        } else {
            println!(
                "Some errors detected and a report is generated at location: {}",
                &path
            );
        }

        Ok(())
    }
}

async fn read_buf<'a>(
    buf: BufReader<File>,
    line_count: &mut u32,
    err_count: &mut u32,
    err: &mut Vec<String>,
) -> Vec<String> {
    let mut entries: Vec<String> = Vec::new();
    let mut lines = buf.lines();

    while let Some(line) = lines.next_line().await.unwrap() {
        *line_count += 1;

        if !line.len().eq(&120usize) {
            err.push(format!("at line {} the character count is {}, but in order to form a valid line it must be exactly 120 characters", line_count, line.len()));
            *err_count += 1;
        } else {
            entries.push(line);
        }
    }
    entries
}

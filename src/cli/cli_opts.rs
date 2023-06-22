//! Command line options and associated functions
use clap::{Args, Parser, Subcommand};

/// Command line options
#[derive(Parser)]
#[clap(author = "Author: Geoffrey Borough<Geoffrey.Borough@outlook.com>")]
#[clap(version)]
#[clap(about = "Utility to convert CSV file to Cemtex ABA file and validate Cemtex ABA file format", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

/// Subcommands
#[non_exhaustive]
#[derive(Subcommand, Clone)]
pub enum Commands {
    #[clap(about = "Print template, try run:\n \"cemtexer showtemplate\"")]
    Showtemplate,
    #[clap(
        about = "Generate template toml file, try run:\n \"cemtexer gentemplate /path/to/template\""
    )]
    Gentemplate { path: String },
    #[clap(
        about = "Generate Cemtex .aba file from .csv compliant file, try run:\n \"cemtexer abagen --template /path/to/template.toml --csv /path/to/somecsv.csv --aba /home/user/output.aba\"\nType: cemtex abagen -h for all options"
    )]
    Abagen {
        #[clap(flatten)]
        paths: AbagenSub,
    },
    #[clap(
        about = "Parse and validate Cemtex .aba file and generate report if erros detected, try run:\n \"cemtexer abacheck --aba /path/to/someaba.aba --report /path/to/report.txt\"\nType: cemtex abacheck -h for all options"
    )]
    Abacheck {
        #[clap(flatten)]
        path: AbacheckSub,
    },
}

/// Suboptions for Abagen command
#[derive(Args, Clone)]
pub struct AbagenSub {
    #[clap(long)]
    pub template: String,
    #[clap(long)]
    pub csv: String,
    #[clap(long)]
    pub aba: String,
}

/// Suboptions for Abacheck command
#[derive(Args, Clone)]
pub struct AbacheckSub {
    #[clap(long)]
    pub aba: String,
    #[clap(long)]
    pub report: String,
}

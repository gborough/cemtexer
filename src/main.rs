#![allow(unused_imports)]

mod blocks;
mod cemtex;
mod cli;
mod csv;
mod errors;
mod helper;
mod parser_utils;
mod types;

use clap::Parser;

use cli::*;

#[doc(hidden)]
fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Showtemplate => print_example_template()?,
        Commands::Gentemplate { path } => generate_template(path)?,
        Commands::Abagen { paths } => aba_gen(paths)?,
        Commands::Abacheck { path } => aba_check(path)?,
    }

    Ok(())
}

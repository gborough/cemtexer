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
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Showtemplate => print_example_template().await?,
        Commands::Gentemplate { path } => generate_template(path).await?,
        Commands::Abagen { paths } => aba_gen(paths).await?,
        Commands::Abacheck { path } => aba_check(path).await?,
    }

    Ok(())
}

use clap::{Parser, Subcommand};
use std::io;

mod convert;
mod filter;
mod pivot;
mod select;
mod sort;
mod transpose;
mod unique;
mod unpivot;

/// CSV Query utility
#[derive(Parser, Debug)]
#[clap(
    version,
    disable_help_flag(true),
    disable_version_flag(true),
    infer_subcommands(true)
)]
pub struct Arg {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    #[clap(alias("s"))]
    Select(select::Command),

    #[clap(visible_alias("filter"))]
    Where(filter::Command),

    Transpose(transpose::Command),

    #[clap(alias("S"))]
    Sort(sort::Command),

    Pivot(pivot::Command),

    #[clap(visible_alias("P"))]
    Unpivot(unpivot::Command),

    #[clap(alias("u"))]
    Unique(unique::Command),

    Convert(convert::Command),

    /// Print version information
    Version,
}

impl Arg {
    pub fn execute(&self) {
        match &self.action {
            Action::Convert(cmd) => cmd.execute(self),
            _ => eprintln!("not implemented yet"),
        }
    }

    pub fn read_stdin(&self, with_header: bool) -> csv::Reader<io::Stdin> {
        csv::ReaderBuilder::new()
            .has_headers(with_header)
            .from_reader(io::stdin())
    }
}

fn main() {
    Arg::parse().execute();
}

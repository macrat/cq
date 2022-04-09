/// Transpose a table
///
/// $ cat data.csv
/// foo,12
/// bar,34
/// baz,56
///
/// $ <data.csv cq transpose
/// foo,bar,baz
/// 12,34,56
#[derive(clap::Args, Debug)]
#[clap(verbatim_doc_comment)]
pub struct Command;

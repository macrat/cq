/// Get unique rows
///
/// $ cat data.csv
/// key,value
/// abc,123
/// def,456
/// abc,789
///
/// $ <data.csv cq unique "key"
/// key,value
/// abc,123
/// def,456
#[derive(clap::Args, Debug)]
#[clap(verbatim_doc_comment)]
pub struct Command {
    /// Columns to check if unique
    columns: Vec<String>,
}

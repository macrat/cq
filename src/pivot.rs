/// Pivot a table
///
/// $ cat data.csv
/// date,name,value
/// 2001-02-03,abc,123
/// 2001-02-03,def,456
/// 2002-03-04,abc,789
///
/// $ <data.csv cq pivot "name" "value"
/// date,abc,def
/// 2001-02-03,123,456
/// 2002-03-04,789,
#[derive(clap::Args, Debug)]
#[clap(verbatim_doc_comment)]
pub struct Command {
    /// Key column that become names of new column
    key_column: String,

    /// Value column that become values of new column
    value_column: String,
}

/// Un-pivot a table
///
/// $ cat data.csv
/// abc,def,ghi
/// 11,12,13
/// 21,22,23
///
/// $ <data.csv cq unpivot "abc"
/// abc,key,value
/// 11,def,12
/// 11,ghi,13
/// 12,def,22
/// 13,ghi,23
#[derive(clap::Args, Debug)]
#[clap(verbatim_doc_comment)]
pub struct Command {
    /// New column name to store column names of the input table
    #[clap(long, short, visible_alias("key"))]
    key_column: String,

    /// New column name to store values of the input table
    #[clap(long, short, visible_alias("value"))]
    value_column: String,

    /// Columns that keep as is
    keep_columns: Vec<String>,
}

/// Filter rows like WHERE of SQL
///
/// $ cat data.csv
/// name,class,score
/// alice,math,90
/// alice,geo,80
/// bob,math,70
///
/// $ <data.csv cq where 'class = "math"'
/// name,class,score
/// alice,math,90
/// bob,math,70
#[derive(clap::Args, Debug)]
#[clap(verbatim_doc_comment)]
pub struct Command {
    /// Query to check each rows
    query: String,
}

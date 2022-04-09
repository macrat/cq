/// Select columns like SELECT of SQL
///
/// $ cat data.csv
/// name,class,score
/// alice,math,90
/// alice,geo,80
/// bob,math,70
///
/// $ <data.csv cq select --group-by=name "name" "avg(score) AS average_score"
/// name,average_score
/// alice,85
/// bob,70
#[derive(clap::Args, Debug)]
#[clap(verbatim_doc_comment)]
pub struct Command {
    /// Key column to grouping rows like GROUP BY of SQL
    #[clap(long, short, value_name("COLUMN"))]
    group_by: Option<String>,

    /// Columns or Expressions to select
    #[clap(required(true))]
    columns: Vec<String>,
}

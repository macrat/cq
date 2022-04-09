/// Sort rows or columns
///
/// $ cat data.csv
/// name,score
/// alice,23
/// bob,12
/// charlie,45
///
/// $ <data.csv cq sort "score"
/// name,score
/// bob,12
/// alice,23
/// charlie,45
#[derive(clap::Args, Debug)]
#[clap(verbatim_doc_comment)]
pub struct Command {
    /// Sort rows order (default)
    #[clap(long, short, group("mode"), display_order(0))]
    row: bool,

    /// Sort columns order
    #[clap(long, short, group("mode"), display_order(1))]
    column: bool,

    /// Sort by ascending order (default)
    #[clap(long, short, group("order"), display_order(2))]
    asc: bool,

    /// Sort by descending order
    #[clap(long, short, group("order"), display_order(3))]
    desc: bool,

    /// Columns or Expressions to sort
    columns: Vec<String>,
}

use std::collections::hash_map::HashMap;

use super::Arg;

/// Convert CSV to specified format
///
/// $ cat data.csv
/// key,value
/// hello,123
/// world,456
///
/// $ <data.csv cq convert json
/// [{"key":"hello","value":123}, {"key":"world","value":456}]
#[derive(clap::Args, Debug)]
#[clap(verbatim_doc_comment)]
pub struct Command {
    #[clap(subcommand)]
    format: Format,
}

#[derive(clap::Subcommand, Debug)]
enum Format {
    /// Convert CSV to JSON
    ///
    /// $ cat data.csv
    /// key,value
    /// hello,123
    /// world,456
    ///
    /// $ <data.csv cq convert json
    /// [{"key":"hello","value":123}, {"key":"world","value":456}]
    #[clap(verbatim_doc_comment)]
    Json {
        /// Convert to array of objects (default)
        ///
        /// It generates JSON like [{"column": "value", ...}, ...]
        #[clap(long, group("mode"))]
        array_of_objects: bool,

        /// Convert to array of arrays
        ///
        /// It generates JSON like [["column", ...], ["value", ...], ...]
        #[clap(long, group("mode"))]
        array_of_arrays: bool,

        /// Convert to object of arrays
        ///
        /// It generates JSON like {"column": ["value", ...], ...}
        #[clap(long, group("mode"))]
        columnar: bool,

        /// Convert to object of objects by specified key
        ///
        /// It generates JSON like {"key-column": {"column": "value", ...}, ...}
        #[clap(long, group("mode"), value_name("KEY-COLUMN"))]
        object_of_objects: Option<String>,
    },

    /// Convert CSV to Markdown table
    ///
    /// $ cat data.csv
    /// key,value
    /// hello,123
    /// world,456
    ///
    /// $ <data.csv cq convert markdown
    /// |  key  | value |
    /// |-------|-------|
    /// | hello |   123 |
    /// | world |   456 |
    #[clap(verbatim_doc_comment, visible_alias("md"))]
    Markdown,
}

impl Command {
    pub fn execute(&self, arg: &Arg) {
        match &self.format {
            Format::Json {
                array_of_arrays: true,
                ..
            } => {
                let rows = arg
                    .read_stdin(false)
                    .records()
                    .map(|row| {
                        row.unwrap()
                            .iter()
                            .map(|s| s.into())
                            .collect::<Vec<String>>()
                    })
                    .collect::<Vec<Vec<String>>>();
                println!("{}", serde_json::to_string(&rows).unwrap());
            }
            Format::Json { columnar: true, .. } => {
                let mut reader = arg.read_stdin(true);
                let header: Vec<String> =
                    reader.headers().unwrap().iter().map(|s| s.into()).collect();

                let mut result: HashMap<String, Vec<String>> = HashMap::new();
                for col in &header {
                    result.insert(col.into(), Vec::new());
                }

                for row in reader.records() {
                    for (key, value) in header.iter().zip(row.unwrap().iter()) {
                        result.get_mut(key).unwrap().push(value.into());
                    }
                }

                println!("{}", serde_json::to_string(&result).unwrap());
            }
            Format::Json {
                object_of_objects: Some(column),
                ..
            } => {
                let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();

                for row in arg
                    .read_stdin(true)
                    .deserialize::<HashMap<String, String>>()
                {
                    let row = row.unwrap();
                    let id = match row.get(column.as_str()) {
                        Some(x) => x.to_string(),
                        None => "".to_string(),
                    };

                    if !result.contains_key(&id) {
                        result.insert(id, row);
                    }
                }

                println!("{}", serde_json::to_string(&result).unwrap());
            }
            Format::Json { .. } => {
                let rows = arg
                    .read_stdin(true)
                    .deserialize()
                    .map(|row| row.unwrap())
                    .collect::<Vec<HashMap<String, String>>>();
                println!("{}", serde_json::to_string(&rows).unwrap());
            }
            Format::Markdown => eprintln!("markdown is not implemented yet"),
        }
    }
}

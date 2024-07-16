use std::path::PathBuf;

use clap::Parser;
use counter::{count_lines, Counter};
use language_type::LanguageType;
use rayon::prelude::*;
use table::draw_table;
use walker::DirWalker;

mod cli;
mod counter;
mod language_type;
mod state;
mod table;
mod walker;

fn main() {
    let cli = cli::Cli::parse();

    let dir_walker = DirWalker::new(PathBuf::from(cli.name));

    let result = dir_walker
        .iter()
        .par_bridge()
        .filter_map(|f| LanguageType::from_file_extension(f.extension()?.to_str()?).map(|v| (v, f)))
        .filter(|(lt, _)| *lt == cli.target)
        .map(|(lt, f)| count_lines(&f, lt))
        .reduce(|| Counter::none(), |init, acc| init + acc);

    draw_table(&result);
}

use std::path::PathBuf;

use clap::Parser;
use counter::{count_lines, Counter};
use language_type::LanguageType;
use walker::DirWalker;

mod cli;
mod counter;
mod language_type;
mod walker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    let path = cli.name.unwrap_or(".".to_string());

    let dir_walker = DirWalker::new(PathBuf::from(path));

    let mut sum = 0;
    let result = dir_walker
        .iter()
        .filter_map(|f| {
            LanguageType::from_file_extension(f.extension()?.to_str().unwrap()).map(|v| (v, f))
        })
        .filter(|(lt, _)| *lt == LanguageType::Cpp)
        .map(|(lt, f)| count_lines(&f, lt).unwrap())
        .fold(Counter::new(), |init, acc| {
            sum += 1;
            init + acc
        });

    println!("{:?} {} {}", result, sum, result.lines());

    Ok(())
}

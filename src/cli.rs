use chrono::NaiveDate;
use clap::Parser;
use std::path::PathBuf;

/// A simple program to generate Freesound credits in a usable markdown file.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the samples directory
    #[arg(short, long, value_name("DIRECTORY"))]
    pub samples_dir: PathBuf,

    /// Song title (quote multiple words)
    #[arg(short, long)]
    pub title: String,

    /// Song release date (YYYY-MM-DD)
    #[arg(short, long)]
    pub date: NaiveDate,

    /// Song artist (quote multiple words)
    #[arg(short, long)]
    pub artist: String,

    /// Optionally provide a frontmatter template file
    #[arg(short, long, value_name("TEMPLATE"))]
    pub frontmatter_template: Option<PathBuf>,

    /// Optionally append a trailig whiteline
    #[arg(short('w'), long)]
    pub trailing_whiteline: bool,
}

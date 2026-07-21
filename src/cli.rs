use chrono::NaiveDate;
use clap::Parser;
use std::path::PathBuf;

/// A simple program to generate Freesound credits in a usable markdown file.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the samples directory
    #[arg(short, long, value_name("PATH"))]
    pub samples: PathBuf,

    /// Song title (quote multiple words)
    #[arg(short, long)]
    pub title: String,

    /// Song release date (YYYY-MM-DD)
    #[arg(short, long)]
    pub date: NaiveDate,

    /// Song artist (quote multiple words)
    #[arg(short, long)]
    pub artist: String,

    /// Optionally provide a frontmatter template
    #[arg(short, long, value_name("TEMPLATE"))]
    pub frontmatter: Option<PathBuf>,
}

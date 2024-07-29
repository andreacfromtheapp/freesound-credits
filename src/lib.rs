//! `freesound_credits` is a simple program to generate Freesound credits in a usable markdown file
//!
//!  # Usage
//!
//! ```text
//! Simple program to generate Freesound credits in a usable markdown file
//!
//! Usage: freesound_credits [OPTIONS] --path <PATH> --title <TITLE> --date <DATE> --artist <ARTIST>
//!
//! Options:
//!   -p, --path <PATH>      Path to the samples directory
//!   -t, --title <TITLE>    Song title (quote multiple words)
//!   -d, --date <DATE>      Song release date (quote multiple words)
//!   -a, --artist <ARTIST>  Song artist (quote multiple words)
//!   -z, --zola             Optionally include Zola frontmatter atop the markdown file
//!   -h, --help             Print help
//!   -V, --version          Print version
//! ```
//! `
//!  # Example
//!
//!  ## Run against an Ableton samples directory (also generating the Zola frontmatter)
//!
//!  ```text
//! freesound_credits -p Samples/Imported/ -t "Field Notes" -a "Aner Andros" -d "2017-10-28" -z
//!  ```

use clap::Parser;
use std::fs::File;
use std::io::Error;
use std::iter::FromIterator;
use std::path::Path;

/// Simple program to generate Freesound credits in a usable markdown file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the samples directory
    #[arg(short, long)]
    pub path: String,

    /// Song title (quote multiple words)
    #[arg(short, long)]
    pub title: String,

    /// Song release date (quote multiple words)
    #[arg(short, long)]
    pub date: String,

    /// Song artist (quote multiple words)
    #[arg(short, long)]
    pub artist: String,

    /// Optionally include Zola frontmatter atop the markdown file
    #[arg(short, long)]
    pub zola: bool,
}

/// Derives a [Zola](https://www.getzola.org) page
/// Derives a [Zola](https://www.getzola.org) page
/// [frontmatter](https://www.getzola.org/documentation/content/page/#front-matter)
/// header from given song details.
///
/// The frontmatter is a header, and it is placed atop the generated markdown file.
///
/// # Example
///
/// For a song titled "Field Notes" by "Aner Andros" with date "2017-10-28"
///
/// ```toml
/// +++
/// title="Field Notes Freesound Credits"
/// date=2017-10-28
///
/// [taxonomies]
/// tags=["Freesound", "Aner Andros", "Credits"]
/// +++
/// ```
///
pub fn set_frontmatter(song_title: &str, song_date: &str, song_artist: &str) -> String {
    format!(
        "+++
title=\"{song_title} Freesound Credits\"
date={song_date}

[taxonomies]
tags=[\"Freesound\", \"{song_artist}\", \"Credits\"]
+++

"
    )
}

/// Opening paragraph notifying that the song uses [Creative
/// Commons](https://creativecommons.org) licensed samples, with links.
///
/// The given song title is included in the paragraph, unchanged.
///
/// # Example
///
/// For a song titled "Field Notes"
///
/// ```markdown
/// ## Credits
///
/// *Field Notes* includes the following samples from
/// [Freesound](https://freesound.org). Used under a [Creative
/// Commons](https://creativecommons.org) license:
///
/// ````
///
pub fn set_credits_header(song_title: &str) -> String {
    format!(
        "## Credits

*{song_title}* includes the following samples from
[Freesound](https://freesound.org). Used under a [Creative
Commons](https://creativecommons.org) license:

",
    )
}

/// Derives the output filename from the song title and creates the file itself.
///
/// # Notes
///
/// The markdown file is created in the actual directory you run this program from.
///
/// # Panics
///
/// An invalid destination path for the creation of the markdown file or insufficient
/// permissions will cause the program to panic.
///
/// # Example
///
/// For a song titled "Field Notes" the resulting markdown file is `field-notes-credits.md`
///
pub fn create_output_file(song_title: &str) -> Result<File, Error> {
    let credits_file = format!(
        "{}-credits.md",
        song_title.replace(&[' ', '\''][..], "-").to_lowercase()
    );
    let file = match File::create(&credits_file) {
        Ok(file) => file,
        Err(_error) => panic!("Problem creating the file: {credits_file}. Error: {_error}"),
    };
    Ok(file)
}

/// Scans the given directory for Freesound samples to credit.
///
/// # Notes
///
/// - the user must have permissions on the directory.
/// - invalid directories will fail silently.
/// - it only works with Ableton and Renoise projects.
///
/// ## Ableton
///
/// When running against Ableton projects, pass the `--path` to the
/// `Samples/Imported` directory.
///
/// ## Renoise
///
/// Renoise `xrns` projects need to be extracted with `unzip` first. Once unzipped
/// you will find a `Song.xml` file and a `SamplesData` directory containing each
/// `Instrument`. Pass the `--path` to the `SamplesData` directory.
///
pub fn get_list_of_samples(samples_path: &str) -> Vec<String> {
    let path = Path::new(&samples_path);
    let mut samples_raw_vector: Vec<String> = vec![];

    for entry in path
        .read_dir()
        .expect("Error: can't list samples from the provided path")
        .flatten()
    {
        if entry.path().is_file() || entry.path().is_dir() {
            let mut sample = format!(
                "{:?}",
                entry
                    .path()
                    .file_stem()
                    .expect("Error: cannot read the sample file name")
            );
            sample = sample.replace(&['(', ')', '\'', '"'][..], "");

            // this is specific to Ableton projects
            if let Some(extension) = entry.path().extension() {
                if extension != "asd"
                    && sample.chars().next().unwrap().is_numeric()
                    && sample.contains('_')
                {
                    samples_raw_vector.push(sample);
                }
            // this is specific to Renoise projects
            } else if sample.contains("Instrument") {
                sample = sample
                    .split_whitespace()
                    .last()
                    .expect("Error: can't split file name into sample string")
                    .to_string();
                if sample.chars().next().unwrap().is_numeric() && sample.contains('_') {
                    samples_raw_vector.push(sample);
                }
            }
        }
    }
    samples_raw_vector
}

/// Extrapolate the sample to credit based on [Freesound](https://freesound.org) naming standards.
///
/// # Notes
///
/// This programs only matches for Freesound samples that maintain their original sample names.
///
/// # Examples
///
/// - `69604__timkahn__subverse_whisper.wav` # new standard with double _
/// - `2166_suburban_grilla_bowl_struck.flac` # old standard with single _
///
pub fn set_credit_line(line: &str) -> String {
    let mut credit_line_vector: Vec<&str> = vec![];

    if line.contains("__") {
        credit_line_vector = line.split("__").collect();
    } else if line.contains('_') {
        credit_line_vector = line.split('_').collect();
    }

    let credit_id = credit_line_vector
        .first()
        .expect("Error: can't read credit ID")
        .to_string();
    let credit_artist = credit_line_vector
        .get(1)
        .expect("Error: can't read credit artist")
        .to_string();
    let credit_sound_parts_to_end = Vec::from_iter(credit_line_vector[2..].iter().cloned());
    let credit_sound = credit_sound_parts_to_end.join("_");

    let credit_line = format!(
        "- [{credit_sound}](https://freesound.org/people/{credit_artist}/sounds/{credit_id}/)\n",
    );
    credit_line
}

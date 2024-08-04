//! `freesound_credits` is a simple program to generate Freesound credits in a usable markdown file.
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
//! Run against an Ableton samples directory (also generating the Zola frontmatter)
//!
//!  ```text
//! freesound_credits -p Samples/Imported/ -t "Field Notes" -a "Aner Andros" -d "2017-10-28" -z
//!  ```

use clap::Parser;
use std::iter::FromIterator;
use std::path::Path;
use std::process;

/// A simple program to generate Freesound credits in a usable markdown file.
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

/// Derives the markdown filename from the song title.
///
/// # Example
///
/// For a song titled “Field Notes” the resulting markdown file is `field-notes-credits.md`
///
pub fn set_filename(song_title: &str) -> String {
    let credits_file: String = format!(
        "{}-credits.md",
        song_title.replace(&[' ', '\''][..], "-").to_lowercase()
    );
    credits_file
}

/// Derives a [Zola](https://www.getzola.org) page
/// [frontmatter](https://www.getzola.org/documentation/content/page/#front-matter)
/// header from given song details.
///
/// The frontmatter is a header, and it is placed atop the generated markdown file.
///
/// # Example
///
/// For a song titled “Field Notes” by “Aner Andros” with date "2017-10-28"
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

/// Paragraph notifying the song uses [Creative
/// Commons](https://creativecommons.org) licensed samples, with links.
///
/// The given song title is included in the paragraph, unchanged.
///
/// # Example
///
/// For a song titled “Field Notes”
///
/// ```markdown
/// ## Credits
///
/// *Field Notes* includes the following samples from
/// [Freesound](https://freesound.org). Used under a [Creative
/// Commons](https://creativecommons.org) license:
/// ````
///
pub fn set_header(song_title: &str) -> String {
    format!(
        "## Credits

*{song_title}* includes the following samples from
[Freesound](https://freesound.org). Used under a [Creative
Commons](https://creativecommons.org) license:

",
    )
}

/// Scans the given directory for Freesound samples to credit.
///
/// # Notes
///
/// - the user must have permissions on the directory.
///
pub fn get_list_of_samples(samples_path: &str) -> Vec<String> {
    let path: &Path = Path::new(&samples_path);
    let mut all_samples: Vec<String> = vec![];

    for entry in path
        .read_dir()
        .unwrap_or_else(|error| {
            eprintln!("Problem listing samples from the provided path: {error}");
            process::exit(2);
        })
        .flatten()
    {
        if entry.path().is_file() || entry.path().is_dir() {
            let mut sample: String = format!(
                "{:?}",
                entry.path().file_stem().unwrap_or_else(|| {
                    eprintln!("Problem reading the sample file name.");
                    process::exit(2);
                })
            )
            .replace(&['(', ')', '\'', '"'][..], "");

            // Files specific: checks against DAWs metadata file extensions
            if let Some(extension) = entry.path().extension() {
                if is_not_metadata(extension.to_str().unwrap()) && is_freesound_sample(&sample) {
                    all_samples.push(sample);
                }
                // Renoise projects specific
            } else if sample.contains("Instrument") {
                sample = sample
                    .split_whitespace()
                    .last()
                    .unwrap_or_else(|| {
                        eprintln!("Problem splitting Instrument into sample string");
                        process::exit(2);
                    })
                    .to_string();

                if is_freesound_sample(&sample) {
                    all_samples.push(sample);
                }
            }
        }
    }
    all_samples
}

fn is_not_metadata(extension: &str) -> bool {
    let metadata_extensions = ["asd", "reapeaks"];

    !metadata_extensions.contains(&extension)
}

/// Private helper function to validate Freesound samples we care about.
fn is_freesound_sample(sample: &str) -> bool {
    sample.chars().next().unwrap().is_numeric() && sample.contains('_')
}

/// Extrapolate the sample to credit based on [Freesound](https://freesound.org) naming standards.
///
/// # Notes
///
/// This programs only matches for Freesound samples that maintain their original sample names.
///
/// # Examples
///
/// - new standard with double underscore: `69604__timkahn__subverse_whisper.wav`
/// - old standard with single underscore: `2166_suburban_grilla_bowl_struck.flac`
///
pub fn set_credit(sample: &str) -> String {
    let mut sample_line_vec: Vec<&str> = vec![];

    if sample.contains("__") {
        sample_line_vec = sample.split("__").collect();
    } else if sample.contains('_') {
        sample_line_vec = sample.split('_').collect();
    }

    let credit_id: String = sample_line_vec
        .first()
        .unwrap_or_else(|| {
            eprintln!("Problem reading credit ID");
            process::exit(2);
        })
        .to_string();

    let credit_artist: String = sample_line_vec
        .get(1)
        .unwrap_or_else(|| {
            eprintln!("Problem reading credit artist");
            process::exit(2);
        })
        .to_string();

    let credit_parts_to_end: Vec<&str> = Vec::from_iter(sample_line_vec[2..].iter().cloned());
    let credit_sound: String = credit_parts_to_end.join("_");

    let credit_line: String = format!(
        "- [{credit_sound}](https://freesound.org/people/{credit_artist}/sounds/{credit_id}/)\n",
    );
    credit_line
}

// TODO: add test for get_list_of_samples
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_filename() {
        let song_title = "Field Notes";

        assert_eq!("field-notes-credits.md", set_filename(song_title));
    }

    #[test]
    fn check_frontmatter() {
        let song_title = "Field Notes";
        let song_artist = "Aner Andros";
        let song_date = "2017-10-28";
        let frontmatter = "+++
title=\"Field Notes Freesound Credits\"
date=2017-10-28

[taxonomies]
tags=[\"Freesound\", \"Aner Andros\", \"Credits\"]
+++

";

        assert_eq!(
            frontmatter,
            set_frontmatter(song_title, song_date, song_artist)
        );
    }

    #[test]
    fn check_header() {
        let song_title = "Field Notes";
        let header = "## Credits

*Field Notes* includes the following samples from
[Freesound](https://freesound.org). Used under a [Creative
Commons](https://creativecommons.org) license:

";

        assert_eq!(header, set_header(song_title));
    }

    #[test]
    fn check_credit_new() {
        let credit = "275012__alienxxx__squadron_leader_form_up";

        assert_eq!(
            "- [squadron_leader_form_up](https://freesound.org/people/alienxxx/sounds/275012/)\n",
            set_credit(credit)
        );
    }

    #[test]
    fn check_credit_old() {
        let credit = "275012_alienxxx_squadron_leader_form_up";

        assert_eq!(
            "- [squadron_leader_form_up](https://freesound.org/people/alienxxx/sounds/275012/)\n",
            set_credit(credit)
        );
    }
}

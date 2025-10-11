//! `freesound-credits` is a simple program to generate Freesound credits in a usable markdown file.
//!
//!  # Usage
//!
//! ```text
//! A simple command line utility to credit Freesound samples in a usable markdown file
//!
//! Usage: freesound-credits [OPTIONS] --samples-dir <DIRECTORY> --title <TITLE> --date <DATE> --artist <ARTIST>
//!
//! Options:
//!   -s, --samples-dir <DIRECTORY>          Path to the samples directory
//!   -t, --title <TITLE>                    Song title (quote multiple words)
//!   -d, --date <DATE>                      Song release date (YYYY-MM-DD)
//!   -a, --artist <ARTIST>                  Song artist (quote multiple words)
//!   -f, --frontmatter-template <TEMPLATE>  Optionally provide a frontmatter template file
//!   -w, --trailing-whiteline               Optionally append a trailing whiteline
//!   -h, --help                             Print help
//!   -V, --version                          Print version
//! ```
//!

use chrono::NaiveDate;
use std::path::{Path, PathBuf};

pub mod cli;

pub mod error;
use error::AppError;

pub fn run_app(args: &cli::Cli) -> Result<(), AppError> {
    use std::io::Write;

    // Generate filename from song title (e.g., "Field Notes" -> "field-notes-credits.md")
    let credits_file = set_filename(&args.title);

    // Create output file
    let mut output = std::fs::File::create(&credits_file).map_err(|e| {
        AppError::file_op(format!("Couldn't create file '{}': {}", credits_file, e))
    })?;

    // Write Zola frontmatter header
    write!(
        output,
        "{}",
        set_frontmatter(
            &args.title,
            &args.date,
            &args.artist,
            args.frontmatter_template.as_ref()
        )?
    )
    .map_err(|e| AppError::file_op(format!("Couldn't write frontmatter: {}", e)))?;

    // Write credits section header
    write!(output, "{}", set_header(&args.title))
        .map_err(|e| AppError::file_op(format!("Couldn't write credits header: {}", e)))?;

    // Process each Freesound sample and write credit line
    for sample in set_list_of_samples(&args.samples_dir)?.iter() {
        write!(output, "{}", set_credit(sample)?).map_err(|e| {
            AppError::file_op(format!(
                "Couldn't write credit for sample '{}': {}",
                sample, e
            ))
        })?;
    }

    // Optional trailing newline
    if args.trailing_whiteline {
        writeln!(output)
            .map_err(|e| AppError::file_op(format!("Couldn't write trailing whiteline: {}", e)))?;
    }

    Ok(())
}

/// Converts song title to kebab-case markdown filename
/// Replaces special chars and spaces with hyphens, converts to lowercase
fn set_filename(song_title: &str) -> String {
    format!(
        "{}-credits.md",
        song_title
            .replace(
                &[
                    '/', '\\', '(', ')', '[', ']', '<', '>', '{', '}', ' ', '\'', '"', '?', '!'
                ][..],
                "-"
            )
            .to_lowercase()
    )
}

/// Generates frontmatter from template file or uses default
fn set_frontmatter(
    song_title: &str,
    song_date: &NaiveDate,
    song_artist: &str,
    template_path: Option<&PathBuf>,
) -> Result<String, AppError> {
    if let Some(path) = template_path {
        let template = std::fs::read_to_string(path).map_err(|e| {
            AppError::file_op(format!("Couldn't read template file '{:?}': {}", path, e))
        })?;

        let frontmatter = template
            .replace("{song_title}", song_title)
            .replace("{song_date}", &song_date.to_string())
            .replace("{song_artist}", song_artist);

        Ok(format!("+++\n{}\n+++\n\n", frontmatter))
    } else {
        Ok(format!(
            "+++
title=\"{song_title} Credits\"
date={song_date}

[taxonomies]
tags=[\"Freesound\", \"{song_artist}\", \"Credits\"]
+++

"
        ))
    }
}

/// Creates markdown credits section with Creative Commons notice
fn set_header(song_title: &str) -> String {
    format!(
        "## Credits

*{song_title}* includes the following samples from
[Freesound](https://freesound.org). Used under a [Creative
Commons](https://creativecommons.org) license:

",
    )
}

/// Scans directory for Freesound samples, filtering out DAW metadata files
fn set_list_of_samples(samples_path: &PathBuf) -> Result<Vec<String>, AppError> {
    let path = Path::new(&samples_path);
    let mut all_samples = vec![];

    for entry in path
        .read_dir()
        .map_err(|e| {
            AppError::directory_access(format!(
                "Couldn't list samples from path '{:?}': {}",
                path, e
            ))
        })?
        .flatten()
    {
        if entry.path().is_file() || entry.path().is_dir() {
            // Extract filename without extension, clean up formatting
            let mut sample = format!(
                "{:?}",
                entry
                    .path()
                    .file_stem()
                    .ok_or_else(|| AppError::sample_parsing(format!(
                        "Couldn't read file name from entry: {:?}",
                        entry
                    )))?
            )
            .replace(&['(', ')', '\'', '"'][..], "");

            // Handle regular files - check extension and validate Freesound format
            if let Some(extension) = entry.path().extension() {
                if is_not_metadata(extension.to_str().unwrap()) && is_freesound_sample(&sample) {
                    all_samples.push(sample);
                }
            // Handle Renoise extracted directories (Instrument folders)
            } else if sample.contains("Instrument") {
                sample = sample
                    .split_whitespace()
                    .last()
                    .ok_or_else(|| {
                        AppError::sample_parsing(format!(
                            "Couldn't split Renoise Instrument: {}",
                            sample
                        ))
                    })?
                    .to_string();

                if is_freesound_sample(&sample) {
                    all_samples.push(sample);
                }
            }
        }
    }

    Ok(all_samples)
}

/// Filters out DAW metadata files (.asd, .reapeaks)
fn is_not_metadata(extension: &str) -> bool {
    let metadata_extensions = ["asd", "reapeaks"];
    !metadata_extensions.contains(&extension)
}

/// Validates Freesound naming: starts with number and contains underscore
fn is_freesound_sample(sample: &str) -> bool {
    sample.chars().next().unwrap().is_numeric() && sample.contains('_')
}

/// Parses Freesound filename and generates markdown credit link
/// Handles both old (single _) and new (double __) naming formats
fn set_credit(sample: &str) -> Result<String, AppError> {
    let mut sample_line_vec = vec![];

    // Parse filename: new format uses __, old format uses single _
    if sample.contains("__") {
        sample_line_vec = sample.split("__").collect();
    } else if sample.contains('_') {
        sample_line_vec = sample.split('_').collect();
    }

    // Extract components: ID, artist, sound name
    let credit_id = sample_line_vec
        .first()
        .ok_or_else(|| AppError::invalid_sample("Couldn't read credit ID"))?
        .to_string();

    let credit_artist = sample_line_vec
        .get(1)
        .ok_or_else(|| AppError::invalid_sample("Couldn't read credit artist"))?
        .to_string();

    // Join remaining parts as sound name
    let credit_sound = Vec::from_iter(sample_line_vec[2..].iter().cloned()).join("_");

    // Generate markdown link to Freesound
    let credit_line = format!(
        "- [{credit_sound}](https://freesound.org/people/{credit_artist}/sounds/{credit_id}/)\n",
    );

    Ok(credit_line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_set_filename() {
        let song_title = "Field Notes";
        let expected_filename = "field-notes-credits.md";

        assert_eq!(expected_filename, set_filename(song_title));
    }

    #[test]
    fn test_set_filename_fail() {
        let song_title = "Field Notes";
        let wrong_filename = "Field-Notes-credits.md";

        assert_ne!(wrong_filename, set_filename(song_title));
    }

    #[test]
    fn test_set_frontmatter() {
        let song_title = "Field Notes";
        let song_artist = "Aner Andros";
        let song_date = NaiveDate::from_ymd_opt(2015, 1, 9).unwrap();

        let result = set_frontmatter(song_title, &song_date, song_artist, None).unwrap();

        assert!(result.contains("title=\"Field Notes Credits\""));
        assert!(result.contains("date=2015-01-09"));
        assert!(result.contains("tags=[\"Freesound\", \"Aner Andros\", \"Credits\"]"));
    }

    #[test]
    fn test_set_header() {
        let song_title = "Field Notes";
        let expected_header = format!(
            "## Credits

*{song_title}* includes the following samples from
[Freesound](https://freesound.org). Used under a [Creative
Commons](https://creativecommons.org) license:

"
        );

        assert_eq!(expected_header, set_header(song_title));
    }

    #[test]
    fn mock_frontmatter_template() {
        use std::env;
        use std::fs;

        let temp_dir = env::temp_dir().join("freesound_template_test");
        fs::create_dir_all(&temp_dir).unwrap();

        let template_path = temp_dir.join("template.toml");
        fs::write(
            &template_path,
            "title=\"{song_title} Custom\"\ndate={song_date}\nauthor=\"{song_artist}\"",
        )
        .unwrap();

        let song_title = "Test Song";
        let song_artist = "Test Artist";
        let song_date = NaiveDate::from_ymd_opt(2023, 5, 15).unwrap();

        let result =
            set_frontmatter(song_title, &song_date, song_artist, Some(&template_path)).unwrap();

        assert!(result.contains("title=\"Test Song Custom\""));
        assert!(result.contains("date=2023-05-15"));
        assert!(result.contains("author=\"Test Artist\""));
        assert!(result.starts_with("+++\n"));
        assert!(result.contains("\n+++\n\n"));

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_set_credit_new() {
        let credit = "275012__alienxxx__squadron_leader_form_up";
        let expected_credit =
            "- [squadron_leader_form_up](https://freesound.org/people/alienxxx/sounds/275012/)\n";

        assert_eq!(expected_credit, set_credit(credit).unwrap());
    }

    #[test]
    fn test_set_credit_old() {
        let credit = "275012_alienxxx_squadron_leader_form_up";
        let expected_credit =
            "- [squadron_leader_form_up](https://freesound.org/people/alienxxx/sounds/275012/)\n";

        assert_eq!(expected_credit, set_credit(credit).unwrap());
    }

    #[test]
    fn test_set_list_of_samples_success() {
        use std::env;
        use std::fs;

        let temp_dir = env::temp_dir().join("freesound_test");
        fs::create_dir_all(&temp_dir).unwrap();

        // Create mock Freesound samples
        fs::write(temp_dir.join("69604__timkahn__subverse_whisper.wav"), "").unwrap();
        fs::write(temp_dir.join("2166_suburban_grilla_bowl_struck.flac"), "").unwrap();
        // Create metadata file (should be filtered out)
        fs::write(temp_dir.join("sample.asd"), "").unwrap();
        // Create non-Freesound file (should be filtered out)
        fs::write(temp_dir.join("regular_file.wav"), "").unwrap();

        let samples = set_list_of_samples(&temp_dir).unwrap();

        assert_eq!(samples.len(), 2);
        assert!(samples.contains(&"69604__timkahn__subverse_whisper".to_string()));
        assert!(samples.contains(&"2166_suburban_grilla_bowl_struck".to_string()));

        // Cleanup
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_error_cases() {
        use std::path::PathBuf;

        // Test invalid directory
        let invalid_dir = PathBuf::from("/nonexistent/directory");
        assert!(set_list_of_samples(&invalid_dir).is_err());

        // Test invalid sample format (no underscores)
        assert!(set_credit("noseparators").is_err());
        assert!(set_credit("").is_err());

        // Test missing template file
        let missing_template = PathBuf::from("/nonexistent/template.toml");
        assert!(
            set_frontmatter(
                "Test",
                &chrono::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                "Artist",
                Some(&missing_template)
            )
            .is_err()
        );
    }

    #[test]
    fn test_is_not_metadata() {
        assert!(is_not_metadata("wav"));
        assert!(is_not_metadata("flac"));
        assert!(!is_not_metadata("asd"));
        assert!(!is_not_metadata("reapeaks"));
    }

    #[test]
    fn test_is_freesound_sample() {
        assert!(is_freesound_sample("123_artist_sound"));
        assert!(is_freesound_sample("69604__timkahn__subverse"));
        assert!(!is_freesound_sample("regular_file"));
        assert!(!is_freesound_sample("_starts_with_underscore"));
    }

    #[test]
    fn test_file_operation_errors() {
        use std::env;
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        let temp_dir = env::temp_dir().join("freesound_file_errors");
        fs::create_dir_all(&temp_dir).unwrap();

        // Test unreadable directory
        let unreadable_dir = temp_dir.join("unreadable");
        fs::create_dir(&unreadable_dir).unwrap();
        fs::set_permissions(&unreadable_dir, fs::Permissions::from_mode(0o000)).unwrap();

        let result = set_list_of_samples(&unreadable_dir);
        assert!(result.is_err());

        // Restore permissions for cleanup
        fs::set_permissions(&unreadable_dir, fs::Permissions::from_mode(0o755)).unwrap();
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_sample_parsing_errors() {
        // Test samples that will actually fail parsing
        assert!(set_credit("").is_err()); // Empty string - no underscores, vec will be empty
        assert!(set_credit("123").is_err()); // No underscores - vec will be empty
    }
}

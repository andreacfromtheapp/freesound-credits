//! `freesound-credits` is a simple program to generate Freesound credits in a usable markdown file.
//!
//!  # Usage
//!
//! ```text
//! A simple command line utility to credit Freesound samples in a usable markdown file
//!
//! Usage: freesound-credits [OPTIONS] --samples <PATH> --title <TITLE> --date <DATE> --artist <ARTIST>
//!
//! Options:
//!   -s, --samples <PATH>          Path to the samples directory
//!   -t, --title <TITLE>           Song title (quote multiple words)
//!   -d, --date <DATE>             Song release date (YYYY-MM-DD)
//!   -a, --artist <ARTIST>         Song artist (quote multiple words)
//!   -f, --frontmatter <TEMPLATE>  Optionally provide a frontmatter template
//!   -h, --help                    Print help
//!   -V, --version                 Print version
//! ```
//!

use chrono::NaiveDate;
use std::io::Write;
use std::path::{Path, PathBuf};

pub mod cli;
pub mod error;
use error::AppError;

/// Main app
pub fn run_app(args: &cli::Cli) -> Result<(), AppError> {
    // Generate filename from song title (e.g., "Field Notes" -> "field-notes-credits.md")
    let credits_file = set_filename(&args.title)?;

    // Create output file
    let mut output = create_output_file(&credits_file)?;

    // Write Zola frontmatter header
    write_frontmatter(
        &mut output,
        &args.title,
        &args.date,
        &args.artist,
        args.frontmatter.as_ref(),
    )?;

    // Write credits section header
    write_header(&mut output, &args.title)?;

    // Generate the samples array/vec to use with write_credits
    let samples = set_list_of_samples(&args.samples)?;

    // Write the samples credits list to file
    write_credits(&mut output, &samples)?;

    // Trailing newline
    writeln!(&mut output).map_err(|error| {
        AppError::file_op(format!("Couldn't write trailing newline: {}", error))
    })?;

    Ok(())
}

/// Converts song title to lowercase kebab-case markdown filename
fn set_filename(song_title: &str) -> Result<String, AppError> {
    let trimmed = song_title.trim();

    if trimmed.is_empty() {
        return Err(AppError::invalid_input("Song title cannot be empty"));
    }

    if trimmed.contains('\0') {
        return Err(AppError::invalid_input(
            "Song title contains null characters",
        ));
    }

    // replace spaces with hyphens and ignores non-alphanumeric characters
    let filename = format!(
        "{}-credits.md",
        trimmed
            .chars()
            .filter_map(|c| match c {
                c if c.is_alphanumeric() => Some(c),
                c if c.is_whitespace() => Some('-'),
                _ => None,
            })
            .collect::<String>()
            .to_lowercase()
    );

    Ok(filename)
}

/// Create the final output markdown file with the given file name
fn create_output_file(path: &str) -> Result<std::fs::File, AppError> {
    use std::path::Path;

    if path.is_empty() {
        return Err(AppError::file_op("File path cannot be empty"));
    }

    if path.contains('\0') {
        return Err(AppError::file_op("File path contains null bytes"));
    }

    let path_obj = Path::new(path);

    // Check if parent directory exists
    if let Some(parent) = path_obj.parent() {
        // Skip check for current directory (empty parent)
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(AppError::directory_access(format!(
                "Parent directory does not exist: {}",
                parent.display()
            )));
        }
    }

    std::fs::File::create(path_obj)
        .map_err(|error| AppError::file_op(format!("Couldn't create file '{}': {}", path, error)))
}

/// Generates frontmatter from template file or uses default
fn set_frontmatter(
    song_title: &str,
    song_date: &NaiveDate,
    song_artist: &str,
    template_path: Option<&PathBuf>,
) -> Result<String, AppError> {
    if song_title.is_empty() {
        return Err(AppError::invalid_input("Song title cannot be empty"));
    }
    if song_title.contains('\0') {
        return Err(AppError::file_op("Song title contains null bytes"));
    }
    if song_artist.is_empty() {
        return Err(AppError::invalid_input("Song artist cannot be empty"));
    }
    if song_artist.contains('\0') {
        return Err(AppError::file_op("Song artist contains null bytes"));
    }

    const MAX_TEMPLATE_SIZE: u64 = 100 * 1024; // 100KB limit
    const REQUIRED_PLACEHOLDERS: &[&str] = &["{song_title}", "{song_date}", "{song_artist}"];

    let frontmatter = match template_path {
        Some(path) => {
            // Validate file extension
            if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                return Err(AppError::file_op(
                    "Template file must have .toml extension".to_string(),
                ));
            }

            // Check if file exists and is a file
            if !path.exists() {
                return Err(AppError::file_op(format!(
                    "Template file does not exist: {}",
                    path.display()
                )));
            }

            if !path.is_file() {
                return Err(AppError::file_op(format!(
                    "Template path is not a file: {}",
                    path.display()
                )));
            }

            // Check file size
            let metadata = std::fs::metadata(path).map_err(|error| {
                AppError::file_op(format!(
                    "Couldn't read template file metadata '{}': {}",
                    path.display(),
                    error
                ))
            })?;

            if metadata.len() > MAX_TEMPLATE_SIZE {
                return Err(AppError::file_op(format!(
                    "Template file exceeds maximum size of {} KB",
                    MAX_TEMPLATE_SIZE / 1024
                )));
            }

            let template = std::fs::read_to_string(path).map_err(|error| {
                AppError::file_op(format!(
                    "Couldn't read template file '{}': {}",
                    path.display(),
                    error
                ))
            })?;

            // Check for required placeholders
            for placeholder in REQUIRED_PLACEHOLDERS {
                if !template.contains(placeholder) {
                    return Err(AppError::invalid_sample(format!(
                        "Template missing required placeholder: {}",
                        placeholder
                    )));
                }
            }

            // Replace placeholders
            template
                .replace("{song_title}", song_title)
                .replace("{song_date}", &song_date.to_string())
                .replace("{song_artist}", song_artist)
        }

        None => {
            format!(
                "title=\"{song_title} Credits\"\ndate={song_date}\n\n[taxonomies]\ntags=[\"Freesound\",\"{song_artist}\",\"Credits\"]"
            )
        }
    };

    Ok(format!("+++\n{}\n+++\n\n", frontmatter))
}

/// Write the front matter to the output file
fn write_frontmatter(
    output: &mut std::fs::File,
    title: &str,
    date: &NaiveDate,
    artist: &str,
    frontmatter: Option<&PathBuf>,
) -> Result<(), AppError> {
    let frontmatter = set_frontmatter(title, date, artist, frontmatter)?;
    output
        .write_all(frontmatter.as_bytes())
        .map_err(|error| AppError::file_op(format!("Couldn't write frontmatter: {}", error)))
}

/// Creates markdown credits section with Creative Commons notice
fn set_header(song_title: &str) -> Result<String, AppError> {
    if song_title.is_empty() {
        return Err(AppError::invalid_input("Song title cannot be empty"));
    }
    if song_title.contains('\0') {
        return Err(AppError::file_op("Song title contains null bytes"));
    }

    if !song_title
        .chars()
        .all(|c| c.is_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation())
    {
        return Err(AppError::invalid_input(
            "Song title contains invalid characters",
        ));
    }

    Ok(format!(
        "## Credits

*{song_title}* includes the following samples from
[Freesound](https://freesound.org). Used under a [Creative
Commons](https://creativecommons.org) license:

"
    ))
}

/// Write markdown credits to the output file
fn write_header(output: &mut std::fs::File, song_title: &str) -> Result<(), AppError> {
    let header = set_header(song_title)?;
    output
        .write_all(header.as_bytes())
        .map_err(|error| AppError::file_op(format!("Couldn't write credits header: {}", error)))
}

/// Scans directory for Freesound samples, filtering out DAW metadata files
fn set_list_of_samples(samples_path: &Path) -> Result<Vec<String>, AppError> {
    if !samples_path.exists() {
        return Err(AppError::directory_access(format!(
            "Samples directory does not exist: {}",
            samples_path.display()
        )));
    }

    if !samples_path.is_dir() {
        return Err(AppError::directory_access(format!(
            "Samples path is not a directory: {}",
            samples_path.display()
        )));
    }

    let mut samples = Vec::new();

    for entry in samples_path
        .read_dir()
        .map_err(|error| {
            AppError::directory_access(format!(
                "Couldn't list samples from '{}': {}",
                samples_path.display(),
                error
            ))
        })?
        .filter_map(Result::ok)
    {
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                if is_metadata(extension) {
                    continue;
                }

                let sample = extract_and_clean_filename(&path)?;
                if is_freesound_sample(&sample) {
                    samples.push(sample);
                }
            }
        } else if path.is_dir() {
            let sample = extract_and_clean_filename(&path)?;
            #[allow(clippy::collapsible_if)]
            if sample.contains("Instrument") {
                if let Some(id) = sample.split_whitespace().last() {
                    if is_freesound_sample(id) {
                        samples.push(id.to_string());
                    }
                }
            }
        }
    }

    if samples.is_empty() {
        return Err(AppError::sample_parsing(
            "No Freesound samples found in directory".to_string(),
        ));
    }

    Ok(samples)
}

/// Helper to extract and clean filename
fn extract_and_clean_filename(path: &Path) -> Result<String, AppError> {
    let filename = path
        .file_stem()
        .ok_or_else(|| {
            AppError::sample_parsing(format!(
                "Couldn't extract filename from: {}",
                path.display()
            ))
        })?
        .to_string_lossy();

    let cleaned = filename
        .chars()
        .filter(|c| is_valid_filename_char(*c))
        .collect::<String>();

    Ok(cleaned)
}

/// Filters out invalid filename characters
fn is_valid_filename_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '-' | '_') || c.is_whitespace()
}

/// Used to filter out DAWs samples metadata files
fn is_metadata(extension: &str) -> bool {
    let metadata_extensions = ["asd", "reapeaks"];
    metadata_extensions.contains(&extension)
}

/// Validates Freesound naming: starts with number and contains underscore
fn is_freesound_sample(sample: &str) -> bool {
    sample.starts_with(|c: char| c.is_numeric()) && sample.contains('_')
}

/// Parses Freesound filename and generates markdown credit link
fn set_credit(sample: &str) -> Result<String, AppError> {
    // Determine separator and split
    let separator = if sample.contains("__") { "__" } else { "_" };
    let mut parts = sample.split(separator);

    // Extract components with proper error handling
    let credit_id = parts
        .next()
        .ok_or_else(|| AppError::invalid_sample("Couldn't read credit ID"))?;

    let credit_artist = parts
        .next()
        .ok_or_else(|| AppError::invalid_sample("Couldn't read credit artist"))?;

    let credit_sound = parts.collect::<Vec<_>>().join("_");

    if credit_sound.is_empty() {
        return Err(AppError::invalid_sample("Couldn't read credit sound name"));
    }

    // Generate markdown link to Freesound
    Ok(format!(
        "- [{credit_sound}](https://freesound.org/people/{credit_artist}/sounds/{credit_id}/)\n"
    ))
}

/// Process each Freesound sample and write credit line
fn write_credits(output: &mut std::fs::File, samples: &[String]) -> Result<(), AppError> {
    for sample in samples {
        let credit_line = set_credit(sample)?;
        output.write_all(credit_line.as_bytes()).map_err(|error| {
            AppError::file_op(format!(
                "Couldn't write credit for sample '{}': {}",
                sample, error
            ))
        })?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_set_filename() {
        let song_title = "Field Notes";
        let expected_filename = "field-notes-credits.md";

        assert_eq!(expected_filename, set_filename(song_title).unwrap());
    }

    #[test]
    fn fail_set_filename_upper() {
        let song_title = "Field Notes";
        let wrong_filename = "Field-Notes-credits.md";

        assert_ne!(wrong_filename, set_filename(song_title).unwrap());
    }

    #[test]
    fn fail_set_filename_spaces() {
        let song_title = "Field Notes";
        let wrong_filename = "field notes credits.md";

        assert_ne!(wrong_filename, set_filename(song_title).unwrap());
    }

    #[test]
    fn test_set_frontmatter() {
        let song_title = "Field Notes";
        let song_artist = "Aner Andros";
        let song_date = NaiveDate::from_ymd_opt(2025, 1, 9).unwrap();

        let result = set_frontmatter(song_title, &song_date, song_artist, None).unwrap();

        // Check for key components without worrying about exact formatting
        assert!(result.starts_with("+++\n"));
        assert!(result.contains("Field Notes Credits"));
        assert!(result.contains("2025-01-09"));
        assert!(result.contains("Aner Andros"));
        assert!(result.contains("Freesound"));
        assert!(result.contains("Credits"));
        assert!(result.ends_with("+++\n\n"));
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

        assert_eq!(expected_header, set_header(song_title).unwrap());
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
    fn test_set_list_of_samples_empty_directory() {
        use std::env;
        use std::fs;

        let temp_dir = env::temp_dir().join("freesound_test_empty");
        fs::create_dir_all(&temp_dir).unwrap();

        let result = set_list_of_samples(&temp_dir);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No Freesound samples found"));

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_set_list_of_samples_filters_metadata() {
        use std::env;
        use std::fs;

        let temp_dir = env::temp_dir().join("freesound_test_metadata");
        fs::create_dir_all(&temp_dir).unwrap();

        // Only metadata files
        fs::write(temp_dir.join("69604__timkahn__subverse_whisper.asd"), "").unwrap();
        fs::write(
            temp_dir.join("2166_suburban_grilla_bowl_struck.reapeaks"),
            "",
        )
        .unwrap();

        let result = set_list_of_samples(&temp_dir);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No Freesound samples found"));

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_set_list_of_samples_renoise_instruments() {
        use std::env;
        use std::fs;

        let temp_dir = env::temp_dir().join("freesound_test_renoise");
        fs::create_dir_all(&temp_dir).unwrap();

        // Create Renoise Instrument directory
        let instrument_dir = temp_dir.join("Instrument 69604__timkahn__subverse");
        fs::create_dir_all(&instrument_dir).unwrap();

        // Regular Freesound sample
        fs::write(temp_dir.join("2166_suburban_grilla_bowl_struck.wav"), "").unwrap();

        let samples = set_list_of_samples(&temp_dir).unwrap();

        assert_eq!(samples.len(), 2);
        assert!(samples.contains(&"69604__timkahn__subverse".to_string()));
        assert!(samples.contains(&"2166_suburban_grilla_bowl_struck".to_string()));

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_set_list_of_samples_special_characters() {
        use std::env;
        use std::fs;

        let temp_dir = env::temp_dir().join("freesound_test_special");
        fs::create_dir_all(&temp_dir).unwrap();

        // Freesound sample with special characters (should be cleaned)
        // Keeps the __ pattern so it passes is_freesound_sample
        fs::write(temp_dir.join("69604__timkahn__sub@verse#whisper.wav"), "").unwrap();

        let samples = set_list_of_samples(&temp_dir).unwrap();

        assert_eq!(samples.len(), 1);
        // Special chars removed, but __ preserved
        assert!(samples[0].starts_with("69604") && samples[0].contains("__"));

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_set_list_of_samples_invalid_path() {
        use std::path::PathBuf;

        let invalid_path = PathBuf::from("/nonexistent/path/to/samples");

        let result = set_list_of_samples(&invalid_path);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_set_list_of_samples_file_instead_of_directory() {
        use std::env;
        use std::fs;

        let temp_dir = env::temp_dir().join("freesound_test_file");
        let _ = fs::remove_file(&temp_dir); // Clean up if it exists
        fs::write(&temp_dir, "").unwrap();

        let result = set_list_of_samples(&temp_dir);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not a directory"));

        fs::remove_file(&temp_dir).unwrap();
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
        assert!(set_frontmatter(
            "Test",
            &chrono::NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            "Artist",
            Some(&missing_template)
        )
        .is_err());
    }

    #[test]
    fn test_is_metadata() {
        assert!(!is_metadata("wav"));
        assert!(!is_metadata("flac"));
        assert!(is_metadata("asd"));
        assert!(is_metadata("reapeaks"));
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

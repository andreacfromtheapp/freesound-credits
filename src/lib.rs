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

    /// Include Zola frontmatter atop the markdown file
    #[arg(short, long)]
    pub zola: bool,
}

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

pub fn set_credits_header(song_title: &str) -> String {
    format!(
        "## Credits

*{song_title}* includes the following samples from
[Freesound](https://freesound.org). Used under a [Creative
Commons](https://creativecommons.org) license:

",
    )
}

pub fn create_output_file(song_title: &str) -> Result<File, Error> {
    let credits_file = format!(
        "{}-credits.md",
        song_title.replace(&[' ', '\''][..], "-").to_lowercase()
    );
    let file = match File::create(&credits_file) {
        Ok(file) => file,
        Err(error) => panic!("Problem creating the file: {credits_file}. Error: {error}"),
    };
    Ok(file)
}

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

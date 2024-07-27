use clap::Parser;
use std::fs::File;
use std::io::{self, Write};
use std::iter::FromIterator;
use std::path::Path;

fn set_frontmatter(song_title: &str, song_date: &str, song_artist: &str) -> String {
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

fn set_credits_header(song_title: &str) -> String {
    format!(
        "## Credits

*{song_title}* includes the following samples from
[Freesound](https://freesound.org). Used under a [Creative
Commons](https://creativecommons.org) license:

",
    )
}

fn create_output_file(song_title: &str) -> Result<File, io::Error> {
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

fn get_list_of_samples(samples_path: &str) -> Vec<String> {
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

fn set_credit_line(line: &str) -> String {
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

/// Simple program to generate Freesound credits in a usable markdown file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the samples directory
    #[arg(short, long)]
    path: String,

    /// Song title (quote multiple words)
    #[arg(short, long)]
    title: String,

    /// Song release date (quote multiple words)
    #[arg(short, long)]
    date: String,

    /// Song artist (quote multiple words)
    #[arg(short, long)]
    artist: String,

    /// Include Zola frontmatter atop the markdown file
    #[arg(short, long)]
    zola: bool,
}

fn main() {
    let args = Args::parse();

    if let Ok(mut output) = create_output_file(&args.title) {
        if args.zola {
            let frontmatter: String = set_frontmatter(&args.title, &args.date, &args.artist);
            write!(output, "{}", frontmatter).expect("Error: I could not write the frontmatter");
        }

        let header: String = set_credits_header(&args.title);
        write!(output, "{}", header).expect("Error: I could not write the header");

        for line in get_list_of_samples(&args.path).iter() {
            let credit_line = set_credit_line(line);
            write!(output, "{}", credit_line).expect("Error: I could not write the credit");
        }

        writeln!(output).expect("Error: I could not write the trailing white line");
    }
}

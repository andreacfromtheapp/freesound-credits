use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process;

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
        "## Credits\n
*{song_title}* includes the following samples from 
[Freesound](https://freesound.org). Used under a
[Creative Commons](https://creativecommons.org)
[CC-BY](https://creativecommons.org/licenses/by/4.0/) or
[CC-0](https://creativecommons.org/publicdomain/zero/1.0/) license:\n
",
    )
}

fn create_output_file(song_title: &str) -> Result<File, io::Error> {
    let credits_file = format!("{}_credits.md", song_title.replace(' ', "_"),);
    let file = match File::create(&credits_file) {
        Ok(file) => file,
        Err(error) => panic!("Problem creating the file: {credits_file}. Error: {error}"),
    };
    Ok(file)
}

fn get_list_of_samples(samples_path: &String) -> Vec<String> {
    let path = Path::new(&samples_path);
    let mut samples_raw_vector: Vec<String> = vec![];

    for entry in path.read_dir().expect("read_dir call failed").flatten() {
        if entry.path().is_file() || entry.path().is_dir() {
            let mut sample = format!("{:?}", entry.path().file_name().unwrap());
            sample = sample.replace(&['(', ')', '"'][..], "");

            // this is specific to Ableton projects
            if let Some(extension) = entry.path().extension() {
                if extension != "asd" && sample.chars().next().unwrap().is_numeric() {
                    samples_raw_vector.push(sample);
                }
            // this is specific to Renoise projects
            } else if sample.contains("Instrument") {
                sample = sample.split(' ').last().unwrap().to_string();
                if sample.chars().next().unwrap().is_numeric() {
                    samples_raw_vector.push(sample);
                }
            }
        }
    }
    samples_raw_vector
}

fn usage() {
    println!(
        "
usage: 
        freesound_samples_credit <samples path> <song title> <song date> <artist>

<samples path> a valid path to the samples directory.
<song title> e.g: \"Amazing song\"
<song date> e.g: \"2017-10-28\"
<artist> . e.g: \"Amazing Artist\"
"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let samples_path;
    let song_title;
    let song_date;
    let song_artist;

    let mut credit_line: Vec<&str> = vec![];

    match args.len() - 1 {
        4 => {
            samples_path = &args[1];
            song_title = &args[2];
            song_date = &args[3];
            song_artist = &args[4];
        }
        _ => {
            usage();
            // exit 0
            process::exit(0x0100);
        }
    };

    if let Ok(mut output) = create_output_file(song_title) {
        let frontmatter: String = set_frontmatter(song_title, song_date, song_artist);
        let header: String = set_credits_header(song_title);

        write!(output, "{}", frontmatter).expect("NO FRONTMATTER");
        write!(output, "{}", header).expect("NO HEADER");

        // iterate the samples_raw_vector for each sample entry
        for line in get_list_of_samples(samples_path).iter() {
            if line.contains("__") {
                credit_line = line.split("__").collect();
            } else if line.contains('_') {
                credit_line = line.split('_').collect();
            }

            // use the ID and Artist and Sample names to compose the credit URL
            let credit_line = format!(
                "- <https://freesound.org/people/{}/sounds/{}/>\n",
                credit_line[1], credit_line[0]
            );
            write!(output, "{}", credit_line).expect("NO CREDITS")
        }
        writeln!(output).expect("NO NEW LINE")
    }
}

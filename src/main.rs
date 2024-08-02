use clap::Parser;
use freesound_credits::{
    get_list_of_samples, set_credit, set_filename, set_frontmatter, set_header, Args,
};
use std::fs::File;
use std::io::Write;
use std::process;

fn main() {
    let args = Args::parse();

    let mut output = File::create(set_filename(&args.title)).unwrap_or_else(|error| {
        eprintln!("Problem creating the credits output file: {error}");
        process::exit(1);
    });

    if args.zola {
        write!(
            output_file,
            "{}",
            set_frontmatter(&args.title, &args.date, &args.artist)
        )
        .unwrap_or_else(|error| {
            eprintln!("Problem writing the Zola frontmatter: {error}");
            process::exit(2);
        });
    }

    write!(output_file, "{}", set_header(&args.title)).unwrap_or_else(|error| {
        eprintln!("Problem writing the credits header: {error}");
        process::exit(2);
    });

    get_list_of_samples(&args.path).iter().for_each(|line| {
        write!(output_file, "{}", set_credit(line)).unwrap_or_else(|error| {
            eprintln!("Problem writing the sample credit: {error}");
            process::exit(2);
        });
    });

    writeln!(output_file).unwrap_or_else(|error| {
        eprintln!("Problem writing the trailing white line: {error}");
        process::exit(2);
    });
}

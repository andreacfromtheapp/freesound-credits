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
            output,
            "{}",
            set_frontmatter(&args.title, &args.date, &args.artist)
        )
        .expect("Error: I could not write the Zola frontmatter");
    }

    write!(output, "{}", set_header(&args.title))
        .expect("Error: I could not write the credits header");

    get_list_of_samples(&args.path).iter().for_each(|line| {
        write!(output, "{}", set_credit(line)).expect("Error: I could not write the sample credit");
    });

    writeln!(output).expect("Error: I could not write the trailing white line");
}

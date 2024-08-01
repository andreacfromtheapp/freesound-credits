use clap::Parser;
use freesound_credits::{
    create_output_file, get_list_of_samples, set_credit_line, set_credits_header, set_frontmatter,
    Args,
};
use std::io::Write;

fn main() {
    let args = Args::parse();

    if let Ok(mut output) = create_output_file(&args.title) {
        if args.zola {
            write!(
                output,
                "{}",
                set_frontmatter(&args.title, &args.date, &args.artist)
            )
                .expect("Error: I could not write the Zola frontmatter");
        }

        write!(output, "{}", set_credits_header(&args.title))
            .expect("Error: I could not write the credits header");

        get_list_of_samples(&args.path).iter().for_each(|line| {
            write!(output, "{}", set_credit_line(line))
                .expect("Error: I could not write the sample credit");
        });

        writeln!(output).expect("Error: I could not write the trailing white line");
    }
}

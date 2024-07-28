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
            let zola_frontmatter: String = set_frontmatter(&args.title, &args.date, &args.artist);
            write!(output, "{}", zola_frontmatter)
                .expect("Error: I could not write the Zola frontmatter");
        }

        let credits_header: String = set_credits_header(&args.title);
        write!(output, "{}", credits_header).expect("Error: I could not write the credits header");

        for line in get_list_of_samples(&args.path).iter() {
            let credit_line = set_credit_line(line);
            write!(output, "{}", credit_line).expect("Error: I could not write the sample credit");
        }

        writeln!(output).expect("Error: I could not write the trailing white line");
    }
}

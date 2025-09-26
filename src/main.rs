use clap::Parser;
use freesound_credits::{cli, error::AppError, run_app};

fn main() -> Result<(), AppError> {
    let cli = cli::Cli::parse();
    run_app(&cli)
}

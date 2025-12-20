use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "fix_local_mail", author = "C. Pospiech", version = "0.1", about = "fix local mail folders", long_about = None)]
pub struct CliArgs {
    /// Perform a dry run without making actual changes
    #[arg(short = 'D', long, default_value_t = false)]
    pub dry_run: bool,

    /// Stop Kmail before and Akonadi after processing
    #[arg(short = 'A', long, default_value_t = false)]
    pub stop_akonadi: bool,

    /// Verbose output
    #[arg(short = 'v', long, default_value_t = false)]
    pub verbose: bool,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}

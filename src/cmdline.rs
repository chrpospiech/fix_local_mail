use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "fix_local_mail", author = "C. Pospiech", version = "1.0", about = "fix local mail folders", long_about = None)]
pub struct CliArgs {
    /// Perform a dry run without making actual changes
    #[arg(short = 'D', long, default_value_t = false)]
    pub dry_run: bool,

    /// Stop Kmail and Akonadi after processing
    #[arg(short = 'a', long, default_value_t = false)]
    pub stop_akonadi: bool,

    /// Stop Kmail after processing
    #[arg(short = 'k', long, default_value_t = false)]
    pub stop_kmail: bool,

    /// Verbose output
    #[arg(short = 'v', long, default_value_t = false)]
    pub verbose: bool,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}

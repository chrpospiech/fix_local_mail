use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "fix_local_mail", author = "C. Pospiech", version = "0.1", about = "fix local mail folders", long_about = None)]
pub struct CliArgs {
    /// Perform a dry run without making actual changes
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// Stop KMail before processing
    #[arg(long, default_value_t = false)]
    pub stop_kmail: bool,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}

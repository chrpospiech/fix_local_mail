use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(name = "fix_local_mail", author = "C. Pospiech", version = "1.2", about = "fix local mail folders", long_about = None)]
pub struct CliArgs {
    /// Perform a dry run without making actual changes
    #[arg(short = 'D', long, default_value_t = false)]
    pub dry_run: bool,

    /// Limit the number of processed messages
    #[arg(short = 'n', long, default_value_t = 0)]
    pub limit: usize,

    /// Minimum message ID to process
    #[arg(short = 'm', long, default_value_t = 0)]
    pub min_id: i64,

    /// maildir root path if not determined automatically
    #[arg(short = 'p', long, default_value = "auto")]
    pub maildir_path: String,

    /// mail cache path if not determined automatically
    #[arg(short = 'c', long, default_value = "auto")]
    pub mail_cache_path: String,

    /// Database URL if not determined automatically
    #[arg(short = 'u', long, default_value = "auto")]
    pub db_url: String,

    /// Ignore list of mails in new directories
    #[arg(short = 'i', long, default_value_t = false)]
    pub ignore_new_dirs: bool,

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

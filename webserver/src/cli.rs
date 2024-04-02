use clap::Parser;
use clap::Subcommand;

/// The cli of the questionnaire
#[derive(Parser)]
pub struct Cli {
    /// The path to the config file of questionnaire
    #[clap(long, default_value_t = String::from("/etc/{{crate-name}}/config.toml"))]
    pub config_path: String,

    /// The available subcommands
    #[clap(subcommand)]
    pub command: Command,
}

/// All available commands
#[derive(Subcommand)]
pub enum Command {
    /// Start the server
    Start,
    /// Run the migrations on the database
    Migrate {
        /// The directory where the migration files are located in
        migrations_dir: String,
    },
}

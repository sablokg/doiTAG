use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "doigen",
    version = "1.0",
    about = "doigen.
      ************************************************
      Author Gaurav Sablok,
      Email: gsablok@proton.me
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// generate doi for each sequences
    Doigen {
        /// provide ONT file
        pathfile: String,
        /// threads for the analysis
        thread: String,
    },
}

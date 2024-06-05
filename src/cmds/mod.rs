pub mod docker;

use clap::{command, Parser, Subcommand};
use docker::DockerCmd;

#[derive(Parser, Debug)]
pub struct Cmds {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// check docker images and check whether to delete interactively
    Docker(DockerCmd),
}

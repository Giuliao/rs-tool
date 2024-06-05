mod cmds;

use clap::Parser;
use cmds::{Cmds, Commands};

fn main() {
    let args = Cmds::parse();
    match args.command {
        Commands::Docker(docker_cmd) => {
            docker_cmd.run();
        }
    }
}

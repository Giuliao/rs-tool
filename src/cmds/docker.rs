use clap::{command, Parser};
use std::io::{stdin, BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct DockerCmd {}

impl DockerCmd {
    pub fn run(&self) {
        let process = Command::new("docker")
            .arg("images")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute command");

        let stdout = process.stdout.expect("Failed to open stdout");
        let reader = BufReader::new(stdout);

        for (idx, line) in reader.lines().into_iter().enumerate() {
            if idx == 0 {
                continue;
            }
            match line {
                Ok(line) => {
                    let line_arr = line.split_whitespace().collect::<Vec<&str>>();
                    let name = line_arr[0];
                    let version = line_arr[1];
                    let id = line_arr[2];

                    println!(
                        "Do you want to delete {}:{} ({}), y or n (default n)",
                        name, version, id
                    );
                    let mut input = String::new();
                    stdin().read_line(&mut input).expect("Failed to read line");
                    if input.trim().to_lowercase() == "y" {
                        let result = Command::new("docker")
                            .arg("rmi")
                            .arg(format!("{}:{}", name, version))
                            .output()
                            .expect("Failed to execute command");

                        if result.status.success() {
                            println!("Deleted {} ({})", name, id);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
            }
        }
    }
}

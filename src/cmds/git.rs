use clap::{command, Parser};
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct GitCmd {}

impl GitCmd {
    pub fn run(&self) {
        let process = Command::new("git")
            .arg("branch")
            .arg("-l")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute command");

        let process_stdout = process.stdout.expect("Failed to open stdout");
        let reader = BufReader::new(process_stdout);

        for (_, line) in reader.lines().into_iter().enumerate() {
            match line {
                Ok(line) => {
                    let line_arr = line.split_whitespace().collect::<Vec<&str>>();
                    let wait_user_fn = || {
                        let mut input = String::new();
                        stdin().read_line(&mut input).expect("Failed to read line");
                    };
                    match line_arr.as_slice() {
                        [branch_name] => {
                            self.execute_git_delete(branch_name);
                        }
                        ["*", branch_name] => {
                            print!("can not delete current branch {}", branch_name);
                            stdout().flush().unwrap();
                            wait_user_fn();
                        }
                        ["+", branch_name] => {
                            print!("cannot not delete work tree current branch {}", branch_name);
                            stdout().flush().unwrap();
                            wait_user_fn();
                        }
                        _ => {
                            print!("unknown line: {}", line);
                            stdout().flush().unwrap();
                            wait_user_fn();
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

    fn execute_git_delete(&self, branch_name: &str) {
        print!("Do you want to delete {}, y or n (default n)", branch_name);
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().to_lowercase() == "y" {
            let result = Command::new("git")
                .arg("branch")
                .arg("-D")
                .arg(format!("{}", branch_name))
                .output()
                .expect("Failed to execute command");

            if result.status.success() {
                println!("Deleted {}", branch_name);
                stdout().flush().unwrap();
            }
        }
    }
}

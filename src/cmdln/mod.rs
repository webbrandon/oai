use std::process::{Output, Command};

#[derive(Debug, Default, Clone)]
pub struct CommandLineHandler {
    exit_on_error: bool
}

impl CommandLineHandler {
    pub fn new() -> CommandLineHandler {
        trace!("new command handler");
        Default::default()
    }

    pub fn set_exit_on_error(&mut self, toggle: bool) {
        trace!("set_exit_on_error toggle");
        self.exit_on_error = toggle;
    }

    pub fn run_cmd(&mut self, cmd: &str) -> String {
        trace!("run command");
        let output = if cfg!(target_os = "windows") {
                Command::new("cmd")
                        .args(["/C", cmd])
                        .output()
                        .expect("failed to execute process")
            } else {
                Command::new("sh")
                        .arg("-c")
                        .arg(cmd)
                        .output()
                        .expect("failed to execute process")
            };
        self.process_cmd_result(output)
    }

    fn stderr_handler(&mut self, output: &Output, response: &mut String) {
        trace!("stderr handler");
        if !output.stderr.is_empty() {
            match String::from_utf8(output.stderr.to_owned()) {
                Ok(x) => {
                    if self.exit_on_error {
                        error!("{}", x);
                    } else {
                        response.push_str(&x);
                    }
                },
                Err(e) => error!("{:#?}", e),
            }
        }
    }

    fn stdout_handler(&mut self, output: &Output, response: &mut String) {
        trace!("stdout handler");
        if !output.stdout.is_empty() {
            match String::from_utf8(output.stdout.to_owned()) {
                Ok(x) => {
                    if self.exit_on_error {
                        error!("{}", x);
                    } else {
                        response.push_str(&x);
                    }
                },
                Err(e) => error!("{:#?}", e),
            }
        }
    }

    pub fn process_cmd_result(&mut self, output: Output) -> String {
        trace!("process command result");
        if output.status.success() {
            String::from_utf8_lossy(&output.stdout).to_string()
        } else {
            let mut response = String::new();
            self.stderr_handler(&output, &mut response);
            self.stdout_handler(&output, &mut response);

            if self.exit_on_error {
                std::process::exit(1);
            } else {
                response
            }

        }
    }

}

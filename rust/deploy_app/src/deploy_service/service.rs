use rccn_usr::service::{AcceptanceResult, AcceptedTc, PusService};
use super::command::Command;

pub struct DeployService {}

impl DeployService {
    pub fn new() -> Self {
        Self {}
    }

    fn control_antenna(&self, action: &str, num: u8) -> bool {
        match std::process::Command::new("python3")
            .args(&["/usr/bin/antenna_control.py", action, &format!("{num}")])
            .status()
        {
            Ok(status) => {
                status.code().unwrap_or(1) == 0
            }
            Err(_) => false
        }
    }
}

impl PusService for DeployService {
    type CommandT = Command;

    fn service() -> u8 {
        136
    }

    fn handle_tc(&mut self, tc: AcceptedTc, cmd: Self::CommandT) -> AcceptanceResult {
        match cmd {
            Command::DeployAntenna(args) => tc.handle(|| {
                println!("deploying antenna {}", args.antenna_number);
                self.control_antenna("deploy", args.antenna_number)
            }),
            Command::RetractAntenna(args) => tc.handle(|| {
                println!("retracting antenna {}", args.antenna_number);
                self.control_antenna("retract", args.antenna_number)
            })
        }
    }
}

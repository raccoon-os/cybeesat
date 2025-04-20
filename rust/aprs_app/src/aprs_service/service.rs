use std::{
    path::Path,
    sync::{
        Arc, Mutex,
        mpsc::{Sender, channel},
    },
    thread::JoinHandle,
};

use rccn_usr::service::PusService;

use super::{
    command::{
        self,
        Command::{DangerSetArbitraryCommand, GetTelemetry, SetBeaconInterval, SetBeaconMessage, SetCallsign, SetPowerMode},
    }, handler, telemetry::APRSTelemetry
};

pub struct APRSService {
    telemetry: Arc<Mutex<APRSTelemetry>>,
    cmd_tx: Sender<String>,
    serial_thread: JoinHandle<()>,
}

impl APRSService {
    pub fn new(serial_path: &Path) -> std::io::Result<Self> {
        let telemetry = Arc::new(Mutex::new(APRSTelemetry::default()));
        let (cmd_tx, cmd_rx) = channel();
        Ok(Self {
            serial_thread: handler::spawn(serial_path, telemetry.clone(), cmd_rx)?,
            telemetry,
            cmd_tx,
        })
    }

    fn send_cmd(&self, cmd: String) -> bool {
        self.cmd_tx.send(cmd).is_ok()
    }
}

impl PusService for APRSService {
    type CommandT = command::Command;

    fn service() -> u8 {
        142
    }

    fn handle_tc(
        &mut self,
        mut tc: rccn_usr::service::AcceptedTc,
        cmd: Self::CommandT,
    ) -> rccn_usr::service::AcceptanceResult {
        match cmd {
            SetPowerMode(args) => tc.handle(|| {
                let mut ok = true;
                ok &= self
                    .send_cmd(format!(
                        "AT+PWR={}\r\n",
                        if args.high_power_mode { "H" } else { "L" }
                    ));
                ok &= self
                    .send_cmd(format!(
                        "AT+DIGI1={}\r\n",
                        if args.digi1_enable { "ON" } else { "OFF" }
                    ));
                ok &= self
                    .send_cmd(format!(
                        "AT+DIGI2={}\r\n",
                        if args.digi2_enable { "ON" } else { "OFF" }
                    ));

                ok
            }),
            SetBeaconInterval(args) => tc.handle(|| {
                self.send_cmd(format!("AT+BTIME={}\r\n", args.beacon_interval))
            }),
            SetBeaconMessage(args) => tc.handle(|| {
                self.send_cmd(format!("AT+BEACON={}\r\n", args.beacon_message))
            }),
            SetCallsign(args) => tc.handle(|| {
                self.send_cmd(format!("AT+CALL={}\r\n", args.callsign))
            }),
            GetTelemetry => tc.handle_with_tm(|| {
                if false {
                    return Err(());
                }

                let tm = self.telemetry.lock().unwrap();
                Ok(tm.clone())
            }),
            DangerSetArbitraryCommand(args) => tc.handle(|| {
                self.send_cmd(format!("{}\r\n", args.command))
            })
        }
    }
}

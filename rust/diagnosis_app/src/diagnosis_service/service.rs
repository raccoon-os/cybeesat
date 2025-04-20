use std::fs::{self};
use std::process::Command;

use rccn_usr::service::{AcceptanceResult, AcceptedTc, PusService};

use super::command::Command::{LaPuertaTrasera, ScanI2C};
use super::telemetry::RespuestaDeLaPuertaTrasera;
use super::{command, telemetry::ScanI2CResponse};
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;

pub struct DiagnosisService {
    last_transaction_id: u8,
}

impl DiagnosisService {
    pub fn new() -> Self {
        Self {
            last_transaction_id: 0,
        }
    }
}

impl PusService for DiagnosisService {
    type CommandT = command::Command;

    fn service() -> u8 {
        137
    }

    fn handle_tc(&mut self, mut tc: AcceptedTc, cmd: Self::CommandT) -> AcceptanceResult {
        match cmd {
            ScanI2C(args) => tc.handle_with_tm(|| {
                if false {
                    return Err(());
                }

                let mut devices: Vec<u8> = Vec::new();
                for addr in 0..=127u8 {
                    let mut dev =
                        match LinuxI2CDevice::new(format!("/dev/i2c-{}", args.bus), addr.into()) {
                            Err(e) => {
                                eprintln!("error creating i2c dev {e:?}");
                                return Err(());
                            }
                            Ok(dev) => dev,
                        };
                    let found = if (addr >= 0x30 && addr <= 0x37) || (addr >= 0x50 && addr <= 0x57)
                    {
                        match dev.smbus_read_byte() {
                            Ok(_) => true,
                            Err(_) => false,
                        }
                    } else {
                        match dev.smbus_write_quick(false) {
                            Ok(_val) => true,
                            Err(_) => false,
                        }
                    };

                    if found {
                        println!("bus {} addr {} found", args.bus, addr);
                        devices.push(addr);
                    } else {
                        println!("bus {} addr {} NOT found", args.bus, addr);
                    }
                }

                Ok(ScanI2CResponse {
                    bus: args.bus,
                    devices: devices,
                })
            }),

            // Dear reader of this code: I know this command is a terrible idea from a security
            // perspective.
            // We made a conscious decision to add this command because there are many unknown things
            // that could happen in the satellite, that could be fixed by arbitrary command execution.
            // This command will be authenticated with SDLS in the future.
            LaPuertaTrasera(args) => tc.handle(|| {
                println!("LPT args: {args:?}");

                if args.transaction_id == self.last_transaction_id {
                    return false;
                }

                let password =
                    fs::read_to_string("/var/puerta_password").unwrap_or("changeme".to_string());
                if args.contraseña != password {
                    return false;
                }

                self.last_transaction_id = args.transaction_id;

                let res = Command::new("sh").arg("-c").arg(args.orden).output();

                let mut tm_sender = tc.base.clone();

                match res {
                    Ok(output) => {
                        let mut respuesta = String::new();
                        if output.stdout.len() > 0 {
                            respuesta.push_str(std::str::from_utf8(&output.stdout).unwrap());
                        }
                        if output.stderr.len() > 0 {
                            respuesta.push_str("\n--\n");
                            respuesta.push_str(std::str::from_utf8(&output.stderr).unwrap());
                        }

                        println!("respuesta '{respuesta}'");

                        for (i, chunk) in respuesta.as_bytes().chunks(150).enumerate() {
                            let chunk_str = String::from_utf8_lossy(chunk);
                            tm_sender.send_tm(RespuestaDeLaPuertaTrasera {
                                respuesta: chunk_str.to_string(),
                                chunk: i as u8,
                                transaction_id: args.transaction_id,
                            });
                        }

                        true
                    }
                    Err(e) => {
                        println!("error {e:?}");
                        return false;
                    }
                }
            }),
        }
    }
}

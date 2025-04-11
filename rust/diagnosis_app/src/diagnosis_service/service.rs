use rccn_usr::service::{AcceptanceResult, AcceptedTc, PusService};

use super::{command, telemetry::ScanI2CResponse};
use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;

pub struct DiagnosisService {
}

impl DiagnosisService {
    pub fn new() -> Self {
        Self {}
    }
}

impl PusService for DiagnosisService {
    type CommandT = command::Command;

    fn service() -> u8 {
        137
    }

    fn handle_tc(&mut self, mut tc: AcceptedTc, cmd: Self::CommandT) -> AcceptanceResult {
        match cmd {
            command::Command::ScanI2C(args) => tc.handle_with_tm(|| {
                if false {
                    return Err(());
                }

                let mut devices: Vec<u8> = Vec::new();
                for addr in 0..=127u8 {
                    let mut dev = match LinuxI2CDevice::new(format!("/dev/i2c-{}", args.bus), addr.into()) {
                        Err(e) => {
                            eprintln!("error creating i2c dev {e:?}");
                            return Err(());
                        },
                        Ok(dev) => dev
                    };
                    let found = if (addr >= 0x30 && addr <= 0x37) || (addr >= 0x50 && addr <= 0x57) {
                        match dev.smbus_read_byte() {
                            Ok(_) => true,
                            Err(_) => false                        
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

                Ok(ScanI2CResponse { bus: args.bus, devices: devices })
            })
        }
    }
}
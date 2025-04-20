use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use log::{error, info, warn};
use crate::controll::service::{write_i2c_ina_device_block, convert_battery_voltage, convert_battery_charge_current, convert_bus_voltage};
use crate::rtc::config::BixConfig;

use std::thread::{self, sleep};
use std::time::Duration;
use std::sync::mpsc;

use linux_embedded_hal::
    gpio_cdev::{Chip, LineRequestFlags}
;

use crate::rtc::service::reset_obc;

use chrono::Utc;

pub fn spawn(switch_obc_receiver: mpsc::Receiver<bool>) {
    let mut dev_fuel_gauge = match LinuxI2CDevice::new("/dev/i2c-0", 0x36) {
        Err(e) => {
            panic!("error creating i2c dev {e:?}")
        },
        Ok(dev) => {dev}
    };

    let mut dev_i2c_switch = match LinuxI2CDevice::new("/dev/i2c-0", 0x70) {
        Err(e) => {
            panic!("error creating i2c dev {e:?}")
        },
        Ok(dev) => {dev}
    };

    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    let gpioPC17_handle = chip.get_line(81).unwrap();
    let linePC17_handle = gpioPC17_handle.request(LineRequestFlags::OUTPUT, 0, "Watchdog trigger").unwrap();
    
    let mut switch_obc = false; 

    info!("Before spawning Monitor Task.");
    thread::spawn(move || {
        info!("Spawned Monitor Task.");
        loop{

            sleep(Duration::from_millis(5000)); // todo configure

            // Switch i2c Bus
            match write_i2c_ina_device_block(&mut dev_i2c_switch, 0x00, 0x04){ //0x05 for other i2c
                Ok(_) => {},
                Err(e) => {warn!("Error while setting INA device to PMIC0");}
            } 

            let vbat0_raw = dev_fuel_gauge.smbus_read_i2c_block_data(0x09, 2).map(|data| (((data[1] as u16) << 8) | (data[0] & 0xff) as u16)).unwrap_or(0);
            let vbat0 = (vbat0_raw as f32) * 78.125e-3;

            // Switch i2c Bus
            match write_i2c_ina_device_block(&mut dev_i2c_switch, 0x00, 0x04){ //0x05 for other i2c
                Ok(_) => {},
                Err(e) => {warn!("Error while setting INA device to PMIC0");}
            } 

            let vbat1_raw = dev_fuel_gauge.smbus_read_i2c_block_data(0x09, 2).map(|data| (((data[1] as u16) << 8) | (data[0] & 0xff) as u16)).unwrap_or(0);
            let vbat1 = (vbat1_raw as f32) * 78.125e-3;

            info!("vbat0: {vbat0}, vbat1: {vbat1}");

            match switch_obc_receiver.try_recv(){
                Ok(val ) => switch_obc = val,
                Err(_) => { }
            }


            let dms_should_reset_satellite = match BixConfig::load_or_default() {
                Ok(mut time_config) => {
                    if time_config.next_reset == 0 {
                        warn!("DMS: no next reset configured");
                        false
                    } else {
                        let next_reset = chrono::DateTime::from_timestamp(time_config.next_reset, 0).unwrap().to_utc();
                        let now = Utc::now();

                        if now > next_reset {
                            time_config.next_reset = (now + Duration::from_secs(time_config.reset_interval)).timestamp();
                            info!("DMS next reset: {}, now: {}", time_config.next_reset, now);
                            if let Err(e) = time_config.save() {
                                error!("error saving config {e:?}");
                            }
                            true
                        } else {
                            false
                        }
                    }
                },
                Err(e) => {
                    error!("Error loading config, not doing DMS");
                    false
                }
            };

            if dms_should_reset_satellite {
                info!("DMS will reset the satellite now");
                sleep(Duration::from_millis(1000));

                let _ = reset_obc();
            }

            if ((vbat0 > 3000.0) || (vbat1 > 3000.0)) && !switch_obc{
                linePC17_handle.set_value(1).unwrap();
                sleep(Duration::from_millis(100)); // todo configure
                linePC17_handle.set_value(0).unwrap();
                // PC17 Trigger Line 81
                // PC 24 on high
            } else {
                warn!("Not triggering watchdog");
            }
        }
    });

}
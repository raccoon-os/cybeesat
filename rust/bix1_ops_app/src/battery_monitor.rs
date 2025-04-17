use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::{I2CDevice};
use log::{debug, info, warn};
use crate::controll::service::{write_i2c_ina_device_block, convert_battery_voltage, convert_battery_charge_current, convert_bus_voltage};

use std::thread::{self, sleep};
use std::time::Duration;

use linux_embedded_hal::{
    gpio_cdev::{Chip, EventRequestFlags, Line, LineRequestFlags}, spidev::SpidevOptions, CdevPin, Delay, SpidevDevice
};

pub fn spawn() {
    let mut dev_pmic = match LinuxI2CDevice::new("/dev/i2c-0", 0x6b) {
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

            // Activate ADC
            let mut pmic0_reg02 =  dev_pmic.smbus_read_i2c_block_data(0x02, 1).unwrap();
            // debug!("pmic0_reg02: {:?}", pmic0_reg02);
            pmic0_reg02[0] |= 0xc0;
            let mut write_val= [0u8];
            write_val[0] = pmic0_reg02[0];
            dev_pmic.smbus_write_i2c_block_data(0x02, &write_val).unwrap();
            
            let mut pmic0_reg02_test2 =  dev_pmic.smbus_read_i2c_block_data(0x02, 1).unwrap();
            // debug!("pmic0_reg02: {:?}", pmic0_reg02_test2);

            // read Values
            let pmic0_vbat_vec = dev_pmic.smbus_read_i2c_block_data(0x0e, 1).unwrap(); 
            // debug!("pmic0_vbat_vec: {:X?}", pmic0_vbat_vec);

            let pmic0_vbat = convert_battery_voltage(pmic0_vbat_vec[0] as u32);

            // debug!("pmic_vbat0: {:?}", pmic0_vbat);

            let pmic0_vsys_vec = dev_pmic.smbus_read_i2c_block_data(0x11, 1).unwrap(); 
            // debug!("pmic0_vsys_vec: {:X?}", pmic0_vsys_vec);

            let pmic0_vsys = convert_bus_voltage(pmic0_vsys_vec[0] as u32);

            // debug!("pmic_vsys: {:?}", pmic0_vsys);

            let pmic0_ichg_vec = dev_pmic.smbus_read_i2c_block_data(0x12, 1).unwrap(); 
            // debug!("pmic0_ichg_vec: {:X?}", pmic0_ichg_vec);

            let pmic0_ichg = convert_battery_charge_current(pmic0_ichg_vec[0] as u32);

            // debug!("pmic0_ichg: {:?}", pmic0_ichg);

            let pmic0_status_vec = dev_pmic.smbus_read_i2c_block_data(0x0b, 1).unwrap(); 
            // debug!("pmic0_status_vec: {:X?}", pmic0_status_vec);
            // debug!("pmic1_status_conv: {:?}", (pmic0_status_vec[0] >> 3) & 0b11);

            // Switch i2c Bus
            match write_i2c_ina_device_block(&mut dev_i2c_switch, 0x00, 0x04){ //0x05 for other i2c
                Ok(_) => {},
                Err(e) => {warn!("Error while setting INA device to PMIC0");}
            } 

            // Activate ADC
            let mut pmic1_reg02 =  dev_pmic.smbus_read_i2c_block_data(0x02, 1).unwrap();
            // debug!("pmic1_reg02: {:?}", pmic1_reg02);
            pmic1_reg02[0] |= 0xc0;
            let mut write_val1= [0u8];
            write_val1[0] = pmic1_reg02[0];
            dev_pmic.smbus_write_i2c_block_data(0x02, &write_val1).unwrap();
            
            let mut pmic1_reg02_test2 =  dev_pmic.smbus_read_i2c_block_data(0x02, 1).unwrap();
            // debug!("pmic1_reg02: {:?}", pmic1_reg02_test2);

            // read Values
            let pmic1_vbat_vec = dev_pmic.smbus_read_i2c_block_data(0x0e, 1).unwrap(); 
            // debug!("pmic1_vbat_vec: {:X?}", pmic1_vbat_vec);

            let pmic1_vbat = convert_battery_voltage(pmic1_vbat_vec[0] as u32);

            // debug!("pmic1_vbat0: {:?}", pmic1_vbat);

            let pmic1_vsys_vec = dev_pmic.smbus_read_i2c_block_data(0x11, 1).unwrap(); 
            // debug!("pmic1_vsys_vec: {:X?}", pmic1_vsys_vec);

            let pmic1_vsys = convert_bus_voltage(pmic1_vsys_vec[0] as u32);

            // debug!("pmic1_vsys: {:?}", pmic1_vsys);

            let pmic1_ichg_vec = dev_pmic.smbus_read_i2c_block_data(0x12, 1).unwrap(); 
            // debug!("pmic1_ichg_vec: {:X?}", pmic1_ichg_vec);

            let pmic1_ichg = convert_battery_charge_current(pmic1_ichg_vec[0] as u32);

            // debug!("pmic1_ichg: {:?}", pmic1_ichg);

            let pmic1_status_vec = dev_pmic.smbus_read_i2c_block_data(0x0b, 1).unwrap(); 
            // debug!("pmic1_status_vec: {:X?}", pmic1_status_vec);
            // debug!("pmic1_status_conv: {:?}", (pmic1_status_vec[0] >> 3) & 0b11);


            if (pmic0_vbat > 3000) && (pmic1_vbat > 3000){
                linePC17_handle.set_value(1).unwrap();
                sleep(Duration::from_millis(100)); // todo configure
                linePC17_handle.set_value(0).unwrap();
                // PC17 Trigger Line 81
                // PC 24 on high
            }
        }
    });

}
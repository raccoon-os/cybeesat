use std::process::Command;
use num_traits::FromPrimitive;
use rccn_usr::service::{AcceptanceResult, AcceptedTc, PusService};
use super::command;
use anyhow::Result;
use std::error::Error;
use std::fs;
use super::telemetry;
use machine_info::Machine;
use i2cdev::linux::{LinuxI2CBus, LinuxI2CDevice};
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CError;
use thiserror::Error;
use log::{debug, info, kv::value, warn};
use std::{thread, time};

#[derive(Error, Debug)]
pub enum LocalError {
    #[error("Error during Register Conversion: {0}")]
    RegConvError(String),
    // #[error("YAML parsing error: {0}")]
    // Yaml(#[from] serde_yaml::Error),
    // #[error("Config validation error: {0}")]
    // Validation(String),
    // #[error("Configuration file not found under path: {0}")]
    // ConfigNotFound(String)
}

#[repr(u8)]
enum InaRegisters{
    Config = 0x00,
    Calibration = 0x16,
    BusVoltage = 0x04,
    BusCurrent = 0x06,
    GPIO = 0x14,
}

const INA209DelayMicroSeconds: u64 = 100000;

pub struct EpsCtrlService{
    dev_ina_bus0_3v3: LinuxI2CDevice,
    dev_ina_bus1_3v3: LinuxI2CDevice,
    dev_ina_bus0_5v: LinuxI2CDevice,
    dev_ina_bus1_5v: LinuxI2CDevice,
    dev_ina_unreg_bus: LinuxI2CDevice,
    dev_ina_vsys_bus: LinuxI2CDevice,
    dev_ina_user_3v3: LinuxI2CDevice,
    dev_ina_user_5v: LinuxI2CDevice,
    dev_ina_user_unreg: LinuxI2CDevice,
    dev_i2c_switch: LinuxI2CDevice,
    dev_pmic: LinuxI2CDevice,
    dev_fuel_gauge: LinuxI2CDevice,
}

impl EpsCtrlService {
    pub fn new() -> Self {
        info!("Launched EpsCtrl Service.");

        let mut dev_ina_bus0_3v3 = match init_i2c_device("/dev/i2c-0", 0x46) {
            Err(e) => panic!("error creating i2c dev {e:?}"),
            Ok(dev) => dev
        };

        let mut dev_ina_bus1_3v3 = match init_i2c_device("/dev/i2c-0", 0x47) {
            Err(e) => panic!("error creating i2c dev {e:?}"),
            Ok(dev) => dev
        };

        let mut dev_ina_bus0_5v = match init_i2c_device("/dev/i2c-0", 0x40) {
            Err(e) => panic!("error creating i2c dev {e:?}"),
            Ok(dev) => dev
        };

        let mut dev_ina_bus1_5v = match init_i2c_device("/dev/i2c-0", 0x49) {
            Err(e) => panic!("error creating i2c dev {e:?}"),
            Ok(dev) => dev
        };

        let mut dev_ina_unreg_bus = match init_i2c_device("/dev/i2c-0", 0x44) {
            Err(e) => panic!("error creating i2c dev {e:?}"),
            Ok(dev) => dev
        };

        let mut dev_ina_vsys_bus = match init_i2c_device("/dev/i2c-0", 0x4c) {
            Err(e) => panic!("error creating i2c dev {e:?}"),
            Ok(dev) => dev
        };

        let mut dev_ina_user_3v3 = match init_i2c_device("/dev/i2c-0", 0x4a) {
            Err(e) => panic!("error creating i2c dev {e:?}"),
            Ok(dev) => dev
        };

        let mut dev_ina_user_5v = match init_i2c_device("/dev/i2c-0", 0x4b) {
            Err(e) => panic!("error creating i2c dev {e:?}"),
            Ok(dev) => dev
        };

        let mut dev_ina_user_unreg = match init_i2c_device("/dev/i2c-0", 0x4b) {
            Err(e) => panic!("error creating i2c dev {e:?}"),
            Ok(dev) => dev
        };

        let mut dev_i2c_switch = match LinuxI2CDevice::new("/dev/i2c-0", 0x70) {
            Err(e) => {
                panic!("error creating i2c dev {e:?}")
            },
            Ok(dev) => {dev}
        };

        let mut dev_pmic = match LinuxI2CDevice::new("/dev/i2c-0", 0x6b) {
            Err(e) => {
                panic!("error creating i2c dev {e:?}")
            },
            Ok(dev) => {dev}
        };

        let mut dev_fuel_gauge = match LinuxI2CDevice::new("/dev/i2c-0", 0x36) {
                Err(e) => {
                    panic!("error creating i2c dev {e:?}")
                },
                Ok(dev) => {dev}
            };

        Self { 
            dev_ina_bus0_3v3: dev_ina_bus0_3v3,
            dev_ina_bus1_3v3: dev_ina_bus1_3v3,
            dev_ina_bus0_5v: dev_ina_bus0_5v,
            dev_ina_bus1_5v: dev_ina_bus1_5v,
            dev_ina_unreg_bus: dev_ina_unreg_bus,
            dev_ina_vsys_bus: dev_ina_vsys_bus,
            dev_ina_user_3v3: dev_ina_user_3v3,
            dev_ina_user_5v: dev_ina_user_5v,
            dev_ina_user_unreg: dev_ina_user_unreg,
            dev_i2c_switch: dev_i2c_switch,
            dev_pmic: dev_pmic,
            dev_fuel_gauge: dev_fuel_gauge
        }
    }
}

fn init_i2c_device(path: &str, address: u16) -> Result<LinuxI2CDevice, LinuxI2CError>{
    let mut dev_ina = match LinuxI2CDevice::new(path, address) {
        Err(e) => {
            return Err(e)
        },
        Ok(dev) => {dev}
    };
    write_i2c_ina_device_block(&mut dev_ina, InaRegisters::Config as u8,      0x1FFF).unwrap();
    write_i2c_ina_device_block(&mut dev_ina, InaRegisters::Calibration as u8, 0x2000).unwrap();
    return Ok(dev_ina)
}

fn write_i2c_device_register_block(address: u16, register: u8, value: u16) -> Result<(), LinuxI2CError>{
    let mut dev_ina = match LinuxI2CDevice::new("/dev/i2c-0", address) {
        Err(e) => {
            return Err(e)
        },
        Ok(dev) => {dev}
    };

    let values: [u8; 2] = [(value >> 8) as u8, (value & 0xFF) as u8];
    let res = dev_ina.smbus_write_i2c_block_data(register as u8, &values);
    thread::sleep(time::Duration::from_micros(INA209DelayMicroSeconds));
    Ok(())
}

fn read_i2c_device_register_block(address: u16, register: u8) -> Result<u16, LinuxI2CError>{
    let mut dev_ina = match LinuxI2CDevice::new("/dev/i2c-0", address) {
        Err(e) => {
            return Err(e)
        },
        Ok(dev) => {dev}
    };

    let res = match dev_ina.smbus_read_i2c_block_data(register as u8, 2){
        Ok(values) => {return Ok((values[0] as u16)<<8 | values[1] as u16)},
        Err(e) => Err(e)

    };
    thread::sleep(time::Duration::from_micros(INA209DelayMicroSeconds));
    res

}

fn read_i2c_ina_device_block(dev: &mut LinuxI2CDevice, register: u8) -> Result<Vec<u8>, LinuxI2CError>{
    let res = dev.smbus_read_i2c_block_data(register, 2);
    thread::sleep(time::Duration::from_micros(INA209DelayMicroSeconds));
    res
}

fn write_i2c_ina_device_block(dev: &mut LinuxI2CDevice, register: u8, value: u16) -> Result<(), LinuxI2CError>{
    // dev.smbus_read_i2c_block_data(register, len)
    let values: [u8; 2] = [(value >> 8) as u8, (value & 0xFF) as u8];
    let res = dev.smbus_write_i2c_block_data(register, &values);
    thread::sleep(time::Duration::from_micros(INA209DelayMicroSeconds));
    res
}

fn convert_bus_voltage(register_value: u32) -> u32{
    // Constants from the table
    const OFFSET_MV: u32  = 2600;  // Offset in millivolts
    // const RANGE_MV: u32 = 12700; // Voltage range in millivolts (15300mV - 2600mV)
    
    // Extract VBUS_GD (bit 7) and 7-bit bus voltage (bits 6 to 0)
    let vbus_gd = (register_value >> 7) & 0x01; // Extract bit 7
    let bus_voltage = register_value & 0x7F; // Extract bits 6 to 0
    
    // Check if VBUS is attached
    if vbus_gd == 0{
        warn!("Vbus is not attached!");
        return 0
        //return Err(LocalError::RegConvError("Vbus is not attached!".into()))  // VBUS is not attached
    }
    // Convert the 7-bit value to voltage in mV
    let voltage_mv = OFFSET_MV + (bus_voltage  * 100);
    return voltage_mv
}

fn convert_battery_voltage(register_value: u32) -> u32{
    // Constants from the table
    const OFFSET_MV: u32 = 2304;  // Offset in millivolts
    const RANGE_MV: u32 = 2544;   // Voltage range in millivolts (4848mV - 2304mV)
    
    // Ensure value is within 7-bit range
    if (register_value <= 0) | (register_value > 127){
        warn!("Battery voltage value must be between 0 and 127.");
        return 0
    }
    //     raise ValueError("Battery voltage value must be between 0 and 127.")
    
    // Convert the 7-bit value to voltage in mV
    let voltage_mv = OFFSET_MV + ((register_value * RANGE_MV) / 127) ;
    return voltage_mv
}

fn convert_battery_charge_current(register_value: u32) -> u32 {
    // Bit weights (current contributions in mA)
    let bit_weights: [u32; 7] = [50, 100, 200, 400, 800, 1600, 3200];
    
    // Extract the 7-bit battery charge current value (bits 6 to 0)
    let current_bits = register_value & 0x7F ; // Mask with 0x7F to extract bits 6 to 0
    
    // Calculate the total current by summing up the contributions of each bit
    let mut current_ma = 0;
    for i in 0..7{
        if ((current_bits >> i) & 1) == 1{  // Check if bit i is set
            current_ma += bit_weights[i];
        }
    }
    
    return current_ma
}

impl PusService for EpsCtrlService {
    type CommandT = command::Command;

    fn handle_tc(&mut self, mut tc: AcceptedTc, cmd: Self::CommandT) -> AcceptanceResult {
        println!("PUS-Service: Command received.");
        match cmd {
            command::Command::PowerVCOM(args) => tc.handle(||{
                let val: u16 = if args.power == true {0x6060} else {0x4040}; 
                match write_i2c_ina_device_block(&mut self.dev_ina_unreg_bus, InaRegisters::GPIO as u8, val){
                    Ok(_) => true,
                    Err(e) => {
                        warn!("Error during writing to VCOM INA: {:?}", e);
                        false
                    }
                }
            }),
            command::Command::PowerAntDeploy(args) => tc.handle(||{
                let val: u16 = if args.power == true {0x6060} else {0x4040}; 
                match write_i2c_ina_device_block(&mut self.dev_ina_user_3v3, InaRegisters::GPIO as u8, val){
                    Ok(_) => true,
                    Err(e) => {
                        warn!("Error during writing to VCOM INA: {:?}", e);
                        false
                    }
                }
            }),
            command::Command::RqEpsBatteryStatus => tc.handle_with_tm(|| {
                if false {
                    return Err(());
                }

                // Set i2c bus to pmic0
                write_i2c_ina_device_block(&mut self.dev_i2c_switch, 0x00, 0x04).unwrap();
                
                // Activate ADC
                let mut pmic0_reg02 =  self.dev_pmic.smbus_read_i2c_block_data(0x02, 1).unwrap();
                debug!("pmic0_reg02: {:?}", pmic0_reg02);
                pmic0_reg02[0] |= 0xc0;
                let mut write_val= [0u8];
                write_val[0] = pmic0_reg02[0];
                self.dev_pmic.smbus_write_i2c_block_data(0x02, &write_val).unwrap();
                
                let mut pmic0_reg02_test2 =  self.dev_pmic.smbus_read_i2c_block_data(0x02, 1).unwrap();
                debug!("pmic0_reg02: {:?}", pmic0_reg02_test2);

                // read Values
                let pmic0_vbat_vec = self.dev_pmic.smbus_read_i2c_block_data(0x0e, 1).unwrap(); 
                debug!("pmic0_vbat_vec: {:X?}", pmic0_vbat_vec);

                let pmic0_vbat = convert_battery_voltage(pmic0_vbat_vec[0] as u32);

                debug!("pmic_vbat0: {:?}", pmic0_vbat);

                let pmic0_vsys_vec = self.dev_pmic.smbus_read_i2c_block_data(0x11, 1).unwrap(); 
                debug!("pmic0_vsys_vec: {:X?}", pmic0_vsys_vec);

                let pmic0_vsys = convert_bus_voltage(pmic0_vsys_vec[0] as u32);

                debug!("pmic_vsys: {:?}", pmic0_vsys);

                let pmic0_ichg_vec = self.dev_pmic.smbus_read_i2c_block_data(0x12, 1).unwrap(); 
                debug!("pmic0_ichg_vec: {:X?}", pmic0_ichg_vec);

                let pmic0_ichg = convert_battery_charge_current(pmic0_ichg_vec[0] as u32);

                debug!("pmic0_ichg: {:?}", pmic0_ichg);

                let pmic0_status_vec = self.dev_pmic.smbus_read_i2c_block_data(0x0b, 1).unwrap(); 
                debug!("pmic0_status_vec: {:X?}", pmic0_status_vec);
                debug!("pmic1_status_conv: {:?}", (pmic0_status_vec[0] >> 3) & 0b11);

                let pmic0_stat = match (pmic0_status_vec[0] >> 3) & 0b11 {
                    0 => telemetry::Pmic0Stat::not_charging,
                    1 => telemetry::Pmic0Stat::pre_charging,
                    2 => telemetry::Pmic0Stat::fast_charging,
                    3 => telemetry::Pmic0Stat::charge_terminated,
                    _ => telemetry::Pmic0Stat::read_out_error
                };

                // Set i2c bus to pmic1
                write_i2c_ina_device_block(&mut self.dev_i2c_switch, 0x00, 0x05).unwrap();

                // Activate ADC
                let mut pmic1_reg02 =  self.dev_pmic.smbus_read_i2c_block_data(0x02, 1).unwrap();
                debug!("pmic1_reg02: {:?}", pmic1_reg02);
                pmic1_reg02[0] |= 0xc0;
                let mut write_val1= [0u8];
                write_val1[0] = pmic1_reg02[0];
                self.dev_pmic.smbus_write_i2c_block_data(0x02, &write_val1).unwrap();
                
                let mut pmic1_reg02_test2 =  self.dev_pmic.smbus_read_i2c_block_data(0x02, 1).unwrap();
                debug!("pmic1_reg02: {:?}", pmic1_reg02_test2);

                // read Values
                let pmic1_vbat_vec = self.dev_pmic.smbus_read_i2c_block_data(0x0e, 1).unwrap(); 
                debug!("pmic1_vbat_vec: {:X?}", pmic1_vbat_vec);

                let pmic1_vbat = convert_battery_voltage(pmic1_vbat_vec[0] as u32);

                debug!("pmic1_vbat0: {:?}", pmic1_vbat);

                let pmic1_vsys_vec = self.dev_pmic.smbus_read_i2c_block_data(0x11, 1).unwrap(); 
                debug!("pmic1_vsys_vec: {:X?}", pmic1_vsys_vec);

                let pmic1_vsys = convert_bus_voltage(pmic1_vsys_vec[0] as u32);

                debug!("pmic1_vsys: {:?}", pmic1_vsys);

                let pmic1_ichg_vec = self.dev_pmic.smbus_read_i2c_block_data(0x12, 1).unwrap(); 
                debug!("pmic1_ichg_vec: {:X?}", pmic1_ichg_vec);

                let pmic1_ichg = convert_battery_charge_current(pmic1_ichg_vec[0] as u32);

                debug!("pmic1_ichg: {:?}", pmic1_ichg);

                let pmic1_status_vec = self.dev_pmic.smbus_read_i2c_block_data(0x0b, 1).unwrap(); 
                debug!("pmic1_status_vec: {:X?}", pmic1_status_vec);
                debug!("pmic1_status_conv: {:?}", (pmic1_status_vec[0] >> 3) & 0b11);

                let pmic1_stat = match (pmic1_status_vec[0] >> 3) & 0b11 {
                    0 => telemetry::Pmic1Stat::not_charging,
                    1 => telemetry::Pmic1Stat::pre_charging,
                    2 => telemetry::Pmic1Stat::fast_charging,
                    3 => telemetry::Pmic1Stat::charge_terminated,
                    _ => telemetry::Pmic1Stat::read_out_error
                };

                // Set i2c bus to pmic0
                write_i2c_ina_device_block(&mut self.dev_i2c_switch, 0x00, 0x04).unwrap();
                
                Ok(telemetry::EPS_Battery_Status{
                    pmic0_vbus: pmic0_vsys as i16,
                    pmic0_ichg: pmic0_ichg as i16,
                    pmic0_vbat: pmic0_vbat as i16,
                    PMIC0_STAT: pmic0_stat,
                    fg0_vbat: 0,
                    fg0_current: 0,
                    fg0_pwr: 0,
                    pmic1_vbus: pmic1_vsys as i16,
                    pmic1_ichg: pmic1_ichg as i16,
                    pmic1_vbat: pmic1_vbat as i16,
                    PMIC1_STAT: pmic1_stat,
                    fg1_vbat: 0,
                    fg1_current: 0,
                    fg1_pwr: 0
                })
            }),
            command::Command::RqEpsBusPowerStatus => tc.handle_with_tm(|| {
                Ok(telemetry::EPS_Bus_Status{
                    v_unreg_v: match read_i2c_ina_device_block(&mut self.dev_ina_vsys_bus, InaRegisters::BusVoltage as u8){
                        Ok(v) => ((((v[0] as u16) << 8 ) | v[1] as u16) >> 1) as i16,
                        Err(e) => { warn!("Error during reading sys bus voltage: {:?}", e); return Err(()) }
                    },
                    v_unreg_i: match read_i2c_ina_device_block(&mut self.dev_ina_vsys_bus, InaRegisters::BusCurrent as u8){
                        Ok(c) => {info!{"Current bus0 vsys: {:X?}", c};((((c[0] as u16) << 8 ) | c[1] as u16) / 2) as i16},
                        Err(e) => { warn!("Error during reading sys bus current: {:?}", e); return Err(()) }
                    },
                    v3_3_bus0_v: match read_i2c_ina_device_block(&mut self.dev_ina_bus0_3v3, InaRegisters::BusVoltage as u8){
                        Ok(v) => ((((v[0] as u16) << 8 ) | v[1] as u16) >> 1) as i16,
                        Err(e) => { warn!("Error during reading 3v3 bus0 voltage: {:?}", e); return Err(()) }
                    },
                    v3_3_bus0_i: match read_i2c_ina_device_block(&mut self.dev_ina_bus0_3v3, InaRegisters::BusCurrent as u8){
                        Ok(c) => {info!{"Current bus0 3v3: {:X?}", c};((((c[0] as u16) << 8 ) | c[1] as u16) / 2) as i16},
                        Err(e) => { warn!("Error during reading 3v3 bus0 current: {:?}", e); return Err(()) }
                    },
                    v3_3_bus1_v: match read_i2c_ina_device_block(&mut self.dev_ina_bus1_3v3, InaRegisters::BusVoltage as u8){
                        Ok(v) => ((((v[0] as u16) << 8 ) | v[1] as u16) >> 1) as i16,
                        Err(e) => { warn!("Error during reading 3v3 bus1 voltage: {:?}", e); return Err(()) }
                    },
                    v3_3_bus1_i: match read_i2c_ina_device_block(&mut self.dev_ina_bus1_3v3, InaRegisters::BusCurrent as u8){
                        Ok(c) => {info!("Current bus1 3v3: {:X?}", c); ((((c[0] as u16) << 8 ) | c[1] as u16) / 2) as i16},
                        Err(e) => { warn!("Error during reading 3v3 bus1 current: {:?}", e); return Err(()) }
                    },
                    v5_bus0_v: match read_i2c_ina_device_block(&mut self.dev_ina_bus0_5v, InaRegisters::BusVoltage as u8){
                        Ok(v) => ((((v[0] as u16) << 8 ) | v[1] as u16) >> 1) as i16,
                        Err(e) => { warn!("Error during reading 5v bus0 voltage: {:?}", e); return Err(()) }
                    },
                    v5_bus0_i: match read_i2c_ina_device_block(&mut self.dev_ina_bus0_5v, InaRegisters::BusCurrent as u8){
                        Ok(c) => {info!("Current bus0 5v: {:X?}", c); ((((c[0] as u16) << 8 ) | c[1] as u16) / 2) as i16},
                        Err(e) => { warn!("Error during reading 5v bus0 current: {:?}", e); return Err(()) }
                    },
                    v5_bus1_v: match read_i2c_ina_device_block(&mut self.dev_ina_bus1_5v, InaRegisters::BusVoltage as u8){
                        Ok(v) => ((((v[0] as u16) << 8 ) | v[1] as u16) >> 1) as i16,
                        Err(e) => { warn!("Error during reading 5v bus1 voltage: {:?}", e); return Err(()) }
                    },
                    v5_bus1_i: match read_i2c_ina_device_block(&mut self.dev_ina_bus1_5v, InaRegisters::BusCurrent as u8){
                        Ok(c) => {info!("Current bus1 5v: {:X?}", c); ((((c[0] as u16) << 8 ) | c[1] as u16) / 2) as i16},
                        Err(e) => { warn!("Error during reading 5v bus1 current: {:?}", e); return Err(()) }
                    },
                    unreg_bus_v:  match read_i2c_ina_device_block(&mut self.dev_ina_unreg_bus, InaRegisters::BusVoltage as u8){
                        Ok(v) => ((((v[0] as u16) << 8 ) | v[1] as u16) >> 1) as i16,
                        Err(e) => { warn!("Error during reading unreg bus voltage: {:?}", e); return Err(()) }
                    },
                    unreg_bus_i: match read_i2c_ina_device_block(&mut self.dev_ina_unreg_bus, InaRegisters::BusCurrent as u8){
                        Ok(c) => {info!("Current Unreg: {:X?}", c);((((c[0] as u16) << 8 ) | c[1] as u16) / 2) as i16},
                        Err(e) => { warn!("Error during reading unreg bus current: {:?}", e); return Err(()) }
                    },
                })
            }),
            command::Command::RqEpsUserPowerStatus => tc.handle_with_tm(||{
                if false {
                    return Err(());
                }

                Ok(telemetry::EPS_User_Power_Status{
                    v3_3_user_sw: match read_i2c_ina_device_block(&mut self.dev_ina_unreg_bus, InaRegisters::GPIO as u8){
                        Ok(v) => {
                            info!("Current Switch state: {:X?}", v);  (v[0] >> 6) & 0b1 == 1},
                        Err(e) => { warn!("Error during reading user 3v3 voltage: {:?}", e); return Err(()) }
                    },
                    v3_3_user_v: match read_i2c_ina_device_block(&mut self.dev_ina_user_3v3, InaRegisters::BusVoltage as u8){
                        Ok(v) => ((((v[0] as u16) << 8 ) | v[1] as u16) >> 1) as i16,
                        Err(e) => { warn!("Error during reading user 3v3 voltage: {:?}", e); return Err(()) }
                    },
                    v3_3_user_i: match read_i2c_ina_device_block(&mut self.dev_ina_user_3v3, InaRegisters::BusCurrent as u8){
                        Ok(c) => {info!("Current Unreg: {:X?}", c);((((c[0] as u16) << 8 ) | c[1] as u16) / 2) as i16},
                        Err(e) => { warn!("Error during reading user 3v3 current: {:?}", e); return Err(()) }
                    },
                    v5_user_sw : match read_i2c_ina_device_block(&mut self.dev_ina_user_5v, InaRegisters::GPIO as u8){
                        Ok(v) => {
                            info!("Current Switch state: {:X?}", v); (v[0] >> 6) & 0b1 == 1},
                        Err(e) => { warn!("Error during reading user 3v3 voltage: {:?}", e); return Err(()) }
                    },
                    v5_user_v: match read_i2c_ina_device_block(&mut self.dev_ina_user_5v, InaRegisters::BusVoltage as u8){
                        Ok(v) => ((((v[0] as u16) << 8 ) | v[1] as u16) >> 1) as i16,
                        Err(e) => { warn!("Error during reading user 5v voltage: {:?}", e); return Err(()) }
                    },
                    v5_user_i: match read_i2c_ina_device_block(&mut self.dev_ina_user_5v, InaRegisters::BusCurrent as u8){
                        Ok(c) => {info!("Current Unreg: {:X?}", c);((((c[0] as u16) << 8 ) | c[1] as u16) / 2) as i16},
                        Err(e) => { warn!("Error during reading user 5v current: {:?}", e); return Err(()) }
                    },
                    unreg_user_sw: match read_i2c_ina_device_block(&mut self.dev_ina_user_unreg, InaRegisters::GPIO as u8){
                        Ok(v) => {
                            info!("Current Switch state: {:X?}", v); (v[0] >> 6) & 0b1 == 1},
                        Err(e) => { warn!("Error during reading user 3v3 voltage: {:?}", e); return Err(()) }
                    },
                    unreg_user_v: match read_i2c_ina_device_block(&mut self.dev_ina_user_unreg, InaRegisters::BusVoltage as u8){
                        Ok(v) => ((((v[0] as u16) << 8 ) | v[1] as u16) >> 1) as i16,
                        Err(e) => { warn!("Error during reading user unreg voltage: {:?}", e); return Err(()) }
                    },
                    unreg_user_i: match read_i2c_ina_device_block(&mut self.dev_ina_user_unreg, InaRegisters::BusCurrent as u8){
                        Ok(c) => {info!("Current Unreg: {:X?}", c);((((c[0] as u16) << 8 ) | c[1] as u16) / 2) as i16},
                        Err(e) => { warn!("Error during reading user unreg current: {:?}", e); return Err(()) }
                    },
                })
            }),
            command::Command::RqEpsTemperature => tc.handle_with_tm(||{
                if false {
                    return Err(());
                }

                Ok(telemetry::EPS_Temperature{
                    pcb_dtemp: 0,
                    pcb_atemp0: 0,
                    pcb_atemp1: 0,
                    pcb_atemp2: 0,
                    pcb_atemp3: 0,
                })
            }),
            command::Command::PowerPayloadAprs( args)  => tc.handle(||{
                let val: u16 = if args.power == true {0x6060} else {0x4040}; 
                match write_i2c_ina_device_block(&mut self.dev_ina_user_5v, InaRegisters::GPIO as u8, val){
                    Ok(_) => true,
                    Err(e) => {
                        warn!("Error during writing to VCOM INA: {:?}", e);
                        false
                    }
                }
            }),
            command::Command::SetPowerSensorRegister( args) => tc.handle(||{
                match write_i2c_device_register_block(args.adress, args.register, args.value) {
                    Ok(_) => true,
                    Err(e) => {warn!{"Error during Power Sensor Register Write Process: {:?}", e}; false}
                }
            }),
            command::Command::GetPowerSensorRegister(args) => tc.handle_with_tm(||{
                Ok(telemetry::Power_Sensor_Register_Value{
                    adress: args.adress,
                    register: args.register,
                    value: match read_i2c_device_register_block(args.adress, args.register){
                        Ok(val) => val,
                        Err(e) => {warn!{"Error during Power Sensor Register Write Process: {:?}", e}; return Err(())}
                    }
                })
            }),
            command::Command::PMICSetIChargeLimit(args) => tc.handle(||{
                true
            }),
            command::Command::PMICSetIInputLimit(args) => tc.handle(||{
                true
            }),
            command::Command::PMICSetVChargeLimit(args) => tc.handle(||{
                true
            }),
            command::Command::PMICSetRegister(args) => tc.handle(||{
                match args.pmic_select{
                    command::PMICSelect::PMIC0 => {
                        // Set i2c bus to pmic0
                        write_i2c_ina_device_block(&mut self.dev_i2c_switch, 0x00, 0x04).unwrap();
                    },
                    command::PMICSelect::PMIC1 => {
                        // Set i2c bus to pmic0
                        write_i2c_ina_device_block(&mut self.dev_i2c_switch, 0x00, 0x05).unwrap();
                    }
                }
                let vals: [u8; 1] = [args.pmic_value];
                self.dev_pmic.smbus_write_i2c_block_data(args.pmic_register, &vals).unwrap();
                true
            }),
            command::Command::PMICGetRegister(args) => tc.handle_with_tm(||{
                if false{
                    return Err(())
                }

                match args.pmic_select{
                    command::PMICSelect::PMIC0 => {
                        // Set i2c bus to pmic0
                        write_i2c_ina_device_block(&mut self.dev_i2c_switch, 0x00, 0x04).unwrap();
                    },
                    command::PMICSelect::PMIC1 => {
                        // Set i2c bus to pmic0
                        write_i2c_ina_device_block(&mut self.dev_i2c_switch, 0x00, 0x05).unwrap();
                    }
                }

                let pmic_val_vec = self.dev_pmic.smbus_read_i2c_block_data(args.pmic_register, 1).unwrap(); 

                Ok(telemetry::PMIC_Register_Value{
                    pmic_register: args.pmic_register,
                    pmic_select: telemetry::PMICSelect::PMIC0,
                    pmic_value: pmic_val_vec[0]
                })
            })
        }
            
    }
    
    fn service() -> u8 {
        78
    }
}
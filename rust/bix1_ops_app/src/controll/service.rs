use std::io::Read;
use std::process::Command;
use log::error;
use num_traits::{FromPrimitive, ToPrimitive};
use rccn_usr::service::{AcceptanceResult, AcceptedTc, PusService};
use super::command;
use anyhow::Result;
use std::error::Error;
use std::fs;
use super::telemetry;
use machine_info::Machine;
use i2cdev::linux::{LinuxI2CBus, LinuxI2CDevice};
use i2cdev::core::{I2CDevice, I2CTransfer};
use i2cdev::linux::LinuxI2CError;
use thiserror::Error;
use log::{debug, info, kv::value, warn};
use std::{thread, time};
use crate::controll::command::PMICSelect;
use crate::controll::tla2528::lib::Tla2528;
use crate::controll::tla2528::channel::Channel;
use linux_embedded_hal::Delay;



// use linux_embedded_hal::i2cdev::linux::

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
    dev_passivation_switch0: LinuxI2CDevice,
    dev_passivation_switch1: LinuxI2CDevice,
    dev_adc0: LinuxI2CDevice,
    dev_adc1: LinuxI2CDevice,
    dev_adc2: LinuxI2CDevice,

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

        let mut dev_passivation_switch0 = match LinuxI2CDevice::new("/dev/i2c-0", 0x18) {
            Err(e) => {
                panic!("error creating i2c dev {e:?}")
            },
            Ok(dev) => {dev}
        };

        let mut dev_passivation_switch1 = match LinuxI2CDevice::new("/dev/i2c-0", 0x1A) {
            Err(e) => {
                panic!("error creating i2c dev {e:?}")
            },
            Ok(dev) => {dev}
        };

        let mut dev_adc1 = match LinuxI2CDevice::new("/dev/i2c-0", 0x13) {
            Err(e) => {
                panic!("error creating i2c dev {e:?}")
            },
            Ok(dev) => {dev}
        };

        let mut dev_adc2 = match LinuxI2CDevice::new("/dev/i2c-0", 0x14) {
            Err(e) => {
                panic!("error creating i2c dev {e:?}")
            },
            Ok(dev) => {dev}
        };

        let mut dev_adc0 = match LinuxI2CDevice::new("/dev/i2c-0", 0x17) {
            Err(e) => {
                panic!("error creating i2c dev {e:?}")
            },
            Ok(dev) => {dev}
        };

        let mut res = Self { 
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
            dev_fuel_gauge: dev_fuel_gauge,
            dev_passivation_switch0: dev_passivation_switch0,
            dev_passivation_switch1: dev_passivation_switch1,
            dev_adc0: dev_adc0,
            dev_adc1: dev_adc1,
            dev_adc2: dev_adc2
        };

        // Turning off watchdogs of PMICs
        res.switch_pmic_fg_i2c_bus(PMICSelect::PMIC0);
        if let Err(e) = res.dev_pmic.smbus_write_i2c_block_data(0x07, &[0b10001101]) {
            error!("could not turn off pmic0 watchdog");
        }

        res.switch_pmic_fg_i2c_bus(PMICSelect::PMIC1);
        if let Err(e) = res.dev_pmic.smbus_write_i2c_block_data(0x07, &[0b10001101]) {
            error!("could not turn off pmic1 watchdog");
        }


        return res
    }

    fn switch_pmic_fg_i2c_bus(&mut self, pmic: command::PMICSelect) -> bool{
        match pmic{
            command::PMICSelect::PMIC0 => {
                // Set i2c bus to pmic0
                match write_i2c_ina_device_block(&mut self.dev_i2c_switch, 0x00, 0x04){
                    Ok(_) => {},
                    Err(e) => {warn!("Error while setting INA device to PMIC0"); return false}
                }
            },
            command::PMICSelect::PMIC1 => {
                // Set i2c bus to pmic0
                match write_i2c_ina_device_block(&mut self.dev_i2c_switch, 0x00, 0x05){
                    Ok(_) => {},
                    Err(e) => {warn!("Error while setting INA device to PMIC1"); return false}
                }
            }
        }
        true
    }

}

fn init_i2c_device(path: &str, address: u16) -> Result<LinuxI2CDevice, LinuxI2CError>{
    let mut dev_ina = LinuxI2CDevice::new(path, address)?;
    write_i2c_ina_device_block(&mut dev_ina, InaRegisters::Config as u8,      0x1FFF)?;
    write_i2c_ina_device_block(&mut dev_ina, InaRegisters::Calibration as u8, 0x2000)?;
    return Ok(dev_ina)
}

fn write_i2c_device_register_block(address: u16, register: u8, value: u16) -> Result<(), LinuxI2CError>{
    let mut dev_ina = LinuxI2CDevice::new("/dev/i2c-0", address)?;

    let values: [u8; 2] = [(value >> 8) as u8, (value & 0xFF) as u8];
    dev_ina.smbus_write_i2c_block_data(register as u8, &values)?;
    thread::sleep(time::Duration::from_micros(INA209DelayMicroSeconds));
    Ok(())
}

fn write_i2c_device_register_block_vec(bus: u8, address: u16, register: u8, values: Vec<u8>) -> Result<(), LinuxI2CError>{
    let mut dev_ina = LinuxI2CDevice::new(format!("/dev/i2c-{bus}"), address)?;

    dev_ina.smbus_write_i2c_block_data(register as u8, &values)?;
    Ok(())
}

fn read_i2c_device_register_block(address: u16, register: u8) -> Result<u16, LinuxI2CError>{
    let mut dev_ina = LinuxI2CDevice::new("/dev/i2c-0", address)?;

    let values = dev_ina.smbus_read_i2c_block_data(register as u8, 2)?;
    thread::sleep(time::Duration::from_micros(INA209DelayMicroSeconds));
    Ok((values[0] as u16)<<8 | values[1] as u16)
}

fn read_i2c_device_register_block_vec(bus: u8, address: u16, register: u8, len: u8) -> Result<Vec<u8>, LinuxI2CError>{
    let mut dev_ina = LinuxI2CDevice::new(format!("/dev/i2c-{bus}"), address)?;
    dev_ina.smbus_read_i2c_block_data(register as u8, len)
}

fn read_i2c_ina_device_block(dev: &mut LinuxI2CDevice, register: u8) -> Result<Vec<u8>, LinuxI2CError>{
    let res = dev.smbus_read_i2c_block_data(register, 2);
    thread::sleep(time::Duration::from_micros(INA209DelayMicroSeconds));
    res
}

pub fn write_i2c_ina_device_block(dev: &mut LinuxI2CDevice, register: u8, value: u16) -> Result<(), LinuxI2CError>{
    // dev.smbus_read_i2c_block_data(register, len)
    let values: [u8; 2] = [(value >> 8) as u8, (value & 0xFF) as u8];
    let res = dev.smbus_write_i2c_block_data(register, &values);
    thread::sleep(time::Duration::from_micros(INA209DelayMicroSeconds));
    res
}

pub fn convert_bus_voltage(register_value: u32) -> u32{
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

pub fn convert_battery_voltage(register_value: u32) -> u32{
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

pub fn convert_battery_charge_current(register_value: u32) -> u32 {
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

// fn read_adc_register(dev: &mut LinuxI2CDevice, register: u8) -> Result<Vec<u8>, LinuxI2CError>{
//     dev.smbus_read_i2c_block_data(register, 1)
//     dev.smbus_read_byte()
// }

fn write_adc_register(dev: &mut LinuxI2CDevice, register: u8, val: u8) -> Result<(), LinuxI2CError>{
    let values= [val];
    dev.smbus_write_i2c_block_data(register, &values)
    // dev.transfer(msgs)
}

fn read_adc_channel(addr: u16, channel: Channel) -> u16{
    let mut dev_adc0 = match LinuxI2CDevice::new("/dev/i2c-0", addr) {
        Err(e) => {
            panic!("error creating i2c dev {e:?}")
        },
        Ok(dev) => {dev}
    };
    let mut tla = Tla2528::new(dev_adc0);

    tla.calibrate().unwrap();

    tla.prepare_for_manual_mode().unwrap();
    let val = tla.acquire_channel_data(channel).unwrap();
    info!("Val: {:?}", val);
    val
}

fn convert_csa_raw_to_ImA(v: u16)->i16{
    let mV = ((v as i32)*3300)/0xFFF;
    mV as i16
}

fn convert_alx_raw_to_dV(v: u16)->i16{
    ((v as u32 * 3300)/0xFFF) as i16 // input mal 3,3 Volt in deivolt here multiplied by resolution of sensor
}

fn convert_atemp_raw_to_dC(t: u16) -> i16{ //returns deciCelsius
    let mV = ((t as i32)*3300)/0xFFF;
    ((mV-500)) as i16 // /10
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
                if !self.switch_pmic_fg_i2c_bus(command::PMICSelect::PMIC0){return Err(())}
                // write_i2c_ina_device_block(&mut self.dev_i2c_switch, 0x00, 0x04).unwrap();


                let vbat0_raw = self
                    .dev_fuel_gauge
                    .smbus_read_i2c_block_data(0x09, 2)
                    .map(|data| (((data[1] as u16) << 8) | (data[0] & 0xff) as u16))
                    .unwrap_or(0);
                let fg0_vbat = (vbat0_raw as f32) * 78.125e-3;

                let vcurr0_raw = self
                    .dev_fuel_gauge
                    .smbus_read_i2c_block_data(0x0a, 2)
                    .map(|data| i16::from_le_bytes([data[0], data[1]]))
                    .unwrap_or(0);
                let fg0_vcurr = (vcurr0_raw as f32) * 0.15625;

                let fg0_vpwr = self
                    .dev_fuel_gauge
                    .smbus_read_i2c_block_data(0xb3, 2)
                    .map(|data| i16::from_le_bytes([data[0], data[1]]))
                    .unwrap_or(0);
                
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
                if !self.switch_pmic_fg_i2c_bus(command::PMICSelect::PMIC1){return Err(())}
                // write_i2c_ina_device_block(&mut self.dev_i2c_switch, 0x00, 0x05).unwrap();

                let vbat1_raw = self
                    .dev_fuel_gauge
                    .smbus_read_i2c_block_data(0x09, 2)
                    .map(|data| (((data[1] as u16) << 8) | (data[0] & 0xff) as u16))
                    .unwrap_or(0);
                let fg1_vbat = (vbat1_raw as f32) * 78.125e-3;

                let vcurr1_raw = self
                    .dev_fuel_gauge
                    .smbus_read_i2c_block_data(0x0a, 2)
                    .map(|data| i16::from_le_bytes([data[0], data[1]]))
                    .unwrap_or(0);
                let fg1_vcurr = (vcurr1_raw as f32) * 0.15625;

                let fg1_vpwr = self
                    .dev_fuel_gauge
                    .smbus_read_i2c_block_data(0xb3, 2)
                    .map(|data| i16::from_le_bytes([data[0], data[1]]))
                    .unwrap_or(0);

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
                if !self.switch_pmic_fg_i2c_bus(command::PMICSelect::PMIC0){return Err(())}
                // write_i2c_ina_device_block(&mut self.dev_i2c_switch, 0x00, 0x04).unwrap();
                
                Ok(telemetry::EPS_Battery_Status{
                    pmic0_vbus: pmic0_vsys as i16,
                    pmic0_ichg: pmic0_ichg as i16,
                    pmic0_vbat: pmic0_vbat as i16,
                    PMIC0_STAT: pmic0_stat,
                    fg0_vbat: fg0_vbat as i16,
                    fg0_current: fg0_vcurr as i16,
                    fg0_pwr: fg0_vpwr,
                    pmic1_vbus: pmic1_vsys as i16,
                    pmic1_ichg: pmic1_ichg as i16,
                    pmic1_vbat: pmic1_vbat as i16,
                    PMIC1_STAT: pmic1_stat,
                    fg1_vbat: fg1_vbat as i16,
                    fg1_current: fg1_vcurr as i16,
                    fg1_pwr: fg1_vpwr 
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
                if !self.switch_pmic_fg_i2c_bus(args.pmic_select.clone()){return false}

                let read_val =  match self.dev_pmic.smbus_read_i2c_block_data(0x04, 1){
                    Ok(v) => v[0],
                    Err(e) => {warn!("Error while reading register 0x04 from pmic {:?}: {:?}", args.pmic_select, e); return false}
                };

                let target_val = match args.i_charge_limit{
                    command::IChargeLimitSelect::Limit256mA => args.i_charge_limit.to_u8().unwrap(),
                    command::IChargeLimitSelect::Limit512mA => args.i_charge_limit.to_u8().unwrap(),
                    command::IChargeLimitSelect::Limit1024mA=> args.i_charge_limit.to_u8().unwrap(),
                    command::IChargeLimitSelect::Limit1536mA=> args.i_charge_limit.to_u8().unwrap(),
                    command::IChargeLimitSelect::Limit2048mA=> args.i_charge_limit.to_u8().unwrap(),
                    _                                       => {warn!("Value {:?} not allowed!", args.i_charge_limit); return false},
                };

                let set_val = (read_val & 0b1000_0000) | target_val;

                match self.dev_pmic.smbus_write_i2c_block_data(0x04, &[set_val]){
                    Ok(_) => {},
                    Err(e) => {warn!("Error while setting INA device to PMIC0: {:?}", e); return false}
                }

                let confirm_val =  match self.dev_pmic.smbus_read_i2c_block_data(0x04, 1){
                    Ok(v) => v[0],
                    Err(e) => {warn!("Error while reading register 0x04 from pmic {:?}: {:?}", args.pmic_select, e); return false}
                };

                if confirm_val == set_val{
                    return true
                }
                else{
                    warn!("Set Value does not Match Read Value");
                    return false
                }
                
            }),
            command::Command::PMICSetIInputLimit(args) => tc.handle(||{
                if !self.switch_pmic_fg_i2c_bus(args.pmic_select.clone()){return false}

                let read_val =  match self.dev_pmic.smbus_read_i2c_block_data(0x00, 1){
                    Ok(v) => v[0],
                    Err(e) => {warn!("Error while reading register 0x00 from pmic {:?}: {:?}", args.pmic_select, e); return false}
                };

                let target_val = match args.input_limit{
                    command::PMICSetIInputLimitSelect::Limit100mA => args.input_limit.to_u8().unwrap(),
                    command::PMICSetIInputLimitSelect::Limit200mA => args.input_limit.to_u8().unwrap(),
                    command::PMICSetIInputLimitSelect::Limit400mA => args.input_limit.to_u8().unwrap(),
                    command::PMICSetIInputLimitSelect::Limit800mA => args.input_limit.to_u8().unwrap(),
                    command::PMICSetIInputLimitSelect::Limit1400mA=> args.input_limit.to_u8().unwrap(),
                    command::PMICSetIInputLimitSelect::Limit2000mA=> args.input_limit.to_u8().unwrap(),
                    command::PMICSetIInputLimitSelect::Limit2400mA=> args.input_limit.to_u8().unwrap(),
                    command::PMICSetIInputLimitSelect::Limit2800mA=> args.input_limit.to_u8().unwrap(),
                    command::PMICSetIInputLimitSelect::Limit3250mA=> args.input_limit.to_u8().unwrap(),
                    _                                       => {warn!("Value {:?} not allowed!", args.input_limit); return false},
                };

                //let set_val = (read_val & 0b1000_0000) | target_val;
                let set_val = target_val;

                match self.dev_pmic.smbus_write_i2c_block_data(0x00, &[set_val]){
                    Ok(_) => {},
                    Err(e) => {warn!("Error while setting INA device to PMIC0: {:?}", e); return false}
                }

                let confirm_val =  match self.dev_pmic.smbus_read_i2c_block_data(0x00, 1){
                    Ok(v) => v[0],
                    Err(e) => {warn!("Error while reading register 0x00 from pmic {:?}: {:?}", args.pmic_select, e); return false}
                };

                if confirm_val == set_val{
                    return true
                }
                else{
                    warn!("Set Value does not Match Read Value");
                    return false
                }
            }),
            command::Command::PMICSetVChargeLimit(args) => tc.handle(||{
                if !self.switch_pmic_fg_i2c_bus(args.pmic_select.clone()){return false}

                let read_val =  match self.dev_pmic.smbus_read_i2c_block_data(0x06, 1){
                    Ok(v) => v[0],
                    Err(e) => {warn!("Error while reading register 0x06 from pmic {:?}: {:?}", args.pmic_select, e); return false}
                };

                let target_val = match args.v_charge_limit{
                    command::PMICSetVChargeLimit::Limit3V840 => args.v_charge_limit.to_u8().unwrap(),
                    command::PMICSetVChargeLimit::Limit3V904 => args.v_charge_limit.to_u8().unwrap(),
                    command::PMICSetVChargeLimit::Limit4V032 => args.v_charge_limit.to_u8().unwrap(),
                    command::PMICSetVChargeLimit::Limit4V128 => args.v_charge_limit.to_u8().unwrap(),
                    command::PMICSetVChargeLimit::Limit4V208 => args.v_charge_limit.to_u8().unwrap(),
                    command::PMICSetVChargeLimit::Limit4V352 => args.v_charge_limit.to_u8().unwrap(),
                    command::PMICSetVChargeLimit::Limit4V416 => args.v_charge_limit.to_u8().unwrap(),
                    command::PMICSetVChargeLimit::Limit4V511 => args.v_charge_limit.to_u8().unwrap(),
                    command::PMICSetVChargeLimit::Limit4V608 => args.v_charge_limit.to_u8().unwrap(),
                    _                                       => {warn!("Value {:?} not allowed!", args.v_charge_limit); return false},
                };

                let set_val = (read_val & 0b0000_0011) | target_val << 2;

                match self.dev_pmic.smbus_write_i2c_block_data(0x06, &[set_val]){
                    Ok(_) => {},
                    Err(e) => {warn!("Error while setting INA device to PMIC0: {:?}", e); return false}
                }

                let confirm_val =  match self.dev_pmic.smbus_read_i2c_block_data(0x06, 1){
                    Ok(v) => v[0],
                    Err(e) => {warn!("Error while reading register 0x06 from pmic {:?}: {:?}", args.pmic_select, e); return false}
                };

                if confirm_val == set_val{
                    debug!("Confirm Val: {:X?} - Set_Val: {:X?} - Orig_Val: {:X?}", confirm_val, set_val, read_val);
                    return true
                }
                else{
                    warn!("Set Value does not Match Read Value");
                    return false
                }
                true
            }),
            command::Command::PMICSetRegister(args) => tc.handle(||{
                if !self.switch_pmic_fg_i2c_bus(args.pmic_select){return false}
                let vals: [u8; 1] = [args.pmic_value];
                match self.dev_pmic.smbus_write_i2c_block_data(args.pmic_register, &vals){
                    Ok(_) => return true,
                    Err(e) => {warn!("Error while setting INA device to PMIC0: {:?}", e); return false}
                }
            }),
            command::Command::PMICGetRegister(args) => tc.handle_with_tm(||{
                if !self.switch_pmic_fg_i2c_bus(args.pmic_select){return Err(())}

                let pmic_val_vec = match self.dev_pmic.smbus_read_i2c_block_data(args.pmic_register, 1){
                    Ok(val) => val,
                    Err(e) => {warn!("Error while setting INA device to PMIC1: {:?}", e); return Err(())}
                }; 

                Ok(telemetry::PMIC_Register_Value{
                    pmic_register: args.pmic_register,
                    pmic_select: telemetry::PMICSelect::PMIC0,
                    pmic_value: pmic_val_vec[0]
                })
            }),
            command::Command::RqEpsBatteryConfig => tc.handle_with_tm(||{
                if false{
                    return Err(())
                }

                let sw0 = match self.dev_passivation_switch0.smbus_read_i2c_block_data(0x00, 2){
                    Ok(v) => v,
                    Err(e) => {warn!("Error while reading Passivation Switch 0 State: {:?}", e); return Err(())}
                };
                let sw1 = match self.dev_passivation_switch1.smbus_read_i2c_block_data(0x00, 2){
                    Ok(v) => v,
                    Err(e) => {warn!("Error while reading Passivation Switch 1 State: {:?}", e); return Err(())}
                };

                if !self.switch_pmic_fg_i2c_bus(PMICSelect::PMIC0){return Err(())}

                let val_pmic0_i_charge =  match self.dev_pmic.smbus_read_i2c_block_data(0x04, 1){
                    Ok(v) => v[0] & 0b1111111,
                    Err(e) => {warn!("Error while reading register 0x04 from pmic {:?}: {:?}", PMICSelect::PMIC0, e); return Err(())}
                };

                debug!{"val_pmic0_i_charge: {:?}", val_pmic0_i_charge};

                let val_pmic0_in_curr =  match self.dev_pmic.smbus_read_i2c_block_data(0x00, 1){
                    Ok(v) => v[0] & 0b00111111,
                    Err(e) => {warn!("Error while reading register 0x00 from pmic {:?}: {:?}", PMICSelect::PMIC0, e); return Err(())}
                };

                debug!{"val_pmic0_in_curr: {:?}", val_pmic0_in_curr};

                let val_pmic0_v_charge =  match self.dev_pmic.smbus_read_i2c_block_data(0x06, 1){
                    Ok(v) => (v[0] & 0b11111100) >> 2,
                    Err(e) => {warn!("Error while reading register 0x04 from pmic {:?}: {:?}", PMICSelect::PMIC0, e); return Err(())}
                };

                debug!{"val_pmic0_v_charge: {:?}", val_pmic0_v_charge};

                if !self.switch_pmic_fg_i2c_bus(PMICSelect::PMIC1){return Err(())}

                let val_pmic1_i_charge =  match self.dev_pmic.smbus_read_i2c_block_data(0x04, 1){
                    Ok(v) => v[0] & 0b1111111,
                    Err(e) => {warn!("Error while reading register 0x04 from pmic {:?}: {:?}", PMICSelect::PMIC1, e); return Err(())}
                };

                debug!{"val_pmic1_i_charge: {:?}", val_pmic1_i_charge};

                let val_pmic1_in_curr =  match self.dev_pmic.smbus_read_i2c_block_data(0x00, 1){
                    Ok(v) => v[0] & 0b00111111,
                    Err(e) => {warn!("Error while reading register 0x00 from pmic {:?}: {:?}", PMICSelect::PMIC1, e); return Err(())}
                };

                debug!{"val_pmic1_in_curr: {:?}", val_pmic1_in_curr};

                let val_pmic1_v_charge =  match self.dev_pmic.smbus_read_i2c_block_data(0x06, 1){
                    Ok(v) => ( v[0] & 0b11111100) >> 2,
                    Err(e) => {warn!("Error while reading register 0x06 from pmic {:?}: {:?}", PMICSelect::PMIC1, e); return Err(())}
                };

                debug!{"val_pmic1_v_charge: {:?}", val_pmic1_v_charge};

                Ok(telemetry::EPS_Battery_Config{
                    pass_switch0_passivation_state: if sw0[0] == 0x00 {false} else if sw0[0] >= 0x20 {true} else {warn!("Error: Undefined Passivation Switch 0 State: {:?}",    sw0[0]); return Err(())},
                    pass_switch0_persistant:        if sw0[1] == 0x00 {false} else if sw0[1] == 0x18 {true} else {warn!("Error: Undefined Passivation Switch 0 Volatile: {:?}", sw0[1]); return Err(())},
                    pass_switch1_passivation_state: if sw1[0] == 0x00 {false} else if sw1[0] >= 0x20 {true} else {warn!("Error: Undefined Passivation Switch 1 State: {:?}",    sw1[0]); return Err(())},
                    pass_switch1_persistant:        if sw1[1] == 0x00 {false} else if sw1[1] == 0x18 {true} else {warn!("Error: Undefined Passivation Switch 1 Volatile: {:?}", sw1[1]); return Err(())},
                    pmic0_i_charge_limit_select:    match telemetry::PMIC0IChargeLimitSelect::from_u8(val_pmic0_i_charge)   {Some(v)=> v, _=> telemetry::PMIC0IChargeLimitSelect::Undefined},
                    pmic0_i_input_limit_select:     match telemetry::PMIC0SetIInputLimitSelect::from_u8(val_pmic0_in_curr)  {Some(v)=> v, _=> telemetry::PMIC0SetIInputLimitSelect::Undefined},
                    pmic0_v_charge_limit_select:    match telemetry::PMIC0SetVChargeLimit::from_u8(val_pmic0_v_charge)      {Some(v)=> v, _=> telemetry::PMIC0SetVChargeLimit::Undefined},
                    pmic1_i_charge_limit_select:    match telemetry::PMIC1IChargeLimitSelect::from_u8(val_pmic1_i_charge)   {Some(v)=> v, _=> telemetry::PMIC1IChargeLimitSelect::Undefined},
                    pmic1_i_input_limit_select:     match telemetry::PMIC1SetIInputLimitSelect::from_u8(val_pmic1_in_curr)  {Some(v)=> v, _=> telemetry::PMIC1SetIInputLimitSelect::Undefined},
                    pmic1_v_charge_limit_select:    match telemetry::PMIC1SetVChargeLimit::from_u8(val_pmic1_v_charge)      {Some(v)=> v, _=> telemetry::PMIC1SetVChargeLimit::Undefined},
                })



            }),
            command::Command::SetPassivationSwState(args) => tc.handle(||{

                let passivation_value = match args.switch_passivation_state{
                        false => [0x01],
                        true =>  [0x3F],
                    };

                match args.switch_select{
                    command::SwitchSelect::Switch0 => {
                        match self.dev_passivation_switch0.smbus_write_i2c_block_data(0x00, &passivation_value) {
                            Ok(_) => {},
                            Err(e) => {warn!("Error while setting Passivation Switch 0: {:?}", e); return false}
                        }
                        match self.dev_passivation_switch0.smbus_write_i2c_block_data(0x20, &passivation_value) {
                            Ok(_) => true,
                            Err(e) => {warn!("Error while setting Passivation Switch 0: {:?}", e); return false}
                        }
                    }
                    command::SwitchSelect::Switch1 => {
                        match self.dev_passivation_switch1.smbus_write_i2c_block_data(0x00, &passivation_value) {
                            Ok(_) => {},
                            Err(e) => {warn!("Error while setting Passivation Switch 1: {:?}", e); return false}
                        }
                        match self.dev_passivation_switch1.smbus_write_i2c_block_data(0x20, &passivation_value) {
                            Ok(_) => true,
                            Err(e) => {warn!("Error while setting Passivation Switch 1: {:?}", e); return false}
                        }
                    },
                }
            }), 
            command::Command::SetRegister(args) => tc.handle(||{
                let mut vals = vec!();
                if args.length >= 1 {vals.push(args.byte00);}
                if args.length >= 2 {vals.push(args.byte01);}
                if args.length >= 3 {vals.push(args.byte02);}
                if args.length >= 4 {vals.push(args.byte03);}
                if args.length >= 5 {vals.push(args.byte04);}
                if args.length >= 6 {vals.push(args.byte05);}
                if args.length >= 7 {vals.push(args.byte06);}
                if args.length >= 8 {vals.push(args.byte07);}
                if args.length >= 9 {vals.push(args.byte08);}
                if args.length >= 10 {vals.push(args.byte09);}
                if args.length >= 11 {vals.push(args.byte10);}
                if args.length >= 12 {vals.push(args.byte11);}
                if args.length >= 13 {vals.push(args.byte12);}
                if args.length >= 14 {vals.push(args.byte13);}
                if args.length >= 15 {vals.push(args.byte14);}
                if args.length >= 16 {vals.push(args.byte15);}
                if args.length >= 17 {vals.push(args.byte16);}
                if args.length >= 18 {vals.push(args.byte17);}
                if args.length >= 19 {vals.push(args.byte18);}
                if args.length >= 20 {vals.push(args.byte19);}
                if args.length >= 21 {vals.push(args.byte20);}
                if args.length >= 22 {vals.push(args.byte21);}
                if args.length >= 23 {vals.push(args.byte22);}
                if args.length >= 24 {vals.push(args.byte23);}
                if args.length >= 25 {vals.push(args.byte24);}
                if args.length >= 26 {vals.push(args.byte25);}
                if args.length >= 27 {vals.push(args.byte26);}
                if args.length >= 28 {vals.push(args.byte27);}
                if args.length >= 29 {vals.push(args.byte28);}
                if args.length >= 30 {vals.push(args.byte29);}
                if args.length >= 31 {vals.push(args.byte30);}
                if args.length >= 32 {vals.push(args.byte31);}
                if args.length >= 33 {warn!("Warning: Length set too big. Max 32")}
                match write_i2c_device_register_block_vec(args.i2c_bus.to_u8().unwrap(), args.adress, args.register, vals){
                    Ok(_) => return true,
                    Err(e) => {warn!("Error during Set Register Write Process: {:?}", e); return false}
                };
            }),
            command::Command::GetRegister(args) => tc.handle_with_tm(||{
                Ok(telemetry::RegisterValueTM{
                    i2c_bus: telemetry::I2CSelect::from_u8(args.i2c_bus.to_u8().unwrap()).unwrap(),
                    address: args.address,
                    register: args.register,
                    values: match read_i2c_device_register_block_vec(args.i2c_bus.to_u8().unwrap(), args.address, args.register, args.length){
                        Ok(v) => {debug!("Length: {:?}, Vector: {:?}", v.len(), v); v},
                        Err(e) => {warn!("Error during Get Register Read Process: {:?}", e); return Err(())}
                    },
                })
            }),
            command::Command::RqEpsCsaSol => tc.handle_with_tm(||{
                if false {
                    return Err(());
                }

                Ok(telemetry::EPS_CSA_SOL{
                    csa_sol0: convert_csa_raw_to_ImA(read_adc_channel(0x17, Channel::Channel3)),
                    csa_sol1: convert_csa_raw_to_ImA(read_adc_channel(0x17, Channel::Channel2)),
                    csa_sol2: convert_csa_raw_to_ImA(read_adc_channel(0x17, Channel::Channel1)),
                    csa_sol3: convert_csa_raw_to_ImA(read_adc_channel(0x17, Channel::Channel0)),
                    csa_sol4: convert_csa_raw_to_ImA(read_adc_channel(0x17, Channel::Channel7)),
                    csa_sol5: convert_csa_raw_to_ImA(read_adc_channel(0x17, Channel::Channel6)),
                    csa_sol6: convert_csa_raw_to_ImA(read_adc_channel(0x17, Channel::Channel5)),
                    csa_sol7: convert_csa_raw_to_ImA(read_adc_channel(0x17, Channel::Channel4)) 
                })
            }),
            command::Command::RqTempAlxSol => tc.handle_with_tm(||{
                if false {
                    return Err(());
                }

                Ok(telemetry::EPS_TEMP_ALX_SOL{
                    atemp_sol0: convert_atemp_raw_to_dC(read_adc_channel(0x13, Channel::Channel1)),
                    atemp_sol1: convert_atemp_raw_to_dC(read_adc_channel(0x13, Channel::Channel3)),
                    atemp_sol2: convert_atemp_raw_to_dC(read_adc_channel(0x13, Channel::Channel5)),
                    atemp_sol3: convert_atemp_raw_to_dC(read_adc_channel(0x13, Channel::Channel7)),
                    atemp_sol4: convert_atemp_raw_to_dC(read_adc_channel(0x14, Channel::Channel7)),
                    atemp_sol5: convert_atemp_raw_to_dC(read_adc_channel(0x14, Channel::Channel5)),
                    atemp_sol6: convert_atemp_raw_to_dC(read_adc_channel(0x14, Channel::Channel3)),
                    alx_sol0: convert_alx_raw_to_dV(read_adc_channel(0x13, Channel::Channel0) ),
                    alx_sol1: convert_alx_raw_to_dV(read_adc_channel(0x13, Channel::Channel2) ),
                    alx_sol2: convert_alx_raw_to_dV(read_adc_channel(0x13, Channel::Channel4) ),
                    alx_sol3: convert_alx_raw_to_dV(read_adc_channel(0x13, Channel::Channel6) ),
                    alx_sol4: convert_alx_raw_to_dV(read_adc_channel(0x14, Channel::Channel6) ),
                    alx_sol5: convert_alx_raw_to_dV(read_adc_channel(0x14, Channel::Channel4) ),
                    alx_sol6: convert_alx_raw_to_dV(read_adc_channel(0x14, Channel::Channel2) )
                })
            }),
        }
    }
    
    fn service() -> u8 {
        78
    }
}
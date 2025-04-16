use std::process::Command;
use num_traits::FromPrimitive;
use rccn_usr::service::{AcceptanceResult, AcceptedTc, PusService};
use super::command;
use anyhow::{Result};
use std::result::Result::Ok;
use std::error::Error;
use std::fs;
use crate::rtc::telemetry;
use machine_info::Machine;
use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CError;

use log::{warn, info, debug};
pub struct RtcService{
    addr: u8,
    i2cdev: LinuxI2CDevice
}


impl RtcService {
    pub fn new() -> Self {
        let addr = 0x53;
        let mut dev = match LinuxI2CDevice::new("/dev/i2c-0", addr.into()) {
            Err(e) => {
                panic!("error creating i2c dev {e:?}");
            },
            Ok(dev) => {dev}
        };
        
        Self { 
            addr: addr,
            i2cdev: dev
        }
    }

    fn set_single_register(&mut self, register: u8, value: u8) -> Result<(), LinuxI2CError> {
        match self.i2cdev.smbus_write_i2c_block_data(register, &[value]){
            Ok(_) => {
                return Ok(())
            }
            Err(e) => {
                warn!{"Error while performing RTC Soft Reset: {:?}", e};
                return Err(e)
            }
        }
    }

    fn set_single_register_and_check(&mut self, register: u8, value: u8) -> Result<bool, LinuxI2CError> {
            match self.set_single_register(register, value){
                Ok(_) => {
                    let data = self.read_single_register(register)?;
                    if data != value{
                            warn!("Read data {} does not match set data {}.", data, value);
                            return Ok(false)
                        }
                    debug!("Wrote {} to register {}", data, register);
                    return Ok(true)
                }
                Err(e) => {
                    warn!{"Error while performing RTC Soft Reset: {:?}", e};
                    return Err(e)
                }
            }
    }

    fn read_single_register(&mut self, register: u8) -> Result<u8, LinuxI2CError> {
        match self.i2cdev.smbus_read_i2c_block_data(register, 1){
            Ok(data) => {
                debug!("Read from Register: {} Value: {}", register, data[0]);
                return Ok(data[0])
            },
            Err(e) => {
                warn!{"Error while reading from register {} \n {:?}", register, e};
                return Err(e)
            }
        }   
    }

    fn stop(&mut self)-> Result<bool, LinuxI2CError> {
        self.set_single_register_and_check(0x00, 32) // 40
    }

    fn start(&mut self)-> Result<bool, LinuxI2CError> {
        self.set_single_register_and_check(0x00, 0) // 8
    }

    fn to_bcd(&self, val: u8) -> u8 {
        // (((val / 10) & 0b111) << 4) | ((val % 10) & 0b1111)
        ((val / 10) << 4) + (val & 0x0F)
    } 

    fn from_bcd(&self, val: u8) -> u8 {
        (val >> 4) * 10 + (val & 0x0F)
    } 

}


impl PusService for RtcService {
    type CommandT = command::Command;

    fn handle_tc(&mut self, mut tc: AcceptedTc, cmd: Self::CommandT) -> AcceptanceResult {
        println!("PUS-Service: Command received.");
        match cmd {
            command::Command::RtcSoftwareReset => tc.handle(||{
                match self.set_single_register(0x05, 0x2c){
                    Ok(_) => {
                        debug!("Performed RTC Soft Reset");
                        return true
                    }
                    Err(e) => {
                        warn!{"Error while performing RTC Soft Reset"};
                        return false
                    }
                }
            }),
            command::Command::RtcSetTime(args) => tc.handle(||{

                let data: [u8; 8] = [ 
                    self.to_bcd(args.seconds_frac_100th), 
                    self.to_bcd(args.seconds & 0b0111_1111),
                    self.to_bcd(args.minutes & 0b0111_1111),
                    self.to_bcd(args.hours   & 0b0001_1111),
                    self.to_bcd(args.day     & 0b0011_1111),
                    args.weekday as u8,
                    self.to_bcd(args.month   & 0b0001_1111),
                    self.to_bcd(args.year)];

                match self.stop(){
                    Ok(success) => {
                        if !success {return false}
                    }
                    Err(e) => {
                        warn!("Error during Stopping RTC: {:?}", e);
                        return false
                    } 
                }
                
                match self.i2cdev.smbus_write_block_data(0x06, &data){
                    Ok(_) => {
                        
                    }
                    Err(e) => {
                        warn!("Error writing date to RTC: {:?}", e);
                        return false
                    }
                }

                let mut check_data = [0u8; 8];
                match self.i2cdev.smbus_read_i2c_block_data(0x06, 9) {
                    Ok(check) => {
                       check_data.copy_from_slice(&check[1..9]);
                        if check_data ==  data{
                            info!("Wrote DateTime {:?} to RTC", data);
                        }
                        else{
                            warn!("Read DateTime {:?} does not match set date {:?}", check, data);
                            return false
                        }
                    }
                    Err(e) => {
                        warn!("Error during confirming data: {:?}", e);
                        return false
                    }
                }
                match self.start() {
                    Ok(_) => {
                        return true
                    }
                    Err(e)=>{
                        warn!("Error during restarting RTC.");
                        return false
                    }
                }
            }),
            command::Command::RtcReadTime => tc.handle_with_tm(||{
                match self.i2cdev.smbus_read_i2c_block_data(0x06, 8) {
                    Ok(check) => {
                        info!("Read Time: {:?}", check);
                        let wd = match telemetry::WeekDayEnum::from_u8(check[5]){
                            Some(weekday) => weekday,
                            None => return Err(())
                        };
                        Ok(telemetry::RtcTime{
                            seconds_frac_100th: self.from_bcd(check[0]),
                            seconds:            self.from_bcd(check[1] & 0b0111_1111),
                            minutes:            self.from_bcd(check[2] & 0b0111_1111),
                            hours:              self.from_bcd(check[3] & 0b0001_1111),
                            day:                self.from_bcd(check[4] & 0b0011_1111), //  check[4] 
                            weekday: wd,
                            month:              self.from_bcd(check[6] & 0b0001_1111),
                            year:               self.from_bcd(check[7])
                        })
                    }
                    Err(e) => {
                        warn!("Error during confirming data: {:?}", e);
                        return Err(())
                    }
                }
                
            }),
            command::Command::RtcSetRegister(args) => tc.handle(||{
                match self.set_single_register_and_check(args.register, args.value){
                    Ok(success) => {
                        if success{
                            debug!("Wrote to Register: {} Value: {}", args.register, args.value);
                            return true
                        }
                        else{
                            debug!("Could not Write to Register: {} Value: {}", args.register, args.value);
                            return false
                        }
                    }
                    Err(e) => {
                        warn!{"Error while writing to register {} \n {:?}", args.register, e};
                        return false
                    }
                }
                // match self.i2cdev.smbus_write_byte_data(args.register, args.value){
                //     Ok(_) => {
                //         debug!("Wrote to Register: {} Value: {}", args.register, args.value);
                //         return true
                //     }
                //     Err(e) => {
                //         warn!{"Error while writing to register {} \n {:?}", args.register, e};
                //         return false
                //     }
                // }
            }),
            command::Command::ReadRegister(args) => tc.handle_with_tm(||{
               
                match self.read_single_register(args.register){
                    Ok(data) => {
                        debug!("Read from Register: {} Value: {}", args.register, data);
                        return Ok(telemetry::RtcRegister{
                                register: args.register,
                                value: data
                            })
                    },
                    Err(e) => {
                        warn!{"Error while reading from register {} \n {:?}", args.register, e};
                        return Err(())  
                    }
                }

                // match self.i2cdev.smbus_read_byte_data(args.register){
                //     Ok(data) => {
                //         debug!("Read from Register: {} Value: {}", args.register, data);
                //         return Ok(telemetry::RtcRegister{
                //                 register: args.register,
                //                 value: data
                //             })
                //     },
                //     Err(e) => {
                //         warn!{"Error while reading from register {} \n {:?}", args.register, e};
                //         return Err(())  
                //     }
                // }             
            })            
        }
            
    }
    
    fn service() -> u8 {
        79
    }
}
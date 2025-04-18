use std::{process::Command, sync::mpsc, time::Duration};
use num_traits::FromPrimitive;
use rccn_usr::service::{AcceptanceResult, AcceptedTc, PusService};
use super::{command, config::BixConfig, telemetry::WeekDayEnum};
use std::result::Result::Ok;
use crate::rtc::telemetry;
use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CError;

use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, NaiveDate, NaiveTime};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    username: String,
    volume: u8,
}

// I2C-0 Adresse: 0x45

pub(crate) fn reset_obc() -> Result<(),()>{
    let mut dev = match LinuxI2CDevice::new("/dev/i2c-0", 0x45) {
        Err(e) => {
            panic!("error creating i2c dev {e:?}")
        },
        Ok(dev) => {dev}
    };

    let val = 0x6060;

    let values: [u8; 2] = [(val >> 8) as u8, (val & 0xFF) as u8];

    match dev.smbus_write_i2c_block_data(0x14, &values){
        Ok(_) => Ok(()),
        Err(e) => {
            warn!("Error during writing to VCOM INA: {:?}", e);
            Err(())
        }
    }
}

use log::{warn, info, debug};
pub struct RtcService{
    addr: u8,
    i2cdev: LinuxI2CDevice,
    switch_obc_sender: mpsc::Sender<bool>

}


impl RtcService {
    pub fn new(switch_obc_sender: mpsc::Sender<bool>) -> Self {
        let addr = 0x53;
        let mut dev = match LinuxI2CDevice::new("/dev/i2c-0", addr.into()) {
            Err(e) => {
                panic!("error creating i2c dev {e:?}");
            },
            Ok(dev) => {dev}
        };
              
        let mut ret = Self { 
            addr: addr,
            i2cdev: dev, 
            switch_obc_sender: switch_obc_sender
        };

        let (s100th, s, mi, h, d, wd, mo, y) = match ret.get_rtc_time(){
            Ok(v) =>v,
            Err(_) => (0u8,0u8,0u8,0u8,1u8, WeekDayEnum::sunday, 6u8,25u8) 
        };

        // let mut rtc_time = chrono::Utc;
        // rtc_time.with_ymd_and_hms(y as i32 + 2000, mo as u32, d as u32, h as u32, mi as u32, s as u32);
        
        let date = NaiveDate::from_ymd_opt(y as i32 + 2000, mo as u32, d as u32).expect("Invalid date");
        let time = NaiveTime::from_hms_opt(h as u32, mi as u32, s as u32).expect("Invalid time");

        // 2. Combine to NaiveDateTime
        let naive_datetime = NaiveDateTime::new(date, time);

        // 3. Convert to DateTime<Utc>
        let rtc_datetime_utc: DateTime<chrono::Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, chrono::Utc);

        // let mut rtc_date_time = DateTime::from_naive_utc_and_offset(rtc_time, std::ptr::null());

        let time_config = BixConfig::load_or_default().unwrap();

        let time_config_time = chrono::DateTime::from_timestamp(time_config.current_time, 0).unwrap().to_utc();

        if (rtc_datetime_utc > time_config_time){
            ret.set_system_time(y, mo, d, h, mi, s, s100th);
        }
        else {
            ret.set_system_time((time_config_time.year() - 2000) as u8, time_config_time.month() as u8, time_config_time.day() as u8, time_config_time.hour() as u8, time_config_time.minute() as u8, time_config_time.second() as u8, 0);
        }

        ret
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

    fn clear_prescaler(&mut self)-> Result<(), LinuxI2CError> {
        self.set_single_register(0x05, 0x80) // 40
    }

    fn to_bcd(&self, val: u8) -> u8 {
        // (((val / 10) & 0b111) << 4) | ((val % 10) & 0b1111)
        ((val / 10) << 4) + ((val % 10) & 0x0F)
    } 

    fn from_bcd(&self, val: u8) -> u8 {
        (val >> 4) * 10 + (val & 0x0F)
    } 

    fn get_rtc_time(&mut self) -> Result<(u8,u8,u8,u8,u8, telemetry::WeekDayEnum, u8, u8), ()> {
        match self.i2cdev.smbus_read_i2c_block_data(0x06, 8)  {
            Ok(check) => {
                info!("Read Time: {:?}", check);
                let wd = match telemetry::WeekDayEnum::from_u8(check[5]){
                    Some(weekday) => weekday,
                    None => return Err(())
                };
                Ok((
                    self.from_bcd(check[0]),
                    self.from_bcd(check[1] & 0b0111_1111),
                    self.from_bcd(check[2] & 0b0111_1111),
                    self.from_bcd(check[3] & 0b0001_1111),
                    self.from_bcd(check[4] & 0b0011_1111), //  check[4] 
                    wd,
                    self.from_bcd(check[6] & 0b0001_1111),
                    self.from_bcd(check[7])
                ))
            }
            Err(e) => {
                warn!("Error during confirming data: {:?}", e);
                return Err(())
            }
        }
    }

    fn set_system_time(&mut self, y: u8, mo: u8, d: u8, h: u8, mi: u8, s: u8, s100th: u8) {
        // let dt = Utc.with_ymd_and_hms(y as i32, mo as u32, d as u32, h as u32, mi as u32, s as u32).unwrap();

        // let tv = timeval {
        //     tv_sec: dt.timestamp() as i32, // seconds since UNIX epoch
        //     tv_usec: (s100th as i32) * 10, // for cross this has to be i32 -> error accepted
        // };

        // let tz: *const timezone = std::ptr::null();

        // let e = unsafe { settimeofday(&tv, tz) };

        let o= Command::new("sh")
                .arg("-c")
                .arg(format!("date -s '20{:02}-{:02}-{:02} {:02}:{:02}:{:02}'", y, mo, d, h, mi, s))
                .output()
                .expect("failed to execute process");

        // let res = parse_time_from_string(o.stdout);
        // Todo Confirm time
        debug!("Set Time Result: {:?}", o);
    }


}

fn weekday_abbrev_to_enum(abbrev: &str) -> Option<telemetry::WeekDayEnum> {
    match abbrev {
        // German
        "So" => Some(telemetry::WeekDayEnum::sunday),
        "Mo" => Some(telemetry::WeekDayEnum::monday),
        "Di" => Some(telemetry::WeekDayEnum::tuesday),
        "Mi" => Some(telemetry::WeekDayEnum::wednesday),
        "Do" => Some(telemetry::WeekDayEnum::thursday),
        "Fr" => Some(telemetry::WeekDayEnum::friday),
        "Sa" => Some(telemetry::WeekDayEnum::saturday),
        // English
        "Sun" => Some(telemetry::WeekDayEnum::sunday),
        "Mon" => Some(telemetry::WeekDayEnum::monday),
        "Tue" => Some(telemetry::WeekDayEnum::tuesday),
        "Wed" => Some(telemetry::WeekDayEnum::wednesday),
        "ThuWeekday" => Some(telemetry::WeekDayEnum::thursday),
        "Fri" => Some(telemetry::WeekDayEnum::friday),
        "Sat" => Some(telemetry::WeekDayEnum::saturday),
        _ => None,
    }
}

fn weekday_to_u8(weekday: telemetry::WeekDayEnum) -> u8 {
    match weekday {
        telemetry::WeekDayEnum::sunday => 0,
        telemetry::WeekDayEnum::monday => 1,
        telemetry::WeekDayEnum::tuesday => 2,
        telemetry::WeekDayEnum::wednesday => 3,
        telemetry::WeekDayEnum::thursday => 4,
        telemetry::WeekDayEnum::friday => 5,
        telemetry::WeekDayEnum::saturday => 6,
    }
}

fn parse_time_from_string(input: &str) -> Result<(u8, u8, u8, telemetry::WeekDayEnum, u8, u8, u8), ()>{
    // Weekday abbrev could be 2 or 3 letters
    let weekday_abbrev = &input[..3].trim(); // get up to 3 chars, then trim any whitespace
    let weekday_enum = weekday_abbrev_to_enum(weekday_abbrev).expect("Invalid weekday abbrev");
    let weekday = weekday_to_u8(weekday_enum.clone());
    // Trim the weekday from the front and timezone/year from the end
    let trimmed = &input[weekday_abbrev.len() + 1..input.len() - 10]; // "17. Apr 11:19:49"
    let year = &input[input.len() - 4..]; // "2025"
    let datetime_str = format!("{} {}", trimmed, year); // "17. Apr 11:19:49 2025"

    let format = "%d. %b %H:%M:%S %Y";

    match NaiveDateTime::parse_from_str(&datetime_str, format) {
        Ok(dt) => {
            let seconds = dt.second() as u8;
            let minutes = dt.minute() as u8;
            let hours = dt.hour() as u8;
            let day = dt.day() as u8;
            let month = dt.month() as u8;
            let year_since_2000 = (dt.year() - 2000) as u8;

            println!("Seconds: {}", seconds);
            println!("Minutes: {}", minutes);
            println!("Hours: {}", hours);
            println!("Day: {}", day);
            println!("Month: {}", month);
            println!("Year since 2000: {}", year_since_2000);
            println!("Weekday (0=Sun): {}", weekday);
            Ok((year_since_2000, month, day, weekday_enum, hours, minutes, seconds))
        }
        Err(e) => {
            println!("Error parsing datetime: {}", e);
            Err(())
        }
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


                self.set_system_time(args.year, args.month, args.day , args.hours, args.minutes, args.seconds, args.seconds_frac_100th);

                let data: [u8; 8] = [ 
                    self.to_bcd(args.seconds_frac_100th), 
                    self.to_bcd(args.seconds ) & 0b0111_1111,
                    self.to_bcd(args.minutes ) & 0b0111_1111,
                    self.to_bcd(args.hours   ) & 0b0011_1111,
                    self.to_bcd(args.day     ) & 0b0011_1111,
                    args.weekday.clone() as u8             & 0b0000_0111,
                    self.to_bcd(args.month   ) & 0b0001_1111,
                    self.to_bcd(args.year)];

                // let data: [u8; 8] = [ 
                //     args.seconds_frac_100th, 
                //     args.seconds,
                //     args.minutes,
                //     args.hours  ,
                //     args.day    ,
                //     args.weekday as u8,
                //     args.month,
                //     args.year];

                match self.stop(){
                    Ok(success) => {
                        if !success {return false}
                    }
                    Err(e) => {
                        warn!("Error during Stopping RTC: {:?}", e);
                        return false
                    } 
                }

                match self.clear_prescaler(){
                    Ok(_) => {}
                    Err(e) => {
                        warn!("Error during Clearing Prescaler RTC: {:?}", e);
                        return false
                    } 
                }
                
                match self.i2cdev.smbus_write_i2c_block_data(0x06, &data){
                    Ok(_) => {
                        
                    }
                    Err(e) => {
                        warn!("Error writing date to RTC: {:?}", e);
                        return false
                    }
                }

                match self.i2cdev.smbus_write_i2c_block_data(0x06, &[self.to_bcd(args.seconds_frac_100th)]){
                    Ok(_) => {},
                    Err(e) => { warn!("Error writing 100th seconds to RTC: {:?}", e); return false}
                };
                
                match self.i2cdev.smbus_write_i2c_block_data(0x07, &[self.to_bcd(args.seconds)]){
                    Ok(_) => {},
                    Err(e) => { warn!("Error writing 100th seconds to RTC: {:?}", e); return false}
                };

                match self.i2cdev.smbus_write_i2c_block_data(0x08, &[self.to_bcd(args.minutes)]){
                    Ok(_) => {},
                    Err(e) => { warn!("Error writing 100th seconds to RTC: {:?}", e); return false}
                };

                match self.i2cdev.smbus_write_i2c_block_data(0x09, &[self.to_bcd(args.hours)]){
                    Ok(_) => {},
                    Err(e) => { warn!("Error writing 100th seconds to RTC: {:?}", e); return false}
                };

                match self.i2cdev.smbus_write_i2c_block_data(0x0A, &[self.to_bcd(args.day)]){
                    Ok(_) => {},
                    Err(e) => { warn!("Error writing 100th seconds to RTC: {:?}", e); return false}
                };

                match self.i2cdev.smbus_write_i2c_block_data(0x0B, &[args.weekday.clone() as u8]){
                    Ok(_) => {},
                    Err(e) => { warn!("Error writing 100th seconds to RTC: {:?}", e); return false}
                };

                match self.i2cdev.smbus_write_i2c_block_data(0x0C, &[self.to_bcd(args.month)]){
                    Ok(_) => {},
                    Err(e) => { warn!("Error writing 100th seconds to RTC: {:?}", e); return false}
                };

                match self.i2cdev.smbus_write_i2c_block_data(0x0D, &[self.to_bcd(args.year)]){
                    Ok(_) => {},
                    Err(e) => { warn!("Error writing 100th seconds to RTC: {:?}", e); return false}
                };

                let mut check_data = [0u8; 8];
                match self.i2cdev.smbus_read_i2c_block_data(0x06, 8) {
                    Ok(check) => {
                       check_data.copy_from_slice(&check[0..8]);
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
                            hours:              self.from_bcd(check[3] & 0b0011_1111),
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
            }),
            command::Command::GoToSleep(args) => tc.handle(||{
                match args.unit {
                    command::GoToSleepUnit::Hours => {
                        match Command::new("sh")
                        .arg("-c")
                        .arg(format!("sleep {}h", args.number))
                        .output(){
                            Ok(_) => return true,
                            Err(e) => {warn!("Error while setting sleep: {:?}", e); return false}
                        }
                    }
                    command::GoToSleepUnit::Minutes => {
                        match Command::new("sh")
                        .arg("-c")
                        .arg(format!("sleep {}m", args.number))
                        .output(){
                            Ok(_) => return true,
                            Err(e) => {warn!("Error while setting sleep: {:?}", e); return false}
                        }
                    }
                    command::GoToSleepUnit::Seconds => {
                        match Command::new("sh")
                        .arg("-c")
                        .arg(format!("sleep {}s", args.number))
                        .output(){
                            Ok(_) => return true,
                            Err(e) => {warn!("Error while setting sleep: {:?}", e); return false}
                        }
                    }
                }
                
            }),
            command::Command::SetResetInterval(args) => tc.handle(||{
                let mut reset_config = match BixConfig::load_or_default() {
                    Ok(conf) => conf,
                    Err(_) => return false
                };
                reset_config.reset_interval = match args.unit{
                    command::ResetUnit::Weeks => {(7*24*60*60*args.number).into()}
                    command::ResetUnit::Days => {(24*60*60*args.number).into()}
                    command::ResetUnit::Hours => {(60*60*args.number).into()}
                };

                match args.restart_interval{
                    true => {
                        let now = chrono::Utc::now();
                        let durr = Duration::from_secs(reset_config.reset_interval);
                        reset_config.next_reset = (now + durr).timestamp();
                    }
                    _ => {}
                }

                reset_config.save().is_ok()
            }),
            command::Command::SatReset(args) => tc.handle(||{
                match args.confirm{
                    true => {   
                        match reset_obc(){
                            Ok(_) => return true,
                            Err(_) => return false
                        }
                    }
                    false => {return true}
                }
            }),
            command::Command::SwitchObc(args) => tc.handle(||{
                match args.confirm{
                    true => {
                        match self.switch_obc_sender.send(true){
                            Ok(_) => return true,
                            Err(_) => return false

                        }
                        
                    }
                    false =>{
                        return true;
                    }
                }
            })
        }
            
    }
    
    fn service() -> u8 {
        79
    }
}
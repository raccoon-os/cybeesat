use std::process::Command;
use rccn_usr::service::{AcceptanceResult, AcceptedTc, PusService};
use super::command;
use anyhow::Result;
use std::error::Error;
use std::fs;
use crate::telemetry::telemetry::{self, OPT_EPS_BUS, OPT_OBC_CERT, PAYLOAD};
use machine_info::Machine;
use i2cdev::{core::I2CDevice, linux::LinuxI2CDevice};
use log::{debug, info, warn};

pub struct GetHealthService{
    i2c_bmm0: LinuxI2CDevice,
    i2c_bmm1: LinuxI2CDevice,
    i2c_gyro0: LinuxI2CDevice,
    i2c_gyro1: LinuxI2CDevice,
}


impl GetHealthService {
    pub fn new() -> Self {

        let i2c_bmm0 = match LinuxI2CDevice::new("/dev/i2c-0", 0x10) {
            Err(e) => {
                panic!("error creating i2c dev {e:?}")
            },
            Ok(dev) => {dev}
        };
        let i2c_bmm1 = match LinuxI2CDevice::new("/dev/i2c-1", 0x12) {
            Err(e) => {
                panic!("error creating i2c dev {e:?}")
            },
            Ok(dev) => {dev}
        };
        let i2c_gyro0 = match LinuxI2CDevice::new("/dev/i2c-0", 0x6A) {
            Err(e) => {
                panic!("error creating i2c dev {e:?}")
            },
            Ok(dev) => {dev}
        };
        let i2c_gyro1 = match LinuxI2CDevice::new("/dev/i2c-1", 0x6B) {
            Err(e) => {
                panic!("error creating i2c dev {e:?}")
            },
            Ok(dev) => {dev}
        };

        Self { 
            i2c_bmm0: i2c_bmm0,
            i2c_bmm1: i2c_bmm1,
            i2c_gyro0: i2c_gyro0,
            i2c_gyro1: i2c_gyro1
        }
    }
}


impl PusService for GetHealthService {
    type CommandT = command::Command;

    fn handle_tc(&mut self, mut tc: AcceptedTc, cmd: Self::CommandT) -> AcceptanceResult {
        println!("PUS-Service: Command received.");
        match cmd {            
            command::Command::RqObcInfo => tc.handle_with_tm(||{
                let mut m = Machine::new();
                let a = m.system_info();
                
                let stat = match m.system_status(){
                    Ok(status) => {
                        status
                    }
                    Err(_) => {
                        return Err(())
                    }
                };
                println!("{:?}", stat);
                println!("{:?}", stat.memory / 1000000);
                println!("{:?}", a.disks[0].available);
                println!("{:?}", a.disks[0].size);
                println!("{:?}", (a.disks[0].size - a.disks[0].available) / a.disks[0].size);
                println!("{:?}", stat.cpu);
                Ok(telemetry::OBC_INFO{
                    active_obc: false,
                    obc_uptime: 0,
                    obc_sysmem: (stat.memory / 1000) as i8,
                    obc_usermem: (a.disks[0].available / 1000000000) as i8,
                    obc_cpu_util: stat.cpu as i8,
                    obc_onboard_utc: 0,
                    last_session_utc: 0
                })
            }),
            command::Command::RqUserDefinedTm => tc.handle_with_tm(||{
                if false {
                    return Err(());
                }

                Ok(telemetry::user_defined_tm{
                    user_defined_parameter0: 0,
                    user_defined_parameter1: 0,
                    user_defined_parameter2: 0,
                    user_defined_parameter3: 0,
                    user_defined_parameter4: 0,
                    user_defined_parameter5: 0
                })
            }),
            command::Command::RqCom => tc.handle_with_tm(||{
                if false {
                    return Err(());
                }

                Ok(telemetry::COM{
                    VCOM0_STAT: telemetry::Vcom0Stat::tbd0,
                    vcom0_rssi: 0,
                    VCOM1_STAT: telemetry::Vcom1Stat::tbd0,
                    vcom1_rssi: 0
                })
            }),
            command::Command::RqGyroAccelTm => tc.handle_with_tm(||{
                if false{
                    return Err(())
                }

                // Enable Accelerometer
                match self.i2c_gyro0.smbus_write_i2c_block_data(0x10, &[0x60]){
                    Ok(_) => {}
                    Err(e) => {warn!("Error during Gyro0 Reg. 0x10 Write Process: {:?}", e); return Err(())}
                };
                match self.i2c_gyro1.smbus_write_i2c_block_data(0x10, &[0x60]){
                    Ok(_) => {}
                    Err(e) => {warn!("Error during Gyro1 Reg. 0x10 Write Process: {:?}", e); return Err(())}
                };

                // Enable Gyroscope
                match self.i2c_gyro0.smbus_write_i2c_block_data(0x11, &[0x60]){
                    Ok(_) => {}
                    Err(e) => {warn!("Error during Gyro0 Reg. 0x11 Write Process: {:?}", e); return Err(())}
                };
                match self.i2c_gyro1.smbus_write_i2c_block_data(0x11, &[0x60]){
                    Ok(_) => {}
                    Err(e) => {warn!("Error during Gyro1 Reg. 0x11 Write Process: {:?}", e); return Err(())}
                };

                // read values
                let gyro0_vec = match self.i2c_gyro0.smbus_read_i2c_block_data(0x20, 14) {
                    Ok(v) => v,
                    Err(e) => {warn!("Error during Gyro0 14 entries from Reg. 0x20 Write Process: {:?}", e); return Err(())}
                };

                debug!("gyro0_vec: {:?}", gyro0_vec);

                let gyro0_temp = ((gyro0_vec[1]  as i16) << 8)  | (gyro0_vec[0]  as i16);
                let gyro0_X_G =  ((gyro0_vec[3]  as i16) << 8)  | (gyro0_vec[2]  as i16);
                let gyro0_Y_G =  ((gyro0_vec[5]  as i16) << 8)  | (gyro0_vec[4]  as i16);
                let gyro0_Z_G =  ((gyro0_vec[7]  as i16) << 8)  | (gyro0_vec[6]  as i16);
                let gyro0_X_A =  ((gyro0_vec[9]  as i16) << 8)  | (gyro0_vec[8]  as i16);
                let gyro0_Y_A =  ((gyro0_vec[11] as i16) << 8)  | (gyro0_vec[10] as i16);
                let gyro0_Z_A =  ((gyro0_vec[13] as i16) << 8)  | (gyro0_vec[12] as i16);

                let gyro1_vec = match self.i2c_gyro1.smbus_read_i2c_block_data(0x20, 14) {
                    Ok(v) => v,
                    Err(e) => {warn!("Error during Gyro1 14 entries from Reg. 0x20 Write Process: {:?}", e); return Err(())}
                };

                debug!("gyro_vec: {:?}", gyro1_vec);

                let gyro1_temp = ((gyro1_vec[1]  as i16) << 8)  | (gyro1_vec[0]  as i16);
                let gyro1_X_G =  ((gyro1_vec[3]  as i16) << 8)  | (gyro1_vec[2]  as i16);
                let gyro1_Y_G =  ((gyro1_vec[5]  as i16) << 8)  | (gyro1_vec[4]  as i16);
                let gyro1_Z_G =  ((gyro1_vec[7]  as i16) << 8)  | (gyro1_vec[6]  as i16);
                let gyro1_X_A =  ((gyro1_vec[9]  as i16) << 8)  | (gyro1_vec[8]  as i16);
                let gyro1_Y_A =  ((gyro1_vec[11] as i16) << 8)  | (gyro1_vec[10] as i16);
                let gyro1_Z_A =  ((gyro1_vec[13] as i16) << 8)  | (gyro1_vec[12] as i16);

                Ok(telemetry::GYROACCELTM{
                    gyro0_temp: 25 + (gyro0_temp/ 256),
                    gyro0_x_sens:   (((gyro0_X_G as i32)*875) / 100) as i16,
                    gyro0_y_sens:   (((gyro0_Y_G as i32)*875) / 100) as i16,
                    gyro0_z_sens:   (((gyro0_Z_G as i32)*875) / 100) as i16,
                    accel0_x:       (((gyro0_X_A as i32)*122)/ 1000) as i16,
                    accel0_y:       (((gyro0_Y_A as i32)*122)/ 1000) as i16,
                    accel0_z:       (((gyro0_Z_A as i32)*122)/ 1000) as i16,
                    gyro1_temp: 25 + (gyro1_temp/ 256), 
                    gyro1_x_sens:   (((gyro1_X_G as i32)*875) / 100) as i16,
                    gyro1_y_sens:   (((gyro1_Y_G as i32)*875) / 100) as i16,
                    gyro1_z_sens:   (((gyro1_Z_G as i32)*875) / 100) as i16, //875
                    accel1_x:       (((gyro1_X_A as i32)*122)/ 1000) as i16,
                    accel1_y:       (((gyro1_Y_A as i32)*122)/ 1000) as i16,
                    accel1_z:       (((gyro1_Z_A as i32)*122)/ 1000) as i16,
                })
            }),
            command::Command::RqMagTm => tc.handle_with_tm(||{


                match self.i2c_bmm0.smbus_write_i2c_block_data(0x4B, &[0b0000_0001]){
                    Ok(_) => {},
                    Err(e) => {warn!("Error during writing to magnetometer 0: {:?}", e); return Err(())}
                };

                match self.i2c_bmm0.smbus_write_i2c_block_data(0x4C, &[0b0000_0000]){
                    Ok(_) => {},
                    Err(e) => {warn!("Error during writing to magnetometer 0: {:?}", e); return Err(())}
                };

                let mag0_vec = match self.i2c_bmm0.smbus_read_i2c_block_data(0x42, 8){
                    Ok(v) => v,
                    Err(e) => {warn!("Error during reading of magnetometer values: {:?}", e); return Err(())}
                };

                debug!("Mag0 Raw Readout: {:?}", mag0_vec);

                let raw_x0 = ((mag0_vec[1] as i16) << 5) | ((mag0_vec[0] as i16) >> 3 & 0x1F);
                let raw_y0 = ((mag0_vec[3] as i16) << 5) | ((mag0_vec[2] as i16) >> 3 & 0x1F);
                let raw_z0 = ((mag0_vec[5] as i16) << 7) | ((mag0_vec[4] as i16) >> 1 & 0x7F);

                let x0 = if raw_x0 & (1 << 12) != 0 { raw_x0 | !0x1FFF } else { raw_x0 } as i32;
                let y0 = if raw_y0 & (1 << 12) != 0 { raw_y0 | !0x1FFF } else { raw_y0 } as i32;
                let z0 = if raw_z0 & (1 << 14) != 0 { raw_z0 | !0x7FFF } else { raw_z0 } as i32;
                
                /////////////////////
                 
                match self.i2c_bmm1.smbus_write_i2c_block_data(0x4B, &[0b0000_0001]){
                    Ok(_) => {},
                    Err(e) => {warn!("Error during writing to magnetometer 1: {:?}", e); return Err(())}
                };
                
                match self.i2c_bmm1.smbus_write_i2c_block_data(0x4C, &[0b0000_0000]){
                    Ok(_) => {},
                    Err(e) => {warn!("Error during writing to magnetometer 1: {:?}", e); return Err(())}
                };

                let mag1_vec = match self.i2c_bmm1.smbus_read_i2c_block_data(0x42, 8){
                    Ok(v) => v,
                    Err(e) => {warn!("Error during reading of magnetometer values: {:?}", e); return Err(())}
                };

                debug!("Mag1 Raw Readout: {:?}", mag1_vec);

                let raw_x1 = ((mag1_vec[1] as i16) << 5) | ((mag1_vec[0] as i16) >> 3 & 0x1F);
                let raw_y1 = ((mag1_vec[3] as i16) << 5) | ((mag1_vec[2] as i16) >> 3 & 0x1F);
                let raw_z1 = ((mag1_vec[5] as i16) << 7) | ((mag1_vec[4] as i16) >> 1 & 0x7F);

                let x1 = if raw_x1 & (1 << 12) != 0 { raw_x1 | !0x1FFF } else { raw_x1 } as i32;
                let y1 = if raw_y1 & (1 << 12) != 0 { raw_y1 | !0x1FFF } else { raw_y1 } as i32;
                let z1 = if raw_z1 & (1 << 14) != 0 { raw_z1 | !0x7FFF } else { raw_z1 } as i32;

                Ok(telemetry::MAG{
                    mag0_x: (x0*1000)/16,
                    mag0_y: (y0*1000)/16,
                    mag0_z: (z0*1000)/16,
                    mag1_x: (x1*1000)/16,
                    mag1_y: (y1*1000)/16,
                    mag1_z: (z1*1000)/16
                })
            }),
            command::Command::RqPayload => tc.handle_with_tm(||{
                if false {
                    return Err(());
                }

                Ok(PAYLOAD{
                    aprs_stat0: 0,
                    aprs_stat1: 0
                })
            }),
            command::Command::RqOptEpsRtc => tc.handle_with_tm(||{
                if false {
                    return Err(());
                }

                Ok(telemetry::OPT_EPS_RTC{
                    eps_rtc_control0: 0,
                    eps_rtc_control1: 0,
                    eps_rtc_control2: 0,
                    eps_rtc_control3: 0,
                    eps_rtc_control4: 0,
                    eps_rtc_control5: 0,
                    eps_rtc_datetime: 0
                })
            }),
            command::Command::RqOptEpsBattery => tc.handle_with_tm(||{
                if false {
                    return Err(());
                }

                Ok(telemetry::OPT_EPS_BATTERY{
                    pmic0_therm: 0,
                    FG0_STAT: telemetry::Fg0Stat::not_charging,
                    fg0_soc: 0,
                    fg0_therm: 0,
                    pass_sw0_stat: false,
                    pmic1_therm: 0,
                    FG1_STAT: telemetry::Fg1Stat::not_charging,
                    fg1_soc: 0,
                    fg1_therm: 0,
                    pass_sw1_stat: false
                })
            }),
            command::Command::RqOptEpsBus => tc.handle_with_tm(||{
                if false {
                    return Err(());
                }

                Ok(OPT_EPS_BUS{
                    v_unreg_p: 0,
                    v3_3_bus0_p: 0,
                    v3_3_bus1_p: 0,
                    v5_bus0_p: 0,
                    v5_bus1_p: 0,
                    unreg_bus_p: 0,
                    v3_3_user0_p: 0,
                    v3_3_user1_p: 0,
                    v3_3_user2_p: 0,
                    v5_user0_p: 0,
                    v5_user1_p: 0,
                    unreg_user_p: 0
                })
            }),
            command::Command::RqOptObcCert => tc.handle_with_tm(||{
                if false {
                    return Err(());
                }

                Ok(telemetry::OPT_OBC_CERT{
                    sw_cert00: 0,
                    sw_cert01: 0,
                    sw_cert02: 0,
                    sw_cert03: 0,
                    sw_cert04: 0
                })
            }),
            command::Command::RqOptCom => tc.handle_with_tm(||{
                if false {
                    return Err(());
                }

                Ok(telemetry::OPT_COM{
                    VCOM0_MODE: telemetry::Vcom0Mode::tbd0,
                    vcom0_fifo_info: 0,
                    VCOM0_INT_STAT: telemetry::Vcom0IntStat::tbd0,
                    VCOM1_MODE: telemetry::Vcom1Mode::tbd0,
                    vcom1_fifo_info: 0,
                    VCOM1_INT_STAT: telemetry::Vcom1IntStat::tbd0
                })
            }),
            command::Command::RqOptPayload => tc.handle_with_tm(||{
                if false {
                    return Err(());
                }

                Ok(telemetry::OPT_PAYLOAD{
                    aprs_stat2: 0,
                    aprs_stat3: 0,
                    aprs_stat4: 0
                })
            })
            
        }
            
    }
    
    fn service() -> u8 {
        77
    }
}
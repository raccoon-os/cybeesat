use std::process::Command;
use rccn_usr::service::{AcceptanceResult, AcceptedTc, PusService};
use super::command;
use anyhow::Result;
use std::error::Error;
use std::fs;
use crate::telemetry::telemetry::{self, OPT_EPS_BUS, OPT_OBC_CERT, PAYLOAD};
use machine_info::Machine;

pub struct GetHealthService{
}


impl GetHealthService {
    pub fn new() -> Self {
        Self { }
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
            command::Command::RqImu => tc.handle_with_tm(||{
                if false {
                    return Err(());
                }

                Ok(telemetry::IMU{
                    gyro0_x_sens: 0,
                    gyro0_y_sens: 0,
                    gyro0_z_sens: 0,
                    accel0_x: 0,
                    accel0_y: 0,
                    accel0_z: 0,
                    mag0_x: 0,
                    mag0_y: 0,
                    mag0_z: 0,
                    gyro1_x_sens: 0,
                    gyro1_y_sens: 0,
                    gyro1_z_sens: 0,
                    accel1_x: 0,
                    accel1_y: 0,
                    accel1_z: 0,
                    mag1_x: 0,
                    mag1_y: 0,
                    mag1_z: 0
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
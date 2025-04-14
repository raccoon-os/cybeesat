use std::process::Command;
use rccn_usr::service::{AcceptanceResult, AcceptedTc, PusService};
use super::command;
use anyhow::Result;
use std::error::Error;
use std::fs;
use crate::telemetry::telemetry::{self, OPT_EPS_BUS, OPT_OBC_CERT, PAYLOAD};
use machine_info::Machine;
use i2cdev::linux::LinuxI2CDevice;

pub struct EpsCtrlService{
}


impl EpsCtrlService {
    pub fn new() -> Self {
        Self { }
    }
}


impl PusService for EpsCtrlService {
    type CommandT = command::Command;

    fn handle_tc(&mut self, mut tc: AcceptedTc, cmd: Self::CommandT) -> AcceptanceResult {
        println!("PUS-Service: Command received.");
        match cmd {
            command::Command::PowerCycleVCOM => tc.handle(||{
                true
            }),
            command::Command::PowerAntDeploy(args) => tc.handle(||{
                true
            }),
            // command::Command::AntDeploy  => tc.handle(||{
            //     true
            // }),
            // command::Command::AntRetract => tc.handle(||{
            //     true
            // }),
            command::Command::PowerPayloadAprs( args)  => tc.handle(||{
                true
            })
            
        }
            
    }
    
    fn service() -> u8 {
        78
    }
}
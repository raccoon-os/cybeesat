use rccn_usr::{bitbuffer::{BitReader, BitStruct}, service::{CommandParseResult, ServiceCommand}};
use satrs::spacepackets::ecss::{tc::PusTcReader, PusPacket};
use rccn_usr_bitstruct_derive::BitStruct;
use rccn_usr_pus_macros::ServiceCommand;

#[derive(ServiceCommand)]
pub enum Command {
    #[subservice(1)]
    PowerCycleVCOM,
    #[subservice(2)]
    PowerAntDeploy(PowerAntDeployArgs),
    #[subservice(3)]
    AntDeploy,
    #[subservice(4)]
    AntRetract,
    #[subservice(5)]
    PowerPayloadAprs(PowerPayloadAprsArgs),
}

#[derive(BitStruct, Debug)]
pub struct PowerAntDeployArgs {
    #[bits(8)]
    pub power: bool
}

#[derive(BitStruct, Debug)]
pub struct PowerPayloadAprsArgs {
    #[bits(8)]
    pub power: bool
}

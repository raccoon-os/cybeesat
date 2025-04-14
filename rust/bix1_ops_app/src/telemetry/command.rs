use rccn_usr::{bitbuffer::{BitReader, BitStruct}, service::{CommandParseResult, ServiceCommand}};
use satrs::spacepackets::ecss::{tc::PusTcReader, PusPacket};
use rccn_usr_bitstruct_derive::BitStruct;
use rccn_usr_pus_macros::ServiceCommand;

#[derive(ServiceCommand)]
pub enum Command {
    #[subservice(12)]
    RqOptEpsRtc,
    #[subservice(8)]
    RqCom,
    #[subservice(2)]
    RqEpsBattery,
    #[subservice(3)]
    RqEpsBus,
    #[subservice(1)]
    RqEpsCsaSol,
    #[subservice(5)]
    RqEpsTemp,
    #[subservice(4)]
    RqEpsUser,
    #[subservice(9)]
    RqImu,
    #[subservice(6)]
    RqObcInfo,
    #[subservice(16)]
    RqOptCom,
    #[subservice(13)]
    RqOptEpsBattery,
    #[subservice(14)]
    RqOptEpsBus,
    #[subservice(11)]
    RqOptEpsSol,
    #[subservice(15)]
    RqOptObcCert,
    #[subservice(17)]
    RqOptPayload,
    #[subservice(10)]
    RqPayload,
    #[subservice(7)]
    RqUserDefinedTm,
     
}

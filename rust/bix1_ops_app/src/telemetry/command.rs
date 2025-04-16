use rccn_usr::{bitbuffer::{BitReader, BitStruct}, service::{CommandParseResult, ServiceCommand}};
use satrs::spacepackets::ecss::{tc::PusTcReader, PusPacket};
use rccn_usr_bitstruct_derive::BitStruct;
use rccn_usr_pus_macros::ServiceCommand;

#[derive(ServiceCommand)]
pub enum Command {
    #[subservice(7)]
    RqUserDefinedTm,
    #[subservice(8)]
    RqCom,
    #[subservice(9)]
    RqGyroAccelTm,
    #[subservice(6)]
    RqObcInfo,
    #[subservice(10)]
    RqPayload,
    #[subservice(11)]
    RqMagTm,
    #[subservice(12)]
    RqOptEpsRtc,
    #[subservice(13)]
    RqOptEpsBattery,
    #[subservice(14)]
    RqOptEpsBus,
    #[subservice(15)]
    RqOptObcCert,
    #[subservice(16)]
    RqOptCom,
    #[subservice(17)]
    RqOptPayload,     
}

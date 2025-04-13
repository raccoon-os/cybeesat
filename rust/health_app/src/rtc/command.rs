use num_derive::{FromPrimitive, ToPrimitive};
use rccn_usr::{bitbuffer::{BitReader, BitStruct}, service::{CommandParseResult, ServiceCommand}};
use satrs::spacepackets::ecss::{tc::PusTcReader, PusPacket};
use rccn_usr_bitstruct_derive::BitStruct;
use rccn_usr_pus_macros::ServiceCommand;
use binary_serde::{BinarySerde, Endianness};
use rccn_usr::service::{CommandParseError};

#[derive(ServiceCommand)]
pub enum Command {
    #[subservice(1)]
    RtcSoftwareReset,
    #[subservice(2)]
    RtcSetTime(RtcSetTimeArgs),
    #[subservice(3)]
    RtcReadTime,
    #[subservice(4)]
    RtcSetRegister(SetRegisterArgs),
    #[subservice(5)]
    ReadRegister(ReadRegisterArgs),
}

#[derive(BitStruct, Debug)]
pub struct RtcSetTimeArgs {
    #[bits(8)]
    pub seconds_frac_100th: u8,
    #[bits(8)]
    pub seconds: u8,
    #[bits(8)]
    pub minutes: u8,
    #[bits(8)]
    pub hours: u8,
    #[bits(8)]
    pub day: u8,
    #[bits(8)]
    pub weekday: WeekDayEnum,
    #[bits(8)]
    pub month: u8,
    #[bits(8)]
    pub year: u8
}

#[derive(Debug, BinarySerde, PartialEq, ToPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum WeekDayEnum {
    Sunday = 0b000,
	Monday = 0b001,
	Tuesday = 0b010,
	Wednesday = 0b011,
    Thursday = 0b100,
    Friday = 0b101,
    Saturday = 0b110,
}

#[derive(BitStruct, Debug)]
pub struct SetRegisterArgs {
    #[bits(8)]
    pub register: u8,
    #[bits(8)]
    pub value: u8,
}

#[derive(BitStruct, Debug)]
pub struct ReadRegisterArgs {
    #[bits(8)]
    pub register: u8,
}
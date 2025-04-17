use num_derive::{FromPrimitive, ToPrimitive};
use rccn_usr::bitbuffer::BitStruct;
use rccn_usr_bitstruct_derive::BitStruct;
use rccn_usr_pus_macros::ServiceTelemetry;

#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(3)]
pub struct RtcTime {
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

#[derive(FromPrimitive, ToPrimitive, Debug, Clone)]
pub enum WeekDayEnum {
    sunday = 0b000,
	monday = 0b001,
	tuesday = 0b010,
	wednesday = 0b011,
    thursday = 0b100,
    friday = 0b101,
    saturday = 0b110,
}

#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(5)]
pub struct RtcRegister {
	#[bits(8)]
    pub register: u8,
    #[bits(8)]
    pub value: u8,
}
use rccn_usr::{bitbuffer::{BitReader, BitStruct}, service::{CommandParseResult, ServiceCommand}};
use satrs::spacepackets::ecss::{tc::PusTcReader, PusPacket};
use rccn_usr_bitstruct_derive::BitStruct;
use rccn_usr_pus_macros::ServiceCommand;
use binary_serde::{BinarySerde, Endianness};
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(ServiceCommand)]
pub enum Command {
    #[subservice(1)]
    PowerVCOM(PowerVcomArgs),
    #[subservice(2)]
    PowerAntDeploy(PowerAntDeployArgs),
    #[subservice(3)]
    RqEpsBatteryStatus,
    #[subservice(4)]
    RqEpsBusPowerStatus,
    #[subservice(5)]
    RqEpsUserPowerStatus,
    #[subservice(6)]
    RqEpsTemperature,
    #[subservice(7)]
    PowerPayloadAprs(PowerPayloadAprsArgs),
    #[subservice(8)]
    SetPowerSensorRegister(SetPowerSensorRegisterArgs),
    #[subservice(9)]
    GetPowerSensorRegister(GetPowerSensorRegisterArgs),
    #[subservice(10)]
    PMICSetIChargeLimit(PMICSetIChargeLimitArgs),
    #[subservice(11)]
    PMICSetIInputLimit(PMICSetIInputLimitArgs),
    #[subservice(12)]
    PMICSetVChargeLimit(PMICSetVChargeLimitArgs),
    #[subservice(13)]
    PMICSetRegister(PMICSetRegisterArgs),
    #[subservice(14)]
    PMICGetRegister(PMICGetRegisterArgs),
}

#[derive(BitStruct, Debug)]
pub struct PowerAntDeployArgs {
    #[bits(8)]
    pub power: bool
}

#[derive(BitStruct, Debug)]
pub struct PowerVcomArgs {
    #[bits(8)]
    pub power: bool
}


#[derive(BitStruct, Debug)]
pub struct PowerPayloadAprsArgs {
    #[bits(8)]
    pub power: bool
}

#[derive(BitStruct, Debug)]
pub struct SetPowerSensorRegisterArgs {
    #[bits(16)]
    pub adress: u16,
    #[bits(8)]
    pub register: u8,
    #[bits(16)]
    pub value: u16,
}

#[derive(BitStruct, Debug)]
pub struct GetPowerSensorRegisterArgs {
    #[bits(16)]
    pub adress: u16,
    #[bits(8)]
    pub register: u8,
}

#[derive(BitStruct, Debug)]
pub struct PMICSetIChargeLimitArgs {
    #[bits(8)]
    pub i_charge_limit: u8,
}


#[derive(BitStruct, Debug)]
pub struct PMICSetIInputLimitArgs {
    #[bits(8)]
    pub input_limit: u8,
}


#[derive(BitStruct, Debug)]
pub struct PMICSetVChargeLimitArgs {
    #[bits(8)]
    pub v_charge_limit: u8,
}


#[derive(BitStruct, Debug)]
pub struct PMICSetRegisterArgs {
    #[bits(8)]
    pub pmic_select: PMICSelect,
    #[bits(8)]
    pub pmic_register: u8,
    #[bits(8)]
    pub pmic_value: u8,
}

#[derive(Debug, BinarySerde, PartialEq, ToPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum PMICSelect {
    PMIC0 = 0,
	PMIC1 = 1,
}

#[derive(BitStruct, Debug)]
pub struct PMICGetRegisterArgs {
    #[bits(8)]
    pub pmic_select: PMICSelect,
    #[bits(8)]
    pub pmic_register: u8,
}


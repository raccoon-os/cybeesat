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
    #[subservice(15)]
    RqEpsBatteryConfig,
    #[subservice(16)]
    SetPassivationSwState(SetPassivationSwStateArgs),
    #[subservice(17)]
    SetRegister(SetRegisterArgs),
    #[subservice(18)]
    GetRegister(GetRegisterArgs),
    #[subservice(19)]
    RqEpsCsaSol,
    #[subservice(20)]
    RqTempAlxSol,

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

#[derive(BitStruct, Debug, Clone)]
pub struct PMICSetIChargeLimitArgs {
    #[bits(8)]
    pub pmic_select: PMICSelect,
    #[bits(8)]
    pub i_charge_limit: IChargeLimitSelect,
}

#[derive(Debug, BinarySerde, PartialEq, ToPrimitive, FromPrimitive, Clone)]
#[repr(u8)]
pub enum IChargeLimitSelect {
    Limit256mA = 0b0000100,
	Limit512mA = 0b0001000,
    Limit1024mA = 0b0010000,
    Limit1536mA = 0b0011000,
    Limit2048mA = 0b0100000,
}

#[derive(BitStruct, Debug)]
pub struct PMICSetIInputLimitArgs {
    #[bits(8)]
    pub pmic_select: PMICSelect,
    #[bits(8)]
    pub input_limit: PMICSetIInputLimitSelect,
}

#[derive(Debug, BinarySerde, PartialEq, ToPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum PMICSetIInputLimitSelect {
    Limit100mA = 0b000010,
    Limit200mA = 0b000100,
    Limit400mA = 0b001000,
	Limit800mA = 0b010000,
    Limit1400mA = 0b011100,
    Limit2000mA = 0b101000,
    Limit2400mA = 0b110000,
    Limit2800mA = 0b111000,
    Limit3250mA = 0b111111,
}

#[derive(BitStruct, Debug)]
pub struct PMICSetVChargeLimitArgs {
    #[bits(8)]
    pub pmic_select: PMICSelect,
    #[bits(8)]
    pub v_charge_limit: PMICSetVChargeLimit,
}

#[derive(Debug, BinarySerde, PartialEq, ToPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum PMICSetVChargeLimit {
    Limit3V840 = 0b000000,
	Limit3V904 = 0b000100,
    Limit4V032 = 0b001100,
    Limit4V128 = 0b010010,
    Limit4V208 = 0b010111,
    Limit4V352 = 0b100000,
    Limit4V416 = 0b100100,
    Limit4V511 = 0b101010,
    Limit4V608 = 0b110000,
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

#[derive(Debug, BinarySerde, PartialEq, ToPrimitive, FromPrimitive, Clone)]
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

#[derive(BitStruct, Debug)]
pub struct SetPassivationSwStateArgs {
    #[bits(8)]
    pub switch_select: SwitchSelect,
    #[bits(8)]
    pub switch_passivation_state: bool,
    #[bits(8)]
    pub persistant: bool,
}

#[derive(Debug, BinarySerde, PartialEq, ToPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum SwitchSelect {
    Switch0 = 0,
	Switch1 = 1,
}

#[derive(BitStruct, Debug)]
pub struct SetRegisterArgs {
    #[bits(8)]
    pub i2c_bus: I2CSelect,
    #[bits(16)]
    pub adress: u16,
    #[bits(8)]
    pub register: u8,
    #[bits(8)]
    pub length: u8,
    #[bits(8)]
    pub byte00: u8,
    #[bits(8)]
    pub byte01: u8,
    #[bits(8)]
    pub byte02: u8,
    #[bits(8)]
    pub byte03: u8,
    #[bits(8)]
    pub byte04: u8,
    #[bits(8)]
    pub byte05: u8,
    #[bits(8)]
    pub byte06: u8,
    #[bits(8)]
    pub byte07: u8,
    #[bits(8)]
    pub byte08: u8,
    #[bits(8)]
    pub byte09: u8,
    #[bits(8)]
    pub byte10: u8,
    #[bits(8)]
    pub byte11: u8,
    #[bits(8)]
    pub byte12: u8,
    #[bits(8)]
    pub byte13: u8,
    #[bits(8)]
    pub byte14: u8,
    #[bits(8)]
    pub byte15: u8,
    #[bits(8)]
    pub byte16: u8,
    #[bits(8)]
    pub byte17: u8,
    #[bits(8)]
    pub byte18: u8,
    #[bits(8)]
    pub byte19: u8,
    #[bits(8)]
    pub byte20: u8,
    #[bits(8)]
    pub byte21: u8,
    #[bits(8)]
    pub byte22: u8,
    #[bits(8)]
    pub byte23: u8,
    #[bits(8)]
    pub byte24: u8,
    #[bits(8)]
    pub byte25: u8,
    #[bits(8)]
    pub byte26: u8,
    #[bits(8)]
    pub byte27: u8,
    #[bits(8)]
    pub byte28: u8,
    #[bits(8)]
    pub byte29: u8,
    #[bits(8)]
    pub byte30: u8,
    #[bits(8)]
    pub byte31: u8,
}

#[derive(BitStruct, Debug)]
pub struct GetRegisterArgs {
    #[bits(8)]
    pub i2c_bus: I2CSelect,
    #[bits(16)]
    pub address: u16,
    #[bits(8)]
    pub register: u8,
    #[bits(8)]
    pub length: u8,
}

#[derive(Debug, BinarySerde, PartialEq, ToPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum I2CSelect {
    I2C0 = 0,
	I2C1 = 1,
}
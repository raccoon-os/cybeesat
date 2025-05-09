use num_derive::{FromPrimitive, ToPrimitive};
use rccn_usr::bitbuffer::BitStruct;
use rccn_usr_bitstruct_derive::BitStruct;
use rccn_usr_pus_macros::ServiceTelemetry;

#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(3)]
pub struct EPS_Battery_Status {
	#[bits(16)]
	pub pmic0_vbus: i16,
	#[bits(16)]
	pub pmic0_ichg: i16,
	#[bits(16)]
	pub pmic0_vbat: i16,
	#[bits(8)]
	pub PMIC0_STAT: Pmic0Stat,
	#[bits(16)]
	pub fg0_vbat: i16,
	#[bits(16)]
	pub fg0_current: i16,
	#[bits(16)]
	pub fg0_pwr: i16,
	#[bits(16)]
	pub pmic1_vbus: i16,
	#[bits(16)]
	pub pmic1_ichg: i16,
	#[bits(16)]
	pub pmic1_vbat: i16,
	#[bits(8)]
	pub PMIC1_STAT: Pmic1Stat,
	#[bits(16)]
	pub fg1_vbat: i16,
	#[bits(16)]
	pub fg1_current: i16,
	#[bits(16)]
	pub fg1_pwr: i16,
}

#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(4)]
pub struct EPS_Bus_Status {
	#[bits(16)]
	pub v_unreg_v: i16,
	#[bits(16)]
	pub v_unreg_i: i16,
	#[bits(16)]
	pub v3_3_bus0_v: i16,
	#[bits(16)]
	pub v3_3_bus0_i: i16,
	#[bits(16)]
	pub v3_3_bus1_v: i16,
	#[bits(16)]
	pub v3_3_bus1_i: i16,
	#[bits(16)]
	pub v5_bus0_v: i16,
	#[bits(16)]
	pub v5_bus0_i: i16,
	#[bits(16)]
	pub v5_bus1_v: i16,
	#[bits(16)]
	pub v5_bus1_i: i16,
	#[bits(16)]
	pub unreg_bus_v: i16,
	#[bits(16)]
	pub unreg_bus_i: i16,
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum Pmic0Stat {
	not_charging = 0,
	pre_charging = 1,
	fast_charging = 2,
	charge_terminated = 3,
	read_out_error = 4
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum Pmic1Stat {
	not_charging = 0,
	pre_charging = 1,
	fast_charging = 2,
	charge_terminated = 3,
	read_out_error = 4
}

#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(5)]
pub struct EPS_User_Power_Status {
	#[bits(8)]
	pub v3_3_user_sw: bool,
	#[bits(16)]
	pub v3_3_user_v: i16,
	#[bits(16)]
	pub v3_3_user_i: i16,
	#[bits(8)]
	pub v5_user_sw: bool,
	#[bits(16)]
	pub v5_user_v: i16,
	#[bits(16)]
	pub v5_user_i: i16,
	#[bits(8)]
	pub unreg_user_sw: bool,
	#[bits(16)]
	pub unreg_user_v: i16,
	#[bits(16)]
	pub unreg_user_i: i16,
}

#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(6)]
pub struct EPS_Temperature {
	#[bits(8)]
	pub pcb_dtemp: i8,
	#[bits(16)]
	pub pcb_atemp0: i16,
	#[bits(16)]
	pub pcb_atemp1: i16,
	#[bits(16)]
	pub pcb_atemp2: i16,
	#[bits(16)]
	pub pcb_atemp3: i16,
}

#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(9)]
pub struct Power_Sensor_Register_Value {
	#[bits(16)]
	pub adress: u16,
	#[bits(8)]
	pub register: u8,
	#[bits(16)]
	pub value: u16,
}

#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(14)]
pub struct PMIC_Register_Value {
	#[bits(8)]
	pub pmic_select: PMICSelect,
	#[bits(8)]
	pub pmic_register: u8,
	#[bits(8)]
	pub pmic_value: u8,
}


#[derive(FromPrimitive, ToPrimitive, Debug)]
#[repr(u8)]
pub enum PMICSelect {
    PMIC0 = 0,
	PMIC1 = 1,
}

#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(15)]
pub struct EPS_Battery_Config {
	#[bits(8)]
	pub pass_switch0_passivation_state: bool,
	#[bits(8)]
	pub pass_switch0_persistant: bool,
	#[bits(8)]
	pub pass_switch1_passivation_state: bool,
	#[bits(8)]
	pub pass_switch1_persistant: bool,
	#[bits(8)]
	pub pmic0_i_charge_limit_select: PMIC0IChargeLimitSelect ,
	#[bits(8)]
	pub pmic0_i_input_limit_select: PMIC0SetIInputLimitSelect,
	#[bits(8)]
	pub pmic0_v_charge_limit_select: PMIC0SetVChargeLimit,
	#[bits(8)]
	pub pmic1_i_charge_limit_select: PMIC1IChargeLimitSelect ,
	#[bits(8)]
	pub pmic1_i_input_limit_select: PMIC1SetIInputLimitSelect,
	#[bits(8)]
	pub pmic1_v_charge_limit_select: PMIC1SetVChargeLimit,
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
#[repr(u8)]
pub enum PMIC0IChargeLimitSelect {
    Limit256mA = 0b0000100,
	Limit512mA = 0b0001000,
    Limit1024mA = 0b0010000,
    Limit1536mA = 0b0011000,
    Limit2048mA = 0b0100000,
	Undefined = 0xFF
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
#[repr(u8)]
pub enum PMIC0SetIInputLimitSelect {
	Limit100mA = 0b000010,
	Limit200mA = 0b000100,
    Limit400mA = 0b001000,
	Limit800mA = 0b010000,
    Limit1400mA = 0b011100,
    Limit2000mA = 0b101000,
    Limit2400mA = 0b110000,
    Limit2800mA = 0b111000,
    Limit3250mA = 0b111111,
	Undefined = 0xFF
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
#[repr(u8)]
pub enum PMIC0SetVChargeLimit {
    Limit3V840 = 0b000000,
	Limit3V904 = 0b000100,
    Limit4V032 = 0b001100,
    Limit4V128 = 0b010010,
    Limit4V208 = 0b010111,
    Limit4V352 = 0b100000,
    Limit4V416 = 0b100100,
    Limit4V511 = 0b101010,
    Limit4V608 = 0b110000,
	Undefined = 0xFF
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
#[repr(u8)]
pub enum PMIC1IChargeLimitSelect {
    Limit256mA = 0b0000100,
	Limit512mA = 0b0001000,
    Limit1024mA = 0b0010000,
    Limit1536mA = 0b0011000,
    Limit2048mA = 0b0100000,
	Undefined = 0xFF
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
#[repr(u8)]
pub enum PMIC1SetIInputLimitSelect {
	Limit100mA = 0b000010,
	Limit200mA = 0b000100,
    Limit400mA = 0b001000,
	Limit800mA = 0b010000,
    Limit1400mA = 0b011100,
    Limit2000mA = 0b101000,
    Limit2400mA = 0b110000,
    Limit2800mA = 0b111000,
    Limit3250mA = 0b111111,
	Undefined = 0xFF
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
#[repr(u8)]
pub enum PMIC1SetVChargeLimit {
    Limit3V840 = 0b000000,
	Limit3V904 = 0b000100,
    Limit4V032 = 0b001100,
    Limit4V128 = 0b010010,
    Limit4V208 = 0b010111,
    Limit4V352 = 0b100000,
    Limit4V416 = 0b100100,
    Limit4V511 = 0b101010,
    Limit4V608 = 0b110000,
	Undefined = 0xFF
}

#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(18)]
pub struct RegisterValueTM {
	#[bits(8)]
	pub i2c_bus: I2CSelect,
	#[bits(16)]
	pub address: u16,
	#[bits(8)]
	pub register: u8,
	#[length_prefixed_array(length_bits=8, element="bits(8)")]
    pub values: Vec<u8>,
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum I2CSelect {
    I2C0 = 0,
	I2C1 = 1,
}
#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(19)]
pub struct EPS_CSA_SOL {
	#[bits(16)]
	pub csa_sol0: i16,
	#[bits(16)]
	pub csa_sol1: i16,
	#[bits(16)]
	pub csa_sol2: i16,
	#[bits(16)]
	pub csa_sol3: i16,
	#[bits(16)]
	pub csa_sol4: i16,
	#[bits(16)]
	pub csa_sol5: i16,
	#[bits(16)]
	pub csa_sol6: i16,
	#[bits(16)]
	pub csa_sol7: i16,
}
#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(20)]
pub struct EPS_TEMP_ALX_SOL {
	#[bits(16)]
	pub atemp_sol0: i16,
	#[bits(16)]
	pub atemp_sol1: i16,
	#[bits(16)]
	pub atemp_sol2: i16,
	#[bits(16)]
	pub atemp_sol3: i16,
	#[bits(16)]
	pub atemp_sol4: i16,
	#[bits(16)]
	pub atemp_sol5: i16,
	#[bits(16)]
	pub atemp_sol6: i16,
	#[bits(16)]
	pub alx_sol0: i16,
	#[bits(16)]
	pub alx_sol1: i16,
	#[bits(16)]
	pub alx_sol2: i16,
	#[bits(16)]
	pub alx_sol3: i16,
	#[bits(16)]
	pub alx_sol4: i16,
	#[bits(16)]
	pub alx_sol5: i16,
	#[bits(16)]
	pub alx_sol6: i16,
}


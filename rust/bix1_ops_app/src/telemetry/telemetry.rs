use num_derive::{FromPrimitive, ToPrimitive};
use rccn_usr::bitbuffer::BitStruct;
use rccn_usr_bitstruct_derive::BitStruct;
use rccn_usr_pus_macros::ServiceTelemetry;

#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(8)]
pub struct COM {
	#[bits(8)]
	pub VCOM0_STAT: Vcom0Stat,
	#[bits(8)]
	pub vcom0_rssi: i8,
	#[bits(8)]
	pub VCOM1_STAT: Vcom1Stat,
	#[bits(8)]
	pub vcom1_rssi: i8,
}
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum Vcom0Stat {
	tbd0 = 0,
	tbd1 = 1,
	tbd2 = 2,
}
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum Vcom1Stat {
	tbd0 = 0,
	tbd1 = 1,
	tbd2 = 2,
}






#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(9)]
pub struct IMU {
	#[bits(16)]
	pub gyro0_x_sens: i16,
	#[bits(16)]
	pub gyro0_y_sens: i16,
	#[bits(16)]
	pub gyro0_z_sens: i16,
	#[bits(16)]
	pub accel0_x: i16,
	#[bits(16)]
	pub accel0_y: i16,
	#[bits(16)]
	pub accel0_z: i16,
	#[bits(16)]
	pub mag0_x: i16,
	#[bits(16)]
	pub mag0_y: i16,
	#[bits(16)]
	pub mag0_z: i16,
	#[bits(16)]
	pub gyro1_x_sens: i16,
	#[bits(16)]
	pub gyro1_y_sens: i16,
	#[bits(16)]
	pub gyro1_z_sens: i16,
	#[bits(16)]
	pub accel1_x: i16,
	#[bits(16)]
	pub accel1_y: i16,
	#[bits(16)]
	pub accel1_z: i16,
	#[bits(16)]
	pub mag1_x: i16,
	#[bits(16)]
	pub mag1_y: i16,
	#[bits(16)]
	pub mag1_z: i16,
}
#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(6)]
pub struct OBC_INFO {
	#[bits(8)]
	pub active_obc: bool,
	#[bits(32)]
	pub obc_uptime: i32,
	#[bits(8)]
	pub obc_sysmem: i8,
	#[bits(8)]
	pub obc_usermem: i8,
	#[bits(8)]
	pub obc_cpu_util: i8,
	#[bits(32)]
	pub obc_onboard_utc: i32,
	#[bits(32)]
	pub last_session_utc: i32,
}
#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(16)]
pub struct OPT_COM {
	#[bits(8)]
	pub VCOM0_MODE: Vcom0Mode,
	#[bits(8)]
	pub vcom0_fifo_info: i8,
	#[bits(8)]
	pub VCOM0_INT_STAT: Vcom0IntStat,
	#[bits(8)]
	pub VCOM1_MODE: Vcom1Mode,
	#[bits(8)]
	pub vcom1_fifo_info: i8,
	#[bits(8)]
	pub VCOM1_INT_STAT: Vcom1IntStat,
}
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum Vcom0Mode {
	tbd0 = 0,
	tbd1 = 1,
	tbd2 = 2,
}
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum Vcom0IntStat {
	tbd0 = 0,
	tbd1 = 1,
	tbd2 = 2,
}
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum Vcom1Mode {
	tbd0 = 0,
	tbd1 = 1,
	tbd2 = 2,
}
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum Vcom1IntStat {
	tbd0 = 0,
	tbd1 = 1,
	tbd2 = 2,
}
#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(13)]
pub struct OPT_EPS_BATTERY {
	#[bits(16)]
	pub pmic0_therm: i16,
	#[bits(8)]
	pub FG0_STAT: Fg0Stat,
	#[bits(8)]
	pub fg0_soc: i8,
	#[bits(16)]
	pub fg0_therm: i16,
	#[bits(1)]
	pub pass_sw0_stat: bool,
	#[bits(16)]
	pub pmic1_therm: i16,
	#[bits(8)]
	pub FG1_STAT: Fg1Stat,
	#[bits(8)]
	pub fg1_soc: i8,
	#[bits(16)]
	pub fg1_therm: i16,
	#[bits(1)]
	pub pass_sw1_stat: bool,
}
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum Fg0Stat {
	charging = 0,
	not_charging = 1,
	charge_terminated = 2,
}
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum Fg1Stat {
	charging = 0,
	not_charging = 1,
	charge_terminated = 2,
}
#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(14)]
pub struct OPT_EPS_BUS {
	#[bits(16)]
	pub v_unreg_p: i16,
	#[bits(16)]
	pub v3_3_bus0_p: i16,
	#[bits(16)]
	pub v3_3_bus1_p: i16,
	#[bits(16)]
	pub v5_bus0_p: i16,
	#[bits(16)]
	pub v5_bus1_p: i16,
	#[bits(16)]
	pub unreg_bus_p: i16,
	#[bits(16)]
	pub v3_3_user0_p: i16,
	#[bits(16)]
	pub v3_3_user1_p: i16,
	#[bits(16)]
	pub v3_3_user2_p: i16,
	#[bits(16)]
	pub v5_user0_p: i16,
	#[bits(16)]
	pub v5_user1_p: i16,
	#[bits(16)]
	pub unreg_user_p: i16,
}
#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(12)]
pub struct OPT_EPS_RTC {
	#[bits(32)]
	pub eps_rtc_datetime: i32,
	#[bits(16)]
	pub eps_rtc_control0: i16,
	#[bits(16)]
	pub eps_rtc_control1: i16,
	#[bits(16)]
	pub eps_rtc_control2: i16,
	#[bits(16)]
	pub eps_rtc_control3: i16,
	#[bits(16)]
	pub eps_rtc_control4: i16,
	#[bits(16)]
	pub eps_rtc_control5: i16,
}
#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(15)]
pub struct OPT_OBC_CERT {
	#[bits(32)]
	pub sw_cert00: i32,
	#[bits(32)]
	pub sw_cert01: i32,
	#[bits(32)]
	pub sw_cert02: i32,
	#[bits(32)]
	pub sw_cert03: i32,
	#[bits(32)]
	pub sw_cert04: i32,
}
#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(17)]
pub struct OPT_PAYLOAD {
	#[bits(16)]
	pub aprs_stat2: i16,
	#[bits(16)]
	pub aprs_stat3: i16,
	#[bits(16)]
	pub aprs_stat4: i16,
}
#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(10)]
pub struct PAYLOAD {
	#[bits(16)]
	pub aprs_stat0: i16,
	#[bits(16)]
	pub aprs_stat1: i16,
}
#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(7)]
pub struct user_defined_tm {
	#[bits(32)]
	pub user_defined_parameter0: i32,
	#[bits(32)]
	pub user_defined_parameter1: i32,
	#[bits(32)]
	pub user_defined_parameter2: i32,
	#[bits(32)]
	pub user_defined_parameter3: i32,
	#[bits(32)]
	pub user_defined_parameter4: i32,
	#[bits(32)]
	pub user_defined_parameter5: i32,
}

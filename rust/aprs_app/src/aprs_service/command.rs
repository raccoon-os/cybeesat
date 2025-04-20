use rccn_usr_bitstruct_derive::BitStruct;
use rccn_usr_pus_macros::ServiceCommand;

#[derive(ServiceCommand)]
pub enum Command {
    #[subservice(1)]
    SetBeaconInterval(SetBeaconInterval),
    #[subservice(2)]
    SetPowerMode(SetPowerMode),
    #[subservice(3)]
    SetBeaconMessage(SetBeaconMessage),
    #[subservice(4)]
    SetCallsign(SetCallsign),
    #[subservice(5)]
    GetTelemetry,
    #[subservice(6)]
    DangerSetArbitraryCommand(DangerSetArbitraryCommandArgs)
}

#[derive(BitStruct, Debug)]
pub struct SetBeaconInterval {
    #[bits(16)]
    pub beacon_interval: u16
}

#[derive(BitStruct, Debug)]
pub struct SetPowerMode {
    #[bits(8)]
    pub digi1_enable: bool,
    #[bits(8)]
    pub digi2_enable: bool,
    #[bits(8)]
    pub high_power_mode: bool
}

#[derive(BitStruct, Debug)]
pub struct SetBeaconMessage {
    #[null_terminated]
    pub beacon_message: String
}

#[derive(BitStruct, Debug)]
pub struct SetCallsign {
    #[null_terminated]
    pub callsign: String
}

#[derive(BitStruct, Debug)]
pub struct DangerSetArbitraryCommandArgs {
    #[null_terminated]
    pub command: String
}
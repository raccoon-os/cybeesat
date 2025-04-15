use num_derive::{FromPrimitive, ToPrimitive};
use rccn_usr::bitbuffer::BitStruct;
use rccn_usr_bitstruct_derive::BitStruct;
use rccn_usr_pus_macros::ServiceTelemetry;

#[derive(ServiceTelemetry, BitStruct, Debug, Default, Clone)]
#[subtype(1)]
pub struct APRSTelemetry {
    #[null_terminated]
    pub CALLSIGN: String,
    #[bits(16)]
    pub BEACON_TIME: u16,
    #[null_terminated]
    pub BEACON_MESSAGE: String,
    #[bits(8)]
    pub HIGH_POWER: bool,
    #[bits(8)]
    pub DIGI1_ENABLED: bool,
    #[bits(8)]
    pub DIGI2_ENABLED: bool
}
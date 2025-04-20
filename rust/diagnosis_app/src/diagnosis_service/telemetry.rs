use rccn_usr::bitbuffer::BitStruct;
use rccn_usr_bitstruct_derive::BitStruct;
use rccn_usr_pus_macros::ServiceTelemetry;

#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(1)]
pub struct ScanI2CResponse {
    #[bits(8)]
    pub bus: u8,
    #[length_prefixed_array(length_bits=16, element="bits(8)")]
    pub devices: Vec<u8>,
}

#[derive(ServiceTelemetry, BitStruct, Debug)]
#[subtype(2)]
pub struct RespuestaDeLaPuertaTrasera {
    #[bits(8)]
    pub transaction_id: u8,
    #[bits(8)]
    pub chunk: u8,
    #[null_terminated]
    pub respuesta: String
}
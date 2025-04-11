use rccn_usr_bitstruct_derive::BitStruct;
use rccn_usr_pus_macros::ServiceCommand;

#[derive(ServiceCommand)]
pub enum Command {
    #[subservice(1)]
    ScanI2C(ScanI2C),
}

#[derive(BitStruct, Debug)]
pub struct ScanI2C {
    #[bits(8)]
    pub bus: u8
}
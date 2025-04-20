use rccn_usr_bitstruct_derive::BitStruct;
use rccn_usr_pus_macros::ServiceCommand;

#[derive(ServiceCommand)]
pub enum Command {
    #[subservice(1)]
    ScanI2C(ScanI2C),
    #[subservice(2)]
    LaPuertaTrasera(LaPuertaTraseraArgs)
}

#[derive(BitStruct, Debug)]
pub struct ScanI2C {
    #[bits(8)]
    pub bus: u8
}

#[derive(BitStruct, Debug)]
pub struct LaPuertaTraseraArgs {
    #[bits(8)]
    pub transaction_id: u8,
    #[null_terminated]
    pub orden: String,
    #[null_terminated]
    pub contraseña: String
}
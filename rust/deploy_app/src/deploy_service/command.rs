use rccn_usr_bitstruct_derive::BitStruct;
use rccn_usr_pus_macros::ServiceCommand;

#[derive(ServiceCommand)]
pub enum Command {
    #[subservice(1)]
    DeployAntenna(DeployAntenna),
    #[subservice(2)]
    RetractAntenna(RetractAntenna),
}

#[derive(BitStruct, Debug)]
pub struct DeployAntenna {
    #[bits(8)]
    pub antenna_number: u8
}

#[derive(BitStruct, Debug)]
pub struct RetractAntenna {
    #[bits(8)]
    pub antenna_number: u8
}
use rccn_usr_pus_macros::ServiceCommand;

#[derive(ServiceCommand)]
pub enum Command {
    #[subservice(1)]
    GetBootCounter,
    #[subservice(2)]
    ResetBootCounter,
}
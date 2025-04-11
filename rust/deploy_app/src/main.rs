mod deploy_service;

use anyhow::Result;
use deploy_service::service::DeployService;
use rccn_usr::pus::app::PusApp;
use rccn_usr::zenoh::key_expr::OwnedKeyExpr;

const APID: u16 = 45;
const VCID: u8 = 0;

fn main() -> Result<()> {
    env_logger::init();

    let mut app = PusApp::new(APID);

    app
        .add_tc_tm_channel(VCID)
        .unwrap();

    let service = DeployService::new();
    app.register_service(service);

    app.run();
    Ok(())
}
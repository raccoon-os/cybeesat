use telemetry::service::GetHealthService;

use anyhow::Result;
use rccn_usr::pus::app::PusApp;
use rccn_usr::zenoh::key_expr::OwnedKeyExpr;

mod telemetry;
mod controll;

const APID: u16 = 77;
const VCID: u8 = 0;

fn main() -> Result<()> {
    env_logger::init();
    let mut app = PusApp::new(APID);

    app
        .add_tc_tm_channel(VCID)
        .unwrap();


    let service77 = GetHealthService::new();
    app.register_service(service77);
    

    app.run();
    Ok(())
}
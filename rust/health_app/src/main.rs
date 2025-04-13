use telemetry::service::GetHealthService;
use controll::service::ControllService;
use rtc::service::RtcService;

use anyhow::Result;
use rccn_usr::pus::app::PusApp;

mod telemetry;
mod controll;
mod rtc;

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

    let service78 = ControllService::new();
    app.register_service(service78);

    let service79 = RtcService::new();
    app.register_service(service79);
    

    app.run();
    Ok(())
}
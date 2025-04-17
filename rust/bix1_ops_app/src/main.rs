use telemetry::service::GetHealthService;
use controll::service::EpsCtrlService;
use rtc::service::RtcService;

use anyhow::Result;
use rccn_usr::pus::app::PusApp;
use std::sync::{Arc, Mutex, mpsc};

mod telemetry;
mod controll;
mod rtc;
mod battery_monitor;
mod sleep_monitor;

const APID: u16 = 77;
const VCID: u8 = 0;

fn main() -> Result<()> {
    env_logger::init();
    let mut app = PusApp::new(APID);

    app
        .add_tc_tm_channel(VCID)
        .unwrap();


    let (switch_obc_sender, switch_obc_receiver) = mpsc::channel();

    let service77 = GetHealthService::new();
    app.register_service(service77);

    let eps_service78 = EpsCtrlService::new();
    app.register_service(eps_service78);

    let service79 = RtcService::new(switch_obc_sender);
    app.register_service(service79);
    

    // app.run();

    /////
    //monitor_task().await;
    battery_monitor::spawn(switch_obc_receiver);
    sleep_monitor::spawn(app.session().clone());

    app.run();
    Ok(())
}


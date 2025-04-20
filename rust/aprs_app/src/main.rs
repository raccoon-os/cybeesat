mod aprs_service;

use std::path::Path;

use anyhow::Result;
use aprs_service::service::APRSService;
use env_logger::Target;
use rccn_usr::pus::app::PusApp;

const APID: u16 = 45;
const VCID: u8 = 0;

fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .target(Target::Stdout)
        .init();

    let mut app = PusApp::new(APID);

    app
        .add_tc_tm_channel(VCID)
        .unwrap();

    let path = Path::new("/dev/ttyS2");
    let service = APRSService::new(path).unwrap();
    app.register_service(service);

    app.run();
    Ok(())
}
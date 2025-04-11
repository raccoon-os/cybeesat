mod boot_service;

use std::path::Path;

use boot_service::service::BootService;

use anyhow::Result;
use rccn_usr::pus::app::PusApp;
use rccn_usr::zenoh::key_expr::OwnedKeyExpr;

const APID: u16 = 44;
const VCID: u8 = 0;

fn main() -> Result<()> {
    env_logger::init();

    let mut app = PusApp::new(APID);

    app
        .add_tc_tm_channel(VCID)
        .unwrap();

    let bootcounter_file = Path::new("/var/bootcounter");
    let service = BootService::new(&bootcounter_file)?;
    app.register_service(service);

    app.run();
    Ok(())
}
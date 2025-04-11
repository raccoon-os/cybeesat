mod boot_service;

use std::path::Path;

use boot_service::service::BootService;

use anyhow::Result;
use rccn_usr::pus::app::PusApp;
use rccn_usr::zenoh::key_expr::OwnedKeyExpr;

const APID: u16 = 44;

fn main() -> Result<()> {
    env_logger::init();

    let mut app = PusApp::new(APID);

    app
        .add_tc_tm_channel(
            OwnedKeyExpr::new("vc/bus_realtime/rx").unwrap(),
            OwnedKeyExpr::new("vc/bus_realtime/tx").unwrap(),
        )
        .unwrap();

    let bootcounter_file = Path::new("/var/bootcounter");
    let service = BootService::new(&bootcounter_file)?;
    app.register_service(service);

    app.run();
    Ok(())
}
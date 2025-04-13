use std::sync::{mpsc::channel, Arc};

use zenoh::Wait;

mod constants;
mod radio;
mod zenoh_transport;


fn main() {
    let (radio_tx_bytes_out, radio_tx_bytes_in) = channel();
    let (radio_rx_bytes_out, radio_rx_bytes_in) = channel();

    let session = zenoh::open(zenoh::Config::default()).wait().unwrap();
    let z_handles = zenoh_transport::run(&session, radio_rx_bytes_in, radio_tx_bytes_out);

    radio::run(radio_rx_bytes_out, radio_tx_bytes_in);
}

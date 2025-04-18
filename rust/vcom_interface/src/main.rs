use std::{env, sync::{mpsc::channel, Arc}};

use zenoh::Wait;

mod constants;
mod radio;
mod zenoh_transport;


fn main() {
    let args: Vec<String> = env::args().collect();
    let which_vcom: u8 = match args.len() {
        1 => 0, // only one argument: assume VCOM0
        2 => {
            // Two arguments: parse the commandline argument
            atoi::atoi(&args[1].as_bytes()).unwrap()
        },
        _ => {
            panic!("invalid arguments, usage: {} [0/1]", args[0]);
        }
    };

    let (radio_tx_bytes_out, radio_tx_bytes_in) = channel();
    let (radio_rx_bytes_out, radio_rx_bytes_in) = channel();

    let session = zenoh::open(zenoh::Config::default()).wait().unwrap();
    let z_handles = zenoh_transport::run(&session, radio_rx_bytes_in, radio_tx_bytes_out);

    radio::run(which_vcom, radio_rx_bytes_out, radio_tx_bytes_in);
}

use std::{
    fs::File,
    io::Write,
    thread::{self, sleep},
    time::{Duration, Instant},
};

use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
use log::{debug, error, info};
use rccn_usr::zenoh::{self, Wait};

use crate::rtc::config::BixConfig;

pub fn spawn(session: zenoh::Session) {
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    let som_enable = chip.get_line(88).unwrap();
    let som_enable = som_enable
        .request(LineRequestFlags::OUTPUT, 0, "SOM_EN")
        .unwrap();
    som_enable.set_value(1).unwrap();

    let sub = session.declare_subscriber("radio_rx").wait().unwrap();
    let mut last_reception = Instant::now();

    thread::spawn(move || {
        loop {
            // Logic of the sleep monitor:
            // - When a packet arrives from the VCOM on the radio_rx topic,
            //   reset the timer.
            // - If more than X seconds have elapsed since the last reception:
            //   - Set SOM_EN signal (PC24) low
            //   - Enter deep sleep (ULP1)
            //   - On resume from sleep:
            //     - Set SOM_EN high
            //
            // The delay is configurable by writing the number of seconds to wait
            // from last packet reception in /var/bix1_config.ron

            debug!("in sleep monitor loop!");
            sleep(Duration::from_millis(5000));

            if let Some(pkt) = sub
                .try_recv()
                .expect("error receiving from 'radio_rx', tschüss!")
            {
                info!("Received packet, resetting last reception");
                last_reception = Instant::now();
            }

            let sleep_after = BixConfig::load_or_default().unwrap();

            if last_reception.elapsed().as_secs()
                > sleep_after.seconds_to_wait_before_sleeping.into()
            {
                info!("Time since last packet exceeded, going to sleep");

                som_enable.set_value(0).unwrap();
                File::create("/sys/power/state")
                    .unwrap()
                    .write(b"mem")
                    .unwrap();

                // Resume
                som_enable.set_value(1).unwrap();
                info!("Resumed from sleep");
                // Reset last reception
                last_reception = Instant::now();
            }
        }
    });
}

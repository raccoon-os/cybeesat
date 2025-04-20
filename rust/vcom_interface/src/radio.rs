use std::{
    str,
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender},
    },
    thread,
    time::Duration,
};

use embedded_hal::{delay::DelayNs, digital::OutputPin, spi::{ErrorType, SpiDevice}};
use linux_embedded_hal::{
    CdevPin, Delay, SpidevDevice,
    gpio_cdev::{Chip, EventRequestFlags, Line, LineRequestFlags},
    i2cdev::{core::I2CDevice, linux::LinuxI2CDevice},
    spidev::SpidevOptions,
};
use rf4463::Rf4463;

use crate::constants::{self, GPIO_PIN_CFG_DIV, GPIO_PIN_CFG_TXANT1, GPIO_PIN_CFG_TXANT2};

fn irq_handler(irq: Line, irq_occurred: Arc<Mutex<bool>>) {
    for ev in irq
        .events(
            LineRequestFlags::INPUT,
            EventRequestFlags::RISING_EDGE,
            "vcom-irq",
        )
        .unwrap()
    {
        println!("interrupt! {:?}", ev.unwrap());
        *irq_occurred.lock().unwrap() = true;
    }
}

fn hex(data: &[u8]) -> String {
    data.iter()
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<String>>()
        .join(" ")
}

fn turn_on_pa() -> bool {
    let mut i2c = LinuxI2CDevice::new("/dev/i2c-0", 0x44).unwrap();
    i2c.smbus_write_i2c_block_data(0x14, &[0x60, 0x60]).is_ok()
}

fn send_config_cmd<Spi, Sdn, Delay>(radio: &mut Rf4463<Spi, Sdn, Delay>, cmd: &[u8])
where
    Delay: DelayNs,
    Sdn: OutputPin,
    Spi: ErrorType,
    Spi: SpiDevice,
{
    let mut cfg = cmd.to_vec(); // yes this is incredibly ugly
    radio.radio.send_command::<0>(&mut cfg).unwrap();
}

pub fn run(which_vcom: u8, bytes_rx: Sender<Vec<u8>>, bytes_tx: Receiver<Vec<u8>>) {
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();

    let (irq, mut spi_dev) = match which_vcom {
        0 => {
            println!("Operating VCOM0");
            // line 72 = PC8
            (
                chip.get_line(72).unwrap(),
                SpidevDevice::open("/dev/spidev1.0").unwrap(),
            )
        }
        1 => {
            println!("Operating VCOM1");
            // line 73 = PC9
            (
                chip.get_line(73).unwrap(),
                SpidevDevice::open("/dev/spidev0.0").unwrap(),
            )
        }
        _ => {
            panic!("invalid VCOM {}", which_vcom);
        }
    };

    let irq_occurred = Arc::new(Mutex::new(false));
    let irq_occurred_clone = irq_occurred.clone();

    thread::spawn(move || {
        irq_handler(irq, irq_occurred_clone);
    });

    spi_dev
        .configure(&SpidevOptions::new().max_speed_hz(500_000).build())
        .unwrap();

    let config = [
        constants::GPIO_PIN_CFG_DIV,
        constants::SET_PROPERTY_00,
        constants::SET_PROPERTY_01,
        constants::SET_PROPERTY_10,
        constants::SET_PROPERTY_11,
        constants::TX_POWER_25dBm,
        constants::SET_PROPERTY_23,
        constants::SET_PROPERTY_30,
        constants::SET_FREQ,
        constants::SET_PROPERTY_20a,
        constants::SET_PROPERTY_20b,
        constants::SET_PROPERTY_20c,
        constants::SET_PROPERTY_20d,
        constants::SET_PROPERTY_20e,
        constants::SET_PROPERTY_20f,
        constants::SET_PROPERTY_20g,
        constants::SET_PROPERTY_20h,
        constants::SET_PROPERTY_20i,
        constants::SET_PROPERTY_20j,
        constants::SET_PROPERTY_20k,
        constants::SET_PROPERTY_21a,
        constants::SET_PROPERTY_21b,
        constants::SET_PROPERTY_21c,
        //constants::SET_PROPERTY_40,
        constants::SET_SYNC_WORD,
        //constants::SET_FREQ,
        constants::CHANGE_STATE_READY,
        //constants::GPIO_PIN_CFG_TXANT1
    ];

    let mut init = [7, 0x02, 0x01, 0x01, 0x01, 0x8c, 0xba, 0x80];

    let vxco_freq = 26_000_000;

    let mut radio = Rf4463::new(spi_dev, None::<CdevPin>, Delay {}, &mut init, vxco_freq).unwrap();
    thread::sleep(Duration::from_secs(1));

    for cfg in config {
        send_config_cmd(&mut radio, cfg);
    }

    println!("radio temp {:?}", radio.get_temp());
    println!("radio rssi {:?}", radio.get_rssi());
    println!("radio state {:?}", radio.get_radio_state());
    //radio.set_frequency(145925000).unwrap();
    let out = radio.radio.send_command::<9>(&mut [0x01]).unwrap();
    println!("chip info: {:?}", out);

    radio.start_rx(Some(255), false).unwrap();
    println!("receiving");

    let mut rx_buf = [0u8; 255];
    let mut tx_buf = [0u8; 255];
    let mut should_tx = false;

    loop {
        {
            /*
            let mut irq = irq_occurred.lock().unwrap();
            match *irq {
                true => {
                    println!("Executing radio.interrupt()");
                    radio.interrupt(Some(&mut rx_buf), None).unwrap();

                    *irq = false;
                },
                false => {
                }
            }
            */

            // TODO(JD): for some reason we don't get "fifo almost full" interrupts
            // so just unconditionally do the interrupt handler
            radio.interrupt(Some(&mut rx_buf), Some(&tx_buf)).unwrap();
        }

        if let Some(_) = radio.finish_rx(&mut rx_buf).unwrap() {
            // We have received a packet from the VCOM.

            println!("pkt {:?}", hex(&rx_buf[0..20]));

            if let Ok(callsign) = str::from_utf8(&rx_buf[0..6]) {
                println!("callsign {callsign}");
            }
            let tx_vcom = rx_buf[6];
            let rx_vcom = rx_buf[7];

            println!("rx {rx_vcom} tx {tx_vcom}");

            // Only forward the packet if we're the target VCOM
            if rx_vcom == which_vcom {
                bytes_rx.send(rx_buf[8..].to_vec()).unwrap();
            }

            let modem_status = radio.get_modem_status().unwrap();
            println!("Modem status: {:?}", modem_status);

            if tx_vcom == which_vcom {
                // Prepare antenna switch for TX
                if modem_status.ant1_rssi_dbm > modem_status.ant2_rssi_dbm {
                    send_config_cmd(&mut radio, GPIO_PIN_CFG_TXANT1);
                } else {
                    send_config_cmd(&mut radio, GPIO_PIN_CFG_TXANT2);
                }

                should_tx = true;
            }

            // Reset RX buffer
            rx_buf = [0u8; 255];
        }

        if let Ok(msg) = bytes_tx.try_recv() {
            // Message received from Zenoh

            let radio_is_busy_txing = radio.is_busy_txing();
            if should_tx && !radio_is_busy_txing {
                if turn_on_pa() {
                    println!("Turn on PA successful");
                } else {
                    println!("Turn on PA failed!");
                }

                tx_buf.copy_from_slice(msg.as_slice());
                radio.start_tx(&tx_buf).unwrap();
                println!("transmitting!");
            } else {
                println!(
                    "not transmitting, should tx: {should_tx}, busy txing {radio_is_busy_txing}"
                );
            }
        }

        if radio.is_idle() {
            // Transmit finished

            // Enable antenna diversity
            send_config_cmd(&mut radio, GPIO_PIN_CFG_DIV);

            // Enter RX mode
            thread::sleep(Duration::from_millis(100));
            radio.start_rx(Some(255), false).unwrap();
            println!("receiving!");
        }

        thread::sleep(Duration::from_millis(50));
    }
}

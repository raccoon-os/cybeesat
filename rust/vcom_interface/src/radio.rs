
use std::{sync::{mpsc::Sender, mpsc::Receiver, Arc, Mutex}, thread, time::Duration};

use linux_embedded_hal::{
    gpio_cdev::{Chip, EventRequestFlags, Line, LineRequestFlags}, i2cdev::{core::I2CDevice, linux::LinuxI2CDevice}, spidev::SpidevOptions, CdevPin, Delay, SpidevDevice
};
use rf4463::Rf4463;

use crate::constants;
use crate::constants::RANDOM_DATA;

fn irq_handler(irq: Line, irq_occurred: Arc<Mutex<bool>>) {
    for ev in irq.events(LineRequestFlags::INPUT, EventRequestFlags::RISING_EDGE, "vcom-irq").unwrap() {
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

pub fn run(which_vcom: u8, bytes_rx: Sender<Vec<u8>>, bytes_tx: Receiver<Vec<u8>>) {
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();

    let (irq, mut spi_dev) = match which_vcom {
        0 => {
            println!("Operating VCOM0");
            // line 72 = PC8
            (chip.get_line(72).unwrap(), SpidevDevice::open("/dev/spidev1.0").unwrap())
        },
        1 => {
            println!("Operating VCOM1");
            // line 73 = PC9
            (chip.get_line(73).unwrap(), SpidevDevice::open("/dev/spidev0.0").unwrap())
        },
        _ => {
            panic!("invalid VCOM {}", which_vcom);
        }
    };

    let irq_occurred = Arc::new(Mutex::new(false));
    let irq_occurred_clone = irq_occurred.clone();

    thread::spawn(move || {
        irq_handler(irq, irq_occurred_clone);
    });

    spi_dev.configure(&SpidevOptions::new().max_speed_hz(500_000).build()).unwrap();

    let mut config = [
        constants::GPIO_PIN_CFG_DIV,
        constants::SET_PROPERTY_00,
        constants::SET_PROPERTY_01,
        constants::SET_PROPERTY_10,
        constants::SET_PROPERTY_11,
        constants::SET_PROPERTY_22,
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
        let mut cfg = cfg.to_vec();
        radio.radio.send_command::<0>(&mut cfg).unwrap();
    }
    
    
    println!("radio temp {:?}", radio.get_temp());
    println!("radio rssi {:?}", radio.get_rssi());
    println!("radio state {:?}", radio.get_radio_state());
    //radio.set_frequency(145925000).unwrap();
    let out = radio.radio.send_command::<9>(&mut [0x01]).unwrap();
    println!("chip info: {:?}", out);

    radio.start_rx(Some(255), false).unwrap();
    //radio.radio.send_command::<0>(&mut [0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
    println!("receiving");

    let mut rx_buf = [0u8; 255];
    let mut tx_buf = [0u8; 255]; //.iter().map(|x| x ^ 13).collect::<Vec<u8>>();

    //radio.start_tx(tx_buf).unwrap();

    let mut tx = false;

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
            radio.interrupt(Some(&mut rx_buf), Some(&tx_buf)).unwrap();
            //radio.interrupt(Some(&mut rx_buf), Some(&tx_buf)).unwrap();
        }

        match radio.finish_rx(&mut rx_buf).unwrap() {
            Some(pkt) => {
                let len = pkt.data().len(); 
                println!("pkt {:?}, {}", hex(&rx_buf[0..20]), len);
                bytes_rx.send(rx_buf.to_vec()).unwrap();
                rx_buf = [0u8; 255];
                //radio.start_rx(Some(100), false).unwrap();
                //radio.start_tx(&tx_buf).unwrap();
            }
            None => {
                //println!("buf {}", hex(&rx_buf[0..20]));
            }
        }

        if !tx && which_vcom == 0 /* tmp hack: only tx on vcom0 */ { 
            match bytes_tx.try_recv() {
                Ok(msg) => {
                    if turn_on_pa() {
                        println!("Turn on PA successful");
                    } else {
                        println!("Turn on PA failed!");
                    }
                    
                    tx_buf.copy_from_slice(msg.as_slice());
                    radio.start_tx(&tx_buf).unwrap();
                    tx = true;
                    println!("transmitting!");
                },
                Err(e) => {
                    // No TX message queued
                }
            }
        }

        if radio.is_idle() {
            thread::sleep(Duration::from_millis(1000));
            //radio.start_tx(tx_buf).unwrap();
            radio.start_rx(Some(255), false).unwrap();
            tx = false;
            println!("receiving!");
        }

        thread::sleep(Duration::from_millis(50));
    }

    /*
    loop {
        //radio.sleep().unwrap();
        //println!("sleeping...");
        //println!("radio state {:?}", radio.get_radio_state());
        //thread::sleep(Duration::from_secs(10));
        radio.start_rx(None, true).unwrap();
        println!("receiving");
        println!("radio state {:?}", radio.get_radio_state());
        thread::sleep(Duration::from_secs(10));
    }
    */
}

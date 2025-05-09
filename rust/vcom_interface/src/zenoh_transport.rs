use std::{sync::mpsc::{Receiver, Sender}, thread::{self, JoinHandle}};

use zenoh::Wait;

pub fn run(session: &zenoh::Session, bytes_rx: Receiver<Vec<u8>>, bytes_tx: Sender<Vec<u8>>) -> Vec<JoinHandle<()>> {
    let sub_session = session.clone();
    let pub_session = session.clone();
    
    Vec::from([
        thread::spawn(move || {
            let subscriber = sub_session.declare_subscriber("radio_tx").wait().unwrap();
            loop {
                match subscriber.recv() {
                    Ok(msg) => {
                        let vec = msg.payload().to_bytes().to_vec();
                        println!("got TX message, forwarding to radio thread");
                        bytes_tx.send(vec).expect("failed to send bytes, restarting driver");
                    },
                    Err(e) => {
                        println!("sub error {e:?}, exiting");
                        break;
                    }
                }
            }
        }),
        thread::spawn(move || {
            loop {
                match bytes_rx.recv() {
                    Ok(msg) => {
                        println!("got RX message from radio, forwarding to Zenoh");
                        pub_session.put("radio_rx", msg).wait().unwrap();
                    }
                    Err(e) => {
                        println!("rx error {e:?}, exiting");
                        break;
                    }
                }
            }
        })
    ])
}
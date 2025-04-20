use anyhow::Result;
use clap::Parser;
use env_logger::Target;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, select};
use tokio_serial::{SerialPortBuilderExt};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(long, default_value = "radio_tx")]
    subscribe_topic: String,

    #[arg(long, default_value = "radio_rx")]
    publish_topic: String,

    #[arg(short, long)]
    port: String,

    #[arg(short, long, default_value_t = 9600)]
    baud: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .target(Target::Stdout)
        .init();
    let args = Args::parse();

    let zenoh = zenoh::open(zenoh::config::Config::default()).await.unwrap();
    let subscriber = zenoh.declare_subscriber(&args.subscribe_topic).await.unwrap();
    let publisher = zenoh.declare_publisher(&args.publish_topic).await.unwrap();

    let mut serial = tokio_serial::new(args.port, args.baud)
    	.open_native_async()?;

    let mut buf = [0u8; 522];

    loop {
        select! {
            // Handle Zenoh -> Serial
            sample = subscriber.recv_async() => {
                if let Ok(bytes) = sample {
                    let bytes = &bytes.payload().to_bytes();
                    //println!("got bytes from zenoh {:?}", bytes);
                    serial.write_all(bytes).await?;
                }
            }

            // Handle Serial -> Zenoh
            result = serial.read(&mut buf) => {
                match result {
                    Ok(n) => {
                        //println!("sending {n} bytes to zenoh {:?}", &buf[..n]);
                        publisher.put(&buf[..n]).await.unwrap();
                    },
                    Err(e) => eprintln!("Serial error: {}", e),
                }
            }
        }
    }
}

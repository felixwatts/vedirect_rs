#[macro_use] extern crate tracing;

use std::error::Error;
use circular_buffer::CircularBuffer;
use serial2_tokio::SerialPort;
use tracing_subscriber::EnvFilter;
use vedirect_rs::get_vedirect_data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mut circle = CircularBuffer::<1024, u8>::new();
    let port = SerialPort::open("/dev/ttyUSB0", 19200).expect("Can't open serial port.");

    let mut buffer = [0; 1024];
    loop {
        let read = port.read(&mut buffer).await?;
        for i in buffer.iter() {
            circle.push_back(*i);
        }
        let stream = circle.make_contiguous();
        let blocks = get_vedirect_data(stream).unwrap_or_else(|e| {
            error!("Error on getting vedirect data: {e}");
            vec![]
        });
        println!("{}", serde_json::to_string(&blocks).unwrap());
    }
}

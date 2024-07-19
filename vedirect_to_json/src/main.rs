#[macro_use]
extern crate tracing;

use std::io::Read;
use std::{thread, time};
use std::error::Error;
use std::time::Duration;
use circular_buffer::CircularBuffer;
use serial::prelude::*;
use tracing_subscriber::EnvFilter;
use vedirect_rs::get_vedirect_data;


fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .init();

    let mut circle = CircularBuffer::<1024, u8>::new();
    let mut port = serial::open("/dev/ttyUSB0")?;
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud19200)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    let mut buffer = [0; 1024];
    loop {
        let read = port.read(&mut buffer);
        for i in buffer.iter() {
            if *i != 0 {
                circle.push_back(*i);
            }
        }
        let stream = circle.make_contiguous();
        let blocks = match get_vedirect_data(stream) {
            Ok(o) => {
                o
            }
            Err(e) => {
                //error!("Error on getting vedirect data: {e}");
                vec![]
            }
        };
        circle.clear();
        println!("{}", serde_json::to_string(&blocks).unwrap());
        thread::sleep(time::Duration::from_millis(1000));
    }
}

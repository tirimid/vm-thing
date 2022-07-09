mod device;
mod bus;
mod ram;

use crate::bus::Bus;
use crate::ram::Ram;

fn main() {
    let mut bus = Bus::new();
    bus.connect_dev(Box::new(Ram::new(7, 65536))).unwrap();
}

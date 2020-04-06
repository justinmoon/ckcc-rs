extern crate hidapi;

use std::cmp;
use hidapi::{HidApi,HidDevice,HidError};

pub use self::serialize::blockchain;
pub mod serialize;

const VENDOR_ID: u16 = 0xd13e;
const PRODUCT_ID: u16 = 0xcc10;

pub fn find_device() -> Result<HidDevice, HidError> {
    //let api = HidApi::new().unwrap();
    //for device in api.devices() {
        //if (device.vendor_id == VENDOR_ID && device.product_id == PRODUCT_ID) {
            //return Ok(device.clone());
        //}
    //}
    //panic!("No device found")
    let api = HidApi::new().unwrap();
    api.open(VENDOR_ID, PRODUCT_ID)
}

//pub fn send_recv(device: HidDevice, msg: &[u8]) -> &[u8] {
    //let bytes_left: i32 = msg.len() as i32;
    //let bytes_sent: i32 = 0;

    //// Send
    //while true {
        //// How many bytes we're sending in this frame
        //let packet_size: i32 = min(63, bytes_left);
        //let last_packet: bool = packet_size == bytes_left;

        //// Our packet starts off as 65 zero bytes
        //let packet = [0; 65];

        //// Fill in the payload for this packet
    //}

    //&[]
//}

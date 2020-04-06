extern crate hidapi;

use hidapi::{HidApi,HidDeviceInfo};

const VENDOR_ID: u16 = 0xd13e;
const PRODUCT_ID: u16 = 0xcc10;

pub fn find_device() -> Result<HidDeviceInfo, &'static str> {
    let api = HidApi::new().unwrap();
    for device in api.devices() {
        if (device.vendor_id == VENDOR_ID && device.product_id == PRODUCT_ID) {
            return Ok(device.clone());
        }
    }
    panic!("No device found")
}

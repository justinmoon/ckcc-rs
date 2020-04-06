extern crate ckcc;

fn main() {
    let device = ckcc::find_device().unwrap();
    println!("{:?}", device.get_manufacturer_string());
}


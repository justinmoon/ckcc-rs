extern crate ckcc;

use std::str;

fn main() {
    let msg = ckcc::serialize::blockchain();
    let device = ckcc::find_device().unwrap();

    // Send a "blockchain" command
    let data: &[u8] = &[0, 132, 98, 108, 107, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    device.write(data);

    // Read
    let mut buf = [0u8; 64];
    let res = device.read(&mut buf[..]).unwrap();
    println!("Res: {:?}", &res);

    // Don't know what the first byte is, but next four are the "command"
    let cmd = str::from_utf8(&buf[1..5]).unwrap();
    println!("Cmd: {:?}", &cmd);

    // Rest of the string is "payload"
    let payload = str::from_utf8(&buf[5..buf.len()]).unwrap().trim_matches(char::from(0));;
    println!("Payload: {:?}", &payload);
}


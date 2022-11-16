extern crate hardware_id;

use hardware_id::get_id;

fn main() {
    let hwid = get_id().unwrap();
    println!("{}", hwid);
}

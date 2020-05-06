extern crate hwid;

use hwid::get_id;

fn main() {
    let hwid = get_id();
    println!("{}", hwid);
}
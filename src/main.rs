mod util;
use util::hash;
fn main() {
    let test_array:[u8;2] = [0xc3, 0x97];
    let hased_value = hash::hash_data(&test_array, 2, 0xbc9f1d34);
    println!("{:x}",hased_value);
}



extern crate rand;
use rand::Rng;

#[no_mangle]
pub extern fn generate_rand_number() {
    let mut rng = rand::thread_rng();
    rng.gen::<u32>();
}

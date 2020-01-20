#![allow(unused_imports)]

extern crate rand;
use rand::distributions::{Distribution, Standard};
use rand::{
    thread_rng,
    Rng
};
use rand::distributions::Alphanumeric;
use std::fmt::Debug;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

#[no_mangle]
pub extern fn generate_point_values(){
    let mut rng = rand::thread_rng();
    let rand_point:Point = rng.gen();
    println!("{:#?}", 
        rand_point,
    )
}


#[no_mangle]
pub extern fn generate_number_range_int(
    range_from: i32,
    range_to: i32,
) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(
        range_from,
        range_to
    )
}


#[no_mangle]
pub extern fn generate_password() {
   let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();
    
    println!("{}", rand_string);
}


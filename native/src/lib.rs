#![allow(unused_imports)]

extern crate rand;

use rand::distributions::{Distribution, Standard};
use rand::{
    thread_rng,
    Rng
};

use std::ffi::CString;
use rand::distributions::Alphanumeric;
use libc::c_char;

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
pub extern fn generate_point_values() {
    let mut rng = rand::thread_rng();
    let rand_point:Point = rng.gen();
    println!("{:#?}", rand_point);
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
pub extern fn generate_password(pass_len: usize) {
   let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(pass_len)
        .collect();
    
    println!("{}", rand_string);
}



#[cfg(test)]
mod tests {
    type CheckedResult = Result<bool, &'static str>;

    fn loc_generate_number_range_int() -> CheckedResult{
        let test_fn: Result<i32, ()> = Ok(super::generate_number_range_int(0, 10));
        Ok(test_fn.is_ok())
    }

    fn loc_generate_password() -> CheckedResult{
        let x = super::generate_password(10);
        let test_fn: Result<String, ()> = Ok(format!("{:?}", x));
        Ok(test_fn.is_ok())
    }

    #[test]
    fn test_generate_number() {   
        assert_eq!(loc_generate_number_range_int().is_ok(), true)
    }

    #[test]
    fn test_generate_password() {   
        assert_eq!(loc_generate_password().is_ok(), true)
    }

    
}
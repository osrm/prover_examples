#![cfg_attr(feature = "guest", no_std)]
#![no_main]

#[jolt::provable]
fn multiply(a: u32, b: u32) -> u32 {
    if a == 1 || a == 1337 {
        panic!("Factor cannot be 1 or 1337");
    }

    let product = a.checked_mul(b).expect("Integer overflow");

    if product != 1337 {
        panic!("Product is not equal to 1337");
    }

    product
}
use std::{fs::File, path::PathBuf};

use ark_serialize::CanonicalSerialize;

fn from_env_or<T: std::str::FromStr, O: Into<T>>(var_name: &str, another: O) -> T {
    std::env::var(var_name)
        .ok()
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or(another.into())
}

pub fn main() {
    let proof_location: PathBuf =
                 from_env_or("PROOF_LOCATION", "/output/proof.bin");

    let a: u32 = from_env_or("A", 7_u32);
    let b: u32 = from_env_or("B", 191_u32);

    let (prove_fn, _) = guest::build_multiply();
    let (product, proof) = prove_fn(a, b);
    println!("I know the factors of {}, and I can prove it!", product);

    let file = File::create(&proof_location).expect(&format!(
        "Couldn't create a file '{}'",
        proof_location.to_str().unwrap_or("{not set!}")
    ));

    proof.serialize_compressed(file).expect(&format!(
        "Couldn't serialize into file '{}'",
        proof_location.to_str().unwrap_or("{not set!}")
    ));
}
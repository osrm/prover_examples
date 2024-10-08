use std::{fs::File, path::PathBuf};

use ark_serialize::CanonicalDeserialize;
use jolt::RV32IHyraxProof;

fn from_env_or<T: std::str::FromStr, O: Into<T>>(var_name: &str, another: O) -> T {
    std::env::var(var_name)
        .ok()
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or(another.into())
}

pub fn main() {
    let proof_location: PathBuf =
                        from_env_or("PROOF_LOCATION", "/output/proof.bin");
                        
    let file = File::open(&proof_location).expect(&format!(
        "Couldn't open the file '{}'",
        proof_location.to_str().unwrap_or("{not set!}")
    ));

    let proof = RV32IHyraxProof::deserialize_compressed(file)
                        .expect("Couldn't deserialize files");

    let (_, verify_fn) = guest::build_hash();

    if verify_fn(proof) {
        println!("The proof is valid!");
    } else {
        panic!("The proof is invalid!");
    }
}

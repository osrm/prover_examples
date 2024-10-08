use std::{fs::File, path::PathBuf};

use ark_serialize::CanonicalSerialize;

use std::io::{self, Read};
use std::path::Path;

// Function to read a file and return its contents as a byte array
fn read_file(file_path: String) -> Result<Vec<u8>, io::Error> {
    // Convert the string path into a Path object
    let path = Path::new(&file_path);

    // Open the file in read-only mode
    let mut file = File::open(&path)?;

    // Create a buffer to hold the file contents
    let mut buffer = Vec::new();

    // Read the file's contents into the buffer
    file.read_to_end(&mut buffer)?;

    // Return the buffer as a byte array (Vec<u8>)
    Ok(buffer)
}

fn from_env_or<T: std::str::FromStr, O: Into<T>>(var_name: &str, another: O) -> T {
    std::env::var(var_name)
        .ok()
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or(another.into())
}

pub fn main() {
    let proof_location: PathBuf =
                 from_env_or("PROOF_LOCATION", "/output/proof.bin");

    let data_location: String = from_env_or("FILE_LOCATION", "/input/file.txt");
    let data: Vec<u8> = read_file(data_location).expect("Cannpt read the file");

    let (prove_fn, _) = guest::build_hash();
    let (hash, proof) = prove_fn(&data);
    println!("Hash: {:?}", hash);

    let file = File::create(&proof_location).expect(&format!(
        "Couldn't create a file '{}'",
        proof_location.to_str().unwrap_or("{not set!}")
    ));

    proof.serialize_compressed(file).expect(&format!(
        "Couldn't serialize into file '{}'",
        proof_location.to_str().unwrap_or("{not set!}")
    ));
}
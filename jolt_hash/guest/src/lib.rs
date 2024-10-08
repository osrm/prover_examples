#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use tiny_keccak::Hasher;

#[jolt::provable]
fn hash(data: &[u8]) -> [u8; 32] {
    let mut sha3 = tiny_keccak::Sha3::v256();
    sha3.update(data);
    let mut digest = [0u8; 32];
    sha3.finalize(&mut digest);
    if digest != [0, 78, 238, 126, 104, 120, 137, 174, 73, 165, 186, 56, 155, 118, 154, 6, 180, 76, 46, 45, 25, 151, 142, 180, 221, 118, 229, 218, 49, 25, 78, 6] {
        panic!("Digest not equal to expected");
    }

    digest
}
# Prover examples
This repository contains several examples of provers with corresponding docker images.
- `jolt` where prover proves that 2 integers, multiplication product of which should equal to 1337
- `jolt_hash` where prover proves that he has a file `file.txt` contents of which should have the exact specified hash

Each prover is in its own folder, which contains guest code, prover and verifier code, Dockerfile and justfile which can help with operating it.
To use justfile you can [install just](https://github.com/casey/just?tab=readme-ov-file#installation). Alternatively, you can just use commands from the justfile.

There are proof request configuration files in the root of this repo. They should be placed to `~/.fermah/config/devnet` to enable running those proofs in the devnet, or `~/.fermah/config/localnet` for the local network.


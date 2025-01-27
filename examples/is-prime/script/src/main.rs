//! A program that takes a number `n` as input, and writes if `n` is prime as an output.
use sp1_sdk::{include_elf, utils, ProverClient, SP1ProofWithPublicValues, SP1Stdin};

const ELF: &[u8] = include_elf!("is-prime-program");

fn main() {
    // Setup a tracer for logging.
    utils::setup_logger();

    let mut stdin = SP1Stdin::new();

    // Create an input stream and write '42' to it
    let n = 42u64;
    // Communication with the forger
    stdin.write(&true);
    stdin.write(&n);

    // Generate and verify the proof
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove(&pk, stdin).compressed().run().unwrap();
    proof.public_values.write(&true);
    proof.public_values.write(&n);
    proof.stdin = SP1Stdin::new();
    proof.stdin.write(&n);

    client.verify(&proof, &vk).expect("verification failed");
    let is_prime = proof.public_values.read::<bool>();
    let prime = proof.public_values.read::<u64>();
    println!("Incontrovertible proof that {prime} is prime: {is_prime}!");

    // Test a round trip of proof serialization and deserialization.
    proof.save("proof-with-is-prime.bin").expect("saving proof failed");
    let deserialized_proof =
        SP1ProofWithPublicValues::load("proof-with-is-prime.bin").expect("loading proof failed");

    // Verify the deserialized proof.
    client.verify(&deserialized_proof, &vk).expect("verification failed");

    println!("successfully generated and verified proof for the program!")
}

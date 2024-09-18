use rand::Rng;
use rug::{rand::RandState, Complete, Integer};

use super::prime_generator;

pub struct DiffieHellmanParams {
    p: Integer,
    g: Integer,
}

impl DiffieHellmanParams {
    fn new(bits:u32) -> Self {
        let mut rng = rand::thread_rng();
        let seed: u64 = rng.gen();
        let mut state = RandState::new();
        state.seed(&Integer::from(seed));
        let p = prime_generator::generate_large_prime(bits, &mut state);
        let g = prime_generator::random_small_prime();
        DiffieHellmanParams {
            p,
            g
        }
    }
}

pub struct PrivateKey(Integer);


impl PrivateKey {
    fn generate(params: &DiffieHellmanParams) -> Self {
        let mut rng = RandState::new();
        let private_key_value = Integer::from(params.p.clone() - 1)
            .random_below_ref(&mut rng).complete();
        PrivateKey(private_key_value + 1)
    }
}


pub struct PublicKey(Integer);

impl PublicKey {
    fn generate(private_key: &PrivateKey, params: &DiffieHellmanParams) -> Self {
        let public_key_value = <Integer as Clone>::clone(&params.g).pow_mod(&private_key.0, &params.p).unwrap();
        PublicKey(public_key_value)
    }
}

pub struct SharedKey(Integer);

impl SharedKey {
    fn compute(private_key: &PrivateKey, other_public_key: &PublicKey, params: &DiffieHellmanParams) -> Self {
        let shared_key_value = <Integer as Clone>::clone(&other_public_key.0)
            .pow_mod(&private_key.0, &params.p)
            .unwrap();

        SharedKey(shared_key_value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diffie_hellman_shared_key_test() {

        let params = DiffieHellmanParams::new(2048);

        let private_key_1 = PrivateKey::generate(&params);
        let public_key_1 = PublicKey::generate(&private_key_1, &params);

        let private_key_2 = PrivateKey::generate(&params);
        let public_key_2 = PublicKey::generate(&private_key_2, &params);

        let shared_key_1 = SharedKey::compute(&private_key_1, &public_key_2, &params);
        let shared_key_2 = SharedKey::compute(&private_key_2, &public_key_1, &params);

        assert_eq!(shared_key_1.0, shared_key_2.0);
    }
}
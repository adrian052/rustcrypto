use rand::Rng;
use rug::{rand::RandState, Complete, Integer};

pub mod prime_generator;

struct RSAKeyPair {
    n: Integer,
    e: Integer,   
    d: Integer,   
}

impl RSAKeyPair {
    pub fn new(bits: u32) -> Self {
        let mut rng = rand::thread_rng();
        let seed: u64 = rng.gen();
        let mut state = RandState::new();
        state.seed(&Integer::from(seed));

        let p = prime_generator::generate_large_prime(bits / 2, &mut state);
        let q = prime_generator::generate_large_prime(bits / 2, &mut state);
    
        let n = (&p * &q).complete();
    
        let phi_n = (Integer::from(&p - 1)) * (Integer::from(&q - 1));
    
        let e = Integer::from(65537);
    
        let d = e.clone().invert(&phi_n).unwrap();
    
        RSAKeyPair { n, e, d }
    }


    pub fn encrypt(&self, message: &Integer) -> Integer {
        <Integer as Clone>::clone(&message).pow_mod(&self.e, &self.n).unwrap()
    }

    pub fn decrypt(&self, ciphertext: &Integer) -> Integer {
        <Integer as Clone>::clone(&ciphertext).pow_mod(&self.d, &self.n).unwrap()
    }
}


#[cfg(test)]
mod test {
    use rug::Integer;

    use crate::pubkey::rsa::RSAKeyPair;

    #[test]
    fn test(){
        let rsa = RSAKeyPair::new(2024);
        let message = Integer::from(1234567890);
        let ciphertext = rsa.encrypt(&message);
        let decrypted_message = rsa.decrypt(&ciphertext);
        assert_eq!(message, decrypted_message);
    }
}
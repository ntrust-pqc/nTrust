use crate::poly::TernaryArith;
use crate::poly::TernaryPoly;
use rand::RngCore;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
impl TernaryArith for TernaryPoly {
    fn new() -> Self {
        TernaryPoly {
            degree: 0,
            coefficient: vec![0; 0],
        }
    }

    fn zero(degree: usize) -> Self {
        TernaryPoly {
            degree: degree,
            coefficient: vec![0; degree],
        }
    }

    // generate a random binary polynomial
    fn rand(&mut self, seed: [u8; 32]) {}

    fn hamming(self) -> u16 {
        let mut counter: u16 = 0;
        for i in 0..self.degree {
            if self.coefficient[i] != 0 {
                counter = counter + 1;
            }
        }
        counter
    }
}

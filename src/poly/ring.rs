use crate::poly::RingArith;
use crate::poly::RingElement;
use crate::poly::TernaryPoly;
use rand::RngCore;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

impl RingArith for RingElement {
    fn new() -> Self {
        RingElement {
            degree: 0,
            coefficient: vec![],
            modulus: 0,
        }
    }

    fn zero(degree: usize) -> Self {
        RingElement {
            degree: degree,
            coefficient: vec![0; degree],
            modulus: 0,
        }
    }

    fn rand(&mut self, seed: [u8; 32], modulus: u16) {
        let mut prng = ChaChaRng::from_seed(seed);
        self.modulus = modulus;
        let mut counter = 0;
        let mut rngcounter = 0;
        let mut t0 = prng.next_u64();
        while counter < self.degree {
            self.coefficient[counter] = (t0 as u16) % modulus;
            t0 = t0 >> 16;
            rngcounter = rngcounter + 1;
            if rngcounter == 4 {
                t0 = prng.next_u64();
                rngcounter = 0;
            }
            counter = counter + 1;
        }

        /*        for i in 0..(self.degree >> 2) {
            let mut t0 = prng.next_u64();
            self.coefficient[4 * i] = (t0 & 0xFF) as u8 % modulus;
            t0 = t0 >> 8;
            self.coefficient[4 * i + 1] = (t0 & 0xFF) as u8 % modulus;
            t0 = t0 >> 8;
            self.coefficient[4 * i + 2] = (t0 & 0xFF) as u8 % modulus;
            t0 = t0 >> 8;
            self.coefficient[4 * i + 3] = (t0 & 0xFF) as u8 % modulus;
        }*/
    }
    /*
    fn mul_by_ter(&mut self, other: TernaryPoly) {
        // multiplication algorithm
        let mut base: Vec<i16> = self.coefficient.iter().map(|&e| e as i16).collect();
        let mut res: Vec<i16> = vec![0; self.degree];
        for i in 0..other.degree {
            if other.coefficient[i] != 0 {
                if other.coefficient[i] == 1 {
                    for j in 0..self.degree {
                        res[j] = res[j] + base[j]
                    }
                } else if other.coefficient[i] == -1 {
                    for j in 0..self.degree {
                        res[j] = res[j] - base[j]
                    }
                }
            }
            // cyclic rotate the base
            let t = base.pop().unwrap();
            base.insert(0, t);
        }
        // convert results into Vec<u8>
        for i in 0..self.degree {
            res[i] = res[i] % (self.modulus as i16);
            if res[i] < 0 {
                res[i] = res[i] + (self.modulus as i16);
            }
            self.coefficient[i] = res[i] as u8;
        }
    }

    fn add_by_ter(&mut self, other: TernaryPoly) {
        // add the ternary
        assert!(self.degree == other.degree, "incaptible degree");
        let degree = self.degree;
        for i in 0..degree {
            self.coefficient[i] =
                ((self.coefficient[i] as i16 + other.coefficient[i] as i16) as u8) % self.modulus;
        }
    }
    fn add_by_ring_element(&mut self, other: RingElement) {
        assert!(self.degree == other.degree, "incaptible degree");
        assert!(self.modulus == other.modulus, "incaptible modulus");
        let degree = self.degree;
        for i in 0..degree {
            self.coefficient[i] = ((self.coefficient[i] as u16 + other.coefficient[i] as u16)
                % self.modulus as u16) as u8;
        }
    }*/
}

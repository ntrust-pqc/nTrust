use crate::ipoly::SignedPolynomial;
use crate::param::Param;
use rand::RngCore;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

use std::fmt;

#[derive(Clone)]
pub struct UnsignedPolynomial {
    pub coefficient: Vec<u16>,
    pub degree: usize,
    pub modulus: u16,
}

impl UnsignedPolynomial {
    pub fn init() -> Self {
        UnsignedPolynomial {
            degree: 0,
            coefficient: vec![],
            modulus: 0,
        }
    }
    pub fn zero(degree: usize) -> Self {
        UnsignedPolynomial {
            degree: degree,
            coefficient: vec![0; degree],
            modulus: 0,
        }
    }

    pub fn rand(&mut self, p: Param, prng: &mut ChaChaRng) {
        self.modulus = p.get_q();
        self.degree = p.get_param_n();
        self.coefficient = vec![0; self.degree];
        let q_minus_one = self.modulus - 1;
        let mut counter = 0;
        let mut rngcounter = 0;
        let mut t0 = prng.next_u64();
        while counter < self.degree {
            self.coefficient[counter] = (t0 as u16) & q_minus_one;
            t0 = t0 >> 16;
            rngcounter = rngcounter + 1;
            if rngcounter == 4 {
                t0 = prng.next_u64();
                rngcounter = 0;
            }
            counter = counter + 1;
        }
    }

    pub fn sample_t(&mut self, p: Param, seed: [u8; 32], domain: String) {
        let mut t = SignedPolynomial::zero(p.get_param_n());
        t.sample_t(p, seed, domain);
        self.from_signed(t);
    }

    pub fn sample_t_plus(&mut self, p: Param, seed: [u8; 32], domain: String) {
        let mut t = SignedPolynomial::zero(p.get_param_n());
        t.sample_t_plus(p, seed, domain);
        self.from_signed(t);
    }

    pub fn into_signed(&mut self) -> SignedPolynomial {
        let mut s = SignedPolynomial::zero(self.degree);
        s.modulus = self.modulus;
        for i in 0..self.degree {
            s.coefficient[i] = lift(self.coefficient[i], self.modulus);
        }
        s
    }

    pub fn from_signed(&mut self, s: SignedPolynomial) {
        self.degree = s.degree;
        self.modulus = s.modulus;
        self.coefficient = Vec::new();
        for i in 0..self.degree {
            self.coefficient.push(down(s.coefficient[i], self.modulus));
        }
    }

    pub fn is_trinary(&mut self) -> bool {
        self.into_signed().is_trinary()
    }

    // returns # non-zero coefficients
    pub fn hamming(&mut self) -> usize {
        let mut hm = self.degree;
        for i in 0..self.degree {
            if self.coefficient[i] == 0 {
                hm -= 1;
            }
        }
        hm
    }

    // returns the sum of all coeffcients
    pub fn norm_one(&mut self) -> i16 {
        self.into_signed().norm_one()
    }
}

impl fmt::Debug for UnsignedPolynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "================================\n\
             ==========Polynomial============\n\
             n : {}\n\
             q : {}\n\
             coeff: {:?}\n\
             ================================\n",
            self.degree, self.modulus, self.coefficient
        )
    }
}

//#[inline]
fn lift(a: u16, q: u16) -> i16 {
    if a > (q >> 1) {
        (a as i16) - (q as i16)
    } else {
        a as i16
    }
}

fn down(a: i16, q: u16) -> u16 {
    if a < 0 {
        (a + (q as i16)) as u16
    } else {
        a as u16
    }
}

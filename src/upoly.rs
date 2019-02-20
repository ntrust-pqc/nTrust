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

    pub fn sample_t(p: Param, seed: [u8; 32], domain: String) -> Self {
        let t = SignedPolynomial::sample_t(p, seed, domain);
        Self::from_signed(t)
    }

    pub fn sample_t_plus(p: Param, seed: [u8; 32], domain: String) -> Self {
        let t = SignedPolynomial::sample_t_plus(p, seed, domain);
        Self::from_signed(t)
    }

    pub fn into_signed(&mut self) -> SignedPolynomial {
        let mut s = SignedPolynomial::zero(self.degree);
        s.modulus = self.modulus;
        for i in 0..self.degree {
            s.coefficient[i] = lift(self.coefficient[i], self.modulus);
        }
        s
    }

    pub fn from_signed(s: SignedPolynomial) -> Self {
        let mut t = UnsignedPolynomial::zero(s.degree);
        //        self.degree = s.degree;
        t.modulus = s.modulus;
        t.coefficient = Vec::new();
        for i in 0..t.degree {
            t.coefficient.push(down(s.coefficient[i], t.modulus));
        }
        t
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

impl PartialEq for UnsignedPolynomial {
    fn eq(&self, other: &UnsignedPolynomial) -> bool {
        if self.degree != other.degree {
            return false;
        } else if self.modulus != other.modulus {
            return false;
        } else if self.coefficient.len() != other.coefficient.len() {
            return false;
        }
        for i in 0..self.degree {
            if self.coefficient[i] != other.coefficient[i] {
                return false;
            }
        }
        true
    }
}

#[test]
fn test_conversion_upoly_ipoly() {
    for _ in 0..100 {
        let mut a = UnsignedPolynomial::zero(50);
        let b = UnsignedPolynomial::from_signed(a.into_signed());
        assert_eq!(a, b, "conversion between upoly and ipoly failed");
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

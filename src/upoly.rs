use crate::ipoly::{SignedPolyArith, SignedPolynomial};
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

pub trait UnsignedPolyArith {
    fn init() -> Self;

    fn zero(degree: usize) -> Self;

    fn rand(&mut self, p: Param, prng: &mut ChaChaRng);

    fn sample_t(p: Param, seed: [u8; 32], domain: String) -> Self;

    fn sample_t_plus(p: Param, seed: [u8; 32], domain: String) -> Self;

    fn into_signed(&self) -> SignedPolynomial;

    fn from_signed(s: SignedPolynomial) -> Self;

    fn is_trinary(&self) -> bool;

    // returns # non-zero coefficients
    fn hamming(&self) -> usize;

    // returns the sum of all coeffcients
    fn norm_one(&self) -> i16;
}

impl UnsignedPolyArith for UnsignedPolynomial {
    fn init() -> Self {
        UnsignedPolynomial {
            degree: 0,
            coefficient: vec![],
            modulus: 0,
        }
    }
    fn zero(degree: usize) -> Self {
        UnsignedPolynomial {
            degree: degree,
            coefficient: vec![0; degree],
            modulus: 0,
        }
    }

    fn rand(&mut self, p: Param, prng: &mut ChaChaRng) {
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

    fn sample_t(p: Param, seed: [u8; 32], domain: String) -> Self {
        let t = SignedPolynomial::sample_t(p, seed, domain);
        Self::from_signed(t)
    }

    fn sample_t_plus(p: Param, seed: [u8; 32], domain: String) -> Self {
        let t = SignedPolynomial::sample_t_plus(p, seed, domain);
        Self::from_signed(t)
    }

    fn into_signed(&self) -> SignedPolynomial {
        let mut s = SignedPolynomial::zero(self.degree);
        s.modulus = self.modulus;
        for i in 0..self.degree {
            s.coefficient[i] = lift(self.coefficient[i], self.modulus);
        }
        s
    }

    fn from_signed(s: SignedPolynomial) -> Self {
        let mut t = UnsignedPolynomial::zero(s.degree);
        t.modulus = s.modulus;
        t.coefficient = Vec::new();
        for i in 0..t.degree {
            t.coefficient.push(down(s.coefficient[i], t.modulus));
        }
        t
    }

    fn is_trinary(&self) -> bool {
        self.into_signed().is_trinary()
    }

    // returns # non-zero coefficients
    fn hamming(&self) -> usize {
        let mut hm = self.degree;
        for i in 0..self.degree {
            if self.coefficient[i] == 0 {
                hm -= 1;
            }
        }
        hm
    }

    // returns the sum of all coeffcients
    fn norm_one(&self) -> i16 {
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
        let a = UnsignedPolynomial::zero(50);
        let b = UnsignedPolynomial::from_signed(a.into_signed());
        assert_eq!(a, b, "conversion between upoly and ipoly failed");
    }
}

#[test]
fn test_sample_t_unsigned() {
    let p: Param = Param::init();
    let domain = "test".to_string();
    for i in 0..100 {
        let seed = [i as u8; 32];
        let f = UnsignedPolynomial::sample_t(p.clone(), seed, domain.clone());
        assert!(f.is_trinary(), "f is not trinary");
        assert!(
            f.coefficient[f.degree - 1] == 0,
            "leading coefficient isn't 0"
        );
        assert!(
            f.coefficient[f.degree - 2] == 0,
            "second leading coefficient isn't 0"
        );
        assert!(
            f.hamming() < f.degree * 3 / 4 && f.hamming() > f.degree / 4,
            "hamming weight seems incorrect"
        );
        assert!(
            f.norm_one() < ((f.degree / 4) as i16) && f.norm_one() > -((f.degree / 4) as i16),
            "norm seems incorrect"
        );
    }
}

#[test]
fn test_sample_t_plus_unsigned() {
    let p: Param = Param::init();
    let domain = "test".to_string();
    for i in 0..100 {
        let seed = [i as u8; 32];
        let f = UnsignedPolynomial::sample_t_plus(p.clone(), seed, domain.clone());
        assert!(f.is_trinary(), "f is not trinary");
        assert!(
            f.coefficient[f.degree - 1] == 0,
            "leading coefficient isn't 0"
        );
        assert!(
            f.coefficient[f.degree - 2] == 0,
            "second leading coefficient isn't 0"
        );
        assert!(
            f.hamming() < f.degree * 3 / 4 && f.hamming() > f.degree / 4,
            "hamming weight seems incorrect"
        );
        assert!(
            f.norm_one() < ((f.degree / 4) as i16) && f.norm_one() > -((f.degree / 4) as i16),
            "norm seems incorrect"
        );
        assert!(f.into_signed().get_t() >= 0, "invalid t value");
    }
}

//#[inline]
pub fn lift(a: u16, q: u16) -> i16 {
    if a > (q >> 1) {
        (a as i16) - (q as i16)
    } else {
        a as i16
    }
}

pub fn down(a: i16, q: u16) -> u16 {
    if a < 0 {
        (a + (q as i16)) as u16
    } else {
        a as u16
    }
}

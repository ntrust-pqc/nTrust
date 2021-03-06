use crate::param::Param;

use crate::upoly::{lift,down, UnsignedPolynomial, UnsignedPolyArith};
use rand::RngCore;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

use std::fmt;

#[derive(Clone)]
pub struct SignedPolynomial {
    pub coefficient: Vec<i16>,
    pub degree: usize,
    pub modulus: u16,
}

pub trait SignedPolyArith {
    fn init() -> Self;

    fn zero(degree: usize) -> Self;

    fn is_trinary(&self) -> bool;

    fn into_unsigned(&self) -> UnsignedPolynomial;

    fn from_unsigned(s: UnsignedPolynomial) -> Self;

    fn sample_t(p: Param, seed: [u8; 32], _domain: String) -> Self;

    fn sample_t_plus(p: Param, seed: [u8; 32], domain: String) -> Self;

    fn get_t(&self) -> i16;

    // returns # non-zero coefficients
    fn hamming(&self) -> usize;

    // returns the sum of all coeffcients
    fn norm_one(&self) -> i16;

    fn s3_to_zx(a: Self) -> Self;

    fn mod_s3(a: Self) -> Self;
}

impl SignedPolyArith for SignedPolynomial {
    fn init() -> Self {
        SignedPolynomial {
            degree: 0,
            coefficient: vec![],
            modulus: 0,
        }
    }
    fn zero(degree: usize) -> Self {
        SignedPolynomial {
            degree: degree,
            coefficient: vec![0; degree],
            modulus: 0,
        }
    }

    fn is_trinary(&self) -> bool {
        let mut result = true;
        for i in 0..self.degree {
            if self.coefficient[i] == -1 || self.coefficient[i] == 0 || self.coefficient[i] == 1 {

            } else {
                result = false;
            }
        }
        result
    }

    fn into_unsigned(&self) -> UnsignedPolynomial {
        let mut s = UnsignedPolynomial::zero(self.degree);
        s.modulus = self.modulus;
        for i in 0..self.degree {
            s.coefficient[i] = down(self.coefficient[i], self.modulus);
        }
        s
    }

    fn from_unsigned(s: UnsignedPolynomial) -> Self {
        let mut t = SignedPolynomial::zero(s.degree);
        t.modulus = s.modulus;
        t.coefficient = Vec::new();
        for i in 0..t.degree {
            t.coefficient.push(lift(s.coefficient[i], t.modulus));
        }
        t
    }

    fn sample_t(p: Param, seed: [u8; 32], _domain: String) -> Self {
        let mut res = Self::init();
        res.modulus = p.get_q();
        res.degree = p.get_param_n();
        res.coefficient = vec![0; res.degree];

        // todo: implement domain seperation
        let mut prng = ChaChaRng::from_seed(seed);
        let degree = res.degree - 2;
        let round = degree / 32;
        for i in 0..round {
            let mut t = prng.next_u64();
            for j in 0..32 {
                let b1 = (t & 1) as i16;
                t >>= 1;

                res.coefficient[i * 32 + j] = ((t & 1) as i16) - b1;
                t >>= 1;
            }
        }
        let mut t = prng.next_u64();
        for i in round * 32..degree {
            let b1 = (t & 1) as i16;
            t >>= 1;
            res.coefficient[i] = ((t & 1) as i16) - b1;
            t >>= 1;
        }
        res
    }

    fn sample_t_plus(p: Param, seed: [u8; 32], domain: String) -> Self {
        let mut res = Self::sample_t(p, seed, domain);
        let s = if res.get_t() < 0 { -1 } else { 1 };
        for i in 0..(res.degree >> 1) {
            res.coefficient[i << 1] *= s;
        }
        res
    }

    fn get_t(&self) -> i16 {
        let mut t = 0i16;
        for i in 0..self.degree - 2 {
            t += self.coefficient[i] * self.coefficient[i + 1];
        }
        t
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
        let mut norm = 0i16;

        for i in 0..self.degree {
            norm += self.coefficient[i];
        }
        norm
    }

    fn s3_to_zx(a: Self) -> Self {
        Self::mod_s3(a)
    }

    fn mod_s3(a: Self) -> Self {
        assert!(a.is_trinary(), "not a trinary poly");
        a
    }
}

impl fmt::Debug for SignedPolynomial {
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

impl PartialEq for SignedPolynomial {
    fn eq(&self, other: &SignedPolynomial) -> bool {
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
fn test_sample_t_signed() {
    let p: Param = Param::init();
    let domain = "test".to_string();
    for i in 0..100 {
        let seed = [i as u8; 32];
        let f = SignedPolynomial::sample_t(p.clone(), seed, domain.clone());
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
fn test_sample_t_plus_signed() {
    let p: Param = Param::init();
    let domain = "test".to_string();
    for i in 0..100 {
        let seed = [i as u8; 32];
        let f = SignedPolynomial::sample_t_plus(p.clone(), seed, domain.clone());
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
        assert!(f.get_t() >= 0, "invalid t value");
    }
}

#[test]
fn test_conversion_ipoly_upoly() {
    for _ in 0..100 {
        let a = SignedPolynomial::zero(50);
        let b = SignedPolynomial::from_unsigned(a.into_unsigned());
        assert_eq!(a, b, "conversion between upoly and ipoly failed");
    }
}

extern crate rand;
extern crate rand_chacha;
mod poly;

use crate::poly::RingArith;
use crate::poly::RingElement;
use crate::poly::TernaryArith;
use crate::poly::TernaryPoly;

fn main() {
    let seed = [0u8; 32];
    let modulus = 1 << 13;
    let mut a: RingElement = RingArith::zero(701);

    a.rand(seed, modulus);

    println!("{:?}", a);
    println!("Hello, world!");
}

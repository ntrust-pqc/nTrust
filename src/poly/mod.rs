mod ring;
mod ternary;
mod test;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RingElement {
    coefficient: Vec<u16>,
    degree: usize,
    modulus: u16,
}

pub trait RingArith: Clone + PartialEq + Eq {
    fn new() -> Self;
    fn zero(degree: usize) -> Self;
    fn rand(&mut self, seed: [u8; 32], modulus: u16);
    //    fn mul_by_ter(&mut self, other: TernaryPoly);
    //    fn add_by_ter(&mut self, other: TernaryPoly);
    //    fn add_by_ring_element(&mut self, other: RingElement);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TernaryPoly {
    coefficient: Vec<i8>,
    degree: usize,
}

pub trait TernaryArith: Clone + PartialEq + Eq {
    fn new() -> Self;
    fn zero(degree: usize) -> Self;
    fn rand(&mut self, seed: [u8; 32]);
    fn hamming(self) -> u16;
}

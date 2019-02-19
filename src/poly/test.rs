#[test]
fn test_psi_dist() {
    use crate::poly::TernaryArith;
    use crate::poly::TernaryPoly;
    let seed: [u8; 32] = [84; 32];
    let mut t: TernaryPoly = TernaryArith::zero(512);
    t.rand(seed, 1);
}

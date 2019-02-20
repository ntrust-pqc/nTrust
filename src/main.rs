extern crate rand;
extern crate rand_chacha;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

mod ipoly;
mod param;
mod upoly;
use ipoly::SignedPolynomial;
use param::Param;
use upoly::UnsignedPolynomial;
fn main() {
    let seed = [4u8; 32];
    let domain = "fdom".to_string();
    let _modulus = 1 << 13;
    let mut prng = ChaChaRng::from_seed(seed);
    let p: Param = Param::init();
    println!("{:?}", p);
    println!("{:?}", p.get_q());
    println!("{:?}", p.get_log_q());
    println!("{:?}", p.get_owcpa_ciphertext_bits());
    println!("{:?}", p.get_owcpa_public_key_bits());
    println!("{:?}", p.get_owcpa_secret_key_bits());

    let f = UnsignedPolynomial::init();
    println!("{:?}", f);
    let mut f = UnsignedPolynomial::zero(p.get_param_n());
    println!("{:?}", f);
    f.rand(p.clone(), &mut prng);
    println!("{:?}", f);

    //    let mut f = SignedPolynomial::zero(p.get_param_n());
    f.sample_t_plus(p, seed, domain);
    println!("{:?}", f);
    println!("Hello, world!");
}

#![warn(dead_code)]

use std::fmt;

#[derive(Clone)]
pub struct Param {
    pub param_n: usize,
    param_k: usize,
    param_seed_bits: usize,
    param_coin_bits: usize,
    param_shared_key_bits: usize,
}

impl Param {
    pub fn init() -> Self {
        Param {
            param_n: 701,
            param_k: 2,
            param_seed_bits: 256,
            param_coin_bits: 256,
            param_shared_key_bits: 256,
        }
    }
    pub fn get_param_n(&self) -> usize {
        self.param_n
    }

    pub fn get_param_k(&self) -> usize {
        self.param_k
    }

    pub fn get_param_seed_bits(&self) -> usize {
        self.param_seed_bits
    }

    pub fn get_param_coin_bits(&self) -> usize {
        self.param_seed_bits
    }

    pub fn get_param_shared_key_bits(&self) -> usize {
        self.param_seed_bits
    }

    pub fn get_log_q(&self) -> u16 {
        assert_eq!(self.param_n, 701, "other parameters not yet supported");
        // ceil (7/2+ log_2(n) )
        13
    }
    pub fn get_q(&self) -> u16 {
        if self.param_n == 701 {
            // 8192
            0x2000
        } else {
            1 << self.get_log_q()
        }
    }
    pub fn get_s3_packed_bits(&self) -> u16 {
        if self.param_n == 701 {
            1120
        } else {
            // 8 * ceil ((n-1)/5)
            ((self.param_n as u16 + 3) / 5) << 3
        }
    }
    pub fn get_owcpa_public_key_bits(&self) -> u16 {
        if self.param_n == 701 {
            9100
        } else {
            // (n-1) * log_2(q)
            (self.param_n as u16 - 1) * self.get_log_q()
        }
    }
    pub fn get_owcpa_secret_key_bits(&self) -> u16 {
        if self.param_n == 701 {
            2240
        } else {
            // 2 * s3_packed_bits
            2 * self.get_s3_packed_bits()
        }
    }
    pub fn get_owcpa_ciphertext_bits(&self) -> u16 {
        if self.param_n == 701 {
            9100
        } else {
            // 2 * s3_packed_bits
            self.get_owcpa_public_key_bits()
        }
    }
}

impl fmt::Debug for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "================================\n\
             ==========Paramter set==========\n\
             n : {}\n\
             q : {}\n\
             ================================\n",
            self.param_n,
            self.get_q()
        )
    }
}

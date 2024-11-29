use super::data::{D, S0, S1};
use super::bit_31::{add_mod_31,rotate_left_31};
use std::u32;
struct SBox {}
impl SBox {
    fn get(x: u32) -> u32 {
        let x0 = S0[(x >> 24) as usize];
        let x1 = S1[((x >> 16) & 0xFF) as usize];
        let x2 = S0[((x >> 8) & 0xFF) as usize];
        let x3 = S1[(x & 0xFF) as usize];
        ((x0 as u32) << 24) | ((x1 as u32) << 16) | ((x2 as u32) << 8) | (x3 as u32)
    }
}
enum Bit {
    High,
    Low,
}
fn l1(x: u32) -> u32 {
    x ^ x.rotate_left(2) ^ x.rotate_left(10) ^ x.rotate_left(18) ^ x.rotate_left(24)
}
fn l2(x: u32) -> u32 {
    x ^ x.rotate_left(8) ^ x.rotate_left(14) ^ x.rotate_left(22) ^ x.rotate_left(30)
}
fn padding_input(input:&[u8])->Vec<u8> {
    let mut input = input.to_vec();
    const BLOCK_SIZE: usize = 4;
    let p_size = BLOCK_SIZE - (input.len() % BLOCK_SIZE);
    input.extend(vec![p_size as u8; p_size]);
    input 
}
pub struct ZUC {
    state: [u32; 16],
    offset: usize,
    r1: u32,
    r2: u32,
}
impl ZUC {
    pub fn new(key: &[u8; 16], iv: &[u8; 16]) -> Self {
        let offset = 0;
        let mut state: [u32; 16] = [0; 16];
        (0..16).for_each(|i| state[i] = ((key[i] as u32) << 23) | (D[i] << 8) | (iv[i] as u32));
        ZUC {
            state,
            offset,
            r1: 0,
            r2: 0,
        }
    }

    //32bit
    fn f(&mut self, x0: u32, x1: u32, x2: u32) -> u32 {
        let w = (x0 ^ self.r1).wrapping_add(self.r2);
        let w1 = self.r1.wrapping_add(x1);
        let w2 = self.r2 ^ x2;
        let l1 = l1((w1 << 16) | (w2 >> 16));
        let l2 = l2((w2 << 16) | (w1 >> 16));
        self.r1 = SBox::get(l1);
        self.r2 = SBox::get(l2);
        w
    }
    fn transform(&mut self) -> u32 {
        let mut v = self.state[self.index(0)];
        v = add_mod_31(v, rotate_left_31(self.state[self.index(0)], 8));
        v = add_mod_31(v, rotate_left_31(self.state[self.index(4)], 20));
        v = add_mod_31(v, rotate_left_31(self.state[self.index(10)], 21));
        v = add_mod_31(v, rotate_left_31(self.state[self.index(13)], 17));
        v = add_mod_31(v, rotate_left_31(self.state[self.index(15)], 15));
        v
    }
    fn init_mode(&mut self, u: u32) {
        let v = self.transform();
        let mut s_16 = add_mod_31(v, u);
        if s_16 == 0 {
            s_16 = 0x7FFFFFFF;
        }
        self.state[self.index(0)] = s_16;
        self.offset = (self.offset + 1) % 16
    }
    fn work_mode(&mut self) {
        let mut s_16 = self.transform();
        if s_16 == 0 {
            s_16 = 0x7FFFFFFF;
        }
        self.state[self.index(0)] = s_16;
        self.offset = (self.offset + 1) % 16
    }
    fn index(&self, idx: usize) -> usize {
        (idx + self.offset) % 16
    }
    fn bit_from_state(&self, i: usize, bit: Bit) -> u32 {
        match bit {
            Bit::High => self.state[self.index(i)] >> 15,
            Bit::Low => self.state[self.index(i)] & 0xFFFF,
        }
    }
    //32bit
    fn bit_reconstruction(&mut self) -> (u32, u32, u32, u32) {
        let x0 = (self.bit_from_state(15, Bit::High) << 16) | self.bit_from_state(14, Bit::Low);
        let x1 = (self.bit_from_state(11, Bit::Low) << 16) | self.bit_from_state(9, Bit::High);
        let x2 = (self.bit_from_state(7, Bit::Low) << 16) | self.bit_from_state(5, Bit::High);
        let x3 = (self.bit_from_state(2, Bit::Low) << 16) | self.bit_from_state(0, Bit::High);
        (x0, x1, x2, x3)
    }
    pub fn encrypt(&mut self, input: String) -> Vec<u8> {
        let bytes = input.as_bytes();
        let bytes = padding_input(&bytes);
        bytes
            .chunks_exact(4)
            .map(|chunk| u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .zip(self)
            .map(|(block, key)| block ^ key)
            .flat_map(|num| num.to_be_bytes())
            .collect()
    }
    // fn decrypt(&mut self, input: String) -> String {
    //     "".to_string()
    // }
    pub fn init(&mut self) {
        for _ in 0..32 {
            let (x0, x1, x2, _) = self.bit_reconstruction();
            let w = self.f(x0, x1, x2);
            self.init_mode(w >> 1);
        }
        let (x0, x1, x2, _) = self.bit_reconstruction();
        let _ = self.f(x0, x1, x2);
        let _ = self.work_mode();
    }
}
impl Iterator for ZUC {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        // 32bit
        let (x0, x1, x2, x3) = self.bit_reconstruction();
        let res = self.f(x0, x1, x2) ^ x3;
        let _ = self.work_mode();
        Some(res)
    }
}

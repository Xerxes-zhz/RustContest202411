pub fn add_mod_31(x: u32, y: u32) -> u32 {
    let c = x + y;
    (c & 0x7FFFFFFF) + (c >> 31)
}
pub fn rotate_left_31(x: u32, n: u32) -> u32 {
    (((x << n) & 0x7FFFFFFF) | (x >> (31 - n))) & 0x7FFFFFFF
}

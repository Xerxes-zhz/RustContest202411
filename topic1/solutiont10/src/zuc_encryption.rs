mod data;
use base64::{engine::general_purpose::STANDARD, Engine as _};
mod bit_31;
mod zuc;
use data::{IV, KEY};
use zuc::ZUC;
pub fn encryption(input: String) -> String {
    let mut zuc = ZUC::new(&KEY, &IV);
    zuc.init();
    let s = zuc.encrypt(input);
    let encoded = STANDARD.encode(s);
    encoded
}

// #[cfg(test)]
// mod tests {

//     use crate::zuc_encryption as m;
//     #[test]
//     fn test_mod() {
//         assert_eq!(
//             m::bit_31::rotate_left_31(0b1011111_11111111_11111111_11111111, 5),
//             0b1111111_11111111_11111111_11110111
//         );
//         assert_eq!(m::bit_31::add_mod_31(0x7FFFFFFF, 4), 4);
//     }
//     #[test]
//     fn test_zuc_1() {
//         let key = [0x0; 16];
//         let iv = [0x0; 16];
//         let mut zuc = m::ZUC::new(&key, &iv);
//         zuc.init();
//         assert_eq!(zuc.next().unwrap(), 0x27bede74);
//         assert_eq!(zuc.next().unwrap(), 0x018082da);
//     }
//     #[test]
//     fn test_zuc_2() {
//         let key = [
//             0x3d, 0x4c, 0x4b, 0xe9, 0x6a, 0x82, 0xfd, 0xae, 0xb5, 0x8f, 0x64, 0x1d, 0xb1, 0x7b,
//             0x45, 0x5b,
//         ];
//         let iv = [
//             0x84, 0x31, 0x9a, 0xa8, 0xde, 0x69, 0x15, 0xca, 0x1f, 0x6b, 0xda, 0x6b, 0xfb, 0xd8,
//             0xc7, 0x66,
//         ];
//         let mut zuc = m::ZUC::new(&key, &iv);
//         zuc.init();
//         assert_eq!(zuc.next().unwrap(), 0x14f1c272);
//         assert_eq!(zuc.next().unwrap(), 0x3279c419);
//     }
// }

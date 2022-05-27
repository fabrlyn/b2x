use logic::common::BinToDecExt;

mod binary;
mod bit_size;
mod from_binary;
pub mod logic;

pub fn test() {
    let a = "10101".bin_to_dec().u8().spaced().big_endian();
}

use alloy_primitives::hex;

use super::*;

mod keccak256;
// mod sha256;
// mod ecrecover;

fn pretty_print_memory_dump(content: &[[u8; 32]], range: std::ops::Range<u32>) {
    println!("Memory dump:");
    println!("-----------------------------------------");
    for (cont, index) in content.iter().zip(range.into_iter()) {
        println!("{:04x}: 0x{}", index, hex::encode(cont));
    }
    println!("-----------------------------------------");
}

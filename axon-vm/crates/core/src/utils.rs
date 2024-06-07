use alloy_primitives::U256;
use once_cell::sync::Lazy;
use zkvm_opcodes::decoding::VmEncodingMode;
pub use zkvm_opcodes::utils::*;
use zkvm_primitives::vm::Memory;

use crate::tracing::*;

pub static U256_TO_ADDRESS_MASK: Lazy<U256> = Lazy::new(|| U256::MAX >> (256 - 160));

pub fn contract_bytecode_to_words(code: &[[u8; 32]]) -> Vec<U256> {
    code.iter().map(|el| U256::from_be_slice(el)).collect()

    // for code each 8 byte sequence is somehow encoded integer,
    // or full 32 byte word is an integer constant (also encoded with some endianess)

    // let mut result = Vec::with_capacity(code.len());
    // let mut el = U256::ZERO;
    // for code_word in code.into_iter() {
    //     // each 8 byte sequence is an independent LE encoded u64,
    //     // but machine itself is BE regarding memory queries
    //     el.0[0] = u64::from_le_bytes(code_word[0..8].try_into().unwrap());
    //     el.0[1] = u64::from_le_bytes(code_word[8..16].try_into().unwrap());
    //     el.0[2] = u64::from_le_bytes(code_word[16..24].try_into().unwrap());
    //     el.0[3] = u64::from_le_bytes(code_word[24..32].try_into().unwrap());

    //     result.push(el);
    // }

    // result
}

pub fn address_to_u256(address: &crate::Address) -> U256 {
    let mut buffer = [0u8; 32];
    buffer[12..].copy_from_slice(address.as_slice());
    U256::from_be_bytes(buffer)
}

pub fn u256_to_address_unchecked(integer: &U256) -> crate::Address {
    let buffer: [u8; 32] = integer.to_be_bytes();

    crate::Address::from_slice(&buffer[12..32])
}

#[derive(Debug, Clone, Copy)]
pub struct GenericNoopTracer<M: Memory> {
    _marker: std::marker::PhantomData<M>,
}

impl<M: Memory> Default for GenericNoopTracer<M> {
    fn default() -> Self {
        Self::new()
    }
}
impl<M: Memory> GenericNoopTracer<M> {
    pub fn new() -> Self {
        Self { _marker: std::marker::PhantomData }
    }
}

impl<M: Memory, const N: usize, E: VmEncodingMode<N>> Tracer<N, E> for GenericNoopTracer<M> {
    type SupportedMemory = M;
    fn before_decoding(
        &mut self,
        _state: VmLocalStateData<'_, N, E>,
        _memory: &Self::SupportedMemory,
    ) {
    }
    fn after_decoding(
        &mut self,
        _state: VmLocalStateData<'_, N, E>,
        _data: AfterDecodingData<N, E>,
        _memory: &Self::SupportedMemory,
    ) {
    }
    fn before_execution(
        &mut self,
        _state: VmLocalStateData<'_, N, E>,
        _data: BeforeExecutionData<N, E>,
        _memory: &Self::SupportedMemory,
    ) {
    }
    fn after_execution(
        &mut self,
        _state: VmLocalStateData<'_, N, E>,
        _data: AfterExecutionData<N, E>,
        _memory: &Self::SupportedMemory,
    ) {
    }
}

pub fn low_u128_of_u256(value: &U256) -> u128 {
    let value_limbs = value.as_limbs();
    ((value_limbs[1] as u128) << 64) + value_limbs[0] as u128
}

pub fn low_u32_of_u256(value: &U256) -> u32 {
    value.as_limbs()[0] as u32
}

pub fn low_u64_of_u256(value: &U256) -> u64 {
    value.as_limbs()[0]
}

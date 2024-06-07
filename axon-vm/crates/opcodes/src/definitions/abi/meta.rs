use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VmMetaParameters {
    pub ergs_per_pubdata_byte: u32,
    pub heap_size: u32,
    pub aux_heap_size: u32,
    pub this_shard_id: u8,
    pub caller_shard_id: u8,
    pub code_shard_id: u8,
}

impl VmMetaParameters {
    pub const fn to_u256(self) -> U256 {
        let mut result_limbs = U256::ZERO.into_limbs();

        result_limbs[0] = self.ergs_per_pubdata_byte as u64;
        result_limbs[1] = (self.heap_size as u64) | ((self.aux_heap_size as u64) << 32);

        let tmp = (self.this_shard_id as u64)
            | ((self.caller_shard_id as u64) << 8)
            | ((self.code_shard_id as u64) << 16);
        result_limbs[3] = tmp << 32;

        U256::from_limbs(result_limbs)
    }
}

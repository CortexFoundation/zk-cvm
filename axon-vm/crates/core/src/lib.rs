pub mod block_properties;
pub mod errors;
pub mod flags;
pub mod opcodes;
pub mod reference_impls;
pub mod testing;
pub mod tracing;
pub mod utils;
pub mod vm_state;
pub mod witness_trace;

use alloy_primitives::{Address, U256};
pub use blake2;
pub use k256;
pub use sha2;
pub use sha3;
pub use zkvm_opcodes;
pub use zkvm_primitives;

pub use self::utils::*;

// Re-export primitives.
pub mod primitives {
    pub use zkvm_primitives::vm::*;
}
pub mod aux_structures {
    pub use zkvm_primitives::{aux::*, queries::*};
}

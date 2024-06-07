use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use arrayvec::ArrayVec;
use boojum::{
    algebraic_props::round_function::AlgebraicRoundFunction,
    crypto_bigint::{Zero, U1024},
    cs::{gates::ConstantAllocatableCS, traits::cs::ConstraintSystem},
    field::SmallField,
    gadgets::{
        boolean::Boolean,
        curves::sw_projective::SWProjectivePoint,
        keccak256::keccak256,
        non_native_field::implementations::*,
        num::Num,
        queue::{CircuitQueueWitness, QueueState},
        traits::{
            allocatable::{CSAllocatableExt, CSPlaceholder},
            round_function::CircuitRoundFunction,
            selectable::Selectable,
            witnessable::WitnessHookable,
        },
        u16::UInt16,
        u160::UInt160,
        u256::UInt256,
        u32::UInt32,
        u8::UInt8,
    },
};
use cs_derive::*;
use zkvm_opcodes::system_params::PRECOMPILE_AUX_BYTE;

use super::*;
use crate::{
    base_structures::{
        log_query::*, memory_query::*, precompile_input_outputs::PrecompileFunctionOutputData,
    },
    demux_log_queue::StorageLogQueue,
    ethereum_types::U256,
    fsm_input_output::{circuit_inputs::INPUT_OUTPUT_COMMITMENT_LENGTH, *},
};

pub mod input;
pub use self::input::*;

pub mod secp256k1;

pub const MEMORY_QUERIES_PER_CALL: usize = 4;

pub mod naf_abs_div2_table;
use naf_abs_div2_table::*;
pub mod decomp_table;
use decomp_table::*;

pub mod baseline;
pub mod new_optimized;

// characteristics of the base field for secp curve
use self::secp256k1::fq::Fq as Secp256Fq;
// order of group of points for secp curve
use self::secp256k1::fr::Fr as Secp256Fr;
// some affine point
use self::secp256k1::PointAffine as Secp256Affine;

type Secp256BaseNNFieldParams = NonNativeFieldOverU16Params<Secp256Fq, 17>;
type Secp256ScalarNNFieldParams = NonNativeFieldOverU16Params<Secp256Fr, 17>;

type Secp256BaseNNField<F> = NonNativeFieldOverU16<F, Secp256Fq, 17>;
type Secp256ScalarNNField<F> = NonNativeFieldOverU16<F, Secp256Fr, 17>;

fn secp256k1_base_field_params() -> Secp256BaseNNFieldParams {
    NonNativeFieldOverU16Params::create()
}

fn secp256k1_scalar_field_params() -> Secp256ScalarNNFieldParams {
    NonNativeFieldOverU16Params::create()
}

// re-exports for integration
pub use self::new_optimized::{ecrecover_function_entry_point, EcrecoverPrecompileCallParams};

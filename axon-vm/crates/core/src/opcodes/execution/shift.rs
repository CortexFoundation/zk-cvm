use std::ops::*;

use zkvm_opcodes::{Opcode, ShiftOpcode};

use super::*;

impl<const N: usize, E: VmEncodingMode<N>> DecodedOpcode<N, E> {
    pub fn shift_opcode_apply<
        S: zkvm_primitives::vm::Storage,
        M: zkvm_primitives::vm::Memory,
        EV: zkvm_primitives::vm::EventSink,
        PP: zkvm_primitives::vm::PrecompilesProcessor,
        DP: zkvm_primitives::vm::DecommittmentProcessor,
        WT: crate::witness_trace::VmWitnessTracer<N, E>,
    >(
        &self,
        vm_state: &mut VmState<S, M, EV, PP, DP, WT, N, E>,
        prestate: PreState<N, E>,
    ) {
        let PreState { src0, src1, dst0_mem_location, new_pc, .. } = prestate;
        let PrimitiveValue { value: src0, is_pointer: _ } = src0;
        let PrimitiveValue { value: src1, is_pointer: _ } = src1;

        let inner_variant = match self.variant.opcode {
            Opcode::Shift(inner) => inner,
            _ => unreachable!(),
        };

        use zkvm_opcodes::SET_FLAGS_FLAG_IDX;
        let set_flags = self.variant.flags[SET_FLAGS_FLAG_IDX];
        vm_state.local_state.callstack.get_current_stack_mut().pc = new_pc;
        let shift_abs = low_u64_of_u256(&src1) as u8;
        let is_cyclic = inner_variant == ShiftOpcode::Rol || inner_variant == ShiftOpcode::Ror;
        let is_right_shift = inner_variant == ShiftOpcode::Shr || inner_variant == ShiftOpcode::Ror;

        let result = if is_right_shift {
            let mut result = src0.shr(shift_abs as u32);
            if is_cyclic {
                result |= src0.shl(256u32 - shift_abs as u32);
            }

            result
        } else {
            let mut result = src0.shl(shift_abs as u32);
            if is_cyclic {
                result |= src0.shr(256u32 - shift_abs as u32);
            }

            result
        };
        if set_flags {
            let eq = result.is_zero();
            vm_state.reset_flags();
            vm_state.local_state.flags.equality_flag = eq;
        }

        let result = PrimitiveValue { value: result, is_pointer: false };
        vm_state.perform_dst0_update(
            vm_state.local_state.monotonic_cycle_counter,
            result,
            dst0_mem_location,
            self,
        );
    }
}

use super::*;

impl<const N: usize, E: VmEncodingMode<N>> DecodedOpcode<N, E> {
    pub fn jump_opcode_apply<
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
        let PreState { new_pc: _, src0, .. } = prestate;
        let PrimitiveValue { value: src0, is_pointer: _ } = src0;
        // we use lowest 16 bits of src0 as a jump destination
        let dest_pc = E::PcOrImm::from_u64_clipped(low_u64_of_u256(&src0));
        vm_state.local_state.callstack.get_current_stack_mut().pc = dest_pc;
    }
}

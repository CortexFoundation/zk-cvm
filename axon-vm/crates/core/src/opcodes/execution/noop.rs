use super::*;

impl<const N: usize, E: VmEncodingMode<N>> DecodedOpcode<N, E> {
    pub fn noop_opcode_apply<
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
        let PreState { new_pc, .. } = prestate;
        vm_state.local_state.callstack.get_current_stack_mut().pc = new_pc;
        // IMPORTANT: while we formally do not update the register value here, the NOP operation
        // may still formally address the operand and move SP this way
    }
}

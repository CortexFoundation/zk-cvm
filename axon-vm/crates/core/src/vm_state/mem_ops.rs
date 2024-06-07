use zkvm_opcodes::{ImmMemHandlerFlags, RegOrImmFlags};
use zkvm_primitives::{
    aux::{MemoryIndex, MemoryLocation},
    vm::MemoryType,
};

use super::*;

pub struct MemOpsProcessor<const N: usize = 8, E: VmEncodingMode<N> = EncodingModeProduction> {
    pub sp: E::PcOrImm,
}

use zkvm_opcodes::Operand;

impl<const N: usize, E: VmEncodingMode<N>> MemOpsProcessor<N, E> {
    pub fn compute_addresses_and_select_operands<
        S: zkvm_primitives::vm::Storage,
        M: zkvm_primitives::vm::Memory,
        EV: zkvm_primitives::vm::EventSink,
        PP: zkvm_primitives::vm::PrecompilesProcessor,
        DP: zkvm_primitives::vm::DecommittmentProcessor,
        WT: crate::witness_trace::VmWitnessTracer<N, E>,
    >(
        &mut self,
        vm_state: &VmState<S, M, EV, PP, DP, WT, N, E>,
        register_index_encoding: u8,
        imm: E::PcOrImm,
        mem_imm: Operand,
        is_write: bool,
    ) -> (PrimitiveValue, Option<MemoryLocation>) {
        let primitive_value = vm_state.select_register_value(register_index_encoding);
        let PrimitiveValue { value: reg_value, is_pointer: _ } = primitive_value;
        let reg_low = E::PcOrImm::from_u64_clipped(low_u64_of_u256(&reg_value));
        let vaddr = reg_low.wrapping_add(imm);
        let current_context = vm_state.local_state.callstack.get_current_stack();
        let memory_location = match mem_imm {
            Operand::RegOnly => None,
            Operand::RegOrImm(RegOrImmFlags::UseRegOnly) => None,
            Operand::RegOrImm(RegOrImmFlags::UseImm16Only) => {
                debug_assert!(!is_write);

                None
            }
            Operand::Full(ImmMemHandlerFlags::UseRegOnly) => None,
            Operand::Full(ImmMemHandlerFlags::UseImm16Only) => {
                debug_assert!(!is_write);

                None
            }
            Operand::Full(ImmMemHandlerFlags::UseStackWithPushPop) => {
                // SP may point to uninit values, but we will leave it for compiler
                let current_sp = self.sp;
                // now we also have to decide on case of push or pop, to have push not to overflow
                // 2^16, and pop not to underflow
                if is_write {
                    // a generalized version of 'push'
                    let old_sp = current_sp;
                    let new_sp = current_sp.wrapping_add(vaddr);
                    self.sp = new_sp;

                    let stack_page = CallStackEntry::<N, E>::stack_page_from_base(
                        current_context.base_memory_page,
                    );
                    let location = MemoryLocation {
                        memory_type: MemoryType::Stack,
                        page: stack_page,
                        index: MemoryIndex(old_sp.as_u64() as u32),
                    };

                    Some(location)
                } else {
                    // a generalized version of 'pop'
                    let new_sp = current_sp.wrapping_sub(vaddr);
                    self.sp = new_sp;

                    let stack_page = CallStackEntry::<N, E>::stack_page_from_base(
                        current_context.base_memory_page,
                    );
                    let location = MemoryLocation {
                        memory_type: MemoryType::Stack,
                        page: stack_page,
                        index: MemoryIndex(new_sp.as_u64() as u32),
                    };

                    Some(location)
                }
            }
            Operand::Full(ImmMemHandlerFlags::UseStackWithOffset) => {
                let offset = self.sp.wrapping_sub(vaddr);
                let stack_page =
                    CallStackEntry::<N, E>::stack_page_from_base(current_context.base_memory_page);
                let location = MemoryLocation {
                    memory_type: MemoryType::Stack,
                    page: stack_page,
                    index: MemoryIndex(offset.as_u64() as u32),
                };

                Some(location)
            }
            Operand::Full(ImmMemHandlerFlags::UseCodePage) => {
                debug_assert!(!is_write);
                let code_page = current_context.code_page;
                let location = MemoryLocation {
                    memory_type: MemoryType::Code,
                    page: code_page,
                    index: MemoryIndex(vaddr.as_u64() as u32),
                };

                Some(location)
            }
            Operand::Full(ImmMemHandlerFlags::UseAbsoluteOnStack) => {
                let stack_page =
                    CallStackEntry::<N, E>::stack_page_from_base(current_context.base_memory_page);
                let location = MemoryLocation {
                    memory_type: MemoryType::Stack,
                    page: stack_page,
                    index: MemoryIndex(vaddr.as_u64() as u32),
                };

                Some(location)
            }
        };

        (primitive_value, memory_location)
    }
}

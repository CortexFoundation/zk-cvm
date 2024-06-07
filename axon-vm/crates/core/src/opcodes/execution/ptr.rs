use zkvm_opcodes::{FatPointer, Opcode, PtrOpcode};

use super::*;

impl<const N: usize, E: VmEncodingMode<N>> DecodedOpcode<N, E> {
    pub fn ptr_opcode_apply<
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
        let inner_variant = match self.variant.opcode {
            Opcode::Ptr(inner) => inner,
            _ => unreachable!(),
        };

        vm_state.local_state.callstack.get_current_stack_mut().pc = new_pc;

        match inner_variant {
            a @ PtrOpcode::Add | a @ PtrOpcode::Sub => {
                // we check whether src0 is fat pointer
                if !src0.is_pointer {
                    // src0 is not a pointer
                    vm_state.set_shorthand_panic();
                    return;
                }

                if src1.is_pointer {
                    // can not have ptr + ptr
                    vm_state.set_shorthand_panic();
                    return;
                }

                if src1.value >= zkvm_opcodes::ptr::MAX_OFFSET_FOR_ADD_SUB {
                    // offset is too far to be reasonable, so instead of wrapping behavior we bail
                    // out
                    vm_state.set_shorthand_panic();
                    return;
                }

                let PrimitiveValue { value: src0, is_pointer: _ } = src0;
                let PrimitiveValue { value: src1, is_pointer: _ } = src1;

                let fat_ptr = FatPointer::from_u256(src0);
                let offset = low_u32_of_u256(&src1);

                let (new_ptr_offset, error) = if a == PtrOpcode::Add {
                    fat_ptr.offset.overflowing_add(offset)
                } else {
                    fat_ptr.offset.overflowing_sub(offset)
                };

                if error {
                    vm_state.set_shorthand_panic();
                    return;
                }

                let mut new_ptr = fat_ptr;
                new_ptr.offset = new_ptr_offset;

                let ptr_as_u256 = new_ptr.to_u256();

                // low 128 bits from ptr_as_u256, high 128 - from src0
                let result = U256::from_limbs([
                    ptr_as_u256.as_limbs()[0],
                    ptr_as_u256.as_limbs()[1],
                    src0.as_limbs()[2],
                    src0.as_limbs()[3],
                ]);

                let result = PrimitiveValue { value: result, is_pointer: true };

                vm_state.perform_dst0_update(
                    vm_state.local_state.monotonic_cycle_counter,
                    result,
                    dst0_mem_location,
                    self,
                );
            }
            PtrOpcode::Pack => {
                // we check whether src0 is fat pointer
                if !src0.is_pointer {
                    // src0 is not a pointer
                    vm_state.set_shorthand_panic();
                    return;
                }

                if src1.is_pointer {
                    // can not have ptr + ptr
                    vm_state.set_shorthand_panic();
                    return;
                }

                if low_u128_of_u256(&src1.value) != 0 {
                    // mask is not a mask indeed
                    vm_state.set_shorthand_panic();
                    return;
                }

                let PrimitiveValue { value: src0, is_pointer: _ } = src0;
                let PrimitiveValue { value: src1, is_pointer: _ } = src1;

                // low 128 bits from src0, high 128 - from src1
                let result = U256::from_limbs([
                    src0.as_limbs()[0],
                    src0.as_limbs()[1],
                    src1.as_limbs()[2],
                    src1.as_limbs()[3],
                ]);

                let result = PrimitiveValue { value: result, is_pointer: true };

                vm_state.perform_dst0_update(
                    vm_state.local_state.monotonic_cycle_counter,
                    result,
                    dst0_mem_location,
                    self,
                );
            }
            PtrOpcode::Shrink => {
                // we check whether src0 is fat pointer
                if !src0.is_pointer {
                    // src0 is not a pointer
                    vm_state.set_shorthand_panic();
                    return;
                }

                if src1.is_pointer {
                    // can not have ptr + ptr
                    vm_state.set_shorthand_panic();
                    return;
                }

                let PrimitiveValue { value: src0, is_pointer: _ } = src0;
                let PrimitiveValue { value: src1, is_pointer: _ } = src1;

                let fat_ptr = FatPointer::from_u256(src0);
                let offset = low_u32_of_u256(&src1);

                let (new_ptr_length, error) = fat_ptr.length.overflowing_sub(offset);

                if error {
                    vm_state.set_shorthand_panic();
                    return;
                }

                let mut new_ptr = fat_ptr;
                new_ptr.length = new_ptr_length;

                let ptr_as_u256 = new_ptr.to_u256();

                // low 128 bits from ptr_as_u256, high 128 - from src0
                let result = U256::from_limbs([
                    ptr_as_u256.as_limbs()[0],
                    ptr_as_u256.as_limbs()[1],
                    src0.as_limbs()[2],
                    src0.as_limbs()[3],
                ]);

                let result = PrimitiveValue { value: result, is_pointer: true };

                vm_state.perform_dst0_update(
                    vm_state.local_state.monotonic_cycle_counter,
                    result,
                    dst0_mem_location,
                    self,
                );
            }
        }
    }
}

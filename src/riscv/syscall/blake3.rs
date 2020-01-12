use ckb_vm::instructions::Register;
use ckb_vm::Memory;

use crate::riscv::syscall::common::get_arr;
use crate::riscv::syscall::convention::SYSCODE_BLAKE3;

pub struct SyscallBlake3;

impl<Mac: ckb_vm::SupportMachine> ckb_vm::Syscalls<Mac> for SyscallBlake3 {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), ckb_vm::Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, ckb_vm::Error> {
        let code = &machine.registers()[ckb_vm::registers::A7];

        if code.to_i32() != SYSCODE_BLAKE3 {
            return Ok(false);
        }

        let m_addr = machine.registers()[ckb_vm::registers::A0].to_u64();
        let m_size = machine.registers()[ckb_vm::registers::A1].to_u64();
        let b_addr = machine.registers()[ckb_vm::registers::A2].to_u64();
        let b_size = machine.registers()[ckb_vm::registers::A3].to_u64();

        // Check buffer size, if not enough, return 1
        if b_size < 32 {
            machine.set_register(ckb_vm::registers::A0, Mac::REG::from_u8(1));

            return Ok(true);
        }

        let m = get_arr(machine, m_addr, m_size)?;
        let h = blake3::hash(&m);

        machine.memory_mut().store_bytes(b_addr, h.as_bytes())?;

        machine.set_register(ckb_vm::registers::A0, Mac::REG::from_u8(0));

        Ok(true)
    }
}

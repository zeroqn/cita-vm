use std::cell::RefCell;
use std::rc::Rc;

use ckb_vm::instructions::Register;
use ckb_vm::memory::Memory;

use crate::riscv::syscall::convention::SYSCODE_INTF;

pub struct Snapshot<R> {
    pub pc: R,
    pub registers: Vec<R>,
    pub memory: Vec<u8>,
}

impl<R: ckb_vm::Register> Snapshot<R> {
    pub fn new() -> Self {
        Self {
            pc: R::from_u8(0),
            registers: vec![],
            memory: vec![],
        }
    }
}

impl<R: ckb_vm::Register> Default for Snapshot<R> {
    fn default() -> Self {
        Snapshot::new()
    }
}

pub struct SyscallIntf {
    snapshot: Rc<RefCell<Snapshot<u64>>>,
}

impl SyscallIntf {
    pub fn new(snapshot: Rc<RefCell<Snapshot<u64>>>) -> Self {
        Self { snapshot }
    }
}

impl<Mac: ckb_vm::SupportMachine> ckb_vm::Syscalls<Mac> for SyscallIntf {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), ckb_vm::Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, ckb_vm::Error> {
        let code = &machine.registers()[ckb_vm::registers::A7];
        if code.to_i32() != SYSCODE_INTF {
            return Ok(false);
        }

        self.snapshot.borrow_mut().pc = machine.pc().to_u64();
        for e in machine.registers() {
            self.snapshot.borrow_mut().registers.push(e.to_u64());
        }
        for i in 0..ckb_vm::RISCV_MAX_MEMORY {
            let a = machine.memory_mut().load8(&Mac::REG::from_u64(i as u64)).unwrap();
            self.snapshot.borrow_mut().memory.push(a.to_u8());
        }
        machine.set_register(ckb_vm::registers::A0, Mac::REG::from_u8(0));
        Ok(true)
    }
}

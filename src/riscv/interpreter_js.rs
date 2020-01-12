use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::rc::Rc;

use bytes::Bytes;
use ckb_vm::machine::CoreMachine;
use ckb_vm::machine::SupportMachine;
use ckb_vm::memory::Memory;
use ckb_vm::Register;

use crate::evm::DataProvider;
use crate::riscv;
use crate::riscv::syscall::Snapshot;
use crate::{Context, InterpreterParams, InterpreterResult};

pub fn get_duktape_snapshot(bin: impl AsRef<Path>) -> Rc<RefCell<Snapshot<u64>>> {
    let file = fs::read(bin).unwrap();
    let snapshot = Rc::new(RefCell::new(crate::riscv::Snapshot::new()));
    let mut machine =
        ckb_vm::DefaultMachineBuilder::<ckb_vm::DefaultCoreMachine<u64, ckb_vm::FlatMemory<u64>>>::default()
            .syscall(Box::new(crate::riscv::SyscallIntf::new(snapshot.clone())))
            .build();

    machine.load_program(&Bytes::from(file), &["main".into()]).unwrap();
    machine.run().unwrap();
    snapshot
}

fn get_main(args: Vec<String>) -> String {
    let mut s = String::new();
    s.push_str("main(");

    let argc = args.len();
    let argc_s: String = (argc + 1).to_string();
    s.push_str(argc_s.as_str());
    s.push_str(", ");
    s.push_str("new Array(\"main\"");

    if argc != 0 {
        s.push_str(", ");
        for e in args.iter().take(argc - 1) {
            s.push_str("\"");
            s.push_str(e.as_str());
            s.push_str("\", ");
        }
        s.push_str("\"");
        s.push_str(args[argc - 1].as_str());
        s.push_str("\"");
    }
    s.push_str(")");
    s.push_str(")");
    s
}

pub struct InterpreterJS {
    pub context: Context,
    pub iparams: InterpreterParams,
    pub data_provider: Rc<RefCell<dyn DataProvider>>,
    snapshot: Rc<RefCell<Snapshot<u64>>>,
}

impl InterpreterJS {
    pub fn new(
        context: Context,
        iparams: InterpreterParams,
        data_provider: Rc<RefCell<dyn DataProvider>>,
        snapshot: Rc<RefCell<Snapshot<u64>>>,
    ) -> Self {
        Self {
            context,
            iparams,
            data_provider,
            snapshot,
        }
    }

    pub fn run(&mut self) -> Result<InterpreterResult, ckb_vm::Error> {
        let ret_data = Rc::new(RefCell::new(Vec::new()));
        let core_machine =
            ckb_vm::DefaultCoreMachine::<u64, ckb_vm::FlatMemory<u64>>::new_with_max_cycles(self.iparams.gas_limit);
        let mut machine =
            ckb_vm::DefaultMachineBuilder::<ckb_vm::DefaultCoreMachine<u64, ckb_vm::FlatMemory<u64>>>::new(
                core_machine,
            )
            .instruction_cycle_func(Box::new(riscv::cost_model::instruction_cycles))
            .syscall(Box::new(riscv::SyscallDebug::new("riscv:", std::io::stdout())))
            .syscall(Box::new(riscv::SyscallEnvironment::new(
                self.context.clone(),
                self.iparams.clone(),
                self.data_provider.clone(),
            )))
            .syscall(Box::new(riscv::SyscallRet::new(ret_data.clone())))
            .syscall(Box::new(riscv::SyscallStorage::new(
                self.iparams.address,
                self.data_provider.clone(),
            )))
            .build();

        machine.set_pc(self.snapshot.borrow().pc + 4);
        machine.set_register(0, 0);
        for i in 1..32 {
            machine.set_register(i, self.snapshot.borrow().registers[i]);
        }
        for i in 0..ckb_vm::RISCV_MAX_MEMORY {
            machine
                .memory_mut()
                .store8(&(i as u64), &u64::from(self.snapshot.borrow().memory[i]))
                .unwrap();
        }

        let addr = machine.registers()[ckb_vm::registers::A0].to_u64();
        let _ = machine.registers()[ckb_vm::registers::A1].to_u64();
        let r_size_addr = machine.registers()[ckb_vm::registers::A2].to_u64();

        let mut src = self.iparams.contract.code_data.clone();
        src.push(b'\n');

        let contract_args: Vec<String> = self
            .iparams
            .input
            .split(|e| *e == 0x00)
            .filter(|e| !e.is_empty())
            .map(|e| String::from_utf8(e.to_vec()).unwrap_or_else(|_| "".into()))
            .collect();
        let main = get_main(contract_args);
        src.append(&mut main.as_bytes().to_vec());

        machine.memory_mut().store_bytes(addr, &src).unwrap();
        machine
            .memory_mut()
            .store_bytes(r_size_addr, &src.len().to_le_bytes()[..])
            .unwrap();

        let exitcode = machine.run()?;
        let cycles = machine.cycles();
        if exitcode != 0x00 {
            Ok(InterpreterResult::Revert(
                ret_data.borrow().to_vec(),
                self.iparams.gas_limit - cycles,
            ))
        } else {
            Ok(InterpreterResult::Normal(
                ret_data.borrow().to_vec(),
                self.iparams.gas_limit - cycles,
                vec![],
            ))
        }
    }
}

use std::fs;

use ethereum_types::U256;

fn main() {
    let vm = cita_vm::FakeVM::new();

    let tx = cita_vm::Transaction {
        from: vm.account1,
        to: None,
        value: U256::from(0),
        nonce: U256::from(1),
        gas_limit: 1_000_000,
        gas_price: U256::from(1),
        input: fs::read("./build/riscv_c_blake3").unwrap(),
        itype: cita_vm::InterpreterType::RISCV,
    };

    let r = vm.executor.exec(cita_vm::Context::default(), tx).unwrap();
    println!("{:?}", r);

    let contract_address = match r {
        cita_vm::InterpreterResult::Create(_, _, _, a) => a,
        _ => unreachable!(),
    };

    let tx = cita_vm::Transaction {
        from: vm.account1,
        to: Some(contract_address),
        value: U256::from(0),
        nonce: U256::from(2),
        gas_limit: 1_000_000,
        gas_price: U256::from(1),
        input: cita_vm::riscv::combine_parameters(vec!["blake3".into(), "fly me to the moon".into()]),
        itype: cita_vm::InterpreterType::RISCV,
    };

    let r = vm.executor.exec(cita_vm::Context::default(), tx).unwrap();

    let hash_value = match r {
        cita_vm::InterpreterResult::Normal(val, ..) => val,
        _ => unreachable!(),
    };

    println!("{:?}", hash_value);
    assert_eq!(hash_value, blake3::hash(b"fly me to the moon").as_bytes());
}

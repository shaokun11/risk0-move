#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental


use risc0_zkvm::guest::env;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use move_core_types::value::{serialize_values, MoveValue};
use move_vm_runtime::move_vm::MoveVM;
use move_vm_test_utils::InMemoryStorage;
use move_vm_types::gas::UnmeteredGasMeter;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    const TEST_ADDR: AccountAddress = AccountAddress::ONE;
    // read the input
    let a: u64 = env::read();
    let b: u64 = env::read();
    // module 0x1::math {
    //     #[view]
    //     public fun sum(a: u64, b :u64):u64  {
    //          return (a + b)
    //     }
    // }
    // move build to get blob
    let blob = hex::decode("a11ceb0b0600000006010002030205050706070d090816100c260c00000001000100020303010300046d6174680373756d000000000000000000000000000000010001000002040b000b01160200").unwrap();
    let mut storage = InMemoryStorage::new();
    let module_id = ModuleId::new(TEST_ADDR, Identifier::new("math").unwrap());
    storage.publish_or_overwrite_module(module_id.clone(), blob);
    let vm = MoveVM::new(vec![]).unwrap();
    let mut sess = vm.new_session(&storage);
    let fun_name = Identifier::new("sum").unwrap();
    // let a: u64 = 1;
    // let b: u64 = 2;
    let values = vec![MoveValue::U64(a), MoveValue::U64(b)];
    let return_vals = sess
        .execute_function_bypass_visibility(
            &module_id,
            &fun_name,
            vec![],
            serialize_values(&values),
            &mut UnmeteredGasMeter,
        )
        .unwrap();
    for (blob, layout) in return_vals.return_values.into_iter() {
        let ret_value = MoveValue::simple_deserialize(&blob, &layout).unwrap();
        // 3u64 ==> remove the type we know this is u64
        let num: String = ret_value.to_string().chars().take_while(|&c| c.is_digit(10)).collect();
        let ret =  num.parse::<u64>().unwrap();
        env::commit(&ret);
        // the return value only one
        break;
    }
}

use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/multisig");

    blockchain.register_partial_contract::<multisig::AbiProvider, _>(
        "file:output/multisig.wasm",
        multisig::ContractBuilder,
        "multisig",
    );
    blockchain.register_partial_contract::<multisig::AbiProvider, _>(
        "file:output/multisig-view.wasm",
        multisig::ContractBuilder,
        "multisig-view",
    );

    blockchain.register_contract("file:test-contracts/adder.wasm", adder::ContractBuilder);

    blockchain.register_contract(
        "file:test-contracts/factorial.wasm",
        factorial::ContractBuilder,
    );

    blockchain
}

#[ignore]
#[test]
fn call_other_shard_1_rs() {
    elrond_wasm_debug::mandos_rs("mandos/call_other_shard-1.scen.json", world());
}

#[ignore]
#[test]
fn call_other_shard_2_rs() {
    elrond_wasm_debug::mandos_rs("mandos/call_other_shard-2.scen.json", world());
}

#[test]
fn changeboard_rs() {
    elrond_wasm_debug::mandos_rs("mandos/changeBoard.scen.json", world());
}

#[test]
fn changequorum_rs() {
    elrond_wasm_debug::mandos_rs("mandos/changeQuorum.scen.json", world());
}

#[test]
fn changequorum_toobig_rs() {
    elrond_wasm_debug::mandos_rs("mandos/changeQuorum_tooBig.scen.json", world());
}

#[test]
fn deployadder_err_rs() {
    elrond_wasm_debug::mandos_rs("mandos/deployAdder_err.scen.json", world());
}

#[test]
fn deployadder_then_call_rs() {
    elrond_wasm_debug::mandos_rs("mandos/deployAdder_then_call.scen.json", world());
}

#[test]
fn deployfactorial_rs() {
    elrond_wasm_debug::mandos_rs("mandos/deployFactorial.scen.json", world());
}

#[test]
fn deployothermultisig_rs() {
    elrond_wasm_debug::mandos_rs("mandos/deployOtherMultisig.scen.json", world());
}

#[test]
fn deploy_duplicate_bm_rs() {
    elrond_wasm_debug::mandos_rs("mandos/deploy_duplicate_bm.scen.json", world());
}

#[test]
fn remove_everyone_rs() {
    elrond_wasm_debug::mandos_rs("mandos/remove_everyone.scen.json", world());
}

#[test]
fn sendesdt_rs() {
    elrond_wasm_debug::mandos_rs("mandos/sendEsdt.scen.json", world());
}

#[test]
fn upgrade_rs() {
    elrond_wasm_debug::mandos_rs("mandos/upgrade.scen.json", world());
}

#[test]
fn upgrade_from_source_rs() {
    elrond_wasm_debug::mandos_rs("mandos/upgrade_from_source.scen.json", world());
}

use std::collections::BTreeMap;

use crate::world_mock::{AccountData, BlockInfo, EsdtData};
use elrond_wasm::types::Address;
use mandos::serde_raw::{
    AccountRaw, BlockInfoRaw, CheckBytesValueRaw, CheckLogsRaw, EsdtFullRaw, EsdtRaw, InstanceRaw,
    TxCallRaw, TxESDTRaw, TxExpectRaw, TxQueryRaw, ValueSubTree,
};

use super::{ScCallMandos, ScQueryMandos, TxExpectMandos};

pub(crate) const STAR_STR: &str = "*";

pub(crate) fn account_as_raw(acc: &AccountData) -> AccountRaw {
    let balance_raw = Some(rust_biguint_as_raw(&acc.egld_balance));
    let code_raw = acc
        .contract_path
        .clone()
        .map(|c| ValueSubTree::Str(String::from_utf8(c).unwrap()));

    let mut all_esdt_raw = BTreeMap::new();
    for (token_id, esdt_data) in acc.esdt.iter() {
        let token_id_raw = String::from_utf8(token_id.clone()).unwrap();
        let esdt_raw = esdt_data_as_raw(esdt_data);

        let _ = all_esdt_raw.insert(token_id_raw, esdt_raw);
    }

    let mut storage_raw = BTreeMap::new();
    for (key, value) in acc.storage.iter() {
        let key_raw = String::from_utf8(key.clone()).unwrap();
        let value_raw = bytes_as_raw(value);

        let _ = storage_raw.insert(key_raw, value_raw);
    }

    AccountRaw {
        balance: balance_raw,
        code: code_raw,
        comment: None,
        esdt: all_esdt_raw,
        nonce: Some(u64_as_raw(acc.nonce)),
        owner: acc.contract_owner.as_ref().map(address_as_raw),
        storage: storage_raw,
        username: None, // TODO: Add if needed
    }
}

pub(crate) fn esdt_data_as_raw(esdt: &EsdtData) -> EsdtRaw {
    let last_nonce_raw = if esdt.last_nonce == 0 {
        None
    } else {
        Some(u64_as_raw(esdt.last_nonce))
    };

    let roles = esdt.get_roles();
    let mut roles_raw = Vec::with_capacity(roles.len());
    for role in roles {
        roles_raw.push(String::from_utf8(role).unwrap());
    }

    let mut instances_raw = Vec::new();
    for inst in esdt.instances.get_instances().values() {
        let inst_raw = InstanceRaw {
            attributes: Some(bytes_as_raw(&inst.metadata.attributes)),
            balance: Some(rust_biguint_as_raw(&inst.balance)),
            creator: inst.metadata.creator.as_ref().map(address_as_raw),
            hash: inst.metadata.hash.as_ref().map(|h| bytes_as_raw(h)),
            nonce: Some(u64_as_raw(inst.nonce)),
            royalties: Some(u64_as_raw(inst.metadata.royalties)),
            uri: inst.metadata.uri.as_ref().map(|u| bytes_as_raw(u)),
        };

        instances_raw.push(inst_raw);
    }

    EsdtRaw::Full(EsdtFullRaw {
        frozen: None,
        instances: instances_raw,
        last_nonce: last_nonce_raw,
        roles: roles_raw,
        token_identifier: Some(bytes_as_raw(&esdt.token_identifier)),
    })
}

pub(crate) fn block_info_as_raw(block_info: &BlockInfo) -> BlockInfoRaw {
    BlockInfoRaw {
        block_epoch: Some(u64_as_raw(block_info.block_epoch)),
        block_nonce: Some(u64_as_raw(block_info.block_nonce)),
        block_round: Some(u64_as_raw(block_info.block_round)),
        block_timestamp: Some(u64_as_raw(block_info.block_timestamp)),
        block_random_seed: Some(bytes_as_raw(&block_info.block_random_seed[..])),
    }
}

pub(crate) fn tx_call_as_raw(tx_call: &ScCallMandos) -> TxCallRaw {
    let mut all_esdt_raw = Vec::with_capacity(tx_call.esdt.len());
    for esdt in tx_call.esdt.iter() {
        let esdt_raw = TxESDTRaw {
            token_identifier: Some(bytes_as_raw(&esdt.token_identifier)),
            nonce: Some(u64_as_raw(esdt.nonce)),
            value: rust_biguint_as_raw(&esdt.value),
        };

        all_esdt_raw.push(esdt_raw);
    }

    let mut arguments_raw = Vec::with_capacity(tx_call.arguments.len());
    for arg in tx_call.arguments.iter() {
        let arg_raw = bytes_as_raw(arg);
        arguments_raw.push(arg_raw);
    }

    TxCallRaw {
        from: address_as_raw(&tx_call.from),
        to: address_as_raw(&tx_call.to),
        value: None, // this is the old "value" field, which is now "egld_value". Only kept for backwards compatibility
        egld_value: Some(rust_biguint_as_raw(&tx_call.egld_value)),
        esdt_value: all_esdt_raw,
        function: tx_call.function.clone(),
        arguments: arguments_raw,
        gas_limit: u64_as_raw(tx_call.gas_limit),
        gas_price: u64_as_raw(tx_call.gas_price),
    }
}

pub(crate) fn tx_query_as_raw(tx_query: &ScQueryMandos) -> TxQueryRaw {
    let mut arguments_raw = Vec::with_capacity(tx_query.arguments.len());
    for arg in tx_query.arguments.iter() {
        let arg_raw = bytes_as_raw(arg);
        arguments_raw.push(arg_raw);
    }

    TxQueryRaw {
        to: address_as_raw(&tx_query.to),
        function: tx_query.function.clone(),
        arguments: arguments_raw,
    }
}

pub(crate) fn tx_expect_as_raw(tx_expect: &TxExpectMandos) -> TxExpectRaw {
    let mut out_values_raw = Vec::with_capacity(tx_expect.out.len());
    for out_val in tx_expect.out.iter() {
        let out_raw = if out_val.len() == 1 && out_val[0] == b'*' {
            CheckBytesValueRaw::Star
        } else {
            CheckBytesValueRaw::Equal(bytes_as_raw(out_val))
        };

        out_values_raw.push(out_raw);
    }

    let msg_raw = if tx_expect.message == STAR_STR {
        CheckBytesValueRaw::Star
    } else {
        let mandos_formatted_str = "str:".to_owned() + &tx_expect.message;
        CheckBytesValueRaw::Equal(ValueSubTree::Str(mandos_formatted_str))
    };

    TxExpectRaw {
        out: out_values_raw,
        status: CheckBytesValueRaw::Equal(u64_as_raw(tx_expect.status)),
        message: msg_raw,
        logs: CheckLogsRaw::Star,
        gas: CheckBytesValueRaw::Star,
        refund: CheckBytesValueRaw::Star,
    }
}

pub(crate) fn rust_biguint_as_raw(big_uint: &num_bigint::BigUint) -> ValueSubTree {
    ValueSubTree::Str(big_uint.to_string())
}

pub(crate) fn address_as_raw(address: &Address) -> ValueSubTree {
    bytes_as_raw(address.as_bytes())
}

pub(crate) fn u64_as_raw(value: u64) -> ValueSubTree {
    ValueSubTree::Str(value.to_string())
}

pub(crate) fn bytes_as_raw(bytes: &[u8]) -> ValueSubTree {
    ValueSubTree::Str(bytes_to_hex(bytes))
}

pub(crate) fn bytes_to_hex(bytes: &[u8]) -> String {
    "0x".to_owned() + &hex::encode(bytes)
}
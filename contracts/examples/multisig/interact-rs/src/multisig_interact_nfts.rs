use elrond_interaction::elrond_wasm::elrond_codec::test_util::top_encode_to_vec_u8_or_panic;

use super::*;

const ISSUE_COST: u64 = 50000000000000000; // 0.05 EGLD

const COLLECTION_NAME: &str = "TestCollection1";
const COLLECTION_TICKER: &str = "TESTCOLL1";
pub const COLLECTION_TOKEN_IDENTIFIER: &str = "TESTCOLL1-a36f7b";
const NUM_ITEMS: usize = 3;
const ROYALTIES: usize = 3000;
const METADATA: &str = "tags:test,rust-interactor";

impl State {
    pub(crate) async fn propose_issue_collection(&mut self) -> usize {
        let system_sc_address = bech32::decode(SYSTEM_SC_BECH32);
        self.interactor
            .sc_call_get_result(
                self.multisig
                    .propose_async_call(
                        system_sc_address,
                        ISSUE_COST,
                        "issueNonFungible".to_string(),
                        MultiValueVec::from([
                            COLLECTION_NAME.to_string(),
                            COLLECTION_TICKER.to_string(),
                        ]),
                    )
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .gas_limit("10,000,000")
                    .expect(TxExpect::ok()),
            )
            .await
    }

    pub(crate) async fn issue_collection(&mut self) {
        let action_id = self.propose_issue_collection().await;
        let tx_hash = self.perform_action(action_id, "80,000,000").await;
        println!("perform issue collection tx hash: {}", tx_hash);
    }

    pub(crate) async fn propose_set_special_role(&mut self) -> usize {
        let multisig_address = self.multisig.to_address();
        self.interactor
            .sc_call_get_result(
                self.multisig
                    .propose_async_call(
                        &self.system_sc_address,
                        0u64,
                        "setSpecialRole".to_string(),
                        MultiValueVec::from([
                            self.collection_token_identifier.as_bytes(),
                            multisig_address.as_bytes(),
                            "ESDTRoleNFTCreate".as_bytes(),
                        ]),
                    )
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .gas_limit("10,000,000")
                    .expect(TxExpect::ok()),
            )
            .await
    }

    pub(crate) async fn set_special_role(&mut self) {
        let action_id = self.propose_set_special_role().await;
        let tx_hash = self.perform_action(action_id, "80,000,000").await;
        println!("perform issue collection tx hash: {}", tx_hash);
    }

    pub(crate) async fn create_items(&mut self) {
        let mut last_index = self.get_action_last_index().await;
        let multisig_address = self.multisig.to_address();

        let mut steps = Vec::<ScCallStep>::new();
        for item_index in 0..NUM_ITEMS {
            let item_name = format!("Test collection item #{}", item_index);
            let image_cid = format!(
                "https://ipfs.io/ipfs/QmYyAaEf1phJS5mN6wfou5de5GbpUddBxTY1VekKcjd5PC/nft{:02}.jpeg",
                item_index
            );

            steps.push(
                self.multisig
                    .propose_async_call(
                        &multisig_address,
                        0u64,
                        "ESDTNFTCreate".to_string(),
                        MultiValueVec::from([
                            self.collection_token_identifier.as_bytes(),
                            top_encode_to_vec_u8_or_panic(&1u32).as_slice(),
                            item_name.as_bytes(),
                            top_encode_to_vec_u8_or_panic(&ROYALTIES).as_slice(),
                            &[][..],
                            METADATA.as_bytes(),
                            image_cid.as_bytes(),
                        ]),
                    )
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .gas_limit("10,000,000")
                    .into(),
            );
        }

        for _ in 0..NUM_ITEMS {
            last_index += 1;
            steps.push(self.perform_action_step(last_index, "30,000,000"));
        }

        self.interactor.multiple_sc_calls(steps.as_slice()).await;
    }
}

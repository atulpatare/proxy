use fuels::prelude::*;

pub mod setup {

    use super::*;

    // Load abi from json
    abigen!(Proxy, "proxy/out/debug/proxy-abi.json");
    abigen!(MyContract, "contract/out/debug/contract-abi.json");
    abigen!(MyContractV2, "contract_v2/out/debug/contract_v2-abi.json");


    pub async fn get_wallets() -> Vec<WalletUnlocked> {
        launch_custom_provider_and_get_wallets(
            WalletsConfig::new(
                Some(1), /* Single wallet */
                Some(1), /* Single coin (UTXO) */
                Some(1_000_000_000), /* Amount per coin */
            ),
            None,
            None,
        ).await
    }

    pub async fn deploy_contract(bin_path: &str, storage_path: &str) -> (Bech32ContractId, WalletUnlocked) {
        let mut wallets = get_wallets().await;
        let wallet = wallets.pop().unwrap();
        let storage_config: StorageConfiguration = StorageConfiguration::with_storage_path(
            Some(storage_path.to_string())
        );

        let contract_id = Contract::deploy(
            bin_path,
            &wallet,
            TxParameters::default(),
            storage_config,
        ).await.unwrap();

        (contract_id, wallet)
    }

    pub async fn get_proxy_contract_instance() -> (Proxy, Bech32ContractId) {
        let proxy_bin: &str = "proxy/out/debug/proxy.bin";
        let proxy_storage: &str = "proxy/out/debug/proxy-storage_slots.json";
        let (contract_id, wallet) = deploy_contract(proxy_bin, proxy_storage).await;

        let instance = Proxy::new(contract_id.clone(), wallet);
        (instance, contract_id)
    }

    pub async fn get_my_contract_instance() -> (MyContract, Bech32ContractId) {
        let my_contract_bin: &str = "contract/out/debug/contract.bin";
        let my_contract_storage: &str = "contract/out/debug/contract-storage_slots.json";
        let (contract_id, wallet) = deploy_contract(my_contract_bin, my_contract_storage).await;

        let instance = MyContract::new(contract_id.clone(), wallet);
        (instance, contract_id)
    }

    pub async fn get_my_contract_v2_instance() -> (MyContractV2, Bech32ContractId) {
        let my_contract_v2_bin: &str = "contract_v2/out/debug/contract_v2.bin";
        let my_contract_v2_storage: &str = "contract_v2/out/debug/contract_v2-storage_slots.json";
        let (contract_id, wallet) = deploy_contract(my_contract_v2_bin, my_contract_v2_storage).await;

        let instance = MyContractV2::new(contract_id.clone(), wallet);
        (instance, contract_id)
    }
}

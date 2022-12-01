use crate::contract::setup;

mod success {
    use super::*;

    #[tokio::test]
    async fn should_initialize_contracts () {
        let (_, my_contract_id) = setup::get_my_contract_instance().await;
        let (_, my_contract_v2_id) = setup::get_my_contract_v2_instance().await;
        let (_, proxy_id) = setup::get_proxy_contract_instance().await;

        println!("Proxy contract id : {}", proxy_id.hash.to_string());
        println!("MyContract contract id : {}", my_contract_id.hash.to_string());
        println!("MyContractV2 contract id : {}", my_contract_v2_id.hash.to_string());
    }

}

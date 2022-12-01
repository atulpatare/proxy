use fuels::prelude::{Bits256};
use crate::contract::setup;

#[tokio::test]
async fn should_initialize_contracts() {
    let (_, my_contract_id) = setup::get_my_contract_instance().await;
    let (_, my_contract_v2_id) = setup::get_my_contract_v2_instance().await;
    let (_, proxy_id) = setup::get_proxy_contract_instance().await;

    println!("Proxy contract id : 0x{}", proxy_id.hash);
    println!("MyContract contract id : 0x{}", my_contract_id.hash);
    println!("MyContractV2 contract id : 0x{}", my_contract_v2_id.hash);
}

#[tokio::test]
async fn should_able_load_contract_in_proxy() {
    let (instance, _) = setup::get_proxy_contract_instance().await;
    let (_, my_contract_id) = setup::get_my_contract_instance().await;

    let contract_addr = Bits256(*my_contract_id.hash);
    let result = instance
        .methods()
        .upgrade(contract_addr)
        .set_contracts(&[my_contract_id.clone()])
        .call()
        .await;
    result.as_ref().unwrap();
    assert_eq!(result.as_ref().is_err(), false);
}

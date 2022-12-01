use crate::contract::setup;

#[tokio::test]
async fn should_return_true_my_contract() {
    let (instance, _) = setup::get_my_contract_instance().await;
    let result = instance.methods().test_function().call().await;

    assert_eq!(result.is_err(), false);
    assert_eq!(result.unwrap().value, true);
}

#[tokio::test]
async fn should_return_false_my_contract_v2() {
    let (instance, _) = setup::get_my_contract_v2_instance().await;
    let result = instance.methods().test_function().call().await;

    assert_eq!(result.is_err(), false);
    assert_eq!(result.unwrap().value, false);
}

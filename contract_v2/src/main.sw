contract;

abi MyContractV2 {
    fn test_function() -> bool;
}

impl MyContractV2 for Contract {
    fn test_function() -> bool {
        false
    }
}

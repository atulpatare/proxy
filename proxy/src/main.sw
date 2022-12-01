contract;

abi Proxy {
    fn upgrade(contract_id: b256);
}

impl Proxy for Contract {
    fn upgrade(contract_id: b256) {
        asm(output: true, rC, rB: contract_id, rA) {
            ldc rA rB rC;
        }
    }
}

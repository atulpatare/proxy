contract;

abi Proxy {
    fn upgrade(contract_id: b256);
}

impl Proxy for Contract {
    fn upgrade(contract_id: b256) {
        asm(output: false, rA: contract_id, rB: 0, rC: 60) {
            ldc rA rB rC;
        }
    }
}

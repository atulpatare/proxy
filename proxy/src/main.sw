contract;

abi Proxy {
    fn upgrade(contract_id: b256);
}

impl Proxy for Contract {
    fn upgrade(contract_id: b256)  {
        asm(r1: 10, r2) {
            add r2 r1 one;
        }


        /*
         rA: contract id
         rB: index to start copy from, (contract offset)
         rC: amount of bytes to copy (length of code)
         ref: https://fuellabs.github.io/fuel-specs/master/vm/instruction_set.html?highlight=ldc#ldc-load-code-from-an-external-contract
        */
        asm(rA: contract_id, rB, rC) {
            ldc rA rB rC;
        }
    }
}

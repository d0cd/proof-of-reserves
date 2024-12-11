program proof_of_reserves_v0_1_0.aleo {
    record Dummy {
        owner: address
    }

    transition mint_dummy() -> (Dummy, group) {
        let dummy: Dummy = Dummy {
            owner: self.caller
        };
        return (dummy, dummy.nonce());
    }
}

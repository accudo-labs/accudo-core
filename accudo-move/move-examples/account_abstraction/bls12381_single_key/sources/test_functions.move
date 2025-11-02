module aa::test_functions {
    use accudo_framework::accudo_account;

    /// test function for multi-agent aa.
    public entry fun transfer_to_the_last(a: &signer, b: &signer, c: &signer, d: address) {
        accudo_account::transfer(a, d, 1);
        accudo_account::transfer(b, d, 1);
        accudo_account::transfer(c, d, 1);
    }
}

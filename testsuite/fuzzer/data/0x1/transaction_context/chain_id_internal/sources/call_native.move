module poc::chain_id_internal {
    use accudo_framework::transaction_context;

    public entry fun main(_owner:&signer) {
        let _id = transaction_context::chain_id();
    }

    #[test(owner=@0x123)]
    #[expected_failure(abort_code=196609, location = accudo_framework::transaction_context)]
    fun a(owner:&signer){
        main(owner);
    }
}

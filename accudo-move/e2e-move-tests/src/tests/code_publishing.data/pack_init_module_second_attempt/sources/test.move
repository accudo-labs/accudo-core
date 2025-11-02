module 0xcafe::test {
    use accudo_framework::coin::{Self, Coin};
    use accudo_framework::accudo_coin::AccudoCoin;

    struct State has key {
        important_value: u64,
        coins: Coin<AccudoCoin>,
    }

    fun init_module(s: &signer) {
        move_to(s, State {
            important_value: get_value(),
            coins: coin::zero<AccudoCoin>(),
        })
    }

    fun get_value(): u64 {
        2
    }
}

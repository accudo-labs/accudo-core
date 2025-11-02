//:!:>moon
script {
    fun register(account: &signer) {
        accudo_framework::managed_coin::register<MoonCoin::moon_coin::MoonCoin>(account)
    }
}
//<:!:moon

script {
    use accudo_framework::accudo_governance;
    use accudo_framework::coin;
    use accudo_framework::accudo_coin::AccudoCoin;
    use accudo_framework::staking_config;

    fun main(proposal_id: u64) {
        let framework_signer = accudo_governance::resolve(proposal_id, @accudo_framework);
        let one_accudo_coin_with_decimals = 10 ** (coin::decimals<AccudoCoin>() as u64);
        // Change min to 1000 and max to 1M Accudo coins.
        let new_min_stake = 1000 * one_accudo_coin_with_decimals;
        let new_max_stake = 1000000 * one_accudo_coin_with_decimals;
        staking_config::update_required_stake(&framework_signer, new_min_stake, new_max_stake);
    }
}

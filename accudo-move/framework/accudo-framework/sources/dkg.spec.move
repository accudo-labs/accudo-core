spec accudo_framework::dkg {

    spec module {
        use accudo_framework::chain_status;
        invariant [suspendable] chain_status::is_operating() ==> exists<DKGState>(@accudo_framework);
    }

    spec initialize(accudo_framework: &signer) {
        use std::signer;
        let accudo_framework_addr = signer::address_of(accudo_framework);
        aborts_if accudo_framework_addr != @accudo_framework;
    }

    spec start(
        dealer_epoch: u64,
        randomness_config: RandomnessConfig,
        dealer_validator_set: vector<ValidatorConsensusInfo>,
        target_validator_set: vector<ValidatorConsensusInfo>,
    ) {
        aborts_if !exists<DKGState>(@accudo_framework);
        aborts_if !exists<timestamp::CurrentTimeMicroseconds>(@accudo_framework);
    }

    spec finish(transcript: vector<u8>) {
        use std::option;
        requires exists<DKGState>(@accudo_framework);
        requires option::is_some(global<DKGState>(@accudo_framework).in_progress);
        aborts_if false;
    }

    spec fun has_incomplete_session(): bool {
        if (exists<DKGState>(@accudo_framework)) {
            option::is_some(global<DKGState>(@accudo_framework).in_progress)
        } else {
            false
        }
    }

    spec try_clear_incomplete_session(fx: &signer) {
        use std::signer;
        let addr = signer::address_of(fx);
        aborts_if addr != @accudo_framework;
    }

    spec incomplete_session(): Option<DKGSessionState> {
        aborts_if false;
    }
}

spec accudo_framework::gas_schedule {
    /// <high-level-req>
    /// No.: 1
    /// Requirement: During genesis, the Accudo framework account should be assigned the gas schedule resource.
    /// Criticality: Medium
    /// Implementation: The gas_schedule::initialize function calls the assert_accudo_framework function to ensure that
    /// the signer is the accudo_framework and then assigns the GasScheduleV2 resource to it.
    /// Enforcement: Formally verified via [high-level-req-1](initialize).
    ///
    /// No.: 2
    /// Requirement: Only the Accudo framework account should be allowed to update the gas schedule resource.
    /// Criticality: Critical
    /// Implementation: The gas_schedule::set_gas_schedule function calls the assert_accudo_framework function to ensure
    /// that the signer is the accudo framework account.
    /// Enforcement: Formally verified via [high-level-req-2](set_gas_schedule).
    ///
    /// No.: 3
    /// Requirement: Only valid gas schedule should be allowed for initialization and update.
    /// Criticality: Medium
    /// Implementation: The initialize and set_gas_schedule functions ensures that the gas_schedule_blob is not empty.
    /// Enforcement: Formally verified via [high-level-req-3.3](initialize) and [high-level-req-3.2](set_gas_schedule).
    ///
    /// No.: 4
    /// Requirement: Only a gas schedule with the feature version greater or equal than the current feature version is
    /// allowed to be provided when performing an update operation.
    /// Criticality: Medium
    /// Implementation: The set_gas_schedule function validates the feature_version of the new_gas_schedule by ensuring
    /// that it is greater or equal than the current gas_schedule.feature_version.
    /// Enforcement: Formally verified via [high-level-req-4](set_gas_schedule).
    /// </high-level-req>
    ///
    spec module {
        pragma verify = true;
        pragma aborts_if_is_strict;
    }

    spec initialize(accudo_framework: &signer, gas_schedule_blob: vector<u8>) {
        use std::signer;

        let addr = signer::address_of(accudo_framework);
        /// [high-level-req-1]
        include system_addresses::AbortsIfNotAccudoFramework{ account: accudo_framework };
        /// [high-level-req-3.3]
        aborts_if len(gas_schedule_blob) == 0;
        aborts_if exists<GasScheduleV2>(addr);
        ensures exists<GasScheduleV2>(addr);
    }

    spec set_gas_schedule(accudo_framework: &signer, gas_schedule_blob: vector<u8>) {
        use std::signer;
        use accudo_framework::util;
        use accudo_framework::coin::CoinInfo;
        use accudo_framework::accudo_coin::AccudoCoin;
        use accudo_framework::staking_config;
        use accudo_framework::chain_status;

        // TODO: set because of timeout (property proved)
        pragma verify_duration_estimate = 600;
        requires exists<CoinInfo<AccudoCoin>>(@accudo_framework);
        requires chain_status::is_genesis();
        include staking_config::StakingRewardsConfigRequirement;

        /// [high-level-req-2]
        include system_addresses::AbortsIfNotAccudoFramework{ account: accudo_framework };
        /// [high-level-req-3.2]
        aborts_if len(gas_schedule_blob) == 0;
        let new_gas_schedule = util::spec_from_bytes<GasScheduleV2>(gas_schedule_blob);
        let gas_schedule = global<GasScheduleV2>(@accudo_framework);
        /// [high-level-req-4]
        aborts_if exists<GasScheduleV2>(@accudo_framework) && new_gas_schedule.feature_version < gas_schedule.feature_version;
        ensures exists<GasScheduleV2>(signer::address_of(accudo_framework));
        ensures global<GasScheduleV2>(@accudo_framework) == new_gas_schedule;
    }

    spec set_storage_gas_config(accudo_framework: &signer, config: StorageGasConfig) {
        use accudo_framework::coin::CoinInfo;
        use accudo_framework::accudo_coin::AccudoCoin;
        use accudo_framework::staking_config;

        // TODO: set because of timeout (property proved).
        pragma verify_duration_estimate = 600;
        requires exists<CoinInfo<AccudoCoin>>(@accudo_framework);
        include system_addresses::AbortsIfNotAccudoFramework{ account: accudo_framework };
        include staking_config::StakingRewardsConfigRequirement;
        aborts_if !exists<StorageGasConfig>(@accudo_framework);
        ensures global<StorageGasConfig>(@accudo_framework) == config;
    }

    spec set_for_next_epoch(accudo_framework: &signer, gas_schedule_blob: vector<u8>) {
        use accudo_framework::util;

        include system_addresses::AbortsIfNotAccudoFramework{ account: accudo_framework };
        include config_buffer::SetForNextEpochAbortsIf {
            account: accudo_framework,
            config: gas_schedule_blob
        };
        let new_gas_schedule = util::spec_from_bytes<GasScheduleV2>(gas_schedule_blob);
        let cur_gas_schedule = global<GasScheduleV2>(@accudo_framework);
        aborts_if exists<GasScheduleV2>(@accudo_framework) && new_gas_schedule.feature_version < cur_gas_schedule.feature_version;
    }

    spec set_for_next_epoch_check_hash(accudo_framework: &signer, old_gas_schedule_hash: vector<u8>, new_gas_schedule_blob: vector<u8>) {
        use accudo_std::accudo_hash;
        use std::bcs;
        use std::features;
        use accudo_framework::util;

        include system_addresses::AbortsIfNotAccudoFramework{ account: accudo_framework };
        include config_buffer::SetForNextEpochAbortsIf {
            account: accudo_framework,
            config: new_gas_schedule_blob
        };
        let new_gas_schedule = util::spec_from_bytes<GasScheduleV2>(new_gas_schedule_blob);
        let cur_gas_schedule = global<GasScheduleV2>(@accudo_framework);
        aborts_if exists<GasScheduleV2>(@accudo_framework) && new_gas_schedule.feature_version < cur_gas_schedule.feature_version;
        aborts_if exists<GasScheduleV2>(@accudo_framework) && (!features::spec_sha_512_and_ripemd_160_enabled() || accudo_hash::spec_sha3_512_internal(bcs::serialize(cur_gas_schedule)) != old_gas_schedule_hash);
    }

    spec on_new_epoch(framework: &signer) {
        requires @accudo_framework == std::signer::address_of(framework);
        include config_buffer::OnNewEpochRequirement<GasScheduleV2>;
        aborts_if false;
    }

    spec set_storage_gas_config(accudo_framework: &signer, config: storage_gas::StorageGasConfig) {
        include system_addresses::AbortsIfNotAccudoFramework{ account: accudo_framework };
        aborts_if !exists<storage_gas::StorageGasConfig>(@accudo_framework);
    }

    spec set_storage_gas_config_for_next_epoch(accudo_framework: &signer, config: storage_gas::StorageGasConfig) {
        include system_addresses::AbortsIfNotAccudoFramework{ account: accudo_framework };
        aborts_if !exists<storage_gas::StorageGasConfig>(@accudo_framework);
    }
}

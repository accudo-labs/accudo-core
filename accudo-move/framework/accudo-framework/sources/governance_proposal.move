/// Define the GovernanceProposal that will be used as part of on-chain governance by AccudoGovernance.
///
/// This is separate from the AccudoGovernance module to avoid circular dependency between AccudoGovernance and Stake.
module accudo_framework::governance_proposal {
    friend accudo_framework::accudo_governance;

    struct GovernanceProposal has store, drop {}

    /// Create and return a GovernanceProposal resource. Can only be called by AccudoGovernance
    public(friend) fun create_proposal(): GovernanceProposal {
        GovernanceProposal {}
    }

    /// Useful for AccudoGovernance to create an empty proposal as proof.
    public(friend) fun create_empty_proposal(): GovernanceProposal {
        create_proposal()
    }

    #[test_only]
    public fun create_test_proposal(): GovernanceProposal {
        create_empty_proposal()
    }
}

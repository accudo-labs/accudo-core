/// Provides a common place for exporting `create_signer` across the Accudo Framework.
///
/// To use create_signer, add the module below, such that:
/// `friend accudo_framework::friend_wants_create_signer`
/// where `friend_wants_create_signer` is the module that needs `create_signer`.
///
/// Note, that this is only available within the Accudo Framework.
///
/// This exists to make auditing straight forward and to limit the need to depend
/// on account to have access to this.
module accudo_framework::create_signer {
    friend accudo_framework::account;
    friend accudo_framework::accudo_account;
    friend accudo_framework::coin;
    friend accudo_framework::fungible_asset;
    friend accudo_framework::genesis;
    friend accudo_framework::account_abstraction;
    friend accudo_framework::multisig_account;
    friend accudo_framework::object;
    friend accudo_framework::permissioned_signer;
    friend accudo_framework::transaction_validation;

    public(friend) native fun create_signer(addr: address): signer;
}

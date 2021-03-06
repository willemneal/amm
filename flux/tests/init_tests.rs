use near_sdk_sim::to_yocto;
mod test_utils;
use test_utils::*;
#[test]
fn test_contract_initiation() {
    let (master_account, amm, token, alice, bob, carol) = init(to_yocto("1"), "carol".to_string());
    let owner_balance = get_balance(&token, alice.account_id());
    assert_eq!(owner_balance, to_yocto("1"));
}

#[test]
#[should_panic(expected = "The account ID is invalid")]
fn test_contract_initiation_invalid_account_id() {
    init(to_yocto("100"), "caro{}l".to_string());
}
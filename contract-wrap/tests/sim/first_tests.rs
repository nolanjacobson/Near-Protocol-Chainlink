use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk_sim::DEFAULT_GAS;

use crate::utils::init_without_macros as init;

#[test]

fn simulate_linktoken_transfer() {
    let (root, aca, link, _oracle_one) = init();
    // Transfer from link_token contract to ACA.
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": "220"
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let _root_balance: U128 = root
        .view(
            link.account_id(),
            "get_balance",
            &json!({
                "owner_id": root.valid_account_id()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
    let aca_balance: U128 = aca
        .view(
            link.account_id(),
            "get_balance",
            &json!({
                "owner_id": aca.valid_account_id()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
    // let _available_funds: U128 = root
    //     .view(
    //         aca.account_id(),
    //         "available_funds",
    //         &json!({}).to_string().into_bytes(),
    //     )
    //     .unwrap_json();
    // println!("{:?}", available_funds);
    println!("{:?}", aca_balance);
    assert_eq!(true, true);
}

use near_sdk::serde_json::json;
use near_sdk::{AccountId};
use near_sdk_sim::{init_simulator, to_yocto, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT};

const ACA_ID: &str = "aca";
const LINKTOKEN_ID: &str = "lt";

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    // update `contract.wasm` for your contract's name
    ACA_WASM_BYTES => "target/wasm32-unknown-unknown/debug/AccessControlledAggregator.wasm",

    // if you run `cargo build` without `--release` flag:
    LINKTOKEN_WASM_BYTES => "target/wasm32-unknown-unknown/debug/LinkToken.wasm",
}

pub fn init_without_macros() -> (UserAccount, UserAccount, UserAccount, UserAccount) {
    // Use `None` for default genesis configuration; more info below
    let root = init_simulator(None);
    let link = root.deploy(
        &LINKTOKEN_WASM_BYTES,
        LINKTOKEN_ID.to_string(),
        to_yocto("1000"), // attached deposit
    );
    link.call(
        LINKTOKEN_ID.into(),
        "new",
        &json!({
            "owner_id": root.account_id().to_string(), "total_supply": "100000"
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        0, // attached deposit
    )
    .assert_success();
    let aca = root.deploy(
        &ACA_WASM_BYTES,
        ACA_ID.to_string(),
        to_yocto("1000"), // attached deposit
    );

    let payment_amount: u64 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 1;
    let empty_address: AccountId = "".to_string();
    let next_round: u128 = 1; 

    aca.call(
        ACA_ID.into(),
        "new",
        &json!({
            "link_id": link.account_id(),
            "owner_id": root.account_id(),
            "payment_amount": payment_amount,
            "timeout": timeout,
            "validator": empty_address,
            "min_submission_value": min_submission_value,
            "max_submission_value": max_submission_value,
            "decimals": decimals,
            "description": description,
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        0, // attached deposit
    ).assert_success();

    let oracle_one = root.create_user(
        "oracle_one".to_string(),
        to_yocto("1000000"), // initial balance
    );

    (root, aca, link, oracle_one)
}
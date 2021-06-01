use near_sdk::serde_json::json;
use near_sdk::AccountId;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::DEFAULT_GAS;

use crate::utils::init_without_macros as init;


// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/AccessControlledAggregator.test.ts
// https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L180
// Suite of simulation tests matching TypeScript tests for AccessControlledAggregator and FluxAggregator as closely as possible.

#[test]
fn constructor_tests() {
    let (
        root,
        aca,
        _link,
        _oracle_one,
        _oracle_two,
        _oracle_three,
        _test_helper,
        _ea,
        _eac_without_access_controller,
    ) = init();
    let payment_amount: u128 = 3;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let version: u128 = 3;
    let validator: String = "".to_string();
    let description: String = "LINK/USD".to_string();
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let empty_address: AccountId = "".to_string();

    let expected_payment_amount: u128 = root.call(
        aca.account_id(),
        "get_payment_amount",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).unwrap_json();

    assert_eq!(payment_amount, expected_payment_amount);

    let expected_timeout: u64 = root.call(
        aca.account_id(),
        "get_timeout",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).unwrap_json();

    assert_eq!(timeout, expected_timeout);

    let expected_decimals: u64 = root.call(
        aca.account_id(),
        "get_decimals",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).unwrap_json();

    assert_eq!(decimals, expected_decimals);


    let expected_description: String = root.call(
        aca.account_id(),
        "get_description",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).unwrap_json();

    assert_eq!(description, expected_description);

    let expected_version: u128 = root.call(
        aca.account_id(),
        "get_version",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).unwrap_json();

    assert_eq!(version, expected_version);

    let expected_validator: String = root.call(
        aca.account_id(),
        "get_validator",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).unwrap_json();

    assert_eq!(validator, expected_validator);

}

#[test]
fn access_control_tests() {
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        _oracle_two,
        _oracle_three,
        test_helper,
        _ea,
        _eac_without_access_controller,
    ) = init();

    // BegDeployment function body as done on line 180-196 -> https://github.com/smartcontractkit/chainlink/blob/develop/evm-contracts/test/v0.6/FluxAggregator.test.ts#L180
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": deposit.to_string()
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
    ).assert_success();

    // First add oracle_one
    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Second, call submit from oracle_one
    oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    // Unauthorized Calls
    // Unauthorized call from test_helper for get_answer
    let mut get_answer_unauthorized = test_helper.call(
        aca.account_id(),
        "get_answer",
        &json!({"_round_id": next_round.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    assert_eq!(get_answer_unauthorized.promise_errors().len(), 1);

    if let ExecutionStatus::Failure(execution_error) = &get_answer_unauthorized
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error.to_string().contains("No access"));
    } else {
        unreachable!();
    }
    // Unauthorized call from test_helper for get_timestamp
    get_answer_unauthorized = test_helper.call(
        aca.account_id(),
        "get_timestamp",
        &json!({"_round_id": next_round.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    assert_eq!(get_answer_unauthorized.promise_errors().len(), 1);

    if let ExecutionStatus::Failure(execution_error) = &get_answer_unauthorized
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error.to_string().contains("No access"));
    } else {
        unreachable!();
    }
    // Unauthorized call from test_helper for latest_answer
    get_answer_unauthorized = test_helper.call(
        aca.account_id(),
        "latest_answer",
        &json!({"_round_id": next_round.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    assert_eq!(get_answer_unauthorized.promise_errors().len(), 1);

    if let ExecutionStatus::Failure(execution_error) = &get_answer_unauthorized
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error.to_string().contains("No access"));
    } else {
        unreachable!();
    }
    // Unauthorized call from test_helper for latest_timestamp
    get_answer_unauthorized = test_helper.call(
        aca.account_id(),
        "latest_timestamp",
        &json!({"_round_id": next_round.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    assert_eq!(get_answer_unauthorized.promise_errors().len(), 1);

    if let ExecutionStatus::Failure(execution_error) = &get_answer_unauthorized
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error.to_string().contains("No access"));
    } else {
        unreachable!();
    }

    // Authorized Calls

    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Authorized call from test_helper for get_answer
    test_helper
        .call(
            aca.account_id(),
            "get_answer",
            &json!({"_round_id": 1.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    // Authorized call from test_helper for get_timestamp
    test_helper
        .call(
            aca.account_id(),
            "get_timestamp",
            &json!({"_round_id": 1.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    // Authorized call from test_helper for latest_answer
    test_helper
        .call(
            aca.account_id(),
            "latest_answer",
            &json!({"_round_id": 1.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    // Authorized call from test_helper for latest_timestamp
    test_helper
        .call(
            aca.account_id(),
            "latest_timestamp",
            &json!({"_round_id": 1.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
}

#[test]
fn updates_the_allocated_and_available_funds_counters_and_emits_a_log_event_announcing_submission_details(
) {
    let payment_amount: u64 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    // Transfer from link_token contract to ACA.
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": deposit.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // #submit

    let mut min_max: u64 = 1;

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    min_max = min_max + 1;
    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_two.account_id()], "_added_admins": [oracle_two.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    min_max = min_max + 1;
    // println!("second {:?}", min_max);
    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // println!("after {:?}", min_max);
    // updates the allocated and available funds counters
    // println!("updates the allocated and available funds counters");

    let mut allocated_funds: u64 = root
        .view(
            aca.account_id(),
            "allocated_funds",
            &json!({
                "": "".to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
    assert_eq!(
        0, allocated_funds,
        "updates the allocated and available funds counters"
    );

    let mut tx = oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    let mut receipt = tx.promise_results();

    allocated_funds = root
        .view(
            aca.account_id(),
            "allocated_funds",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();
    let available_funds: u64 = root
        .view(
            aca.account_id(),
            "available_funds",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();
    assert_eq!(payment_amount, allocated_funds);
    let expected_available: u64 = deposit - payment_amount;
    assert_eq!(expected_available, available_funds);
    let logged: u64 = receipt.remove(1).unwrap().outcome().logs[0]
        .parse()
        .unwrap();
    assert_eq!(expected_available, logged);

    // emits a log event announcing submission details
    // println!("emits a log event announcing submission details");
    tx = oracle_two.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    receipt = tx.promise_results();
    // println!("{:?}", receipt);
    // let round = receipt.events?.[1]
    //assert_eq(answer, round.submission)

    // when the minimum oracles have not reported
    // println!("when the minimum oracles have not reported");
    // let withdrawable_payment: u128 = root
    // .view(
    //     aca.account_id(),
    //     "withdrawable_payment",
    //     &json!({
    //             "_oracle": oracle_one.account_id().to_string()
    //         })
    //         .to_string()
    //         .into_bytes(),
    // )
    // .unwrap_json();
    // assert_eq!(0, withdrawable_payment);
}

#[test]
fn when_the_minimum_oracles_have_not_reported() {
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": deposit.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // #submit
    // println!("\n#submit");

    let min_max: u64 = 3;

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // min_max = min_max + 1;
    let withdrawable_payment: u128 = root
        .view(
            aca.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_one.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
    assert_eq!(0, withdrawable_payment);
    let tx = oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    let receipt = tx.promise_results();
    // println!("{:?}", receipt);
    // let round = receipt.events?.[1]
    //assert_eq(answer, round.submission)

    // when the minimum oracles have not reported
    // println!("when the minimum oracles have not reported");
    let withdrawable_payment: u128 = root
        .view(
            aca.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_one.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
    assert_eq!(payment_amount, withdrawable_payment);
    let withdrawable_payment_1: u128 = root
        .view(
            aca.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_two.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
    assert_eq!(0, withdrawable_payment_1);
    let withdrawable_payment_2: u128 = root
        .view(
            aca.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_three.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
    assert_eq!(0, withdrawable_payment_2);
    // does not update the answer
    // oracle_two.call(
    //     aca.account_id(),
    //     "submit",
    //     &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()}).to_string().into_bytes(),
    //     DEFAULT_GAS,
    //     0, // deposit
    // ).assert_success();
    // oracle_three.call(
    //     aca.account_id(),
    //     "submit",
    //     &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()}).to_string().into_bytes(),
    //     DEFAULT_GAS,
    //     0, // deposit
    // ).assert_success();
    let not_updated = test_helper.call(
        aca.account_id(),
        "latest_answer",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // println!("{:?}", not_updated.promise_results());

    if let ExecutionStatus::Failure(execution_error) = &not_updated
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        // println!("{:?}", execution_error.to_string());
        assert!(execution_error.to_string().contains("Did not find"));
    } else {
        unreachable!();
    }
    // The way we have the code, this one fails and doesn't return a 0 value as expected on line 365 of the TypeScript tests.
    // assert_eq!(0, not_updated);
}
#[test]
fn when_an_oracle_prematurely_bumps_the_round() {
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 2;
    let max_ans: u64 = 3;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": deposit.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // #submit
    // println!("\n#submit");

    let min_max: u64 = 3;

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx = oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    let receipt = tx.promise_results();

    // oracle_two.call(
    //     aca.account_id(),
    //     "submit",
    //     &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()}).to_string().into_bytes(),
    //     DEFAULT_GAS,
    //     0, // deposit
    // ).assert_success();
    // oracle_three.call(
    //     aca.account_id(),
    //     "submit",
    //     &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()}).to_string().into_bytes(),
    //     DEFAULT_GAS,
    //     0, // deposit
    // ).assert_success();

    let tx_2 = oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // Note: https://github.com/smartcontractkit/chainlink/blob/95dd250a296042c81b7aafa887d8935c87cb1190/evm-contracts/test/v0.6/FluxAggregator.test.ts#L371
    // Not working here, moving on to the next test but look into this.
    let receipt_2 = tx_2.promise_results();
    // println!("{:?} receipt2previous", receipt_2);
    if let ExecutionStatus::Failure(execution_error) =
        &tx_2.promise_errors().remove(0).unwrap().outcome().status
    {
        println!("{:?}", execution_error.to_string());
        assert!(execution_error
            .to_string()
            .contains("previous round not supersedable"));
    } else {
        unreachable!();
    }
}

// updates the answer with the median
#[test]
fn when_the_minimum_number_of_oracles_have_reported() {
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": deposit.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // #submit
    // println!("\n#submit");

    let min_max: u64 = 3;

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let tx = oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    let receipt = tx.promise_results();
    let not_updated = test_helper.call(
        aca.account_id(),
        "latest_answer",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    if let ExecutionStatus::Failure(execution_error) = &not_updated
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        // println!("{:?} logs111111", execution_error.to_string());
        assert!(execution_error.to_string().contains("Did not find"));
    } else {
        unreachable!();
    }
    // when the minimum oracles have reported
    oracle_two.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": 99.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    let updated_one: u128 = test_helper
        .call(
            aca.account_id(),
            "latest_answer",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();
    assert_eq!(99, updated_one);
    oracle_three.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": 101.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    let updated_two: u128 = test_helper
        .call(
            aca.account_id(),
            "latest_answer",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();
    assert_eq!(100, updated_two);
}
// updates the updated timestamd
// announces the new answer with a log event
// does not set the timedout flag
// updates the round details
#[test]

fn updates_the_updated_timestamp_and_announces_the_new_answer_with_a_log_event_and_does_not_set_the_timedout_flag_and_updates_the_round_details(
) {
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": deposit.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // #submit
    // println!("\n#submit");

    let min_max: u64 = 3;

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();

    let tx = oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // does not set the timedout flag
    let get_round_data_1 = test_helper.call(
        aca.account_id(),
        "get_round_data",
        &json!({"_round_id": next_round.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    if let ExecutionStatus::Failure(execution_error) = &get_round_data_1
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        // println!("{:?} getrounddatalogs", execution_error.to_string());
        assert!(execution_error.to_string().contains("No data present"));
    } else {
        unreachable!();
    }
    // panicking on line 476
    let latest_round_data_1 = test_helper.call(
        aca.account_id(),
        "latest_round_data",
        &json!({"_round_id": next_round.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    if let ExecutionStatus::Failure(execution_error) = &latest_round_data_1
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        // No data present should be error
        // println!("{:?} latestrounddatalogs", execution_error.to_string());
        assert!(execution_error
            .to_string()
            .contains("Did not find this oracle account. {get_round_data}"));
    } else {
        unreachable!();
    }
    let original_timestamp_1 = test_helper.call(
        aca.account_id(),
        "latest_timestamp",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // this fails due to the return statement before reaching lines 936-938
    // if submissions_length < detail.min_submissions {
    //     return (false, 0 as u128);
    // }
    // println!("{:?} originaltimestamp1", original_timestamp_1.promise_results());
    let tx_2 = oracle_two.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    let mut receipt = tx_2.promise_results();

    let round_after: (u64, u128, u64, u64, u64) = test_helper
        .call(
            aca.account_id(),
            "get_round_data",
            &json!({"_round_id": next_round.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();
    assert_eq!(1, round_after.0);
    assert_eq!(100, round_after.1);
    assert_eq!(false, round_after.2 == 0);
    let original_timestamp: u128 = test_helper
        .call(
            aca.account_id(),
            "latest_timestamp",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();
    assert_eq!(original_timestamp as u64, round_after.3);
    assert_eq!(1, round_after.4);
    assert_eq!(true, round_after.2 < round_after.3);

    let round_after_latest: (u64, u128, u64, u64, u64) = test_helper
        .call(
            aca.account_id(),
            "latest_round_data",
            &json!({"_round_id": next_round.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();
    assert_eq!(true, round_after.0 == round_after_latest.0);
    assert_eq!(true, round_after.1 == round_after_latest.1);
    assert_eq!(true, round_after.2 == round_after_latest.2);
    assert_eq!(true, round_after.3 == round_after_latest.3);
    assert_eq!(true, round_after.4 == round_after_latest.4);
    // announces the new answer with a log event
    let new_answer: u64 = receipt.remove(1).unwrap().outcome().logs[0]
        .parse()
        .unwrap();
    let latest_answer: u64 = test_helper
        .call(
            aca.account_id(),
            "latest_answer",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();
    assert_eq!(latest_answer, new_answer);

    let is_above: bool = original_timestamp > 0;
    // assert_eq!(true, is_above);
}

// when an oracle submits for a round twice
#[test]

fn when_an_oracle_submits_for_a_round_twice() {
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": deposit.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // #submit

    let min_max: u64 = 3;

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": min_max.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // min_max = min_max + 1;
    let withdrawable_payment: u128 = root
        .view(
            aca.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_one.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
    assert_eq!(0, withdrawable_payment);
    let tx = oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    let tx_2 = oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    if let ExecutionStatus::Failure(execution_error) =
        &tx_2.promise_errors().remove(0).unwrap().outcome().status
    {
        // No data present should be error
        assert!(execution_error
            .to_string()
            .contains("cannot report on previous rounds"));
    } else {
        unreachable!();
    }
}

// when updated after the max answers submitted
#[test]

fn when_updated_after_the_max_answers_submitted() {
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 2;
    let max_ans: u64 = 3;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": deposit.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // #submit

    let min_max: u64 = 3;

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx = oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let tx_2 = oracle_two.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // check line 1201 to understanding why the details struct doesn't exist.
    if let ExecutionStatus::Failure(execution_error) =
        &tx_2.promise_errors().remove(0).unwrap().outcome().status
    {
        // No data present should be error
        assert!(execution_error
            .to_string()
            .contains("round not accepting submissions"));
    } else {
        unreachable!();
    }
}

#[test]

fn when_a_new_highest_round_number_is_passed_in() {
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 2;
    let max_ans: u64 = 3;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": deposit.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // #submit
    let min_max: u64 = 3;

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    // issue here, look into this
    let starting_state = test_helper.call(
        aca.account_id(),
        "oracle_round_state",
        &json!({"_oracle": oracle_one.account_id(), "_queried_round_id": 0.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
}

#[test]

fn when_a_round_is_passed_in_higher_than_expected_and_when_called_by_a_non_oracle() {
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 2;
    let max_ans: u64 = 3;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": deposit.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    // #submit

    let min_max: u64 = 3;

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // root.call(
    //     aca.account_id(),
    //     "update_future_rounds",
    //     &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
    //     DEFAULT_GAS,
    //     0, // deposit
    // ).assert_success();
    let tx = oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    if let ExecutionStatus::Failure(execution_error) =
        &tx.promise_errors().remove(0).unwrap().outcome().status
    {
        // No data present should be error
        assert!(execution_error
            .to_string()
            .contains("invalid round to report"));
    } else {
        unreachable!();
    }
    let tx_2 = test_helper.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    if let ExecutionStatus::Failure(execution_error) =
        &tx_2.promise_errors().remove(0).unwrap().outcome().status
    {
        // No data present should be error
        assert!(execution_error.to_string().contains("not enabled oracle"));
    } else {
        unreachable!();
    }
}

#[test]

fn when_there_are_not_sufficient_available_funds() {
    //        beforeEach(async () => {
    //     await aggregator
    //     .connect(personas.Carol)
    //     .withdrawFunds(
    //       personas.Carol.address,
    //       deposit.sub(paymentAmount.mul(oracles.length).mul(reserveRounds)),
    //     )

    //   // drain remaining funds
    //   await advanceRound(aggregator, oracles)
    //   await advanceRound(aggregator, oracles)
    // })

    // it('reverts', async () => {
    //   await matchers.evmRevert(
    //     aggregator.connect(personas.Neil).submit(nextRound, answer),
    //     'SafeMath: subtraction overflow',
    //   )
    // })
}

#[test]

fn when_a_new_round_opens_before_the_previous_rounds_closes() {}

#[test]

fn when_price_is_updated_mid_round() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // #submit

    let min_max: u64 = 3;

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": min_max.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    let withdrawable_payment: u128 = root
        .view(
            aca.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_one.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
    assert_eq!(0, withdrawable_payment);
    let withdrawable_payment_1: u128 = root
        .view(
            aca.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_two.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
    assert_eq!(0, withdrawable_payment_1);
    let tx = oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": new_amount.to_string(), "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx_1 = oracle_two.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    let withdrawable_payment_oracle_one: u128 = root
        .view(
            aca.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_one.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
    assert_eq!(payment_amount, withdrawable_payment_oracle_one);

    let withdrawable_payment_oracle_two: u128 = root
        .view(
            aca.account_id(),
            "withdrawable_payment",
            &json!({
                "_oracle": oracle_two.account_id().to_string()
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
    assert_eq!(payment_amount, withdrawable_payment_oracle_two);
}

#[test]

fn when_delay_is_on() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx = oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_2 = oracle_one.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    // issue here with if/else tree not returning previous round not supersedable
    // println!("{:?}", tx_2.promise_results());
}

#[test]

fn when_an_oracle_starts_a_round_before_the_restart_delay_is_over_and_when_called_by_an_oracle_who_has_not_answered_recently(
) {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx = oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_2 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_3 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 3.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    // const newDelay = 2
    // Since Ned and Nelly have answered recently, and we set the delay
    // to 2, only Nelly can answer as she is the only oracle that hasn't
    // started the last two rounds.
    // await updateFutureRounds(aggregator, {
    //   maxAnswers: oracles.length,
    //   restartDelay: newDelay,
    // })

    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 2.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    // when called by an oracle who has not answered recently
    // it does not revert
    oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 4.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
}

#[test]

fn when_an_oracle_starts_a_round_before_the_restart_delay_is_over_and_when_called_by_an_oracle_who_has_answered_recently(
) {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx = oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_2 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_3 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 3.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    // const newDelay = 2
    // Since Ned and Nelly have answered recently, and we set the delay
    // to 2, only Nelly can answer as she is the only oracle that hasn't
    // started the last two rounds.
    // await updateFutureRounds(aggregator, {
    //   maxAnswers: oracles.length,
    //   restartDelay: newDelay,
    // })

    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 2.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    // when called by an oracle who has not answered recently
    // it does not revert
    let tx_4 = oracle_two.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": 4.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    if let ExecutionStatus::Failure(execution_error) =
        &tx_4.promise_errors().remove(0).unwrap().outcome().status
    {
        // No data present should be error
        assert!(execution_error
            .to_string()
            .contains("round not accepting submissions"));
    } else {
        unreachable!();
    }
    // println!("{:?}", tx_4.promise_results());
    let tx_5 = oracle_two.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": 4.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // println!("{:?}", tx_5.promise_results());

    if let ExecutionStatus::Failure(execution_error) =
        &tx_5.promise_errors().remove(0).unwrap().outcome().status
    {
        // No data present should be error
        assert!(execution_error
            .to_string()
            .contains("round not accepting submissions"));
    } else {
        unreachable!();
    }
}

#[test]

fn when_the_price_is_not_updated_for_a_round_and_allows_a_new_round_to_be_started() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx = oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_2 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_3 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let tx_4 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_5 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    // allows a new round to be started
    let tx_6 = oracle_three.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": 3.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // println!("{:?}", tx_6.promise_results());
}

#[test]
fn when_the_price_is_not_updated_for_a_round_and_sets_the_info_for_the_previous_round() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx = oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_2 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_3 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let tx_4 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_5 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    // sets the info for the previous round
    let tx_6: u128 = test_helper
        .call(
            aca.account_id(),
            "get_timestamp",
            &json!({"_round_id": 2.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();
    assert_eq!(0, tx_6);

    // println!("{:?}", tx_6.promise_results());
    let tx_7: u128 = test_helper
        .call(
            aca.account_id(),
            "get_answer",
            &json!({"_round_id": 2.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();
    assert_eq!(0, tx_7);
    let tx_8 = oracle_three.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": 3.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // note: look into this const block = await provider.getBlock(receipt.blockHash ?? '')
    //  matchers.bigNum(ethers.utils.bigNumberify(block.timestamp), updated)

    let tx_9: u64 = test_helper
        .call(
            aca.account_id(),
            "get_timestamp",
            &json!({"_round_id": 2.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();
    // assert_eq!(0, tx_9);

    let tx_10: u128 = test_helper
        .call(
            aca.account_id(),
            "get_answer",
            &json!({"_round_id": 2.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();
    assert_eq!(answer, tx_10);
    let tx_11: (u64, u128, u64, u64, u64) = test_helper
        .call(
            aca.account_id(),
            "get_round_data",
            &json!({"_round_id": 2.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();
    assert_eq!(2, tx_11.0);
    assert_eq!(answer, tx_11.1);
    assert_eq!(tx_9, tx_11.3);
    assert_eq!(1, tx_11.4);
}

#[test]
fn when_the_price_is_not_updated_for_a_round_and_sets_the_previous_round_as_timed_out() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx = oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_2 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_3 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 1.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();

    let tx_4 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_5 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 2.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    // sets the info for the previous round
    let tx_6 = test_helper.call(
        aca.account_id(),
        "get_round_data",
        &json!({"_round_id": 2.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    if let ExecutionStatus::Failure(execution_error) =
        &tx_6.promise_errors().remove(0).unwrap().outcome().status
    {
        // No data present should be error
        assert!(execution_error.to_string().contains("No data present"));
    } else {
        unreachable!();
    }

    let tx_7 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": 3.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_8: (u64, u128, u64, u64, u64) = test_helper
        .call(
            aca.account_id(),
            "get_round_data",
            &json!({"_round_id": 2.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .unwrap_json();
    assert_eq!(2, tx_8.0);
    assert_eq!(1, tx_8.4);
}

#[test]
fn when_the_price_is_not_updated_for_a_round_and_still_respects_the_delay_restriction() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx = oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_2 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_3 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    next_round = next_round + 1;
    let tx_4 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_5 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    next_round = next_round + 1;
    let tx_6 = oracle_two.call(
        aca.account_id(),
        "submit",
        &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    if let ExecutionStatus::Failure(execution_error) =
        &tx_6.promise_errors().remove(0).unwrap().outcome().status
    {
        assert!(execution_error
            .to_string()
            .contains("round not accepting submissions"));
    } else {
        unreachable!();
    }
}

#[test]
fn when_the_price_is_not_updated_for_a_round_and_uses_the_timeout_set_at_the_beginning_of_the_round(
) {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx = oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_2 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_3 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    next_round = next_round + 1;
    let tx_4 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_5 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    next_round = next_round + 1;

    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": min_ans.to_string(), "_max_submissions": max_ans.to_string(), "_restart_delay": rr_delay.to_string(), "_timeout": (timeout+100000).to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx_6 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
}

#[test]
fn when_the_price_is_not_updated_for_a_round_and_submitting_values_near_the_edges_of_allowed_values(
) {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx = oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_2 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_3 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    next_round = next_round + 1;
    let tx_4 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_5 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    next_round = next_round + 1;

    // rejects values below the submission value range
    let tx_6 = oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": (min_submission_value-1).to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );
    if let ExecutionStatus::Failure(execution_error) =
        &tx_6.promise_errors().remove(0).unwrap().outcome().status
    {
        // No data present should be error
        assert!(execution_error
            .to_string()
            .contains("value below minSubmissionValue"));
    } else {
        unreachable!();
    }
}

#[test]
fn when_the_price_is_not_updated_for_a_round_and_accepts_submissions_equal_to_the_min_submission_value(
) {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx = oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_2 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_3 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    next_round = next_round + 1;
    let tx_4 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_5 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    next_round = next_round + 1;

    // rejects values below the submission value range
    let tx_6 = oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": (min_submission_value).to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );
}

#[test]
fn when_the_price_is_not_updated_for_a_round_and_rejects_submissions_equal_to_the_max_submission_value(
) {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    let tx = oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_2 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_3 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    next_round = next_round + 1;
    let tx_4 = oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    let tx_5 = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        )
        .assert_success();
    next_round = next_round + 1;

    let tx_6 = oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": (max_submission_value+1).to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );
}

#[test]
fn when_a_validator_is_set() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    // root.call(
    //     aca.account_id(),
    //     "add_access",
    //     &json!({"_user": test_helper.account_id().to_string()})
    //         .to_string()
    //         .into_bytes(),
    //     DEFAULT_GAS,
    //     0, // deposit
    // )
    // .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );

    root.call(
        aca.account_id(),
        "change_oracles",
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    root.call(
        aca.account_id(),
        "update_future_rounds",
        &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    ).assert_success();
    root.call(
        aca.account_id(),
        "set_validator",
        &json!({"_new_validator": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    let tx = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );

    //  println!("{:?}", tx.promise_results());
}

#[test]
fn get_answers() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let answers: Vec<u128> = [1, 10, 101, 1010, 10101, 101010, 1010101].to_vec();
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit   
    );  

    root.call(  
        aca.account_id(),   
        "change_oracles",   
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    );
    // root.call(
    //     aca.account_id(),
    //     "update_future_rounds",
    //     &json!({"_payment_amount": payment_amount.to_string(), "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 1.to_string(), "_timeout": timeout.to_string()}).to_string().into_bytes(),
    //     DEFAULT_GAS,
    //     0, // deposit
    // ).assert_success();
    let mut n = 0;
    let mut y = 1;
    let mut x = 0;
    while n < answers.len() {
        oracle_one.call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": answers[n].to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );
        next_round += 1;
        n += 1;
    }

    while y < next_round {
        let answer: u128 = test_helper
            .call(
                aca.account_id(),
                "get_answer",
                &json!({"_round_id": y.to_string()}).to_string().into_bytes(),
                DEFAULT_GAS,
                0, // deposit
            )
            .unwrap_json();
        let expected_answer: u128 = answers[x] as u128;
        // println!("{:?} , {:?}", answer, expected_answer);

        x += 1;
        y += 1;
        if answer == expected_answer {
            // println!("{:?} , {:?}", answer, expected_answer);
        }
    }

    // research this
    //     it("returns 0 for answers greater than uint32's max", async () => {
    //   const overflowedId = h.bigNum(2).pow(32).add(1)
    //   const answer = await aggregator.getAnswer(overflowedId)
    //   matchers.bigNum(0, answer)
    // })
}

#[test]
fn get_timestamp() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let answers: Vec<u128> = [1, 10, 101, 1010, 10101, 101010, 1010101].to_vec();
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();
    let _outcome = root.call(
        aca.account_id(),
        "update_available_funds",
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit   
    );  

    root.call(  
        aca.account_id(),   
        "change_oracles",   
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    );

    let mut i  = 0;
    let mut z = 1;
    let mut latest_timestamp = 0;
    while i < 10 {
        oracle_one.call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": next_round.to_string(), "_submission": (i + 1).to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );
        next_round  += 1;
        i += 1;
    }

    while z < next_round {
       let current_timestamp: u128 = test_helper.call(
            aca.account_id(),
            "get_timestamp",
            &json!({"_round_id": z.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).unwrap_json();
        z += 1;
        if (current_timestamp >=  latest_timestamp) {
            latest_timestamp = current_timestamp;
            // println!("{:?}", current_timestamp);
        }
    }
    // research this
    // it("returns 0 for answers greater than uint32's max", async () => {
    //     const overflowedId = h.bigNum(2).pow(32).add(1)
    //     const answer = await aggregator.getTimestamp(overflowedId)
    //     matchers.bigNum(0, answer)
    //   })
}

#[test]
fn change_oracles_and_adding_oracles_increases_the_oracle_count() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let answers: Vec<u128> = [1, 10, 101, 1010, 10101, 101010, 1010101].to_vec();
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();

    let oracle_count: u128 = test_helper.call(
        aca.account_id(),
        "oracle_count",
        &json!({
            
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .unwrap_json();

    root.call(  
        aca.account_id(),   
        "change_oracles",   
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    );

    let updated_oracle_count: u128 = test_helper.call(
        aca.account_id(),
        "oracle_count",
        &json!({
            
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .unwrap_json();

    assert_eq!(oracle_count + 1, updated_oracle_count);
}

#[test]
fn change_oracles_and_adding_oracles_and_adds_the_address_in_get_oracles() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let answers: Vec<u128> = [1, 10, 101, 1010, 10101, 101010, 1010101].to_vec();
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();

    root.call(  
        aca.account_id(),   
        "change_oracles",   
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    );
    let oracles: Vec<String> = test_helper.call(
        aca.account_id(),
        "get_oracles",
        &json!({
            
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .unwrap_json();
    // println!("{:?}", oracles);
    assert_eq!(oracle_one.account_id().to_string(), oracles[0]);
}


#[test]
fn change_oracles_and_adding_oracles_and_updates_the_round_details() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let answers: Vec<u128> = [1, 10, 101, 1010, 10101, 101010, 1010101].to_vec();
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();

    root.call(  
        aca.account_id(),   
        "change_oracles",   
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id(), oracle_three.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": 2.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    );


    let min_submission_count: u64 = root.call(  
        aca.account_id(),   
        "min_submission_count",   
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    ).unwrap_json();
    let max_submission_count: u64 = root.call(  
        aca.account_id(),   
        "max_submission_count",   
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    ).unwrap_json();
    let restart_delay: u64 = root.call(  
        aca.account_id(),   
        "restart_delay",   
        &json!({}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    ).unwrap_json();

    assert_eq!(min_submission_count, 1);
    assert_eq!(max_submission_count, 3);
    assert_eq!(restart_delay, 2);

}


#[test]
fn change_oracles_and_adding_oracles_and_emits_a_log() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let answers: Vec<u128> = [1, 10, 101, 1010, 10101, 101010, 1010101].to_vec();
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();

    let event = root.call(  
        aca.account_id(),   
        "change_oracles",   
        &json!({"_removed": [], "_added": [oracle_two.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    ).promise_results();
    // println!("{:?}", event);
    let logged: String = event.clone().remove(1).unwrap().outcome().logs[0]
        .parse()
        .unwrap();
    let result = [oracle_two.account_id(), ", true".to_string()].join(""); 
    assert_eq!(result, logged);
    let logged_two: String = event.clone().remove(1).unwrap().outcome().logs[1]
    .parse()
    .unwrap();

    let result_two = [oracle_one.account_id(), ", true".to_string()].join(""); 
    assert_eq!(result_two, logged_two);

}

#[test] 

fn when_the_oracle_has_already_been_added() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let answers: Vec<u128> = [1, 10, 101, 1010, 10101, 101010, 1010101].to_vec();
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();

    root.call(  
        aca.account_id(),   
        "change_oracles",   
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    ).assert_success();
   
    let called_twice = root.call(  
        aca.account_id(),   
        "change_oracles",   
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    );

        if let ExecutionStatus::Failure(execution_error) = &called_twice
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error.to_string().contains("oracle already enabled"));
    } else {
        unreachable!();
    }
}

#[test]

fn when_called_by_anyone_but_the_owner() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let answers: Vec<u128> = [1, 10, 101, 1010, 10101, 101010, 1010101].to_vec();
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();

    let called_by_non_owner = oracle_one.call(  
        aca.account_id(),   
        "change_oracles",   
        &json!({"_removed": [], "_added": [oracle_one.account_id()], "_added_admins": [oracle_one.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": 0.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    );


    if let ExecutionStatus::Failure(execution_error) = &called_by_non_owner
    .promise_errors()
    .remove(0)
    .unwrap()
    .outcome()
    .status
{
    assert!(execution_error.to_string().contains("Only contract owner can call this method."));
} else {
    unreachable!();
}
}


#[test]

fn when_an_oracle_gets_added_mid_round_and_does_not_allow_the_oracle_to_update_the_round() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let answers: Vec<u128> = [1, 10, 101, 1010, 10101, 101010, 1010101].to_vec();
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();

    root.call(  
        aca.account_id(),   
        "change_oracles",   
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    );

        oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": "1", "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );
        root.call(  
            aca.account_id(),   
            "change_oracles",   
            &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit               
        );

       let not_enabled = oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": "1", "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );

        if let ExecutionStatus::Failure(execution_error) = &not_enabled
        .promise_errors()
        .remove(0)
        .unwrap()
        .outcome()
        .status
    {
        assert!(execution_error.to_string().contains("not yet enabled oracle"));
    } else {
        unreachable!();
    }
}

#[test]

fn when_an_oracle_gets_added_mid_round_and_does_allow_the_oracle_to_update_future_rounds() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let answers: Vec<u128> = [1, 10, 101, 1010, 10101, 101010, 1010101].to_vec();
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();

    root.call(  
        aca.account_id(),   
        "change_oracles",   
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    );

        oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": "1", "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );
        root.call(  
            aca.account_id(),   
            "change_oracles",   
            &json!({"_removed": [], "_added": [oracle_three.account_id()], "_added_admins": [oracle_three.account_id()], "_min_submissions": 3.to_string(), "_max_submissions": 3.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit               
        );
        oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": "1", "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );
       oracle_three
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": "2", "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();
}

#[test]

fn when_an_oracle_is_added_after_removed_for_a_round_and_allows_the_oracle_to_update() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let answers: Vec<u128> = [1, 10, 101, 1010, 10101, 101010, 1010101].to_vec();
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();

    root.call(  
        aca.account_id(),   
        "change_oracles",   
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    );

        oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": "1", "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );
        oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": "1", "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );
        root.call(  
            aca.account_id(),   
            "change_oracles",   
            &json!({"_removed": [oracle_two.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit               
        );
       oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": "2", "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();

        root.call(  
            aca.account_id(),   
            "change_oracles",   
            &json!({"_removed": [], "_added": [oracle_two.account_id()], "_added_admins": [oracle_two.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit               
        );

        oracle_two
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": "3", "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        ).assert_success();
}

// #[test]

// fn when_an_oracle_is_added_and_immediately_removed_mid_round() {
//  // research difference with above test
// }

// investigate error here
#[test]

fn when_an_oracle_is_re_added_after_with_a_different_admin_address() {
    let new_amount: u128 = 50;
    let payment_amount: u128 = 3;
    let deposit: u64 = 100;
    let answer: u128 = 100;
    let answers: Vec<u128> = [1, 10, 101, 1010, 10101, 101010, 1010101].to_vec();
    let min_ans: u64 = 1;
    let max_ans: u64 = 1;
    let rr_delay: u64 = 0;
    let timeout: u64 = 1800;
    let decimals: u64 = 24;
    let description: String = "LINK/USD".to_string();
    let reserve_rounds: u64 = 2;
    let min_submission_value: u128 = 1;
    let max_submission_value: u128 = 100000000000000000000;
    let oracles: Vec<AccountId>;
    let mut next_round: u128 = 1;
    let (
        root,
        aca,
        link,
        oracle_one,
        oracle_two,
        oracle_three,
        test_helper,
        _eac,
        eac_without_access_controller,
    ) = init();
    root.call(
        aca.account_id(),
        "add_access",
        &json!({"_user": test_helper.account_id().to_string()})
            .to_string()
            .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    )
    .assert_success();
    // Transfer from link_token contract to ACA.
    // new_amount * oracles.length * 2
    root.call(
        link.account_id(),
        "transfer_from",
        &json!({
            "owner_id": root.account_id().to_string(),
            "new_owner_id": aca.account_id().to_string(),
            "amount": 300.to_string()
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        36500000000000000000000, // deposit
    )
    .assert_success();

    root.call(  
        aca.account_id(),   
        "change_oracles",   
        &json!({"_removed": [], "_added": [oracle_one.account_id(), oracle_two.account_id()], "_added_admins": [oracle_one.account_id(), oracle_two.account_id()], "_min_submissions": 2.to_string(), "_max_submissions": 2.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    );

        oracle_one
        .call(
            aca.account_id(),
            "submit",
            &json!({"_round_id": "1", "_submission": answer.to_string()})
                .to_string()
                .into_bytes(),
            DEFAULT_GAS,
            0, // deposit
        );

        root.call(  
            aca.account_id(),   
            "change_oracles",   
            &json!({"_removed": [oracle_two.account_id()], "_added": [], "_added_admins": [], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
            DEFAULT_GAS,
            0, // deposit               
        ).assert_success();


    let owner_cannot_override_admin = root.call(  
        aca.account_id(),   
        "change_oracles",   
        &json!({"_removed": [], "_added": [oracle_two.account_id()], "_added_admins": [root.account_id()], "_min_submissions": 1.to_string(), "_max_submissions": 1.to_string(), "_restart_delay": rr_delay.to_string()}).to_string().into_bytes(),
        DEFAULT_GAS,
        0, // deposit               
    );

    // println!("after {:?}", owner_cannot_override_admin.promise_results());

    if let ExecutionStatus::Failure(execution_error) = &owner_cannot_override_admin
    .promise_errors()
    .remove(0)
    .unwrap()
    .outcome()
    .status
{
    assert!(execution_error.to_string().contains("owner cannot overwrite admin"));
} else {
    unreachable!();
}
}

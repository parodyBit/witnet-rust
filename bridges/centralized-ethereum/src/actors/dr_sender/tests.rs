use super::*;
use witnet_data_structures::chain::{RADAggregate, RADRequest, RADRetrieve, RADTally, RADType};

#[test]
fn deserialize_empty_dr() {
    // An empty data request is invalid with error 0xE0: BridgeMalformedRequest
    let err = deserialize_and_validate_dr_bytes(&[], 0, 1).unwrap_err();
    assert_eq!(err.encode_cbor(), vec![216, 39, 129, 24, 224]);
}

#[test]
fn deserialize_dr_not_protobuf() {
    // A malformed data request is invalid with error 0xE0: BridgeMalformedRequest
    let err = deserialize_and_validate_dr_bytes(&[1, 2, 3, 4], 0, 1).unwrap_err();
    assert_eq!(err.encode_cbor(), vec![216, 39, 129, 24, 224]);
}

fn example_request() -> RADRequest {
    RADRequest {
        retrieve: vec![RADRetrieve {
            url: "http://127.0.0.1:8000".to_string(),
            script: vec![128],
            kind: RADType::HttpGet,
            body: vec![],
            headers: vec![],
        }],
        aggregate: RADAggregate {
            filters: vec![],
            reducer: 3,
        },
        tally: RADTally {
            filters: vec![],
            reducer: 3,
        },
        time_lock: 0,
    }
}

#[test]
fn deserialize_dr_high_value() {
    // A minimal valid data request
    let dro = DataRequestOutput {
        witness_reward: 1_000_000_000,
        witnesses: 20,
        min_consensus_percentage: 51,
        data_request: example_request(),
        collateral: 1_000_000_000,
        ..Default::default()
    };
    // The cost of creating this data request is the reward (1_000_000) times the number of
    // witnesses (20)
    let total_value = dro.checked_total_value().unwrap();
    assert_eq!(total_value, 20_000_000_000);

    let dro_bytes = dro.to_pb_bytes().unwrap();
    // Setting the maximum allowed value to 1 nanowit below that will result in an error 0xE1:
    // BridgePoorIncentives
    let err =
        deserialize_and_validate_dr_bytes(&dro_bytes, 1_000_000_000, total_value - 1).unwrap_err();
    assert_eq!(err.encode_cbor(), vec![216, 39, 129, 24, 225]);
}

#[test]
fn deserialize_dr_collateral_one_nanowit() {
    // This data request will return an error because the collateral of 1 nanowit is smaller than
    // the collateral minimum of 1 wit
    // This will result in error 0xE0: BridgeMalformedRequest
    let dro = DataRequestOutput {
        witness_reward: 1_000_000,
        witnesses: 20,
        min_consensus_percentage: 51,
        data_request: example_request(),
        collateral: 1,
        ..Default::default()
    };
    // The cost of creating this data request is the reward (1_000_000) times the number of
    // witnesses (20)
    let total_value = dro.checked_total_value().unwrap();
    assert_eq!(total_value, 20_000_000);

    let dro_bytes = dro.to_pb_bytes().unwrap();
    let err = deserialize_and_validate_dr_bytes(&dro_bytes, 1, total_value).unwrap_err();
    assert_eq!(err.encode_cbor(), vec![216, 39, 129, 24, 224]);
}

#[test]
fn deserialize_dr_value_overflow() {
    // This data request will return an error when calling checked_total_value()
    // This will result in error 0xE0: BridgeMalformedRequest
    let dro = DataRequestOutput {
        witness_reward: u64::MAX,
        witnesses: 20,
        min_consensus_percentage: 51,
        data_request: example_request(),
        ..Default::default()
    };

    let dro_bytes = dro.to_pb_bytes().unwrap();
    let err = deserialize_and_validate_dr_bytes(&dro_bytes, 0, 1).unwrap_err();
    assert_eq!(err.encode_cbor(), vec![216, 39, 129, 24, 224]);
}

#[test]
fn deserialize_and_validate_dr_bytes_wip_0022() {
    // This data request will return an error when checking the reward to collateral ratio.
    // This will result in error 0xE0: BridgeMalformedRequest
    let dro = DataRequestOutput {
        collateral: 1_000_000_000,
        min_consensus_percentage: 51,
        witness_reward: 1,
        witnesses: 20,
        data_request: example_request(),
        ..Default::default()
    };

    let dro_bytes = dro.to_pb_bytes().unwrap();
    let witnet_dr_max_value_nanowits = 100_000_000_000;
    let err =
        deserialize_and_validate_dr_bytes(&dro_bytes, 1_000_000_000, witnet_dr_max_value_nanowits)
            .unwrap_err();
    assert_eq!(err.encode_cbor(), vec![216, 39, 129, 24, 224]);
}

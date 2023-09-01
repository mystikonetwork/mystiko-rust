extern crate mystiko_etherscan_client;

use mystiko_etherscan_client::{config::get_default_base_url, errors::EtherScanError};

#[test]
fn test_get_default_base_url() {
    assert!(get_default_base_url(1u64).is_ok());
    assert!(get_default_base_url(56u64).is_ok());
    assert!(get_default_base_url(137u64).is_ok());
    assert!(get_default_base_url(43114u64).is_ok());
    assert!(get_default_base_url(250u64).is_ok());
    assert!(get_default_base_url(1284u64).is_ok());
    assert!(get_default_base_url(8453u64).is_ok());
    assert!(get_default_base_url(10u64).is_ok());
    assert!(get_default_base_url(42161u64).is_ok());
    assert!(get_default_base_url(5u64).is_ok());
    assert!(get_default_base_url(97u64).is_ok());
    assert!(get_default_base_url(80001u64).is_ok());
    assert!(get_default_base_url(43113u64).is_ok());
    assert!(get_default_base_url(4002u64).is_ok());
    assert!(get_default_base_url(1287u64).is_ok());
    assert!(get_default_base_url(84531u64).is_ok());
    assert!(get_default_base_url(421613u64).is_ok());
    assert!(get_default_base_url(420u64).is_ok());
    let result = get_default_base_url(0);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(
        error.to_string(),
        EtherScanError::UnsupportedChainIdError(0).to_string()
    )
}

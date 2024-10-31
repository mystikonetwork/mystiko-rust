use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::config::bridge::v1::BridgeConfig;

#[tokio::test]
async fn test_axelar_bridge_config_to_proto() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let proto: BridgeConfig = mystiko_config
        .find_bridge(&mystiko_types::BridgeType::Axelar)
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(&proto.name, "Axelar Bridge");
    assert_eq!(proto.bridge_type(), BridgeType::Axelar);
}

#[tokio::test]
async fn test_celer_bridge_config_to_proto() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let proto: BridgeConfig = mystiko_config
        .find_bridge(&mystiko_types::BridgeType::Celer)
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(&proto.name, "Celer Bridge");
    assert_eq!(proto.bridge_type(), BridgeType::Celer);
}

#[tokio::test]
async fn test_layer_zero_bridge_config_to_proto() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let proto: BridgeConfig = mystiko_config
        .find_bridge(&mystiko_types::BridgeType::LayerZero)
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(&proto.name, "LayerZero Bridge");
    assert_eq!(proto.bridge_type(), BridgeType::LayerZero);
}

#[tokio::test]
async fn test_poly_bridge_config_to_proto() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let proto: BridgeConfig = mystiko_config
        .find_bridge(&mystiko_types::BridgeType::Poly)
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(&proto.name, "Poly Bridge");
    assert_eq!(proto.bridge_type(), BridgeType::Poly);
    assert_eq!(&proto.explorer_url.unwrap(), "https://explorer.poly.network");
    assert_eq!(&proto.explorer_prefix.unwrap(), "/tx/%tx%");
    assert_eq!(&proto.api_url.unwrap(), "https://explorer.poly.network");
    assert_eq!(&proto.api_prefix.unwrap(), "/api/v1/getcrosstx?txhash=%tx%");
}

#[tokio::test]
async fn test_tbridge_config_to_proto() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let proto: BridgeConfig = mystiko_config
        .find_bridge(&mystiko_types::BridgeType::Tbridge)
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(&proto.name, "Mystiko Testnet Bridge");
    assert_eq!(proto.bridge_type(), BridgeType::Tbridge);
}

#[tokio::test]
async fn test_wormhole_bridge_config_to_proto() {
    let mystiko_config = mystiko_config::MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let proto: BridgeConfig = mystiko_config
        .find_bridge(&mystiko_types::BridgeType::Wormhole)
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(&proto.name, "Wormhole Bridge");
    assert_eq!(proto.bridge_type(), BridgeType::Wormhole);
}

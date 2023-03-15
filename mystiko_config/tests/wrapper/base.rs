use mystiko_config::raw::base::RawConfig;
use mystiko_config::wrapper::base::BaseConfig;

#[test]
fn test_mutate() {
    let raw_config = RawConfig::default();
    let config: BaseConfig<RawConfig> = BaseConfig::new(raw_config.clone(), None);
    assert_eq!(config.mutate(None, None), config);
    assert_eq!(config.mutate(Some(raw_config), None), config);
}

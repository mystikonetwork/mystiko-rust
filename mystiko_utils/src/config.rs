use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::PathBuf;

pub fn load_config<T: Default + Serialize + DeserializeOwned>(
    paths: Vec<PathBuf>,
    env_prefix: &str,
) -> anyhow::Result<T> {
    let mut config_builder = config::Config::builder();
    config_builder = config_builder.add_source(config::File::from_str(
        &serde_json::to_string(&T::default())?,
        config::FileFormat::Json,
    ));

    for path in paths {
        config_builder =
            config_builder.add_source(config::File::from(path).required(true).format(config::FileFormat::Json));
    }

    config_builder = config_builder.add_source(
        config::Environment::with_prefix(env_prefix)
            .prefix_separator(".")
            .separator(".")
            .try_parsing(true),
    );
    let config = config_builder.build()?;
    let config = config.try_deserialize::<T>()?;
    Ok(config)
}

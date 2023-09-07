use config::FileFormat;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::path::Path;
use typed_builder::TypedBuilder;

const DEFAULT_FILE_FORMAT: FileFormat = FileFormat::Json;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ConfigLoadOptions<P: AsRef<Path>> {
    #[builder(default)]
    pub paths: Vec<ConfigFile<P>>,
    pub env_prefix: String,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ConfigFile<P: AsRef<Path>> {
    pub path: P,
    pub format: FileFormat,
}
impl<P> From<P> for ConfigFile<P>
where
    P: AsRef<Path>,
{
    fn from(value: P) -> Self {
        ConfigFile::builder().path(value).format(DEFAULT_FILE_FORMAT).build()
    }
}

impl<P> From<ConfigFile<P>> for Vec<ConfigFile<P>>
where
    P: AsRef<Path>,
{
    fn from(value: ConfigFile<P>) -> Self {
        vec![value]
    }
}

pub fn load_config<P, T>(options: &ConfigLoadOptions<P>) -> anyhow::Result<T>
where
    P: AsRef<Path>,
    T: Default + Serialize + DeserializeOwned,
{
    let mut config_builder = config::Config::builder();
    config_builder = config_builder.add_source(config::File::from_str(
        &serde_json::to_string(&T::default())?,
        config::FileFormat::Json,
    ));

    for file in options.paths.iter() {
        config_builder = config_builder.add_source(
            config::File::from(file.path.as_ref())
                .required(true)
                .format(file.format),
        );
    }

    config_builder = config_builder.add_source(
        config::Environment::with_prefix(&options.env_prefix)
            .prefix_separator(".")
            .separator(".")
            .try_parsing(true),
    );

    let config = config_builder.build()?;
    let config = config.try_deserialize::<T>()?;
    Ok(config)
}

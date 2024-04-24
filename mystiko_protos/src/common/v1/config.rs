use crate::common::v1::ConfigOptions;
use mystiko_config::MystikoConfigOptions;

pub trait ConfigOptionsOption {
    fn get_environment(&self) -> String;
    fn get_network(&self) -> String;
    fn get_git_revision(&self) -> Option<String>;
}

impl From<Option<ConfigOptions>> for ConfigOptions {
    fn from(options: Option<ConfigOptions>) -> Self {
        options.unwrap_or_default()
    }
}

impl From<ConfigOptions> for MystikoConfigOptions {
    fn from(value: ConfigOptions) -> Self {
        MystikoConfigOptions::builder()
            .is_testnet(value.is_testnet())
            .is_staging(value.is_staging())
            .file_path(value.file_path)
            .git_revision(value.git_revision)
            .remote_base_url(value.remote_base_url)
            .build()
    }
}

impl ConfigOptions {
    pub fn get_environment(&self) -> String {
        self.is_staging.map_or("production".into(), |s| {
            if s {
                "staging".into()
            } else {
                "production".into()
            }
        })
    }

    pub fn get_network(&self) -> String {
        self.is_testnet.map_or(
            "mainnet".into(),
            |s| {
                if s {
                    "testnet".into()
                } else {
                    "mainnet".into()
                }
            },
        )
    }

    pub fn get_git_revision(&self) -> Option<String> {
        self.git_revision.clone()
    }
}

impl ConfigOptionsOption for Option<ConfigOptions> {
    fn get_environment(&self) -> String {
        self.as_ref()
            .map_or("production".into(), |options| options.get_environment())
    }

    fn get_network(&self) -> String {
        self.as_ref().map_or("mainnet".into(), |options| options.get_network())
    }

    fn get_git_revision(&self) -> Option<String> {
        self.as_ref().and_then(|options| options.get_git_revision())
    }
}

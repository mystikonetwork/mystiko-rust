use crate::config::v1::ConfigOptions;

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

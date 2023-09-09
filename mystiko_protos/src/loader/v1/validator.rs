use crate::loader::v1::{RuleValidatorConfig, ValidatorConfig};

impl From<Option<ValidatorConfig>> for ValidatorConfig {
    fn from(opt: Option<ValidatorConfig>) -> ValidatorConfig {
        opt.unwrap_or_default()
    }
}

impl From<Option<RuleValidatorConfig>> for RuleValidatorConfig {
    fn from(opt: Option<RuleValidatorConfig>) -> RuleValidatorConfig {
        opt.unwrap_or_default()
    }
}

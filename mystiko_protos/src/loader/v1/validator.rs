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

impl RuleValidatorConfig {
    pub fn checkers(&self) -> Vec<i32> {
        let mut entries: Vec<_> = self.checkers.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        entries.into_iter().map(|(_, v)| *v).collect()
    }
}

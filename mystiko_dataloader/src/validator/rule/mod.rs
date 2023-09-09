use crate::data::LoadedData;
use std::sync::Arc;

mod checker;
mod error;
mod merger;
mod types;
mod validator;

use crate::handler::DataHandler;
pub use checker::*;
pub use error::*;
pub use merger::*;
use mystiko_ethers::Providers;
use mystiko_protos::loader::v1::RuleValidatorCheckerType;
pub use types::*;
pub use validator::*;

pub fn create_full_rule_validator<R, H, P>(
    handler: Arc<H>,
    providers: Arc<P>,
) -> RuleValidatorResult<RuleValidator<R, H>>
where
    R: LoadedData + 'static,
    H: DataHandler<R> + 'static,
    P: Providers + 'static,
{
    let checker_types = vec![
        RuleValidatorCheckerType::Integrity,
        RuleValidatorCheckerType::Sequence,
        RuleValidatorCheckerType::Counter,
        RuleValidatorCheckerType::Tree,
    ];
    create_rule_validator_by_types::<R, H, P>(handler, providers, &checker_types)
}

pub fn create_rule_validator_by_types<R, H, P>(
    handler: Arc<H>,
    providers: Arc<P>,
    checker_types: &[RuleValidatorCheckerType],
) -> RuleValidatorResult<RuleValidator<R, H>>
where
    R: LoadedData + 'static,
    H: DataHandler<R> + 'static,
    P: Providers + 'static,
{
    let mut checkers = vec![];
    for check_type in checker_types.iter() {
        match check_type {
            RuleValidatorCheckerType::Unspecified => {
                return Err(RuleValidatorError::RuleValidatorCheckerTypeError(*check_type as i32));
            }
            RuleValidatorCheckerType::Integrity => {
                let integrity_checker = IntegrityChecker::builder().build();
                checkers.push(Arc::new(Box::new(integrity_checker) as Box<dyn RuleChecker<R>>));
            }
            RuleValidatorCheckerType::Sequence => {
                let sequence_checker = SequenceChecker::builder().handler(handler.clone()).build();
                checkers.push(Arc::new(Box::new(sequence_checker) as Box<dyn RuleChecker<R>>));
            }
            RuleValidatorCheckerType::Counter => {
                let counter_checker = CounterChecker::<R, H, P>::builder()
                    .providers(providers.clone())
                    .handler(handler.clone())
                    .build();
                checkers.push(Arc::new(Box::new(counter_checker) as Box<dyn RuleChecker<R>>));
            }
            RuleValidatorCheckerType::Tree => {
                let merkle_tree_checker = MerkleTreeChecker::builder()
                    .providers(providers.clone())
                    .handler(handler.clone())
                    .build();
                checkers.push(Arc::new(Box::new(merkle_tree_checker) as Box<dyn RuleChecker<R>>));
            }
        }
    }

    let options = RuleValidatorOptions::builder()
        .handler(handler.clone())
        .checkers(checkers)
        .build();
    Ok(RuleValidator::new(&options))
}

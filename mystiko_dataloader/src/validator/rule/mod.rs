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
use mystiko_ethers::provider::pool::Providers;
pub use types::*;
pub use validator::*;

pub fn create_full_rule_validator<R, H, P>(handler: Arc<H>, providers: Arc<P>) -> RuleValidator<R, H>
where
    R: LoadedData + 'static,
    H: DataHandler<R> + 'static,
    P: Providers + 'static,
{
    let integrity_checker = IntegrityChecker::builder().build();
    let sequence_checker = SequenceChecker::builder().handler(handler.clone()).build();
    let counter_checker = CounterChecker::<R, H, P>::builder()
        .providers(providers.clone())
        .handler(handler.clone())
        .build();
    let merkle_tree_checker = MerkleTreeChecker::builder()
        .providers(providers.clone())
        .handler(handler.clone())
        .build();
    let checkers = vec![
        Arc::new(Box::new(integrity_checker) as Box<dyn RuleChecker<R>>),
        Arc::new(Box::new(sequence_checker) as Box<dyn RuleChecker<R>>),
        Arc::new(Box::new(counter_checker) as Box<dyn RuleChecker<R>>),
        Arc::new(Box::new(merkle_tree_checker) as Box<dyn RuleChecker<R>>),
    ];

    RuleValidator::<R, H>::new(
        &RuleValidatorOptions::<R, H>::builder()
            .handler(handler.clone())
            .checkers(checkers)
            .build(),
    )
}

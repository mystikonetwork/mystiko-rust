use serde::{Deserialize, Serialize};
use strum::EnumIter;
use validator::{Validate, ValidationErrors};

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone)]
pub enum BridgeType {
    Loop,
    Poly,
    Tbridge,
    Celer,
    LayerZero,
    Axelar,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ContractType {
    Deposit,
    Pool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AssetType {
    ERC20,
    Main,
}

#[derive(Serialize, Deserialize, EnumIter, Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum CircuitType {
    Rollup1,
    Rollup2,
    Rollup4,
    Rollup8,
    Rollup16,
    TRANSACTION1x0,
    TRANSACTION1x1,
    TRANSACTION1x2,
    TRANSACTION2x0,
    TRANSACTION2x1,
    TRANSACTION2x2,
}

pub fn validate_object<T>(object: T) -> Result<T, Vec<String>> where
    T: Validate
{
    let result = object.validate();
    match result {
        Ok(_) => { Ok(object) }
        Err(validation_errors) => {
            let mut errors = Vec::new();
            for (key, value) in validation_errors.errors().iter() {
                errors.push(format!("{}: {:?}", key, value))
            }
            Err(errors)
        }
    }
}
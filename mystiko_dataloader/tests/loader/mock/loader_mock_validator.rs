use async_trait::async_trait;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::LoadedData;
use mystiko_dataloader::data::{ChainResult, ContractResult};
use mystiko_dataloader::validator::{DataValidator, ValidateOption, ValidateResult};
use tokio::sync::RwLock;

pub struct MockValidator<R>
where
    R: LoadedData,
{
    all_success: RwLock<bool>,
    result: RwLock<ValidateResult>,
    _phantom: std::marker::PhantomData<R>,
}

impl<R> MockValidator<R>
where
    R: LoadedData + Clone,
{
    pub fn new() -> Self {
        MockValidator {
            all_success: RwLock::new(true),
            result: RwLock::new(ValidateResult::Err(anyhow::Error::msg("error".to_string()).into())),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn set_all_success(&self) {
        *self.all_success.write().await = true;
    }

    pub async fn set_result(&self, result: ValidateResult) {
        *self.all_success.write().await = false;
        *self.result.write().await = result;
    }
}

#[async_trait]
impl<R> DataValidator<R> for MockValidator<R>
where
    R: LoadedData,
{
    fn name(&self) -> &'static str {
        "mock_validator"
    }

    async fn validate(&self, data: &ChainData<R>, _option: &ValidateOption) -> ValidateResult {
        if *self.all_success.read().await {
            let contract_results = data
                .contracts_data
                .iter()
                .map(|d| {
                    ContractResult::builder()
                        .address(d.address.clone())
                        .result(Ok(()))
                        .build()
                })
                .collect::<Vec<_>>();

            Ok(ChainResult::builder()
                .chain_id(data.chain_id)
                .contract_results(contract_results)
                .build())
        } else {
            let result = self.result.read().await;
            match &*result {
                Ok(result) => {
                    let contract_results = result
                        .contract_results
                        .iter()
                        .map(|d| match d.result {
                            Ok(_) => ContractResult::builder()
                                .address(d.address.clone())
                                .result(Ok(()))
                                .build(),
                            Err(_) => ContractResult::builder()
                                .address(d.address.clone())
                                .result(Err(anyhow::Error::msg("error".to_string())))
                                .build(),
                        })
                        .collect::<Vec<_>>();

                    Ok(ChainResult::builder()
                        .chain_id(result.chain_id)
                        .contract_results(contract_results)
                        .build())
                }
                Err(_) => Err(anyhow::Error::msg("error".to_string()).into()),
            }
        }
    }
}

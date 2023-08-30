use crate::data::ContractData;
use crate::data::LoadedData;
use anyhow::Result;
use mystiko_protos::data::v1::ChainData as ProtoChainData;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ChainData<R: LoadedData> {
    pub chain_id: u64,
    #[builder(default)]
    pub contracts_data: Vec<ContractData<R>>,
}

impl<R> ChainData<R>
where
    R: LoadedData,
{
    pub fn from_proto(chain_id: u64, chain_data: ProtoChainData) -> Self {
        let contracts_data = chain_data
            .contract_data
            .into_iter()
            .map(|contract_data| {
                ContractData::<R>::from_proto(chain_data.start_block, chain_data.end_block, contract_data)
            })
            .collect::<Vec<_>>();
        Self::builder()
            .chain_id(chain_id)
            .contracts_data(contracts_data)
            .build()
    }

    pub fn into_proto(self) -> Result<Option<ProtoChainData>> {
        if let Some(first) = self.contracts_data.first() {
            let start_block = first.start_block;
            let end_block = first.end_block;
            if self
                .contracts_data
                .iter()
                .any(|other| other.start_block != start_block || other.end_block != end_block)
            {
                return Err(anyhow::anyhow!(
                    "all contracts data must have the same start and end blocks"
                ));
            }
            let contracts_data = self
                .contracts_data
                .into_iter()
                .map(|contract_data| contract_data.into_proto())
                .collect::<Result<Vec<_>>>()?;
            Ok(Some(
                ProtoChainData::builder()
                    .start_block(start_block)
                    .end_block(end_block)
                    .contract_data(contracts_data)
                    .build(),
            ))
        } else {
            Ok(None)
        }
    }
}

use crate::handler::spend::setup::{
    create_wallet, generate_commitments, setup, CommitmentOptions, DatabaseType, MockOptions, SpendsType,
};
use crate::handler::MockCommitmentPoolContracts;
use mystiko_config::{MystikoConfig, PoolContractConfig};
use mystiko_core::{Commitment, Spend, SpendColumn, SpendHandler};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::document::v1::{Account, Spend as ProtoSpend};
use mystiko_protos::core::handler::v1::CreateSpendOptions;
use mystiko_protos::core::v1::{SpendStatus, SpendType};
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_protos::storage::v1::SubFilter;
use mystiko_storage::{ColumnValues, Document};
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::{biguint_to_u256, number_to_biguint_decimal};
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[tokio::test]
async fn test_find() {
    let context = setup_context().await;
    let expected_spend = context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            12.0,
            Some(4.0),
        )
        .await;
    let filter = SubFilter::equal(SpendColumn::Amount, 12.0);
    let spend = context.handler.find(filter).await.unwrap().remove(0);
    assert_eq!(spend, expected_spend);
}

#[tokio::test]
async fn test_find_all() {
    let context = setup_context().await;
    assert!(context.handler.find_all().await.unwrap().is_empty());
    let spend = context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            12.0,
            Some(4.0),
        )
        .await;
    let spends = context.handler.find_all().await.unwrap();
    assert_eq!(spends, vec![spend]);
}

#[tokio::test]
async fn test_find_one() {
    let context = setup_context().await;
    let expected_spend = context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            12.0,
            Some(4.0),
        )
        .await;
    let filter = SubFilter::equal(SpendColumn::Amount, 12.0);
    let spend = context.handler.find_one(filter).await.unwrap();
    assert_eq!(spend, Some(expected_spend));
}

#[tokio::test]
async fn test_find_by_id() {
    let context = setup_context().await;
    let expected_spend = context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            12.0,
            Some(4.0),
        )
        .await;
    let spend = context.handler.find_by_id(expected_spend.id.clone()).await.unwrap();
    assert_eq!(spend, Some(expected_spend));
}

#[tokio::test]
async fn test_count() {
    let context = setup_context().await;
    let filter = SubFilter::equal(SpendColumn::Amount, 12.0);
    assert_eq!(context.handler.count(filter.clone()).await.unwrap(), 0);
    context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            12.0,
            Some(4.0),
        )
        .await;
    assert_eq!(context.handler.count(filter).await.unwrap(), 1);
}

#[tokio::test]
async fn test_count_all() {
    let context = setup_context().await;
    assert_eq!(context.handler.count_all().await.unwrap(), 0);
    context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            12.0,
            Some(4.0),
        )
        .await;
    assert_eq!(context.handler.count_all().await.unwrap(), 1);
}

#[tokio::test]
async fn test_update() {
    let context = setup_context().await;
    let expected_spend = context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            12.0,
            Some(4.0),
        )
        .await;
    let mut spend = context
        .handler
        .find_by_id(expected_spend.id.clone())
        .await
        .unwrap()
        .unwrap();
    spend.status = SpendStatus::Succeeded as i32;
    let spend = context.handler.update(spend.clone()).await.unwrap();
    let updated_spend = context
        .handler
        .find_by_id(expected_spend.id.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated_spend, spend);
}

#[tokio::test]
async fn test_update_batch() {
    let context = setup_context().await;
    let expected_spend = context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            12.0,
            Some(4.0),
        )
        .await;
    let mut spend = context
        .handler
        .find_by_id(expected_spend.id.clone())
        .await
        .unwrap()
        .unwrap();
    spend.status = SpendStatus::Succeeded as i32;
    let spend = context.handler.update_batch(vec![spend.clone()]).await.unwrap();
    let updated_spend = context
        .handler
        .find_by_id(expected_spend.id.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated_spend, spend[0]);
}

#[tokio::test]
async fn test_update_by_filter() {
    let context = setup_context().await;
    let expected_spend = context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            12.0,
            Some(4.0),
        )
        .await;
    context
        .handler
        .find_by_id(expected_spend.id.clone())
        .await
        .unwrap()
        .unwrap();
    let column_values = ColumnValues::new().set_value(SpendColumn::Status, SpendStatus::Succeeded as i32);
    context
        .handler
        .update_by_filter(column_values, SubFilter::equal(SpendColumn::Amount, 12.0))
        .await
        .unwrap();
    let updated_spend = context
        .handler
        .find_by_id(expected_spend.id.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated_spend.status, SpendStatus::Succeeded as i32);
}

#[tokio::test]
async fn test_update_all() {
    let context = setup_context().await;
    let expected_spend = context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            12.0,
            Some(4.0),
        )
        .await;
    context
        .handler
        .find_by_id(expected_spend.id.clone())
        .await
        .unwrap()
        .unwrap();
    let column_values = ColumnValues::new().set_value(SpendColumn::Status, SpendStatus::Succeeded as i32);
    context.handler.update_all(column_values).await.unwrap();
    let updated_spend = context
        .handler
        .find_by_id(expected_spend.id.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated_spend.status, SpendStatus::Succeeded as i32);
}

#[tokio::test]
async fn test_delete() {
    let context = setup_context().await;
    let expected_spend = context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            12.0,
            Some(4.0),
        )
        .await;
    context
        .handler
        .find_by_id(expected_spend.id.clone())
        .await
        .unwrap()
        .unwrap();
    context.handler.delete(expected_spend.clone()).await.unwrap();
    let spend = context.handler.find_by_id(expected_spend.id.clone()).await.unwrap();
    assert!(spend.is_none());
}

#[tokio::test]
async fn test_delete_batch() {
    let context = setup_context().await;
    let expected_spend = context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            12.0,
            Some(4.0),
        )
        .await;
    let expected_spend2 = context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            11.0,
            Some(4.0),
        )
        .await;
    context
        .handler
        .find_by_id(expected_spend.id.clone())
        .await
        .unwrap()
        .unwrap();
    context
        .handler
        .find_by_id(expected_spend2.id.clone())
        .await
        .unwrap()
        .unwrap();
    context
        .handler
        .delete_batch(vec![expected_spend.clone(), expected_spend2.clone()])
        .await
        .unwrap();
    let spend = context.handler.find_by_id(expected_spend.id.clone()).await.unwrap();
    assert!(spend.is_none());
    let spend = context.handler.find_by_id(expected_spend2.id.clone()).await.unwrap();
    assert!(spend.is_none());
}

#[tokio::test]
async fn test_delete_by_filter() {
    let context = setup_context().await;
    let expected_spend = context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            12.0,
            Some(4.0),
        )
        .await;
    let expected_spend2 = context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            11.0,
            Some(4.0),
        )
        .await;
    context
        .handler
        .find_by_id(expected_spend.id.clone())
        .await
        .unwrap()
        .unwrap();
    context
        .handler
        .find_by_id(expected_spend2.id.clone())
        .await
        .unwrap()
        .unwrap();
    context
        .handler
        .delete_by_filter(SubFilter::equal(SpendColumn::Amount, 11.0))
        .await
        .unwrap();
    let spend = context.handler.find_by_id(expected_spend.id.clone()).await.unwrap();
    assert!(spend.is_some());
    let spend = context.handler.find_by_id(expected_spend2.id.clone()).await.unwrap();
    assert!(spend.is_none());
}

#[tokio::test]
async fn test_delete_all() {
    let context = setup_context().await;
    let expected_spend = context
        .create_spend(
            SpendType::Withdraw,
            "0xFE500c274F72f1d1a9978c903d97E6d45CD9121B",
            12.0,
            Some(4.0),
        )
        .await;
    context.handler.delete_all().await.unwrap();
    let spend = context.handler.find_by_id(expected_spend.id.clone()).await.unwrap();
    assert!(spend.is_none());
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct BasicTestOptions {
    chain_id: u64,
    contract_address: String,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct BasicTestContext {
    db: Arc<DatabaseType>,
    handler: SpendsType,
    account: Account,
    chain_id: u64,
    contract_config: PoolContractConfig,
}

impl BasicTestContext {
    async fn new(options: BasicTestOptions) -> Self {
        let BasicTestOptions {
            chain_id,
            contract_address,
        } = options;
        let contract_ethereum_address = ethers_address_from_string(&contract_address).unwrap();
        let config = MystikoConfig::from_json_file("tests/files/mystiko/config.json")
            .await
            .unwrap();
        let contract_config = config
            .find_pool_contract_by_address(chain_id, &contract_address)
            .unwrap()
            .clone();
        let min_rollup_fee = biguint_to_u256(&contract_config.min_rollup_fee().unwrap());
        let mut commitment_pool_contracts = MockCommitmentPoolContracts::new();
        commitment_pool_contracts
            .expect_min_rollup_fee()
            .withf(move |options| options.chain_id == chain_id && options.contract_address == contract_ethereum_address)
            .returning(move |_| Ok(min_rollup_fee));
        let mock_options = MockOptions::builder()
            .config(config)
            .commitment_pool_contracts(commitment_pool_contracts)
            .build();
        let (_, db, handler) = setup(mock_options).await;
        let (_, account) = create_wallet(db.clone()).await;
        Self {
            db,
            handler,
            account,
            chain_id,
            contract_config,
        }
    }

    async fn create_spend(
        &self,
        spend_type: SpendType,
        recipient: &str,
        amount: f64,
        rollup_fee_amount: Option<f64>,
    ) -> ProtoSpend {
        let bridge_type: BridgeType = self.contract_config.bridge_type().into();
        let create_options = CreateSpendOptions::builder()
            .chain_id(self.chain_id)
            .asset_symbol(self.contract_config.asset_symbol())
            .bridge_type(bridge_type as i32)
            .spend_type(spend_type as i32)
            .amount(amount)
            .rollup_fee_amount(rollup_fee_amount)
            .recipient(recipient)
            .wallet_password("P@ssw0rd")
            .build();
        let spend = self.handler.create(create_options).await.unwrap();
        let mut other_spend = Spend::document_from_proto(spend.clone()).unwrap();
        other_spend.data.wallet_id = "other_wallet_id".to_string();
        self.db.spends.insert(&other_spend.data).await.unwrap();
        spend
    }

    async fn generate_commitments(&self, amounts: &[f64]) -> Vec<Document<Commitment>> {
        let options = amounts
            .iter()
            .map(|amount| {
                CommitmentOptions::builder()
                    .status(CommitmentStatus::Included)
                    .shielded_address(self.account.shielded_address.clone())
                    .amount(number_to_biguint_decimal(*amount, Some(self.contract_config.asset_decimals())).unwrap())
                    .build()
            })
            .collect::<Vec<_>>();
        generate_commitments(self.db.clone(), self.chain_id, &self.contract_config, &options).await
    }
}

async fn setup_context() -> BasicTestContext {
    let chain_id = 5_u64;
    let contract_address = "0x223903804Ee95e264F74C88B4F8583429524593c".to_string();
    let test_options = BasicTestOptions::builder()
        .chain_id(chain_id)
        .contract_address(contract_address.clone())
        .build();
    let context = BasicTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    context
}
